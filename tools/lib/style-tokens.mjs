// Parsing of apps/desktop/src/styles/tokens.css and colour helpers.
// Shared code for tools/build-preview-tokens.mjs and tools/check-styles.mjs.
//
// The source is the app: `:root {light}`, `@media (prefers-color-scheme: dark)
// { :root:not(...) {dark} }`, `:root[data-theme="dark"] {dark}`, then a second
// `:root {metrics}` — the same as in the app, since metrics do not depend on
// the theme. The media block is a duplicate of the dark theme for the system
// preference; we do not keep a second source for it.

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

export const ROOT_DIR = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
export const APP_STYLES_DIR = join(ROOT_DIR, 'apps', 'desktop', 'src', 'styles');
export const TOKENS_SRC = join(APP_STYLES_DIR, 'tokens.css');
export const COMPONENTS_SRC = join(APP_STYLES_DIR, 'components.css');
export const APPS_DESIGN = join(ROOT_DIR, 'apps', 'design');
export const DESIGN_CHROME_SRC = join(APPS_DESIGN, '.vitepress', 'theme', 'style.css');

/** Extracts a block body by selector name, starting the search at `from`.
 *  The selector is matched from the start of a line — otherwise a mention in
 *  a comment would match. The end of the block is returned too: it is needed to
 *  find the next block with the same selector name (there are two in the file:
 *  the light theme and the metrics, both `:root`). */
function blockBody(css, selector, from = 0) {
  const anchor = new RegExp(`^${selector.replace(/[.[\]"]/g, '\\$&')}\\s*\\{`, 'm');
  const found = anchor.exec(css.slice(from));
  if (!found) throw new Error(`Block ${selector} not found`);
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
  throw new Error(`Block ${selector} is not closed`);
}

/** Pairs of "token name → value" from a block body. */
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

// --------------------------------------------------------------- colour

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

// Semantic palette roles, shown in the "Palette" section of the style guide.
export const ROLES = [
  ['--ground', 'window ground'],
  ['--surface', 'cards, rail, fields'],
  ['--surface-sunken', 'headers, sunken areas'],
  ['--line', 'borders'],
  ['--ink', 'body text'],
  ['--ink-secondary', 'descriptions'],
  ['--ink-muted', 'metadata, paths'],
];
