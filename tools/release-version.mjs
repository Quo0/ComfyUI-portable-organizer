// Raises the version in the five places that carry it, and renames the
// CHANGELOG.md «Unreleased» section to the released one.
//
// Only tauri.conf.json is checked by the workflow — the other four are hunted
// by eye afterwards, which is exactly why they drift. Here they move together
// or not at all: the script refuses to write anything if the five do not agree
// on the current version, because a disagreement means the previous release was
// left half-raised and guessing which number is the real one is how a wrong
// version reaches a user.
//
// The text of the section is not written here. It is read by whoever is
// deciding whether to install the update; a generated line would tell them
// nothing.
//
// Usage:
//   node tools/release-version.mjs 0.2.0        an exact version
//   node tools/release-version.mjs minor        patch | minor | major
//   node tools/release-version.mjs 0.2.0 --date 2026-09-04
//   node tools/release-version.mjs --check      read-only: do the five agree
//                                               and is the section in place

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

/**
 * The five places, each with the way its version is found. A regexp rather than
 * a parser even for the JSON files: rewriting them through JSON.stringify would
 * reformat the whole file and bury the one changed line in the diff.
 *
 * Every pattern captures the version in group 2, with the text around it in
 * groups 1 and 3, so the replacement puts back exactly what it took.
 */
const FILES = [
  {
    path: 'apps/desktop/src-tauri/tauri.conf.json',
    note: 'the single source — the workflow checks the tag against it',
    pattern: /^(\s*"version":\s*")([^"]+)(")/m,
  },
  {
    path: 'apps/desktop/src-tauri/Cargo.toml',
    pattern: /^(version = ")([^"]+)(")/m,
  },
  {
    // The lock file holds versions of every dependency as well, so the entry is
    // anchored on the package name rather than on the first match. The line
    // break is `\r?\n` and not `\n`: git checks this repository out with CRLF,
    // and a pattern that only knows LF stops matching the moment the file comes
    // back from `git restore` — silently, looking like a changed file format.
    path: 'apps/desktop/src-tauri/Cargo.lock',
    pattern: /^(name = "cpo-desktop"\r?\nversion = ")([^"]+)(")/m,
  },
  {
    path: 'apps/desktop/package.json',
    pattern: /^(\s*"version":\s*")([^"]+)(")/m,
  },
  {
    path: 'package.json',
    pattern: /^(\s*"version":\s*")([^"]+)(")/m,
  },
];

const CHANGELOG = 'CHANGELOG.md';

const args = process.argv.slice(2);
const check = args.includes('--check');
const dateArg = takeOption('--date');
const target = args.find((a) => !a.startsWith('--'));

if (!check && !target) {
  fail(
    'the first argument must be the version or patch|minor|major, ' +
      'for example: node tools/release-version.mjs 0.2.0',
  );
}

const current = readCurrent();

if (check) {
  runCheck();
} else {
  runBump();
}

/** Read-only: the state a release starts from. */
function runCheck() {
  console.log(`Current version: ${current}`);
  for (const file of FILES) console.log(`  ok  ${file.path}`);

  const section = findSection(readFileSync(join(root, CHANGELOG), 'utf8'), current);

  if (!section) {
    console.log(`  --  ${CHANGELOG}: no «## ${current}» section yet`);
    console.log('\nThe version is consistent; the CHANGELOG section is written at release time.');
    return;
  }

  if (!section.body) fail(`the «## ${current}» section in ${CHANGELOG} is empty`);

  console.log(`  ok  ${CHANGELOG}: «${section.heading}», ${section.body.split('\n').length} lines`);
  console.log(`\nReady for the tag: git tag v${current} && git push origin v${current}`);
}

