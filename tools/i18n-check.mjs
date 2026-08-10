// Паритет локалей с en.json.
//
// Самая ценная часть затеи с переводами: без проверки локали расходятся
// молча. Недостающий ключ показывает английский текст посреди русского
// интерфейса, лишний — мёртвый груз, который никто не удалит.
//
// Проверяются четыре вещи:
//   1. недостающие ключи,
//   2. лишние ключи,
//   3. пустые значения,
//   4. набор подстановок `{...}` — перевод, потерявший `{reason}`,
//      выглядит целым, но теряет половину смысла.

import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const dir = join(root, 'apps', 'desktop', 'src', 'i18n', 'locales');

const SOURCE = 'en';
const LOCALES = ['ru', 'zh-Hans', 'es'];

const read = (name) => JSON.parse(readFileSync(join(dir, `${name}.json`), 'utf8'));

/** Разворачивает вложенный объект в плоскую карту `ключ.через.точку` → строка. */
function flatten(obj, prefix = '', out = new Map()) {
  for (const [k, v] of Object.entries(obj)) {
    const key = prefix ? `${prefix}.${k}` : k;
    if (v && typeof v === 'object' && !Array.isArray(v)) flatten(v, key, out);
    else out.set(key, v);
  }
  return out;
}

/** Подстановки вида {reason}. Формы множественного числа разделены `|`. */
const placeholders = (value) =>
  new Set(String(value).match(/\{[a-zA-Z0-9_]+\}/g) ?? []);

const source = flatten(read(SOURCE));
const problems = [];

for (const locale of LOCALES) {
  const target = flatten(read(locale));

  for (const key of source.keys()) {
    if (!target.has(key)) problems.push(`${locale}: нет ключа ${key}`);
  }
  for (const key of target.keys()) {
    if (!source.has(key)) problems.push(`${locale}: лишний ключ ${key}`);
  }
  for (const [key, value] of target) {
    if (!source.has(key)) continue;
    if (typeof value !== 'string' || value.trim() === '') {
      problems.push(`${locale}: пустое значение ${key}`);
      continue;
    }
    const want = placeholders(source.get(key));
    const got = placeholders(value);
    const lost = [...want].filter((p) => !got.has(p));
    const extra = [...got].filter((p) => !want.has(p));
    if (lost.length) problems.push(`${locale}: ${key} потерял ${lost.join(', ')}`);
    if (extra.length) problems.push(`${locale}: ${key} содержит лишние ${extra.join(', ')}`);
  }
}

if (problems.length) {
  console.error(`Локали разошлись с ${SOURCE}.json:\n`);
  for (const p of problems) console.error(`  ${p}`);
  console.error(`\nВсего расхождений: ${problems.length}`);
  process.exit(1);
}

console.log(
  `Локали совпадают с ${SOURCE}.json: ${source.size} ключей × ${LOCALES.length} переводов`,
);
