// Проверка «конфиг подхватился» на настоящей сборке ComfyUI.
//
// Единственный способ убедиться, что общие модели работают: посмотреть не
// в интерфейс, а в API самого ComfyUI. `GET /internal/folder_paths` отдаёт
// `{ключ: [пути]}` — объективный ответ на вопрос, увидел ли он наши пути,
// встал ли общий путь первым при `is_default` и не потерялись ли локальные.
//
// Стенд `fake-instance` это не заменяет: он заглушка и никакого YAML не
// разбирает. Проверять разбор можно только на настоящей сборке.
//
// Запускается с `--cpu`: проверяется разбор конфига, а не работа
// видеокарты, и занимать видеопамять ради этого незачем.
//
// Запуск:
//   node tools/fixtures/make-shared-root.mjs
//   cargo run --manifest-path apps/desktop/src-tauri/Cargo.toml --example check_shared
//   node tools/check-shared-live.mjs <путь-к-сборке> <путь-к-yaml> <общий-корень>

import { spawn } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const [build, config, sharedRoot] = process.argv.slice(2);
if (!build || !config || !sharedRoot) {
  console.error(
    'Использование: node tools/check-shared-live.mjs <сборка> <yaml> <общий-корень>',
  );
  process.exit(2);
}

// Порт заведомо не тот, что у инстансов пользователя: проверка не должна
// подраться с работающей сборкой.
const PORT = 8189;

console.log('--- конфиг ---\n' + readFileSync(config, 'utf8'));

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
    '--extra-model-paths-config',
    config,
  ],
  { cwd: build, stdio: ['ignore', 'pipe', 'pipe'] },
);

let log = '';
const collect = (chunk) => {
  log += chunk;
  // `utils/extra_config.py:32` логирует каждый добавленный путь. Строки
  // полезны для глаз, но выводом судить нельзя: имя с кириллицей в него
  // не попадает из-за кодировки консоли, хотя путь при этом добавлен.
  for (const line of String(chunk).split('\n')) {
    if (line.includes('Adding extra search path')) console.log('  ' + line.trim());
  }
};
child.stdout.on('data', collect);
child.stderr.on('data', collect);

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function waitReady(limitMs) {
  const until = Date.now() + limitMs;
  while (Date.now() < until) {
    try {
      if ((await fetch(`http://127.0.0.1:${PORT}/system_stats`)).ok) return true;
    } catch {
      // Ещё не поднялся — это нормально, холодный старт идёт десятки секунд.
    }
    if (child.exitCode !== null) return false;
    await sleep(1000);
  }
  return false;
}

let failures = 0;
const check = (name, ok, detail = '') => {
  console.log(`${ok ? '  OK  ' : 'ПРОВАЛ'} ${name}${detail ? ' — ' + detail : ''}`);
  if (!ok) failures += 1;
};

try {
  if (!(await waitReady(240000))) {
    console.error('Сервер не поднялся. Хвост лога:\n' + log.slice(-3000));
    process.exit(1);
  }
  console.log('\nСервер готов, спрашиваем /internal/folder_paths\n');

  const paths = await (
    await fetch(`http://127.0.0.1:${PORT}/internal/folder_paths`)
  ).json();
  const norm = (p) => p.replace(/\\/g, '/').toLowerCase();
  const shared = norm(sharedRoot);
  const of = (key) => (paths[key] ?? []).map(norm);

  check(
    'общий путь виден',
    of('checkpoints').some((p) => p.startsWith(shared)),
    JSON.stringify(of('checkpoints')),
  );
  // `is_default` управляет только тем, куда попадёт вновь скачанное.
  check('общий путь первый при is_default', of('checkpoints')[0]?.startsWith(shared) === true);
  // Главное обещание раздела: подключение ничего не отбирает.
  check(
    'локальные модели инстанса на месте',
    of('checkpoints').some((p) => !p.startsWith(shared)),
    JSON.stringify(of('checkpoints')),
  );
  check(
    'unet и diffusion_models слились в один ключ',
    of('diffusion_models').filter((p) => p.startsWith(shared)).length === 2,
    JSON.stringify(of('diffusion_models')),
  );
  check(
    'clip и text_encoders слились в один ключ',
    of('text_encoders').filter((p) => p.startsWith(shared)).length === 2,
    JSON.stringify(of('text_encoders')),
  );
  check(
    'custom_nodes из общей папки НЕ добавлен',
    !of('custom_nodes').some((p) => p.startsWith(shared)),
    JSON.stringify(of('custom_nodes')),
  );

  // Цель для новых загрузок — `paths[0]`. Из двух папок, слитых в один
  // ключ, ею обязана быть каноническая: при `is_default` пути вставляются
  // в начало, то есть порядок из YAML переворачивается.
  check(
    'цель загрузок — diffusion_models, а не unet',
    of('diffusion_models')[0] === `${shared}/diffusion_models`,
    JSON.stringify(of('diffusion_models')[0]),
  );
  check(
    'цель загрузок — text_encoders, а не clip',
    of('text_encoders')[0] === `${shared}/text_encoders`,
    JSON.stringify(of('text_encoders')[0]),
  );
  check(
    'папка с кириллицей в имени добавлена',
    of('_архив').some((p) => p.startsWith(shared)),
    JSON.stringify(of('_архив')),
  );

  console.log(`\nПроверок провалено: ${failures}`);
} finally {
  child.kill();
  await sleep(500);
}

process.exit(failures > 0 ? 1 : 0);
