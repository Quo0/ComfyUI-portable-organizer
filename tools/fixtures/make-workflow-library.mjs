// The workflow library rig for Phase 2.6.
//
// The library is a folder of files; the manifest only enriches it with tags and
// notes. So the scanner has to be checked against a real file system, and the
// rig must have every divergence between folder and manifest represented, not
// just the happy path.
//
// The node names are taken from NODE_CLASS_MAPPINGS of a real ComfyUI 0.30
// build (WIP\q1) rather than invented: a workflow that is "compatible with
// everything" has to be genuinely compatible, otherwise the check tests
// imagination.
//
// Run: node tools/fixtures/make-workflow-library.mjs

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, 'workflow-library');

/** A minimal workflow in the format ComfyUI itself saves. */
const workflow = (nodes) => ({
  last_node_id: nodes.length,
  last_link_id: 0,
  nodes: nodes.map((type, i) => ({
    id: i + 1,
    type,
    pos: [100 + i * 60, 100],
    size: [210, 46],
    flags: {},
    order: i,
    mode: 0,
    inputs: [],
    outputs: [],
    properties: { 'Node name for S&R': type },
  })),
  links: [],
  groups: [],
  config: {},
  extra: {},
  version: 0.4,
});

/** Core classes only — such a workflow opens in any build. */
const CORE = [
  'CheckpointLoaderSimple',
  'CLIPTextEncode',
  'EmptyLatentImage',
  'KSampler',
  'VAEDecode',
  'SaveImage',
];

/** Nodes from custom packages: a clean build does not have them. */
const CUSTOM = [
  'CheckpointLoaderSimple',
  'IPAdapterUnifiedLoader',
  'ReActorFaceSwap',
  'KSampler',
  'SaveImage',
];

const files = {
  // Compatible with anything.
  'basic-txt2img.json': JSON.stringify(workflow(CORE), null, 2),

  // A nested subfolder — US-WF-06/AC-5.
  'flux/portrait-v3.json': JSON.stringify(workflow(CUSTOM), null, 2),

  // The file exists, the manifest entry does not — US-WF-06/AC-1, AC-2.
  'sdxl/base-upscale.json': JSON.stringify(workflow(CORE), null, 2),

  // Not a workflow even though it is JSON: no nodes array — US-WF-03/AC-7.
  'not-a-workflow.json': JSON.stringify({ hello: 'world' }, null, 2),

  // Broken JSON — the scanner must survive it and not take the whole list down.
  'broken.json': '{ "nodes": [ {"type": "KSampler"',

  // An unrelated file: it must not appear in the workflow list — US-WF-01/AC-7.
  'README.txt': 'Workflow library rig. The files are toys.\n',
};

/**
 * The manifest lives in the library itself, not in the app's data: the library
 * must survive an app reinstall and a move to another machine.
 *
 * The `lost/deleted.json` entry deliberately points at a file that does not
 * exist — that is how it is checked that a workflow deleted behind the app's
 * back is marked as lost rather than silently disappearing (US-WF-06/AC-3).
 */
const manifest = {
  version: 1,
  items: {
    'basic-txt2img.json': {
      favorite: true,
      tags: ['basic', 'txt2img'],
      note: 'The simplest graph on stock nodes. Opens everywhere.',
      addedAt: 1754000000000,
    },
    'flux/portrait-v3.json': {
      favorite: true,
      tags: ['flux', 'portrait'],
      note: 'Requires IPAdapter and ReActor. Will not open without them.',
      addedAt: 1754000100000,
      sourceInstanceId: 'q1',
    },
    'lost/deleted.json': {
      favorite: false,
      tags: ['lost'],
      note: 'The file is missing — the entry must show up as lost.',
      addedAt: 1754000200000,
    },
  },
};

mkdirSync(root, { recursive: true });
for (const [rel, content] of Object.entries(files)) {
  const path = join(root, rel);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content, 'utf8');
}
writeFileSync(join(root, '_library.json'), `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');

console.log(`Library rig built: ${root}`);
console.log(`  files: ${Object.keys(files).length + 1}`);
console.log(`  workflows in the manifest: ${Object.keys(manifest.items).length}`);
console.log('  branches: nested folder, file without an entry, entry without a file,');
console.log('            not-a-workflow, broken JSON, unrelated file');
