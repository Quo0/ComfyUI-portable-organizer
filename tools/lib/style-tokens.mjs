// Разбор apps/desktop/src/styles/tokens.css и работа с цветом.
// Общий код для tools/build-preview-tokens.mjs и tools/check-styles.mjs.
//
// Источник — приложение: `:root {светлая}`, `@media (prefers-color-scheme:
// dark) { :root:not(...) {тёмная} }`, `:root[data-theme="dark"] {тёмная}`,
// затем второй `:root {метрики}` — то же самое, что и в приложении, метрики
// от темы не зависят. Медиа-блок — дубликат тёмной темы для системного
// предпочтения, второй источник для неё не заводим.

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

export const ROOT_DIR = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
export const APP_STYLES_DIR = join(ROOT_DIR, 'apps', 'desktop', 'src', 'styles');
export const TOKENS_SRC = join(APP_STYLES_DIR, 'tokens.css');
export const COMPONENTS_SRC = join(APP_STYLES_DIR, 'components.css');
export const APPS_DESIGN = join(ROOT_DIR, 'apps', 'design');
export const DESIGN_CHROME_SRC = join(APPS_DESIGN, '.vitepress', 'theme', 'style.css');

/** Достаёт тело блока по имени селектора, начиная поиск с символа `from`.
 *  Селектор ищется с начала строки — иначе совпадёт упоминание в комментарии.
 *  Возвращает и конец блока — нужен, чтобы найти следующий блок с тем же
 *  именем селектора (в файле их два: светлая тема и метрики, оба `:root`). */
function blockBody(css, selector, from = 0) {
  const anchor = new RegExp(`^${selector.replace(/[.[\]"]/g, '\\$&')}\\s*\\{`, 'm');
  const found = anchor.exec(css.slice(from));
  if (!found) throw new Error(`Блок ${selector} не найден`);
  const start = from + found.index;
  const open = css.indexOf('{', start);
  let depth = 0;
  for (let i = open; i < css.length; i++) {
    if (css[i] === '{') depth++;
    else if (css[i] === '}') {
      depth--;
      if (depth === 0) return { body: css.slice(open + 1, i), end: i + 1 };
    }
  }
  throw new Error(`Блок ${selector} не закрыт`);
}

/** Пары «имя токена → значение» из тела блока. */
export function declarations(body) {
  const out = new Map();
  for (const m of body.matchAll(/(--[\w-]+)\s*:\s*([^;]+);/g)) {
    out.set(m[1], m[2].trim());
  }
  return out;
}

export function loadTokens() {
  const css = readFileSync(TOKENS_SRC, 'utf8');
  const light = blockBody(css, ':root');
  const dark = blockBody(css, ':root[data-theme="dark"]', light.end);
  const metrics = blockBody(css, ':root', dark.end);
  return {
    css,
    lightBody: light.body,
    darkBody: dark.body,
    metricsBody: metrics.body,
    light: declarations(light.body),
    dark: declarations(dark.body),
    metrics: declarations(metrics.body),
  };
}

// ---------------------------------------------------------------- цвет

export function hexToRgb(hex) {
  const h = hex.trim().replace('#', '');
  const full = h.length === 3 ? h.split('').map((c) => c + c).join('') : h;
  return [0, 2, 4].map((i) => parseInt(full.slice(i, i + 2), 16));
}

function channel(v) {
  const s = v / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}

export function luminance(hex) {
  const [r, g, b] = hexToRgb(hex).map(channel);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

export function contrast(a, b) {
  const la = luminance(a);
  const lb = luminance(b);
  const [hi, lo] = la > lb ? [la, lb] : [lb, la];
  return (hi + 0.05) / (lo + 0.05);
}

export const ratio = (a, b) => Math.round(contrast(a, b) * 100) / 100;

export const isHex = (v) => /^#[0-9a-fA-F]{3,8}$/.test(v.trim());

export const ACCENTS = [
  ['ember', 'Ember'],
  ['amber', 'Amber'],
  ['moss', 'Moss'],
  ['teal', 'Teal'],
  ['azure', 'Azure'],
  ['indigo', 'Indigo'],
  ['orchid', 'Orchid'],
  ['rose', 'Rose'],
];

// Семантические роли палитры, показанные в разделе «Палитра» стайлгайда.
export const ROLES = [
  ['--ground', 'основание окна'],
  ['--surface', 'карточки, рейл, поля'],
  ['--surface-sunken', 'шапки, вдавленные зоны'],
  ['--line', 'границы'],
  ['--ink', 'основной текст'],
  ['--ink-secondary', 'описания'],
  ['--ink-muted', 'метаданные, пути'],
];
