// Проверка клиента API против настоящей сборки ComfyUI.
//
// Скрипт поднимает указанную сборку и передаёт порт примеру
// `check_comfy_live`, который ходит на сервер **нашим же** клиентом.
// Проверяется наш код, а не поведение ComfyUI.
//
// Стенд `fake-instance` этого не заменяет: у него нет ни /userdata,
// ни /object_info, он заглушка супервизора.
//
// Запускается с `--cpu`: проверяется работа с API, а не видеокарта.
//
// Запуск: node tools/check-workflows-live.mjs <путь-к-сборке>

import { spawn, spawnSync } from 'node:child_process';
import { existsSync, rmSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const build = process.argv[2];
if (!build) {
  console.error('Использование: node tools/check-workflows-live.mjs <путь-к-сборке>');
  process.exit(2);
}

const PORT = 8189;
/** Файл, который оставит за собой проверка. Убираем его в конце. */
const LEFTOVER = resolve(build, 'ComfyUI/user/default/workflows/cpo-live-check.json');

const child = spawn(
  resolve(build, 'python_embeded/python.exe'),
  [
    '-s',
    'ComfyUI/main.py',
    '--windows-standalone-build',
    '--cpu',
    '--port',
    String(PORT),
    '--disable-auto-launch',
  ],
  { cwd: build, stdio: ['ignore', 'pipe', 'pipe'] },
);

let log = '';
child.stdout.on('data', (c) => (log += c));
child.stderr.on('data', (c) => (log += c));

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

try {
  const until = Date.now() + 240000;
  let ready = false;
  while (Date.now() < until && !ready) {
    try {
      ready = (await fetch(`http://127.0.0.1:${PORT}/system_stats`)).ok;
    } catch {
      // Холодный старт идёт десятки секунд — это нормально.
    }
    if (child.exitCode !== null) break;
    if (!ready) await sleep(1000);
  }
  if (!ready) {
    console.error('Сервер не поднялся:\n' + log.slice(-2000));
    process.exit(1);
  }
  console.log(`Сервер готов на :${PORT}, гоняем клиент\n`);

  const run = spawnSync(
    'cargo',
    [
      'run',
      '-q',
      '--manifest-path',
      resolve(root, 'apps/desktop/src-tauri/Cargo.toml'),
      '--example',
      'check_comfy_live',
      '--',
      String(PORT),
    ],
    { stdio: 'inherit' },
  );
  process.exitCode = run.status ?? 1;
} finally {
  child.kill();
  await sleep(500);
  // За собой прибираем: сборка пользователя должна вернуться в то же
  // состояние, в каком была до проверки.
  rmSync(LEFTOVER, { force: true });
  console.log(`\nПрибрано: ${LEFTOVER} → ${existsSync(LEFTOVER) ? 'ОСТАЛСЯ' : 'нет'}`);
}
