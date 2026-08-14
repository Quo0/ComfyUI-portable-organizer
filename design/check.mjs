// Проверки палитры до публикации. Падает с кодом 1 при нарушении.
// Запуск: node design/check.mjs

import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { loadTokens, DESIGN_DIR, ACCENTS, ratio, isHex } from './lib/tokens.mjs';

const { light, dark, metrics } = loadTokens();
const problems = [];
const note = (m) => problems.push(m);

// 1. Полнота тёмной темы -----------------------------------------------------
// Токен, объявленный только в светлом блоке, остаётся неопределённым в тёмной
// теме — компонент разваливается ровно в одном из трёх состояний темы.
for (const key of light.keys()) if (!dark.has(key)) note(`нет в тёмной теме: ${key}`);
for (const key of dark.keys()) if (!light.has(key)) note(`нет в светлой теме: ${key}`);

// 2. Все используемые переменные определены ----------------------------------
// Учитываем и токены приложения, и собственные токены страницы просмотра
// (--page-*), которые объявлены прямо в ней и по замыслу в tokens.css не входят.
const PAGES = ['preview.html', 'screens.html'];
const html = PAGES.map((f) => readFileSync(join(DESIGN_DIR, f), 'utf8')).join('\n');
const inPage = [...html.matchAll(/(--[\w-]+)\s*:/g)].map((m) => m[1]);
const defined = new Set([...light.keys(), ...dark.keys(), ...metrics.keys(), ...inPage]);
const used = new Set([...html.matchAll(/var\((--[\w-]+)/g)].map((m) => m[1]));
for (const v of used) if (!defined.has(v)) note(`не определена, но используется: ${v}`);

// Токены приложения, объявленные, но нигде не использованные, — сигнал,
// что палитра разошлась с макетом.
for (const key of light.keys()) if (!used.has(key)) note(`объявлен, но не используется: ${key}`);

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

// 5. Пары тем не пишутся руками ------------------------------------------------
// Панель светлой и тёмной темы обязана разворачиваться из одного <template>:
// продублированная руками пара со временем расходится по содержимому,
// и сравнивать темы становится не с чем.
const src = ['preview.src.html', 'screens.src.html']
  .map((f) => readFileSync(join(DESIGN_DIR, f), 'utf8')).join('\n');
const manual = (src.match(/<figure class="panel/g) || []).length;
if (manual) note(`пар, продублированных руками: ${manual} — используйте <template> и data-pair`);

// 6. Кадры с прокруткой ---------------------------------------------------------
// Окно фиксированной высоты — единственный способ показать прокрутку.
// Без него макет просто вырастет, и демонстрировать будет нечего.
const screens = readFileSync(join(DESIGN_DIR, 'screens.html'), 'utf8');
const EXPECTED_SCROLL = 7;

const scrollCount = (screens.match(/data-scroll/g) || []).length;
if (scrollCount !== EXPECTED_SCROLL) {
  note(`макетов с прокруткой: ${scrollCount}, ожидалось ${EXPECTED_SCROLL}`);
}

// Каждый такой макет обязан иметь фиксированную высоту и область прокрутки.
for (const m of screens.matchAll(/<div class="frame" data-scroll>[\s\S]{0,200}?<div class="win([^"]*)"/g)) {
  if (!m[1].includes('fixed')) note('макет с прокруткой без фиксированной высоты окна');
}

// Высота строк, чтобы считать содержимое в «единицах строки». Числа взяты
// из самих компонентов: `.path-item` — две трети отступа плюс строка,
// `.prog` — заголовок, полоса и путь, `.card` — три строки с полями.
// `.card` весит три строки, но в `.cards.grid` карточки идут по две в ряд,
// и высоту даёт ряд, а не карточка. Без поправки кадр с карточками набирал
// вдвое больше единиц, чем занимает на экране, и порог проходил бы, даже
// если данных вдвое меньше нужного.
const ROW_UNITS = [
  [/class="path-item[ "]/g, 1],
  [/class="cat[ "]/g, 1],
  [/class="wf-row[ "]/g, 1],
  [/class="compat-row[ "]/g, 1],
  [/class="prog"/g, 2],
  [/class="card[ "]/g, 3],
];
// Область данных в окне на 560 пикселей — примерно 400. Порог с запасом:
// полоса прокрутки, появляющаяся на два пикселя, ничего не доказывает.
const MIN_UNITS = 14;

