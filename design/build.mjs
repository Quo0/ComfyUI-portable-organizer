// Собирает из tokens.css два артефакта:
//   dist/tokens.css  — рабочий файл приложения, все три состояния темы
//   preview.html     — страница просмотра, обе темы на одной странице
//
// Запуск: node design/build.mjs

import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { loadTokens, DESIGN_DIR, ACCENTS, ratio } from './lib/tokens.mjs';

const { lightBody, darkBody, metricsBody, light, dark } = loadTokens();

// ------------------------------------------------- dist/tokens.css (приложение)

// Тёмные объявления повторяются дважды: медиа-запрос нельзя объединить
// с обычным селектором в один блок. Повтор генерируется, а не пишется руками.
const appCss = `/* СГЕНЕРИРОВАНО из design/tokens/tokens.css — не редактировать.
 * Пересобрать: node design/build.mjs
 */

:root {${lightBody}}

@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {${darkBody.replace(/\n  /g, '\n    ')}}
}

:root[data-theme="dark"] {${darkBody}}

:root {${metricsBody}}
`;

mkdirSync(join(DESIGN_DIR, 'dist'), { recursive: true });
writeFileSync(join(DESIGN_DIR, 'dist', 'tokens.css'), appCss, 'utf8');

// ------------------------------------------------------ инлайн для превью

// На странице просмотра токены приложения НЕ попадают на :root — иначе
// обрамление самой страницы окрасилось бы светлой темой приложения
// независимо от темы зрителя. Только .t-light / .t-dark.
const previewTokens = [
  `.t-light {${lightBody}}`,
  `.t-dark {${darkBody}}`,
  `:root {${metricsBody}}`,
].join('\n\n');

// ------------------------------------------------ таблица ролей палитры
// Значения печатаются из токенов, а не вписываются в разметку руками —
// иначе подписанный hex разойдётся с реальным цветом при первой же правке.
const ROLES = [
  ['--ground', 'основание окна'],
  ['--surface', 'карточки, рейл, поля'],
  ['--surface-sunken', 'шапки, вдавленные зоны'],
  ['--line', 'границы'],
  ['--ink', 'основной текст'],
  ['--ink-secondary', 'описания'],
  ['--ink-muted', 'метаданные, пути'],
];

const rolesFor = (tokens) => ROLES.map(([name, what]) =>
  `<div class="role"><i style="background:var(${name})"></i><div><b>${name.slice(2)}</b><span>${what}</span></div><code>${tokens.get(name)}</code></div>`,
).join('\n');

const rolesPair = `<div class="pair">
<figure class="panel t-light"><figcaption>Светлая</figcaption><div class="app"><div class="roles">
${rolesFor(light)}
</div></div></figure>
<figure class="panel t-dark"><figcaption>Тёмная</figcaption><div class="app"><div class="roles">
${rolesFor(dark)}
</div></div></figure>
</div>`;

// ---------------------------------------------------- сетка акцентов

const swatches = ACCENTS.map(([key, label]) => {
  const l = light.get(`--accent-${key}`);
  const d = dark.get(`--accent-${key}`);
  // Цвет берётся переменной, а не подставленным hex: образец должен меняться
  // вместе с палитрой, иначе сетка разойдётся с токенами при первой же правке.
  return `            <div class="sw">
              <div class="sw-chips">
                <span class="sw-chip t-light" style="background:var(--ground)"><i style="background:var(--accent-${key})"></i></span>
                <span class="sw-chip t-dark" style="background:var(--ground)"><i style="background:var(--accent-${key})"></i></span>
              </div>
              <div class="sw-name">${label}</div>
              <div class="sw-meta"><code>${l}</code> · ${ratio(l, light.get('--ground'))}:1</div>
              <div class="sw-meta"><code>${d}</code> · ${ratio(d, dark.get('--ground'))}:1</div>
            </div>`;
}).join('\n');

// ---------------------------------------------------------------- сборка

// ------------------------------------------------- пары «светлая / тёмная»
// Содержимое панели пишется один раз в <template>, обе темы разворачиваются
// при сборке. Руками дублировать нельзя: тринадцать пар разойдутся
// по содержимому, и сравнивать темы станет не с чем.
function expandPairs(src) {
  let count = 0;
  const out = src.replace(
    /<div class="pair" data-pair="([^"]*)">\s*<template>([\s\S]*?)<\/template>\s*<\/div>/g,
    (_, caption, body) => {
      count++;
      const [lightCap, darkCap] = caption.split('|');
      const panel = (theme, cap) =>
        `<figure class="panel t-${theme}">\n<figcaption>${cap}</figcaption>\n<div class="app">${body}</div>\n</figure>`;
      return `<div class="pair">\n${panel('light', lightCap)}\n${panel('dark', darkCap)}\n</div>`;
    },
  );
  return { out, count };
}

