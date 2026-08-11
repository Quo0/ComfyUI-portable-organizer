// Добавление ключа интерфейса сразу во все четыре локали.
//
// Без этого добавление одной строки — четыре правки в четырёх файлах,
// и потерянный перевод обнаруживается только `i18n:check`, а то и глазами
// в чужой локали. Скрипт делает операцию атомарной: либо все четыре файла,
// либо ни одного.
//
// Порядок ключей сохраняется: новый ключ дописывается в конец своей группы,
// как это делает рука. Иначе диф превращался бы в перетасовку всего файла.
//
// Использование:
//   node tools/i18n-add.mjs install.run.preparing \
//     --en "Checking the folders…" --ru "Проверяем папки…" \
//     --es "Comprobando…" --zh "正在检查…"
//
//   node tools/i18n-add.mjs --file keys.json
//   где keys.json: { "install.run.preparing": { "en": "…", "ru": "…", … } }

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const dir = join(root, 'apps', 'desktop', 'src', 'i18n', 'locales');

/** Флаг `--zh` короче имени файла `zh-Hans`, но писать его каждый раз длинно. */
const LOCALES = { en: 'en', ru: 'ru', es: 'es', zh: 'zh-Hans' };

const args = process.argv.slice(2);
if (args.length === 0 || args.includes('--help')) {
  console.log(readFileSync(fileURLToPath(import.meta.url), 'utf8').split('\n\n')[0]);
  process.exit(args.length === 0 ? 1 : 0);
}

const force = args.includes('--force');

/** Разбирает либо один ключ с флагами, либо файл с пачкой ключей. */
function parseInput() {
  const fileAt = args.indexOf('--file');
  if (fileAt !== -1) {
    const path = args[fileAt + 1];
    if (!path) fail('--file без пути к файлу');
    return JSON.parse(readFileSync(path, 'utf8'));
  }

  const key = args[0];
  if (key.startsWith('--')) fail('первым аргументом должен идти ключ');

  const values = {};
  for (const short of Object.keys(LOCALES)) {
    const at = args.indexOf(`--${short}`);
    if (at === -1) fail(`нет перевода --${short}`);
    const value = args[at + 1];
    if (value === undefined || value.startsWith('--')) fail(`пустое значение --${short}`);
    values[short] = value;
  }
  return { [key]: values };
}

function fail(message) {
  console.error(`i18n-add: ${message}`);
  process.exit(1);
}

const entries = Object.entries(parseInput());
if (entries.length === 0) fail('ни одного ключа');

for (const [key, values] of entries) {
  for (const short of Object.keys(LOCALES)) {
    const value = values[short];
    if (typeof value !== 'string' || value.trim() === '') {
      fail(`${key}: нет или пуст перевод «${short}»`);
    }
  }
}

// Сначала читаем и правим всё в памяти: падение на третьей локали не должно
// оставлять первые две уже записанными.
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
        fail(`${key}: в ${file.path} на «${part}» уже лежит строка, а не группа`);
      }
      node = node[part];
    }
    const leaf = parts.at(-1);
    if (node[leaf] !== undefined && !force) {
      fail(`${key}: уже есть в ${LOCALES[short]}.json, перезапись только с --force`);
    }
    node[leaf] = values[short];
  }
}

for (const { path, data } of files.values()) {
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`, 'utf8');
}

console.log(`Добавлено ключей: ${entries.length}, в ${files.size} локали.`);
console.log('Дальше: pnpm i18n:check');