// Область прокрутки ищется внутри контентной части, а не во всём макете:
// прокручивающийся блок рейла есть в каждом окне и удовлетворил бы проверку,
// даже если сами данные никуда не скроллятся.
const scrollBlocks = screens.split('data-scroll').slice(1);
scrollBlocks.forEach((block, i) => {
  const frame = block.split('</section>')[0].split('variant-head')[0];
  const contentAt = frame.indexOf('class="content');
  const content = contentAt === -1 ? '' : frame.slice(contentAt);
  if (!/class="scroll"|class="log"/.test(content)) {
    note(`макет с прокруткой №${i + 1}: в контентной части нет области прокрутки`);
    return;
  }

  // Структура — половина дела: пока данных меньше области, прокрутки
  // не возникает, и кадр доказывает ровно ничего. Пять таких кадров
  // прошли проверку и висели пустыми, пока не заметили глазами.
  let units = ROW_UNITS.reduce(
    (sum, [re, weight]) => sum + (content.match(re) || []).length * weight,
    0,
  );
  // Половина карточек в сетке стоит вторым столбцом и высоты не добавляет.
  if (/class="cards grid"/.test(content)) {
    units -= (content.match(/class="card[ "]/g) || []).length * 1.5;
  }
  const consoleAt = content.indexOf('class="console"');
  if (consoleAt !== -1) {
    const text = content.slice(consoleAt, content.indexOf('</div>', consoleAt));
    units += (text.match(/\n/g) || []).length * 0.6;
  }

  if (units < MIN_UNITS) {
    note(
      `макет с прокруткой №${i + 1}: данных на ${units.toFixed(1)} строки`
      + ` при пороге ${MIN_UNITS} — прокрутки не будет`,
    );
  }
});

// На экране работающего инстанса прокрутки быть не должно: она сдвинула бы
// прямоугольник, по которому позиционируется нативное окно ComfyUI.
const running = screens.slice(screens.indexOf('Инстанс работает'));
const runningScreen = running.slice(0, running.indexOf('</section>'));
if (/class="scroll"/.test(runningScreen)) {
  note('на экране работающего инстанса есть область прокрутки — прямоугольник вебвью разъедется');
}

// 7. dist/components.css содержит только компоненты -----------------------------
// Файл подключается в приложении глобально; попавшее туда обрамление страниц
// просмотра ломало бы вёрстку приложения.
const components = readFileSync(join(DESIGN_DIR, 'dist', 'components.css'), 'utf8');
for (const leak of ['.band', '.masthead', '.wrap', '.swatches', '.longform', '--page-', '@TOKENS@']) {
  if (components.includes(leak)) note(`в components.css просочилось «${leak}» — это только для страниц просмотра`);
}
// И наоборот: ключевые компоненты обязаны там быть.
for (const need of ['.nav', '.card', '.btn', '.scroll', '.chip', '.toast']) {
  if (!components.includes(need)) note(`в components.css нет «${need}»`);
}

// 8. Строки проверки на длинных текстах ---------------------------------------
const MIN_LONGFORM = 6;
const longRuns = (html.match(/data-longform/g) || []).length;
if (longRuns < MIN_LONGFORM) {
  note(`строк проверки на длинных текстах: ${longRuns}, ожидалось ${MIN_LONGFORM}`);
}

// ---------------------------------------------------------------------------
if (problems.length) {
  console.error('ПРОВЕРКИ НЕ ПРОЙДЕНЫ:\n' + problems.map((p) => '  · ' + p).join('\n'));
  process.exit(1);
}
console.log(`Проверки пройдены: ${light.size} токенов на тему, ${used.size} использованных переменных, ${ACCENTS.length} акцентов.`);
