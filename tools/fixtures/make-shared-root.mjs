// Стенд общей папки моделей для Фазы 2.5.
//
// Генератор YAML строит секции из реально существующих подпапок корня,
// а не из захардкоженного списка. Значит, проверять его надо на настоящей
// файловой системе, и в стенде должны быть представлены все ветви:
// канонические категории, устаревшие имена под map_legacy, чёрный список
// и нераспознанное.
//
// Имена категорий взяты из реальной сборки ComfyUI 0.30 (WIP\q1), а не
// придуманы: список ключей растёт от версии к версии, и выдуманное имя
// проверяло бы фантазию, а не код.
//
// Файлы нулевого размера: содержимое моделей никого не интересует,
// а гигабайты в стенде — нет.
//
// Запуск: node tools/fixtures/make-shared-root.mjs

import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, 'shared-models');

/** Канонические категории — попадают в YAML под своим именем. */
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
 * Устаревшие имена. `folder_paths.py:111-114` маппит их на канонические,
 * поэтому обе папки обязаны съехаться в один ключ многострочным блоком:
 * unet + diffusion_models → diffusion_models, clip + text_encoders →
 * text_encoders. Это самая тонкая ветвь генератора.
 */
const LEGACY = {
  unet: ['flux-fake.safetensors'],
  clip: ['clip-l-fake.safetensors'],
};

/**
 * Чёрный список. Ключ валиден для ComfyUI и есть в его же примере конфига,
 * поэтому пользователь легко заведёт такую папку. Шаринг кастомных нод
 * отменяет причину существования проекта — генератор обязан исключить
 * этот ключ, даже когда папка реально лежит в корне.
 */
const BLACKLISTED = { custom_nodes: ['ComfyUI-Manager-fake/__init__.py'] };

/** Нераспознанное: в UI показывается списком, в YAML не попадает. */
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

// Файл в корне, а не папка: сканер обязан его пропустить, а не принять
// за категорию с пустым именем.
writeFileSync(join(root, 'README.txt'), 'Стенд общей папки моделей. Файлы пустые.\n');

console.log(`Стенд общей папки собран: ${root}`);
console.log(`  категорий: ${dirs}, файлов: ${files + 1}`);
console.log(`  канонических: ${Object.keys(CANONICAL).length}`);
console.log(`  устаревших имён (map_legacy): ${Object.keys(LEGACY).join(', ')}`);
console.log(`  в чёрном списке: ${Object.keys(BLACKLISTED).join(', ')}`);
console.log(`  нераспознанных: ${Object.keys(UNKNOWN).length}`);
