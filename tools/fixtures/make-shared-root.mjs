// The shared models folder rig for Phase 2.5.
//
// The YAML generator builds its sections from the subfolders that actually
// exist in the root, not from a hardcoded list. So it has to be checked against
// a real file system, and the rig must have every branch represented: canonical
// categories, legacy names under map_legacy, the blacklist and the
// unrecognised.
//
// The category names are taken from a real ComfyUI 0.30 build (WIP\q1) rather
// than invented: the list of keys grows from version to version, and a made-up
// name would test imagination instead of code.
//
// Zero-size files: nobody cares about the contents of the models, and gigabytes
// in a rig are not wanted.
//
// Run: node tools/fixtures/make-shared-root.mjs

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, 'shared-models');

/** Canonical categories — they go into the YAML under their own name. */
const CANONICAL = {
  checkpoints: ['sd15-fake.safetensors', 'sdxl-fake.safetensors'],
  loras: ['style-fake.safetensors'],
  vae: ['vae-fake.safetensors'],
  controlnet: ['canny-fake.safetensors'],
  upscale_models: ['esrgan-fake.pth'],
  embeddings: ['bad-hands-fake.pt'],
  clip_vision: [],
  text_encoders: ['t5-fake.safetensors'],
  diffusion_models: [],
};

/**
 * Legacy names. `folder_paths.py:111-114` maps them onto the canonical ones, so
 * both folders must converge into a single key as a multi-line block:
 * unet + diffusion_models → diffusion_models, clip + text_encoders →
 * text_encoders. This is the generator's subtlest branch.
 */
const LEGACY = {
  unet: ['flux-fake.safetensors'],
  clip: ['clip-l-fake.safetensors'],
};

/**
 * The blacklist. The key is valid for ComfyUI and appears in its own example
 * config, so a user can easily end up with such a folder. Sharing custom nodes
 * cancels the very reason this project exists — the generator must exclude this
 * key even when the folder really is in the root.
 */
const BLACKLISTED = { custom_nodes: ['ComfyUI-Manager-fake/__init__.py'] };

/** Unrecognised: shown as a list in the UI, never written into the YAML.
 *  The Cyrillic names stay as they are on purpose — this is the non-ASCII path
 *  case, and check-shared-live.mjs asserts on this exact key. */
const UNKNOWN = {
  '_архив': ['старое-fake.safetensors'],
  'my notes': ['readme.txt'],
};

mkdirSync(root, { recursive: true });

let dirs = 0;
let files = 0;
for (const group of [CANONICAL, LEGACY, BLACKLISTED, UNKNOWN]) {
  for (const [name, entries] of Object.entries(group)) {
    mkdirSync(join(root, name), { recursive: true });
    dirs += 1;
    for (const entry of entries) {
      const path = join(root, name, entry);
      mkdirSync(dirname(path), { recursive: true });
      writeFileSync(path, '');
      files += 1;
    }
  }
}

// A file in the root, not a folder: the scanner must skip it rather than take
// it for a category with an empty name.
writeFileSync(join(root, 'README.txt'), 'Shared models folder rig. The files are empty.\n');

console.log(`Shared folder rig built: ${root}`);
console.log(`  categories: ${dirs}, files: ${files + 1}`);
console.log(`  canonical: ${Object.keys(CANONICAL).length}`);
console.log(`  legacy names (map_legacy): ${Object.keys(LEGACY).join(', ')}`);
console.log(`  blacklisted: ${Object.keys(BLACKLISTED).join(', ')}`);
console.log(`  unrecognised: ${Object.keys(UNKNOWN).length}`);
