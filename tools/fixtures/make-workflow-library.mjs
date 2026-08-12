// Стенд библиотеки воркфлоу для Фазы 2.6.
//
// Библиотека — это папка с файлами; манифест лишь обогащает её тегами
// и заметками. Значит проверять сканер надо на настоящей файловой системе,
// и в стенде обязаны быть представлены все расхождения между папкой
// и манифестом, а не только счастливый путь.
//
// Имена нод взяты из NODE_CLASS_MAPPINGS реальной сборки ComfyUI 0.30
// (WIP\q1), а не придуманы: воркфлоу «совместим со всем» должен быть
// совместим по-настоящему, иначе проверка проверяет фантазию.
//
// Запуск: node tools/fixtures/make-workflow-library.mjs

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, 'workflow-library');

/** Минимальный воркфлоу в формате, который сохраняет сам ComfyUI. */
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

/** Только базовые классы — такой воркфлоу открывается в любой сборке. */
const CORE = [
  'CheckpointLoaderSimple',
  'CLIPTextEncode',
  'EmptyLatentImage',
  'KSampler',
  'VAEDecode',
  'SaveImage',
];

/** Ноды из кастомных пакетов: в чистой сборке их нет. */
const CUSTOM = [
  'CheckpointLoaderSimple',
  'IPAdapterUnifiedLoader',
  'ReActorFaceSwap',
  'KSampler',
  'SaveImage',
];

const files = {
  // Совместим с чем угодно.
  'basic-txt2img.json': JSON.stringify(workflow(CORE), null, 2),

  // Вложенная подпапка — US-WF-06/AC-5.
  'flux/portrait-v3.json': JSON.stringify(workflow(CUSTOM), null, 2),

  // Файл есть, записи в манифесте нет — US-WF-06/AC-1, AC-2.
  'sdxl/base-upscale.json': JSON.stringify(workflow(CORE), null, 2),

  // Не воркфлоу, хотя и JSON: массива nodes нет — US-WF-03/AC-7.
  'not-a-workflow.json': JSON.stringify({ hello: 'world' }, null, 2),

  // Битый JSON — сканер обязан пережить и не уронить весь список.
  'broken.json': '{ "nodes": [ {"type": "KSampler"',

  // Посторонний файл: в список воркфлоу попадать не должен — US-WF-01/AC-7.
  'README.txt': 'Стенд библиотеки воркфлоу. Файлы игрушечные.\n',
};

/**
 * Манифест лежит в самой библиотеке, а не в данных приложения: она обязана
 * пережить переустановку приложения и переезд на другую машину.
 *
 * Запись `lost/deleted.json` намеренно указывает на несуществующий файл —
 * так проверяется, что удалённый мимо приложения воркфлоу помечается
 * потерянным, а не исчезает молча (US-WF-06/AC-3).
 */
const manifest = {
  version: 1,
  items: {
    'basic-txt2img.json': {
      favorite: true,
      tags: ['базовый', 'txt2img'],
      note: 'Простейший граф на стоковых нодах. Открывается везде.',
      addedAt: 1754000000000,
    },
    'flux/portrait-v3.json': {
      favorite: true,
      tags: ['flux', 'портрет'],
      note: 'Требует IPAdapter и ReActor. Без них не откроется.',
      addedAt: 1754000100000,
      sourceInstanceId: 'q1',
    },
    'lost/deleted.json': {
      favorite: false,
      tags: ['потерян'],
      note: 'Файла нет — запись обязана показаться потерянной.',
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

console.log(`Стенд библиотеки собран: ${root}`);
console.log(`  файлов: ${Object.keys(files).length + 1}`);
console.log(`  воркфлоу в манифесте: ${Object.keys(manifest.items).length}`);
console.log('  ветви: вложенная папка, файл без записи, запись без файла,');
console.log('         не-воркфлоу, битый JSON, посторонний файл');
