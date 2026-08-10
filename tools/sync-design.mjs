// Переносит собранный дизайн в приложение.
//
// Токены и компоненты не переписываются под приложение руками: источник
// правды — design/tokens и design/styles, а design/build.mjs проверяет их
// на паритет тем и контраст. Копия здесь нужна лишь потому, что Vite
// не должен тянуть файлы из-за пределов apps/desktop.
//
// Копии в .gitignore: они производные, и держать их в истории значит
// ловить конфликт при каждой правке токена.

import { copyFileSync, existsSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const from = join(root, 'design', 'dist');
const to = join(root, 'apps', 'desktop', 'src', 'styles');

const files = ['tokens.css', 'components.css'];

const missing = files.filter((f) => !existsSync(join(from, f)));
if (missing.length) {
  console.error(
    `design/dist не собран: нет ${missing.join(', ')}.\n` +
      'Сначала `node design/build.mjs`.',
  );
  process.exit(1);
}

mkdirSync(to, { recursive: true });
for (const f of files) copyFileSync(join(from, f), join(to, f));

console.log(`Дизайн перенесён в apps/desktop/src/styles: ${files.join(', ')}`);
