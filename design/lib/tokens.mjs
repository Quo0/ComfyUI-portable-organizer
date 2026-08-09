// Разбор tokens.css и работа с цветом. Общий код для build.mjs и check.mjs.

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

export const DESIGN_DIR = dirname(dirname(fileURLToPath(import.meta.url)));
export const TOKENS_SRC = join(DESIGN_DIR, 'tokens', 'tokens.css');

/** Достаёт тело блока по имени селектора: .theme-light { ... }
 *  Селектор ищется с начала строки — иначе совпадёт упоминание в комментарии. */
function blockBody(css, selector) {
  const anchor = new RegExp(`^${selector.replace(/[.[\]]/g, '\\$&')}\\s*\\{`, 'm');
  const found = anchor.exec(css);
  if (!found) throw new Error(`Блок ${selector} не найден в tokens.css`);
  const start = found.index;
  const open = css.indexOf('{', start);
  let depth = 0;
  for (let i = open; i < css.length; i++) {
    if (css[i] === '{') depth++;
    else if (css[i] === '}') {
      depth--;
      if (depth === 0) return css.slice(open + 1, i);
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
  const lightBody = blockBody(css, '.theme-light');
  const darkBody = blockBody(css, '.theme-dark');
  const metricsBody = blockBody(css, ':root');
  return {
    css,
    lightBody,
    darkBody,
    metricsBody,
    light: declarations(lightBody),
    dark: declarations(darkBody),
    metrics: declarations(metricsBody),
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
