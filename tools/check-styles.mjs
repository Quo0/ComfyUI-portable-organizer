// Проверки палитры и макетов до публикации. Падает с кодом 1 при нарушении.
// Запуск: node tools/check-styles.mjs (или pnpm design:check — сначала
// пересобирает apps/design/.vitepress/theme/preview-tokens.css).
//
// Источники: apps/desktop/src/styles/{tokens,components}.css (приложение,
// источник правды) и apps/design/{styleguide,screens,components,.vitepress}
// (VitePress-витрина — макеты и её собственная обвязка страницы).

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { createRequire } from 'node:module';
import {
  loadTokens, APPS_DESIGN, COMPONENTS_SRC, DESIGN_CHROME_SRC,
  ACCENTS, ratio, isHex,
} from './lib/style-tokens.mjs';

const { light, dark, metrics } = loadTokens();
const problems = [];
const note = (m) => problems.push(m);

// ---------------------------------------------------------------- утилиты

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
  ...listFiles(join(APPS_DESIGN, 'styleguide'), ['.md']),
  ...listFiles(join(APPS_DESIGN, 'screens'), ['.md']),
];
const designVueFiles = listFiles(join(APPS_DESIGN, 'components'), ['.vue']);
const designSources = [...designMdFiles, ...designVueFiles];
const designText = designSources.map((f) => readFileSync(f, 'utf8')).join('\n');

// 1. Полнота тёмной темы -----------------------------------------------------
// Токен, объявленный только в светлом блоке, остаётся неопределённым в тёмной
// теме — компонент разваливается ровно в одном из трёх состояний темы.
for (const key of light.keys()) if (!dark.has(key)) note(`нет в тёмной теме: ${key}`);
for (const key of dark.keys()) if (!light.has(key)) note(`нет в светлой теме: ${key}`);

// 2. Все использованные переменные определены ----------------------------------
// Учитываем и токены приложения, и собственные токены страницы просмотра
// (--page-*), которые объявлены в apps/design/.vitepress/theme/style.css —
// он сам входит в область сканирования: подключается в apps/design как
// внешний файл, а не инлайнится в разметку, но правила внутри него всё равно
// реально доезжают до браузера, и без этого текста проверка не увидит
// токены, использованные только в CSS-правилах компонентов, а не в inline.
const componentsText = readFileSync(COMPONENTS_SRC, 'utf8');
const chromeText = readFileSync(DESIGN_CHROME_SRC, 'utf8');
const varScanText = designText + '\n' + componentsText + '\n' + chromeText;

