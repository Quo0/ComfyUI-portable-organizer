#!/usr/bin/env node
// Copies the .agents/ tree into .claude/ for Claude Code.
//
// The source of truth is .agents/ — the vendor-neutral location, so a second
// agent tool reads the same folders instead of its own copy. Claude Code looks
// only in .claude/skills/ and .claude/agents/ (plus the personal ~/.claude and
// plugins), so what it needs has to physically be there.
//
// Copies, not symlinks. This repository used to hold three skills as Windows
// junctions, and the junctions were never in git at all: the index carried both
// trees as real files, so a fresh clone got two independent copies that drifted
// apart with nothing to notice it. Committing symlinks instead does not fix it
// — core.symlinks is off on Windows unless the machine has Developer Mode, and
// git checks such an entry out as a text file containing the target path. The
// skill then loads as garbage rather than failing loudly.
//
// Runs from the root `prepare` script, so a fresh clone gets its skills and
// agents on the first `pnpm install`. Like tools/hooks/install.mjs, it never
// fails the install: a missing skill is an inconvenience, a dead install is a
// broken clone. `--check` is the opposite — it reports drift and exits non-zero.

import {
  cpSync,
  existsSync,
  lstatSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  rmdirSync,
  rmSync,
} from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

/**
 * The two kinds differ in shape, so each says what counts as one of its items:
 * a skill is a folder with a SKILL.md inside, a subagent is a single .md file.
 * Anything else in the folder is not ours and is not carried over.
 */
const KINDS = [
  {
    name: 'skills',
    isItem: (dir, entry) => entry.isDirectory() && existsSync(join(dir, entry.name, 'SKILL.md')),
  },
  {
    name: 'agents',
    isItem: (_dir, entry) => entry.isFile() && entry.name.endsWith('.md'),
  },
];

const check = process.argv.includes('--check');

function itemsIn(dir, isItem) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir, { withFileTypes: true })
    .filter((entry) => isItem(dir, entry))
    .map((entry) => entry.name)
    .sort();
}

/** Every file of a folder item, relative to it, with forward slashes. */
function filesIn(dir) {
  return readdirSync(dir, { recursive: true, withFileTypes: true })
    .filter((entry) => entry.isFile())
    .map((entry) => relative(dir, join(entry.parentPath, entry.name)).replaceAll('\\', '/'))
    .sort();
}

/**
 * Remove a stale entry under .claude/. A leftover junction from an older
 * checkout must be unlinked, not walked into: rmSync with recursive would
 * delete the source through it.
 */
function drop(path) {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink()) rmdirSync(path);
  else if (stat.isDirectory()) rmSync(path, { recursive: true });
  else rmSync(path);
}

const sameBytes = (a, b) => readFileSync(a).equals(readFileSync(b));

/** Same shape, same files, same bytes — nothing to copy. */
function matches(from, to) {
  if (!existsSync(to) || lstatSync(to).isSymbolicLink()) return false;
  if (lstatSync(from).isFile()) return lstatSync(to).isFile() && sameBytes(from, to);
  if (!lstatSync(to).isDirectory()) return false;
  const [a, b] = [filesIn(from), filesIn(to)];
  if (a.length !== b.length || a.some((file, i) => file !== b[i])) return false;
  return a.every((file) => sameBytes(join(from, file), join(to, file)));
}

/**
 * One kind. Returns what drifted; an empty source is left alone rather than
 * treated as "delete everything" — a source that failed to check out must not
 * wipe a working copy.
 */
function plan({ name, isItem }) {
  const source = join(root, '.agents', name);
  const target = join(root, '.claude', name);
  const items = itemsIn(source, isItem);

  if (items.length === 0) {
    return { name, source, target, items, outdated: [], stale: [], empty: true };
  }

  return {
    name,
    source,
    target,
    items,
    outdated: items.filter((item) => !matches(join(source, item), join(target, item))),
    stale: existsSync(target) ? readdirSync(target).filter((entry) => !items.includes(entry)) : [],
    empty: false,
  };
}

function main() {
  const plans = KINDS.map(plan);

  for (const { name, empty } of plans) {
    if (empty) console.warn(`[agents] .agents/${name}/ is empty — leaving .claude/${name}/ alone.`);
  }

  const drifted = plans.filter((p) => p.outdated.length > 0 || p.stale.length > 0);

  if (check) {
    if (drifted.length === 0) {
      const counted = plans.map((p) => `${p.items.length} ${p.name}`).join(', ');
      console.log(`[agents] .claude/ matches the source: ${counted}.`);
      return 0;
    }
    for (const { name, outdated, stale } of drifted) {
      if (outdated.length > 0) console.error(`[agents] ${name} out of date: ${outdated.join(', ')}`);
      if (stale.length > 0) {
        console.error(`[agents] ${name} no longer in the source: ${stale.join(', ')}`);
      }
    }
    console.error('[agents] run `pnpm agents:sync`.');
    return 1;
  }

  for (const { source, target, outdated, stale } of drifted) {
    mkdirSync(target, { recursive: true });
    for (const item of stale) drop(join(target, item));
    for (const item of outdated) {
      const to = join(target, item);
      if (existsSync(to)) drop(to);
      cpSync(join(source, item), to, { recursive: true });
    }
  }

  const counted = plans.map((p) => `${p.items.length} ${p.name}`).join(', ');
  if (drifted.length === 0) {
    console.log(`[agents] already in place: ${counted}.`);
    return 0;
  }

  const changed = drifted
    .map((p) => `${p.name}: ${[...p.outdated, ...p.stale.map((s) => `-${s}`)].join(', ')}`)
    .join('; ');
  console.log(
    `[agents] .claude/ updated — ${changed}. Now ${counted}. Claude Code reads` +
      ' skills and agents at session start; restart to pick them up.',
  );
  return 0;
}

try {
  process.exit(main());
} catch (error) {
  // In --check the failure is the answer; in a sync it must not kill the install.
  console.warn(`[agents] sync failed: ${error.message}`);
  process.exit(check ? 1 : 0);
}