/** Raise the version everywhere and close the CHANGELOG section. */
function runBump() {
  const next = resolveTarget(target, current);

  if (compare(next, current) <= 0) {
    fail(`${next} is not higher than the current ${current}`);
  }

  const date = dateArg ?? today();
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date)) fail(`--date expects YYYY-MM-DD, got ${date}`);

  // The CHANGELOG is prepared first: it is the only step that can refuse for a
  // reason the script cannot fix, and refusing after four files are already
  // written leaves the repository in the state this script exists to prevent.
  const changelogPath = join(root, CHANGELOG);
  const changelog = readFileSync(changelogPath, 'utf8');
  const heading = `## ${next} — ${date}`;

  const unreleased = findSection(changelog, 'Unreleased');
  const already = findSection(changelog, next);
  let updatedChangelog = null;

  if (unreleased) {
    if (!unreleased.body) {
      fail(
        `the «${unreleased.heading}» section in ${CHANGELOG} is empty. ` +
          'An empty section fails the release the same way a missing one does — ' +
          'write what changed for the people deciding whether to install it.',
      );
    }
    updatedChangelog = changelog.replace(unreleased.heading, heading);
  } else if (already) {
    if (!already.body) fail(`the «${already.heading}» section in ${CHANGELOG} is empty`);
    console.log(`${CHANGELOG}: «${already.heading}» is already in place, left as it is`);
  } else {
    fail(
      `${CHANGELOG} has neither «## Unreleased» nor «## ${next}». ` +
        'Write the section first — its text is what the update panel shows.',
    );
  }

  for (const file of FILES) {
    const full = join(root, file.path);
    const text = readFileSync(full, 'utf8');
    writeFileSync(full, text.replace(file.pattern, `$1${next}$3`), 'utf8');
    console.log(`  ${current} → ${next}  ${file.path}`);
  }

  if (updatedChangelog !== null) {
    writeFileSync(changelogPath, updatedChangelog, 'utf8');
    console.log(`  Unreleased → ${next}  ${CHANGELOG}`);
  }

  console.log(`\nNext: node tools/release-notes.mjs v${next}  — see what becomes the release body`);
  console.log('      pnpm ui-design:check && pnpm i18n:check && pnpm typecheck');
  console.log(`      commit, push, then git tag v${next} && git push origin v${next}`);
}

/**
 * The current version, from all five at once. A single file could be read
 * faster, but then the drift this script exists to catch would go unnoticed
 * until someone read the diff.
 */
function readCurrent() {
  const found = FILES.map((file) => {
    const text = readFileSync(join(root, file.path), 'utf8');
    const match = text.match(file.pattern);
    if (!match) fail(`no version found in ${file.path} — the file's format has changed`);
    return { path: file.path, version: match[2] };
  });

  const versions = [...new Set(found.map((f) => f.version))];

  if (versions.length > 1) {
    const list = found.map((f) => `  ${f.version.padEnd(12)} ${f.path}`).join('\n');
    fail(
      `the five places disagree on the current version:\n${list}\n` +
        'Bring them together by hand before raising anything.',
    );
  }

  return versions[0];
}

/** `patch` | `minor` | `major` | an exact version → the version to write. */
function resolveTarget(value, from) {
  const steps = { major: 0, minor: 1, patch: 2 };

  if (value in steps) {
    const [core] = from.split('-');
    const parts = core.split('.').map(Number);
    if (parts.length !== 3 || parts.some(Number.isNaN)) {
      fail(`cannot step from ${from}: it is not a X.Y.Z version`);
    }
    // A prerelease steps to its own release: 0.2.0-beta.1 + patch is 0.2.0, not
    // 0.2.1 — the number the prerelease was preparing has not gone out yet.
    if (from.includes('-') && value === 'patch') return core;
    const index = steps[value];
    parts[index] += 1;
    for (let i = index + 1; i < 3; i += 1) parts[i] = 0;
    return parts.join('.');
  }

  if (!/^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test(value)) {
    fail(`«${value}» is neither a version nor patch|minor|major`);
  }

  return value;
}

/** Semantic comparison, prereleases included: 0.2.0-beta.1 < 0.2.0. */
function compare(a, b) {
  const [coreA, preA] = a.split(/-(.+)/);
  const [coreB, preB] = b.split(/-(.+)/);
  const numsA = coreA.split('.').map(Number);
  const numsB = coreB.split('.').map(Number);

  for (let i = 0; i < 3; i += 1) {
    if (numsA[i] !== numsB[i]) return numsA[i] - numsB[i];
  }

  if (preA && !preB) return -1;
  if (!preA && preB) return 1;
  if (!preA && !preB) return 0;
  return preA < preB ? -1 : preA > preB ? 1 : 0;
}

/**
 * A section of the CHANGELOG by heading: from `## <name>` to the next heading
 * of the same level. The name is matched at the start of the heading so that
 * the date after the version does not have to be predicted.
 */
function findSection(text, name) {
  const lines = text.split(/\r?\n/);
  const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const from = lines.findIndex((line) => new RegExp(`^##\\s+${escaped}(\\s|$)`).test(line));

  if (from === -1) return null;

  const rest = lines.slice(from + 1);
  const to = rest.findIndex((line) => line.startsWith('## '));

  return {
    heading: lines[from],
    body: (to === -1 ? rest : rest.slice(0, to)).join('\n').trim(),
  };
}

function takeOption(name) {
  const index = args.indexOf(name);
  if (index === -1) return null;
  const value = args[index + 1];
  if (!value || value.startsWith('--')) fail(`${name} expects a value`);
  args.splice(index, 2);
  return value;
}

/** Local date: the release is dated by the day it was cut, not by UTC. */
function today() {
  const now = new Date();
  const pad = (n) => String(n).padStart(2, '0');
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
}

function fail(message) {
  console.error(`Release stopped: ${message}`);
  process.exit(1);
}