// ------------------------------------------------- строки для прокрутки
// Кадр с прокруткой доказывает правило «прокручивается только область
// данных», а доказать его можно лишь настоящим объёмом: содержимое обязано
// быть выше области. Двадцать строк, выписанных руками, расходятся между
// собой ровно как пары тем выше — и раздувают исходник, который и без того
// на тысячу восемьсот строк.
//
// Содержимое пишется один раз, сборка повторяет его нужное число раз:
//   <template data-repeat="5">…несколько разных строк…</template>
function expandRepeats(src) {
  let count = 0;
  const out = src.replace(
    /<template data-repeat="(\d+)">([\s\S]*?)<\/template>/g,
    (_, times, body) => {
      count++;
      return body.repeat(Number(times));
    },
  );
  return { out, count };
}

// Обе страницы делят один стилевой файл: копия стилей на второй странице
// разошлась бы с первой, и компоненты в экранах перестали бы совпадать
// с образцами.
const sheet = readFileSync(join(DESIGN_DIR, 'styles', 'app.css'), 'utf8');

// ------------------------------------------------- dist/components.css
// В приложение едут только компоненты. Обрамление страниц просмотра —
// полосы, колонки трассировки, сетка образцов — отсеивается по списку
// классов, которые существуют только здесь.
const PREVIEW_ONLY = new Set([
  'wrap', 'masthead', 'eyebrow', 'lede', 'facts', 'band', 'rail', 'ids', 'stack',
  'sub', 'aside', 'pair', 'panel', 'app', 'roles', 'role', 'swatches', 'sw',
  'sw-chips', 'sw-chip', 'sw-name', 'sw-meta', 'longform', 'lf-head', 'lf-rows',
  'lf-row', 'closing', 'scale', 'scale-row',
]);

function componentsOnly(css) {
  const stripped = css.replace(/\/\*[\s\S]*?\*\//g, '');
  const out = [];
  let i = 0;
  while (i < stripped.length) {
    const open = stripped.indexOf('{', i);
    if (open === -1) break;
    let depth = 1;
    let j = open + 1;
    while (j < stripped.length && depth > 0) {
      if (stripped[j] === '{') depth++;
      else if (stripped[j] === '}') depth--;
      j++;
    }
    const selector = stripped.slice(i, open).trim();
    const rule = stripped.slice(i, j).trim();
    i = j;
    if (!selector) continue;
    // Токены живут в отдельном файле; блок --page-* принадлежит просмотру.
    if (selector.startsWith(':root') || selector.startsWith('@media (prefers-color-scheme')) continue;
    if (selector === '*' || selector === 'body') continue;

    // В @media и @supports надо заходить внутрь: иначе блок проходит целиком
    // вместе с правилами обрамления, которые лежат в нём.
    if (selector.startsWith('@media') || selector.startsWith('@supports')) {
      const inner = componentsOnly(stripped.slice(open + 1, j - 1));
      if (inner.trim()) out.push(`${selector} {\n${inner}\n}`);
      continue;
    }
    if (selector.startsWith('@')) { out.push(rule); continue; }

    const classes = [...selector.matchAll(/\.([\w-]+)/g)].map((m) => m[1]);
    if (classes.some((c) => PREVIEW_ONLY.has(c))) continue;
    out.push(rule);
  }
  return out.join('\n');
}

writeFileSync(
  join(DESIGN_DIR, 'dist', 'components.css'),
  '/* СГЕНЕРИРОВАНО из design/styles/app.css — не редактировать.\n'
  + ' * Пересобрать: node design/build.mjs\n'
  + ' * Подключается глобально один раз, вместе с tokens.css.\n */\n\n'
  + componentsOnly(sheet) + '\n',
  'utf8',
);

function render(srcName, outName, artifactName) {
  const tpl = readFileSync(join(DESIGN_DIR, srcName), 'utf8');
  const substituted = tpl
    .replace('/* @STYLES@ */', () => sheet)
    .replace('/* @TOKENS@ */', () => previewTokens)
    .replace('<!-- @ACCENTS@ -->', () => swatches)
    .replace('<!-- @ROLES@ -->', () => rolesPair);
  // Порядок не важен: пары и повторы независимы, вложенных шаблонов нет.
  const { out: paired, count } = expandPairs(substituted);
  const { out: html } = expandRepeats(paired);

  // Полный документ — для локального просмотра двойным кликом.
  writeFileSync(join(DESIGN_DIR, outName), html, 'utf8');

  // Версия для публикации: артефакт сам оборачивает содержимое в
  // <!doctype>/<head>/<body>, поэтому отдаём стили плюс содержимое body.
  const styles = [...html.matchAll(/<style>[\s\S]*?<\/style>/g)].map((m) => m[0]).join('\n');
  const bodyInner = html.match(/<body>([\s\S]*)<\/body>/)[1];
  writeFileSync(join(DESIGN_DIR, 'dist', artifactName), styles + '\n' + bodyInner, 'utf8');
  return count;
}

const pairCount = render('preview.src.html', 'preview.html', 'artifact.html');
const screenCount = render('screens.src.html', 'screens.html', 'screens.html');

const metricCount = metricsBody.match(/--[\w-]+\s*:/g).length;
console.log(`dist/tokens.css  — ${light.size} токенов на тему + ${metricCount} метрик`);
console.log(`preview.html     — ${ACCENTS.length} акцентов, ${pairCount} пар «светлая/тёмная»`);
console.log(`screens.html     — экраны по сценариям`);
console.log('dist/            — версии для публикации');
