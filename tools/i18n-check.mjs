// Locale parity against en.json.
//
// The most valuable part of the whole translation effort: without a check the
// locales drift apart silently. A missing key shows English text in the middle
// of a Russian UI; an extra one is dead weight nobody will ever remove.
//
// Four things are checked:
//   1. missing keys,
//   2. extra keys,
//   3. empty values,
//   4. the set of `{...}` interpolations — a translation that lost `{reason}`
//      looks intact but loses half its meaning.

import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const dir = join(root, 'apps', 'desktop', 'src', 'i18n', 'locales');

const SOURCE = 'en';
const LOCALES = ['ru', 'zh-Hans', 'es'];

const read = (name) => JSON.parse(readFileSync(join(dir, `${name}.json`), 'utf8'));

/** Flattens a nested object into a map of `dot.separated.key` → string. */
function flatten(obj, prefix = '', out = new Map()) {
  for (const [k, v] of Object.entries(obj)) {
    const key = prefix ? `${prefix}.${k}` : k;
    if (v && typeof v === 'object' && !Array.isArray(v)) flatten(v, key, out);
    else out.set(key, v);
  }
  return out;
}

/** Interpolations of the form {reason}. Plural forms are separated by `|`. */
const placeholders = (value) =>
  new Set(String(value).match(/\{[a-zA-Z0-9_]+\}/g) ?? []);

const source = flatten(read(SOURCE));
const problems = [];

for (const locale of LOCALES) {
  const target = flatten(read(locale));

  for (const key of source.keys()) {
    if (!target.has(key)) problems.push(`${locale}: missing key ${key}`);
  }
  for (const key of target.keys()) {
    if (!source.has(key)) problems.push(`${locale}: extra key ${key}`);
  }
  for (const [key, value] of target) {
    if (!source.has(key)) continue;
    if (typeof value !== 'string' || value.trim() === '') {
      problems.push(`${locale}: empty value ${key}`);
      continue;
    }
    const want = placeholders(source.get(key));
    const got = placeholders(value);
    const lost = [...want].filter((p) => !got.has(p));
    const extra = [...got].filter((p) => !want.has(p));
    if (lost.length) problems.push(`${locale}: ${key} lost ${lost.join(', ')}`);
    if (extra.length) problems.push(`${locale}: ${key} has extra ${extra.join(', ')}`);
  }
}

if (problems.length) {
  console.error(`The locales have diverged from ${SOURCE}.json:\n`);
  for (const p of problems) console.error(`  ${p}`);
  console.error(`\nDivergences in total: ${problems.length}`);
  process.exit(1);
}

console.log(
  `The locales match ${SOURCE}.json: ${source.size} keys × ${LOCALES.length} translations`,
);
