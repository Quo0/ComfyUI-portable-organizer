// Palette and mockup checks before publishing. Exits with code 1 on a violation.
// Run: node tools/check-styles.mjs (or pnpm ui-design:check — which first rebuilds
// apps/ui-design/.vitepress/theme/preview-tokens.css).
//
// Sources: apps/desktop/src/styles/{tokens,components}.css (the app, source of
// truth) and apps/ui-design/{styleguide,screens,components,.vitepress} (the
// VitePress showcase — the mockups and its own page chrome).

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { createRequire } from 'node:module';
import {
  loadTokens, APPS_UI_DESIGN, COMPONENTS_SRC, UI_DESIGN_CHROME_SRC,
  ACCENTS, ratio, isHex,
} from './lib/style-tokens.mjs';

const { light, dark, metrics } = loadTokens();
const problems = [];
const note = (m) => problems.push(m);

// --------------------------------------------------------------- utilities

function listFiles(dir, exts) {
  const out = [];
  for (const entry of readdirSync(dir)) {
    if (entry === 'node_modules' || entry.startsWith('.')) continue;
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) out.push(...listFiles(full, exts));
    else if (exts.some((e) => entry.endsWith(e))) out.push(full);
  }
  return out;
}

const designMdFiles = [
  ...listFiles(join(APPS_UI_DESIGN, 'styleguide'), ['.md']),
  ...listFiles(join(APPS_UI_DESIGN, 'screens'), ['.md']),
];
const designVueFiles = listFiles(join(APPS_UI_DESIGN, 'components'), ['.vue']);
const designSources = [...designMdFiles, ...designVueFiles];
const designText = designSources.map((f) => readFileSync(f, 'utf8')).join('\n');

// 1. Dark theme completeness -------------------------------------------------
// A token declared only in the light block stays undefined in the dark theme —
// the component falls apart in exactly one of the three theme states.
for (const key of light.keys()) if (!dark.has(key)) note(`missing in the dark theme: ${key}`);
for (const key of dark.keys()) if (!light.has(key)) note(`missing in the light theme: ${key}`);

// 2. Every used variable is defined -------------------------------------------
// Both the app's tokens and the preview page's own tokens (--page-*) count; the
// latter are declared in apps/ui-design/.vitepress/theme/style.css, which is part
// of the scanned area itself: apps/ui-design pulls it in as an external file
// rather than inlining it into the markup, but the rules inside it still do
// reach the browser, and without that text the check would not see tokens used
// only in component CSS rules rather than inline.
const componentsText = readFileSync(COMPONENTS_SRC, 'utf8');
const chromeText = readFileSync(UI_DESIGN_CHROME_SRC, 'utf8');
const varScanText = designText + '\n' + componentsText + '\n' + chromeText;

const definedVars = new Set([...light.keys(), ...dark.keys(), ...metrics.keys(), ...[...varScanText.matchAll(/(--[\w-]+)\s*:/g)].map((m) => m[1])]);
const usedVars = new Set([...varScanText.matchAll(/var\((--[\w-]+)/g)].map((m) => m[1]));
// `var(--accent-${a.key})` in Roles.vue/Swatches.vue is a JS template literal,
// not a finished token name; the regex stops at `${` and catches only the stub
// `--accent-`. We drop the stubs and add the accents explicitly: the components
// really do read each of them in a loop over ACCENTS, and that is their
// "usage" — just not as text a static regex can catch.
for (const v of [...usedVars]) if (v.endsWith('-')) usedVars.delete(v);
for (const [key] of ACCENTS) usedVars.add(`--accent-${key}`);
for (const v of usedVars) if (!definedVars.has(v)) note(`used but not defined: ${v}`);

// An app token that is declared but never used anywhere is a signal that the
// palette has drifted from the mockup.
for (const key of light.keys()) if (!usedVars.has(key)) note(`declared but not used: ${key}`);

// 3. Contrast ----------------------------------------------------------------
const AA_TEXT = 4.5;
const AA_LARGE = 3;

const textPairs = [
  ['--ink', '--ground', AA_TEXT],
  ['--ink', '--surface', AA_TEXT],
  ['--ink-secondary', '--ground', AA_TEXT],
  ['--ink-secondary', '--surface', AA_TEXT],
  ['--ink-muted', '--ground', AA_TEXT],
  ['--ink-muted', '--surface', AA_TEXT],
  ['--state-stopped', '--surface', AA_TEXT],
  ['--state-starting', '--surface', AA_TEXT],
  ['--state-running', '--surface', AA_TEXT],
  ['--state-crashed', '--surface', AA_TEXT],
  ['--state-unavailable', '--surface', AA_LARGE],
  ['--console-ink', '--console-bg', AA_TEXT],
  ['--console-dim', '--console-bg', AA_LARGE],
  ['--btn-primary-ink', '--btn-primary-bg', AA_TEXT],
  ['--btn-primary-ink', '--btn-primary-hover', AA_TEXT],
  ['--line', '--surface', 1.2],
];

for (const [theme, tokens] of [['light', light], ['dark', dark]]) {
  for (const [fg, bg, min] of textPairs) {
    const r = ratio(tokens.get(fg), tokens.get(bg));
    if (r < min) note(`${theme}: ${fg} on ${bg} = ${r}:1, needs ${min}:1`);
  }
  // Accents must be readable on their own background — the user picks the
  // instance colour themselves, and offering a knowingly unreadable one is not
  // allowed.
  for (const [key] of ACCENTS) {
    const r = ratio(tokens.get(`--accent-${key}`), tokens.get('--ground'));
    if (r < AA_LARGE) note(`${theme}: accent-${key} on the background = ${r}:1, needs ${AA_LARGE}:1`);
    const rs = ratio(tokens.get(`--accent-${key}`), tokens.get('--surface'));
    if (rs < AA_LARGE) note(`${theme}: accent-${key} on a surface = ${rs}:1, needs ${AA_LARGE}:1`);
  }
}

// 4. Values are valid colours -------------------------------------------------
// Shadows are not colours, they have their own grammar; everything else must be
// hex, otherwise the contrast calculation silently skips the token.
for (const [theme, tokens] of [['light', light], ['dark', dark]]) {
  for (const [k, v] of tokens) {
    if (k.includes('shadow')) continue;
    if (!isHex(v)) note(`${theme}: ${k} = «${v}» — expected hex`);
  }
}

// 5. Frames with scrolling ----------------------------------------------------
// The `scroll` prop on <Window> is not the same as `fixed`: `fixed` also exists
// purely for a fixed height (an hd frame needs one to compute its scale),
// without claiming "this actually scrolls". `scroll` claims exactly that, and
// a frame holding less data than its area proves precisely nothing.
const screensText = designMdFiles
  .filter((f) => f.includes(join('apps', 'ui-design', 'screens')))
  .map((f) => ({ file: f, text: readFileSync(f, 'utf8') }));

const EXPECTED_SCROLL = 8;
const fixedWindows = [];
for (const { file, text } of screensText) {
  for (const m of text.matchAll(/<Window\b[^>]*>/g)) {
    if (/\bscroll\b/.test(m[0])) fixedWindows.push({ file, index: m.index, text });
  }
}
if (fixedWindows.length !== EXPECTED_SCROLL) {
  note(`frames with scrolling (<Window scroll>): ${fixedWindows.length}, expected ${EXPECTED_SCROLL}`);
}

// Row heights, so that content can be counted in "row units".
const ROW_UNITS = [
  [/class="path-item[ "]/g, 1],
  [/class="cat[ "]/g, 1],
  [/class="wf-row[ "]/g, 1],
  [/class="compat-row[ "]/g, 1],
  [/class="prog"/g, 2],
  [/class="card[ "]/g, 3],
];
const MIN_UNITS = 14;

