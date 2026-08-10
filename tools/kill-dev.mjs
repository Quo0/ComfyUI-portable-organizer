// Снимает процесс, занявший порт дев-сервера Vite.
//
// Нужен потому, что закрытие окна приложения крестиком оставляет Vite
// жить: следующий `pnpm dev:desktop` падает с «Port 1420 is already in use».
// Штатный способ остановки — Ctrl+C в терминале, этот скрипт для случаев,
// когда терминал уже закрыт.
//
// Ищем именно слушателя порта. `taskkill /PID 1420` убил бы процесс
// с номером 1420 — номера выдаёт система, и под ним окажется что угодно.

import { execFileSync } from 'node:child_process';

const PORT = Number(process.argv[2] ?? 1420);

function listeners(port) {
  let out = '';
  try {
    out = execFileSync('netstat', ['-ano'], { encoding: 'utf8' });
  } catch {
    console.error('netstat недоступен');
    return [];
  }

  const pids = new Set();
  for (const line of out.split('\n')) {
    if (!line.includes('LISTENING')) continue;
    const parts = line.trim().split(/\s+/);
    const local = parts[1] ?? '';
    // Отрезаем адрес: порт всегда после последнего двоеточия, и это
    // единственный способ не спутать 1420 с 11420 или с адресом [::1].
    if (local.slice(local.lastIndexOf(':') + 1) !== String(port)) continue;
    const pid = parts[parts.length - 1];
    if (pid && pid !== '0') pids.add(pid);
  }
  return [...pids];
}

const pids = listeners(PORT);
if (pids.length === 0) {
  console.log(`Порт ${PORT} свободен`);
  process.exit(0);
}

for (const pid of pids) {
  try {
    execFileSync('taskkill', ['/F', '/PID', pid], { stdio: 'pipe' });
    console.log(`Снят процесс ${pid}, державший порт ${PORT}`);
  } catch (e) {
    console.error(`Не удалось снять ${pid}: ${e.message}`);
  }
}
