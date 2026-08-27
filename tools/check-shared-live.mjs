// The "did the config take" check against a real ComfyUI build.
//
// The only way to be sure shared models work is to look not at the UI but at
// ComfyUI's own API. `GET /internal/folder_paths` returns `{key: [paths]}` — an
// objective answer to whether it saw our paths, whether the shared path came
// first under `is_default`, and whether the local ones were lost.
//
// The `fake-instance` rig does not replace this: it is a stub and parses no
// YAML at all. Parsing can only be checked against a real build.
//
// Started with `--cpu`: what is being checked is config parsing, not the GPU,
// and there is no reason to occupy video memory for it.
//
// Run:
//   node tools/fixtures/make-shared-root.mjs
//   cargo run --manifest-path apps/desktop/src-tauri/Cargo.toml --example check_shared
//   node tools/check-shared-live.mjs <build-path> <yaml-path> <shared-root>

import { spawn } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const [build, config, sharedRoot] = process.argv.slice(2);
if (!build || !config || !sharedRoot) {
  console.error(
    'Usage: node tools/check-shared-live.mjs <build> <yaml> <shared-root>',
  );
  process.exit(2);
}

// A port deliberately different from the user's instances: the check must not
// pick a fight with a running build.
const PORT = 8189;

console.log('--- config ---\n' + readFileSync(config, 'utf8'));

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
  // `utils/extra_config.py:32` logs every path it adds. The lines are useful to
  // the eye, but the output cannot be used to judge: a name with non-ASCII
  // characters never makes it there because of the console encoding, even
  // though the path was in fact added.
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
      // Not up yet — that is normal, a cold start takes tens of seconds.
    }
    if (child.exitCode !== null) return false;
    await sleep(1000);
  }
  return false;
}

let failures = 0;
const check = (name, ok, detail = '') => {
  console.log(`${ok ? '  OK  ' : ' FAIL '} ${name}${detail ? ' — ' + detail : ''}`);
  if (!ok) failures += 1;
};

try {
  if (!(await waitReady(240000))) {
    console.error('The server did not come up. Tail of the log:\n' + log.slice(-3000));
    process.exit(1);
  }
  console.log('\nServer ready, asking /internal/folder_paths\n');

  const paths = await (
    await fetch(`http://127.0.0.1:${PORT}/internal/folder_paths`)
  ).json();
  const norm = (p) => p.replace(/\\/g, '/').toLowerCase();
  const shared = norm(sharedRoot);
  const of = (key) => (paths[key] ?? []).map(norm);

  check(
    'the shared path is visible',
    of('checkpoints').some((p) => p.startsWith(shared)),
    JSON.stringify(of('checkpoints')),
  );
  // `is_default` only controls where newly downloaded files land.
  check('the shared path comes first under is_default', of('checkpoints')[0]?.startsWith(shared) === true);
  // The section's central promise: connecting takes nothing away.
  check(
    "the instance's local models are still there",
    of('checkpoints').some((p) => !p.startsWith(shared)),
    JSON.stringify(of('checkpoints')),
  );
  check(
    'unet and diffusion_models merged into one key',
    of('diffusion_models').filter((p) => p.startsWith(shared)).length === 2,
    JSON.stringify(of('diffusion_models')),
  );
  check(
    'clip and text_encoders merged into one key',
    of('text_encoders').filter((p) => p.startsWith(shared)).length === 2,
    JSON.stringify(of('text_encoders')),
  );
  check(
    'custom_nodes from the shared folder was NOT added',
    !of('custom_nodes').some((p) => p.startsWith(shared)),
    JSON.stringify(of('custom_nodes')),
  );

  // The target for new downloads is `paths[0]`. Of two folders merged into one
  // key it must be the canonical one: under `is_default` paths are inserted at
  // the front, which reverses the order from the YAML.
  check(
    'download target is diffusion_models, not unet',
    of('diffusion_models')[0] === `${shared}/diffusion_models`,
    JSON.stringify(of('diffusion_models')[0]),
  );
  check(
    'download target is text_encoders, not clip',
    of('text_encoders')[0] === `${shared}/text_encoders`,
    JSON.stringify(of('text_encoders')[0]),
  );
  // The folder name below stays in Cyrillic on purpose: this is the non-ASCII
  // path case, and it is exactly what the console encoding hides from the log
  // above. Translating it would delete the check, not translate it.
  check(
    'a folder with non-ASCII characters in its name was added',
    of('_архив').some((p) => p.startsWith(shared)),
    JSON.stringify(of('_архив')),
  );

  console.log(`\nChecks failed: ${failures}`);
} finally {
  child.kill();
  await sleep(500);
}

process.exit(failures > 0 ? 1 : 0);
