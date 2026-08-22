// Сверка тега с версией приложения и вырезка секции CHANGELOG.md.
//
// Оба отказа обязаны случиться до сборки, а не после: опубликованный
// релиз, где тег говорит одно, а приложение показывает другое, чинится
// только новым релизом, и понять, что установлено у пользователя,
// становится нечем. Релиз без описания изменений чинится тоже не сразу,
// а описание, дописанное задним числом, никто уже не прочитает.
//
// Использование:
//   node tools/release-notes.mjs v0.2.0 [release-notes.md]
//
// Без второго аргумента секция печатается в stdout.

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

const tag = process.argv[2];
const out = process.argv[3];

if (!tag) fail('первым аргументом должен идти тег, например v0.2.0');

const config = JSON.parse(
  readFileSync(join(root, 'apps', 'desktop', 'src-tauri', 'tauri.conf.json'), 'utf8'),
);

// Тег без `v` — это и есть версия. Дефис в нём означает предрелиз
// (`v0.2.0-beta.1`), и на сверку с версией приложения это не влияет:
// у Tauri версия тоже семантическая и суффикс держит.
const version = tag.replace(/^v/, '');

if (config.version !== version) {
  fail(
    `тег ${tag} не совпадает с версией приложения: ` +
      `в tauri.conf.json стоит ${config.version}`,
  );
}

const changelog = readFileSync(join(root, 'CHANGELOG.md'), 'utf8');

/**
 * Секция версии: от её заголовка до следующего заголовка того же уровня.
 *
 * Заголовок ищется по началу строки и номеру версии, а не по точному
 * совпадению целиком: после номера обычно стоит дата, и требовать её
 * формат — лишний повод уронить выпуск.
 */
const lines = changelog.split(/\r?\n/);
const from = lines.findIndex((line) =>
  new RegExp(`^## \\s*${version.replace(/\./g, '\\.')}(\\s|$)`).test(line),
);

if (from === -1) {
  fail(
    `в CHANGELOG.md нет секции «## ${version}». ` +
      'Переименуйте «Не выпущено» в неё перед тем, как ставить тег.',
  );
}

const rest = lines.slice(from + 1);
const to = rest.findIndex((line) => line.startsWith('## '));
const body = (to === -1 ? rest : rest.slice(0, to)).join('\n').trim();

if (!body) fail(`секция «## ${version}» в CHANGELOG.md пуста`);

if (out) {
  writeFileSync(join(root, out), `${body}\n`, 'utf8');
  console.log(`Секция ${version} записана в ${out}`);
} else {
  console.log(body);
}

function fail(message) {
  console.error(`Выпуск остановлен: ${message}`);
  process.exit(1);
}
