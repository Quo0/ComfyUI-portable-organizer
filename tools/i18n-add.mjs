// Adds a UI key to all four locales at once.
//
// Without this, adding one string means four edits in four files, and a lost
// translation surfaces only through `i18n:check` — or by eye, in a locale
// nobody reads. The script makes the operation atomic: either all four files or
// none of them.
//
// Key order is preserved: a new key is appended at the end of its own group,
// the way a hand would do it. Otherwise the diff would turn into a reshuffle of
// the whole file.
//
// Usage:
//   node tools/i18n-add.mjs install.run.preparing \
//     --en "Checking the folders…" --ru "Проверяем папки…" \
//     --es "Comprobando…" --zh "正在检查…"
//
//   node tools/i18n-add.mjs --file keys.json
//   where keys.json: { "install.run.preparing": { "en": "…", "ru": "…", … } }

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const dir = join(root, 'apps', 'desktop', 'src', 'i18n', 'locales');

/** The `--zh` flag is shorter than the file name `zh-Hans`, which is tedious to
 *  type every time. */
const LOCALES = { en: 'en', ru: 'ru', es: 'es', zh: 'zh-Hans' };

const args = process.argv.slice(2);
if (args.length === 0 || args.includes('--help')) {
  console.log(readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n\n')[0]);
  process.exit(args.length === 0 ? 1 : 0);
}

const force = args.includes('--force');

/** Parses either a single key with flags, or a file with a batch of keys. */
function parseInput() {
  const fileAt = args.indexOf('--file');
  if (fileAt !== -1) {
    const path = args[fileAt + 1];
    if (!path) fail('--file without a file path');
    return JSON.parse(readFileSync(path, 'utf8'));
  }

  const key = args[0];
  if (key.startsWith('--')) fail('the first argument must be a key');

  const values = {};
  for (const short of Object.keys(LOCALES)) {
    const at = args.indexOf(`--${short}`);
    if (at === -1) fail(`no --${short} translation`);
    const value = args[at + 1];
    if (value === undefined || value.startsWith('--')) fail(`empty --${short} value`);
    values[short] = value;
  }
  return { [key]: values };
}

function fail(message) {
  console.error(`i18n-add: ${message}`);
  process.exit(1);
}

const entries = Object.entries(parseInput());
if (entries.length === 0) fail('no keys at all');

for (const [key, values] of entries) {
  for (const short of Object.keys(LOCALES)) {
    const value = values[short];
    if (typeof value !== 'string' || value.trim() === '') {
      fail(`${key}: the «${short}» translation is missing or empty`);
    }
  }
}

// First read and edit everything in memory: failing on the third locale must
// not leave the first two already written.
const files = new Map();
for (const [short, name] of Object.entries(LOCALES)) {
  const path = join(dir, `${name}.json`);
  files.set(short, { path, data: JSON.parse(readFileSync(path, 'utf8')) });
}

for (const [key, values] of entries) {
  const parts = key.split('.');
  for (const [short, file] of files) {
    let node = file.data;
    for (const part of parts.slice(0, -1)) {
      if (node[part] === undefined) node[part] = {};
      if (typeof node[part] !== 'object' || Array.isArray(node[part])) {
        fail(`${key}: in ${file.path} «${part}» already holds a string, not a group`);
      }
      node = node[part];
    }
    const leaf = parts.at(-1);
    if (node[leaf] !== undefined && !force) {
      fail(`${key}: already exists in ${LOCALES[short]}.json, overwrite only with --force`);
    }
    node[leaf] = values[short];
  }
}

for (const { path, data } of files.values()) {
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`, 'utf8');
}

console.log(`Keys added: ${entries.length}, across ${files.size} locales.`);
console.log('Next: pnpm i18n:check');
