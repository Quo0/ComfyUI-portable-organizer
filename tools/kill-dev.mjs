// Kills the process holding the Vite dev server port.
//
// Needed because closing the app window with the X leaves Vite alive: the next
// `pnpm dev:desktop` fails with "Port 1420 is already in use". The regular way
// to stop it is Ctrl+C in the terminal; this script is for when the terminal is
// already gone.
//
// We look for the port's listener specifically. `taskkill /PID 1420` would kill
// the process numbered 1420 — the numbers are handed out by the system, and
// anything at all could be behind that one.

import { execFileSync } from 'node:child_process';

const PORT = Number(process.argv[2] ?? 1420);

function listeners(port) {
  let out = '';
  try {
    out = execFileSync('netstat', ['-ano'], { encoding: 'utf8' });
  } catch {
    console.error('netstat is unavailable');
    return [];
  }

  const pids = new Set();
  for (const line of out.split('\n')) {
    if (!line.includes('LISTENING')) continue;
    const parts = line.trim().split(/\s+/);
    const local = parts[1] ?? '';
    // Cut off the address: the port always follows the last colon, and this is
    // the only way not to confuse 1420 with 11420 or with the address [::1].
    if (local.slice(local.lastIndexOf(':') + 1) !== String(port)) continue;
    const pid = parts[parts.length - 1];
    if (pid && pid !== '0') pids.add(pid);
  }
  return [...pids];
}

const pids = listeners(PORT);
if (pids.length === 0) {
  console.log(`Port ${PORT} is free`);
  process.exit(0);
}

for (const pid of pids) {
  try {
    execFileSync('taskkill', ['/F', '/PID', pid], { stdio: 'pipe' });
    console.log(`Killed process ${pid}, which held port ${PORT}`);
  } catch (e) {
    console.error(`Could not kill ${pid}: ${e.message}`);
  }
}
