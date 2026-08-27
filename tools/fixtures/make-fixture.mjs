// Completes the `fake-instance` rig into a valid instance.
//
// Everything except `python_embeded` is in git: that one is a junction to the
// system Python, and a link cannot be put into a repository. Copying the whole
// interpreter would mean a hundred megabytes of history for something that is
// already on the machine.
//
// The rig exists because the supervisor cannot be debugged against a real
// build: a cold start takes minutes, a crash has to be arranged by hand, and
// a hang cannot be reproduced at all.
//
// Run: node tools/fixtures/make-fixture.mjs

import { execFileSync } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, rmSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const instance = join(here, 'fake-instance');
const link = join(instance, 'python_embeded');

/**
 * A second copy of the rig under a path with a space and non-ASCII characters.
 *
 * The trap is named in the plan: `d:\program files\Модели ИИ\...` shows up in
 * users' setups more often than one would think, and it breaks quoting on spawn
 * and the resolution of `..\` inside `advanced\`. Checking this on an English
 * path without spaces is pointless — everything always works there.
 *
 * The name below stays in Cyrillic on purpose: it is the test data, not a note.
 */
const oddInstance = join(here, 'стенд с пробелом');

/** The installed Python's folder. We find the interpreter and take its directory. */
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
    // WindowsApps holds a stub that opens the app store rather than an
    // interpreter. It would pass a file-existence check and wreck the whole rig.
    if (path.includes('WindowsApps')) continue;
    if (existsSync(path)) return dirname(path);
  }
  return null;
}

/** Builds the copy of the rig under a path with a space and non-ASCII characters. */
function makeOdd(pythonDir) {
  rmSync(oddInstance, { recursive: true, force: true });
  // We copy everything except the junction: it cannot be copied as content, and
  // there is no need — we make our own identical one.
  cpSync(instance, oddInstance, {
    recursive: true,
    filter: (src) => !src.endsWith('python_embeded') && !src.includes('python_embeded'),
  });
  execFileSync('cmd', ['/c', 'mklink', '/J', join(oddInstance, 'python_embeded'), pythonDir], {
    stdio: 'pipe',
  });
  console.log(`Copy with a space and non-ASCII characters: ${oddInstance}`);
}

if (existsSync(link)) {
  console.log('The rig is already built:', link);
  const pythonDir = findPython();
  if (pythonDir && !existsSync(join(oddInstance, 'python_embeded'))) makeOdd(pythonDir);
  process.exit(0);
}

const pythonDir = findPython();
if (!pythonDir) {
  console.error(
    'Could not find a system Python. Install one and make sure `where python`\n' +
      'shows a real interpreter rather than the WindowsApps stub.',
  );
  process.exit(1);
}

mkdirSync(instance, { recursive: true });

try {
  // A junction, not a symlink: symbolic links on Windows require administrator
  // rights or developer mode, a junction does not.
  execFileSync('cmd', ['/c', 'mklink', '/J', link, pythonDir], { stdio: 'pipe' });
} catch (e) {
  console.error(`Could not create the junction: ${e.message}`);
  process.exit(1);
}

const ok = existsSync(join(link, 'python.exe')) && existsSync(join(instance, 'ComfyUI', 'main.py'));
console.log(`Rig built: ${instance}`);
console.log(`  python_embeded -> ${pythonDir}`);
console.log(`  valid as an instance: ${ok ? 'yes' : 'NO'}`);

makeOdd(pythonDir);
