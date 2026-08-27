// Checks the API client against a real ComfyUI build.
//
// The script starts the given build and passes the port to the
// `check_comfy_live` example, which talks to the server with **our own**
// client. What is being checked is our code, not ComfyUI's behaviour.
//
// The `fake-instance` rig does not replace this: it has neither /userdata nor
// /object_info, it is a supervisor stub.
//
// Started with `--cpu`: what is being checked is API work, not the GPU.
//
// Run: node tools/check-workflows-live.mjs <build-path>

import { spawn, spawnSync } from 'node:child_process';
import { existsSync, rmSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const build = process.argv[2];
if (!build) {
  console.error('Usage: node tools/check-workflows-live.mjs <build-path>');
  process.exit(2);
}

const PORT = 8189;
/** The file the check leaves behind. We remove it at the end. */
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
      // A cold start takes tens of seconds — that is normal.
    }
    if (child.exitCode !== null) break;
    if (!ready) await sleep(1000);
  }
  if (!ready) {
    console.error('The server did not come up:\n' + log.slice(-2000));
    process.exit(1);
  }
  console.log(`Server ready on :${PORT}, running the client\n`);

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
  // We clean up after ourselves: the user's build must come back to the same
  // state it was in before the check.
  rmSync(LEFTOVER, { force: true });
  console.log(`\nCleaned up: ${LEFTOVER} → ${existsSync(LEFTOVER) ? 'STILL THERE' : 'gone'}`);
}