const definedVars = new Set([...light.keys(), ...dark.keys(), ...metrics.keys(), ...[...varScanText.matchAll(/(--[\w-]+)\s*:/g)].map((m) => m[1])]);
const usedVars = new Set([...varScanText.matchAll(/var\((--[\w-]+)/g)].map((m) => m[1]));
// `var(--accent-${a.key})` в Roles.vue/Swatches.vue — шаблонный литерал JS,
// а не готовое имя токена; regex обрывается на `${` и ловит только обрубок
// `--accent-`. Обрубки убираем, а сами акценты добавляем явно: компоненты
// реально читают каждый в цикле по ACCENTS, это и есть их «использование»,
// просто не текстом, который ловит статический regex.
for (const v of [...usedVars]) if (v.endsWith('-')) usedVars.delete(v);
for (const [key] of ACCENTS) usedVars.add(`--accent-${key}`);
for (const v of usedVars) if (!definedVars.has(v)) note(`не определена, но используется: ${v}`);

// Токены приложения, объявленные, но нигде не использованные, — сигнал,
// что палитра разошлась с макетом.
for (const key of light.keys()) if (!usedVars.has(key)) note(`объявлен, но не используется: ${key}`);

// 3. Контраст ----------------------------------------------------------------
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

for (const [theme, tokens] of [['светлая', light], ['тёмная', dark]]) {
  for (const [fg, bg, min] of textPairs) {
    const r = ratio(tokens.get(fg), tokens.get(bg));
    if (r < min) note(`${theme}: ${fg} на ${bg} = ${r}:1, нужно ${min}:1`);
  }
  // Акценты обязаны читаться на своём основании — пользователь выбирает
  // цвет инстанса сам, и предлагать заведомо нечитаемый нельзя.
  for (const [key] of ACCENTS) {
    const r = ratio(tokens.get(`--accent-${key}`), tokens.get('--ground'));
    if (r < AA_LARGE) note(`${theme}: accent-${key} на фоне = ${r}:1, нужно ${AA_LARGE}:1`);
    const rs = ratio(tokens.get(`--accent-${key}`), tokens.get('--surface'));
    if (rs < AA_LARGE) note(`${theme}: accent-${key} на поверхности = ${rs}:1, нужно ${AA_LARGE}:1`);
  }
}

// 4. Значения — валидные цвета -----------------------------------------------
// Тени — не цвет, у них своя грамматика; остальное обязано быть hex,
// иначе расчёт контраста молча пропустит токен.
for (const [theme, tokens] of [['светлая', light], ['тёмная', dark]]) {
  for (const [k, v] of tokens) {
    if (k.includes('shadow')) continue;
    if (!isHex(v)) note(`${theme}: ${k} = «${v}» — ожидался hex`);
  }
}

// 5. Кадры с прокруткой -------------------------------------------------------
// Проп `scroll` у <Window> — не то же самое, что `fixed`: `fixed` бывает
// и просто ради фиксированной высоты (hd-кадру она нужна для расчёта
// масштаба), без утверждения «тут реально скроллится». `scroll` именно
// это утверждает, и кадр, где данных меньше области, доказывает ровно
// ничего.
const screensText = designMdFiles
  .filter((f) => f.includes(join('apps', 'design', 'screens')))
  .map((f) => ({ file: f, text: readFileSync(f, 'utf8') }));

const EXPECTED_SCROLL = 8;
const fixedWindows = [];
for (const { file, text } of screensText) {
  for (const m of text.matchAll(/<Window\b[^>]*>/g)) {
    if (/\bscroll\b/.test(m[0])) fixedWindows.push({ file, index: m.index, text });
  }
}
if (fixedWindows.length !== EXPECTED_SCROLL) {
  note(`кадров с прокруткой (<Window scroll>): ${fixedWindows.length}, ожидалось ${EXPECTED_SCROLL}`);
}

// Высота строк, чтобы считать содержимое в «единицах строки».
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
  // Содержимое кадра — от открывающего <Window ...> до соответствующего
  // </Window>. Вложенных <Window> в макетах нет, ищем первое закрытие.
  const closeAt = text.indexOf('</Window>', index);
  const block = closeAt === -1 ? text.slice(index) : text.slice(index, closeAt);

  if (!/class="scroll"|class="log"/.test(block)) {
    note(`${file}: кадр с прокруткой №${i + 1} — в содержимом нет области прокрутки (.scroll/.log)`);
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
    note(`${file}: кадр с прокруткой №${i + 1} — данных на ${units.toFixed(1)} строки при пороге ${MIN_UNITS}, прокрутки не будет`);
  }
});

// На экране работающего инстанса прокрутки быть не должно: она сдвинула бы
// прямоугольник, по которому позиционируется нативное окно ComfyUI.
const runningScreen = screensText.find((s) => s.file.endsWith('instance-running.md'));
if (runningScreen && /class="scroll"/.test(runningScreen.text)) {
  note('screens/instance-running.md: есть область прокрутки — прямоугольник вебвью разъедется');
}

// 6. Каждый использованный значок — настоящий значок из @lucide/vue ----------
// Значки в apps/design те же компоненты @lucide/vue, что и в приложении
// (apps/desktop). Проверка — что каждый импортированный в макетах значок
// реально существует в пакете, а не опечатка.
const require_ = createRequire(join(APPS_DESIGN, 'package.json'));
let lucideExports;
try {
  lucideExports = new Set(Object.keys(require_('@lucide/vue')));
} catch {
  lucideExports = null;
}
if (!lucideExports) {
  note('не удалось загрузить @lucide/vue из apps/design/node_modules — проверка значков не выполнена, нужен pnpm install');
} else {
  const usedIcons = new Set();
  for (const m of designText.matchAll(/import\s*\{([^}]*)\}\s*from\s*'@lucide\/vue'/g)) {
    for (const name of m[1].split(',')) {
      const trimmed = name.trim();
      if (trimmed) usedIcons.add(trimmed);
    }
  }
  for (const icon of usedIcons) if (!lucideExports.has(icon)) note(`используется значок «${icon}», которого нет в @lucide/vue`);
}

// 7. Строки проверки на длинных текстах ---------------------------------------
const MIN_LONGFORM = 6;
const longRuns = (designText.match(/class="longform"/g) || []).length;
if (longRuns < MIN_LONGFORM) {
  note(`строк проверки на длинных текстах: ${longRuns}, ожидалось не меньше ${MIN_LONGFORM}`);
}

// ---------------------------------------------------------------------------
if (problems.length) {
  console.error('ПРОВЕРКИ НЕ ПРОЙДЕНЫ:\n' + problems.map((p) => '  · ' + p).join('\n'));
  process.exit(1);
}
console.log(`Проверки пройдены: ${light.size} токенов на тему, ${usedVars.size} использованных переменных, ${ACCENTS.length} акцентов, ${fixedWindows.length} кадров с прокруткой.`);
