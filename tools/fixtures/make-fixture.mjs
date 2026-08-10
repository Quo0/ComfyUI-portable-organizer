// Достраивает стенд `fake-instance` до валидного инстанса.
//
// В git лежит всё, кроме `python_embeded`: это junction на системный Python,
// а ссылку в репозиторий не положишь. Копировать интерпретатор целиком —
// сотня мегабайт в истории ради того, что уже есть на машине.
//
// Стенд нужен потому, что отлаживать супервизор на реальной сборке нельзя:
// холодный старт идёт минуты, падение приходится подстраивать руками,
// а зависание не воспроизвести вовсе.
//
// Запуск: node tools/fixtures/make-fixture.mjs

import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const instance = join(here, 'fake-instance');
const link = join(instance, 'python_embeded');

/** Папка установленного Python. Ищем сам интерпретатор и берём его директорию. */
function findPython() {
  let out = '';
  try {
    out = execFileSync('where', ['python'], { encoding: 'utf8' });
  } catch {
    return null;
  }
  for (const line of out.split('\n')) {
    const path = line.trim();
    if (!path) continue;
    // WindowsApps содержит заглушку, открывающую магазин приложений,
    // а не интерпретатор. Она бы прошла проверку существования файла
    // и сорвала бы весь стенд.
    if (path.includes('WindowsApps')) continue;
    if (existsSync(path)) return dirname(path);
  }
  return null;
}

if (existsSync(link)) {
  console.log('Стенд уже собран:', link);
  process.exit(0);
}

const pythonDir = findPython();
if (!pythonDir) {
  console.error(
    'Не нашёл системный Python. Установите его и убедитесь, что `where python`\n' +
      'показывает настоящий интерпретатор, а не заглушку из WindowsApps.',
  );
  process.exit(1);
}

mkdirSync(instance, { recursive: true });

try {
  // Junction, а не symlink: символические ссылки на Windows требуют прав
  // администратора или режима разработчика, junction — нет.
  execFileSync('cmd', ['/c', 'mklink', '/J', link, pythonDir], { stdio: 'pipe' });
} catch (e) {
  console.error(`Не удалось создать junction: ${e.message}`);
  process.exit(1);
}

const ok = existsSync(join(link, 'python.exe')) && existsSync(join(instance, 'ComfyUI', 'main.py'));
console.log(`Стенд собран: ${instance}`);
console.log(`  python_embeded -> ${pythonDir}`);
console.log(`  валиден как инстанс: ${ok ? 'да' : 'НЕТ'}`);
