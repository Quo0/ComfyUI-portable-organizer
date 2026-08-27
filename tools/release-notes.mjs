// Checks the tag against the app version and cuts out the CHANGELOG.md section.
//
// Both refusals must happen before the build, not after: a published release
// where the tag says one thing and the app shows another can only be fixed with
// a new release, and there is no way left to tell what a user has installed.
// A release with no description of the changes is not fixed quickly either, and
// a description backfilled after the fact is one nobody will read.
//
// Usage:
//   node tools/release-notes.mjs v0.2.0 [release-notes.md]
//
// Without the second argument the section is printed to stdout.

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

const tag = process.argv[2];
const out = process.argv[3];

if (!tag) fail('the first argument must be the tag, for example v0.2.0');

const config = JSON.parse(
  readFileSync(join(root, 'apps', 'desktop', 'src-tauri', 'tauri.conf.json'), 'utf8'),
);

// The tag without the `v` is the version. A hyphen in it means a prerelease
// (`v0.2.0-beta.1`), and that does not affect the comparison with the app
// version: Tauri's version is semantic too and keeps the suffix.
const version = tag.replace(/^v/, '');

if (config.version !== version) {
  fail(
    `tag ${tag} does not match the app version: ` +
      `tauri.conf.json says ${config.version}`,
  );
}

const changelog = readFileSync(join(root, 'CHANGELOG.md'), 'utf8');

/**
 * The version's section: from its heading to the next heading of the same level.
 *
 * The heading is matched by start of line and version number rather than by an
 * exact full match: a date usually follows the number, and demanding a format
 * for it is one more way to fail a release for nothing.
 */
const lines = changelog.split(/\r?\n/);
const from = lines.findIndex((line) =>
  new RegExp(`^## \\s*${version.replace(/\./g, '\\.')}(\\s|$)`).test(line),
);

if (from === -1) {
  fail(
    `CHANGELOG.md has no «## ${version}» section. ` +
      'Rename «Unreleased» to it before tagging.',
  );
}

const rest = lines.slice(from + 1);
const to = rest.findIndex((line) => line.startsWith('## '));
const body = (to === -1 ? rest : rest.slice(0, to)).join('\n').trim();

if (!body) fail(`the «## ${version}» section in CHANGELOG.md is empty`);

if (out) {
  writeFileSync(join(root, out), `${body}\n`, 'utf8');
  console.log(`Section ${version} written to ${out}`);
} else {
  console.log(body);
}

function fail(message) {
  console.error(`Release stopped: ${message}`);
  process.exit(1);
}