fixedWindows.forEach(({ file, index, text }, i) => {
  // A frame's content runs from the opening <Window ...> to the matching
  // </Window>. There are no nested <Window> elements in the mockups, so we look
  // for the first closing tag.
  const closeAt = text.indexOf('</Window>', index);
  const block = closeAt === -1 ? text.slice(index) : text.slice(index, closeAt);

  if (!/class="scroll"|class="log"/.test(block)) {
    note(`${file}: scrolling frame #${i + 1} — its content has no scroll area (.scroll/.log)`);
    return;
  }

  let units = ROW_UNITS.reduce((sum, [re, weight]) => sum + (block.match(re) || []).length * weight, 0);
  if (/class="cards grid"/.test(block)) {
    units -= (block.match(/class="card[ "]/g) || []).length * 1.5;
  }
  const consoleAt = block.indexOf('class="console"');
  if (consoleAt !== -1) {
    const consoleText = block.slice(consoleAt, block.indexOf('</div>', consoleAt));
    units += (consoleText.match(/\n/g) || []).length * 0.6;
  }

  if (units < MIN_UNITS) {
    note(`${file}: scrolling frame #${i + 1} — ${units.toFixed(1)} rows of data against a threshold of ${MIN_UNITS}, it will not scroll`);
  }
});

// The running instance screen must have no scrolling at all: it would shift the
// rectangle the native ComfyUI window is positioned by.
const runningScreen = screensText.find((s) => s.file.endsWith('instance-running.md'));
if (runningScreen && /class="scroll"/.test(runningScreen.text)) {
  note('screens/instance-running.md: there is a scroll area — the webview rectangle will drift');
}

// 6. Every icon used is a real icon from @lucide/vue --------------------------
// The icons in apps/ui-design are the same @lucide/vue components as in the app
// (apps/desktop). The check is that every icon imported in the mockups really
// exists in the package and is not a typo.
const require_ = createRequire(join(APPS_UI_DESIGN, 'package.json'));
let lucideExports;
try {
  lucideExports = new Set(Object.keys(require_('@lucide/vue')));
} catch {
  lucideExports = null;
}
if (!lucideExports) {
  note('could not load @lucide/vue from apps/ui-design/node_modules — the icon check did not run, pnpm install is needed');
} else {
  const usedIcons = new Set();
  for (const m of designText.matchAll(/import\s*\{([^}]*)\}\s*from\s*'@lucide\/vue'/g)) {
    for (const name of m[1].split(',')) {
      const trimmed = name.trim();
      if (trimmed) usedIcons.add(trimmed);
    }
  }
  for (const icon of usedIcons) if (!lucideExports.has(icon)) note(`icon «${icon}» is used but does not exist in @lucide/vue`);
}

// 7. Long-text proof rows -----------------------------------------------------
const MIN_LONGFORM = 6;
const longRuns = (designText.match(/class="longform"/g) || []).length;
if (longRuns < MIN_LONGFORM) {
  note(`long-text proof rows: ${longRuns}, expected at least ${MIN_LONGFORM}`);
}

// ---------------------------------------------------------------------------
if (problems.length) {
  console.error('CHECKS FAILED:\n' + problems.map((p) => '  · ' + p).join('\n'));
  process.exit(1);
}
console.log(`Checks passed: ${light.size} tokens per theme, ${usedVars.size} used variables, ${ACCENTS.length} accents, ${fixedWindows.length} scrolling frames.`);
