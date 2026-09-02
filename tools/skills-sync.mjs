#!/usr/bin/env node
// Copies skills from .agents/skills/ into .claude/skills/.
//
// The source of truth is .agents/skills/ — the vendor-neutral location, so a
// second agent tool reads the same folder instead of its own copy. Claude Code
// scans only .claude/skills/, ~/.claude/skills/ and plugins, so what it needs
// has to physically be there.
//
// Copies, not symlinks. This repository used to hold three skills as Windows
// junctions, and the junctions were never in git at all: the index carried both
// trees as real files, so a fresh clone got two independent copies that drifted
// apart with nothing to notice it. Committing symlinks instead does not fix it
// — core.symlinks is off on Windows unless the machine has Developer Mode, and
// git checks such an entry out as a text file containing the target path. The
// skill then loads as garbage rather than failing loudly.
//
// Runs from the root `prepare` script, so a fresh clone gets its skills on the
// first `pnpm install`. Like tools/hooks/install.mjs, it never fails the
// install: a missing skill is an inconvenience, a dead install is a broken
// clone. `--check` is the opposite — it reports drift and exits non-zero.

import { cpSync, existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, rmdirSync, rmSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const source = join(root, '.agents', 'skills');
const target = join(root, '.claude', 'skills');

const check = process.argv.includes('--check');

/** Skill folders: a directory with a SKILL.md is a skill, anything else is not. */
function skillsIn(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir, { withFileTypes: true })
    .filter((e) => e.isDirectory() && existsSync(join(dir, e.name, 'SKILL.md')))
    .map((e) => e.name)
    .sort();
}

/** Every file of a skill, relative to its folder, with forward slashes. */
function filesIn(dir) {
  return readdirSync(dir, { recursive: true, withFileTypes: true })
    .filter((e) => e.isFile())
    .map((e) => `${relative(dir, join(e.parentPath, e.name)).replaceAll('\\', '/')}`)
    .sort();
}

/**
 * Remove a stale entry under .claude/skills/. A leftover junction from an older
 * checkout must be unlinked, not walked into: rmSync with recursive would
 * delete the source through it.
 */
function drop(path) {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink()) rmdirSync(path);
  else if (stat.isDirectory()) rmSync(path, { recursive: true });
  else rmSync(path);
}

/** Same name, same files, same bytes — nothing to copy. */
function matches(from, to) {
  if (!existsSync(to) || lstatSync(to).isSymbolicLink()) return false;
  const [a, b] = [filesIn(from), filesIn(to)];
  if (a.length !== b.length || a.some((f, i) => f !== b[i])) return false;
  return a.every((f) => readFileSync(join(from, f)).equals(readFileSync(join(to, f))));
}

function main() {
  const skills = skillsIn(source);
  if (skills.length === 0) {
    console.warn('[skills] .agents/skills/ has no skills — nothing to sync.');
    return 0;
  }

  const stale = existsSync(target)
    ? readdirSync(target).filter((name) => !skills.includes(name))
    : [];
  const outdated = skills.filter((name) => !matches(join(source, name), join(target, name)));

  if (check) {
    if (stale.length === 0 && outdated.length === 0) {
      console.log(`[skills] .claude/skills/ matches the source, ${skills.length} skills.`);
      return 0;
    }
    if (outdated.length > 0) console.error(`[skills] out of date: ${outdated.join(', ')}`);
    if (stale.length > 0) console.error(`[skills] no longer in the source: ${stale.join(', ')}`);
    console.error('[skills] run `pnpm skills:sync`.');
    return 1;
  }

  mkdirSync(target, { recursive: true });
  for (const name of stale) drop(join(target, name));
  for (const name of outdated) {
    const to = join(target, name);
    if (existsSync(to)) drop(to);
    cpSync(join(source, name), to, { recursive: true });
  }

  const changed = [...outdated, ...stale.map((n) => `-${n}`)];
  console.log(
    changed.length === 0
      ? `[skills] ${skills.length} skills already in place.`
      : `[skills] ${skills.length} skills in .claude/skills/, updated: ${changed.join(', ')}.` +
          ' Claude Code reads skills at session start — restart to pick them up.',
  );
  return 0;
}

try {
  process.exit(main());
} catch (error) {
  // In --check the failure is the answer; in a sync it must not kill the install.
  console.warn(`[skills] sync failed: ${error.message}`);
  process.exit(check ? 1 : 0);
}
