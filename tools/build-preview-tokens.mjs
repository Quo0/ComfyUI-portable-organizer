// Строит apps/design/.vitepress/theme/preview-tokens.css из
// apps/desktop/src/styles/tokens.css — источник теперь в приложении,
// витрина только проецирует те же значения на .t-light/.t-dark.
//
// Зачем это вообще нужно, а не прямой импорт: панели ThemePair.vue
// показывают обе темы приложения одновременно, независимо от темы
// зрителя. `:root[data-theme="dark"]` внутри страницы не сработает —
// `:root` матчится только на `<html>`, а не на произвольный `<div>`.
// Метрики (отступы, радиусы, типографика, тайминги) от темы не зависят
// и идут на `:root` тем же способом, что и в самом приложении.
//
// Запуск: node tools/build-preview-tokens.mjs

import { writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { loadTokens, APPS_DESIGN } from './lib/style-tokens.mjs';

const { lightBody, darkBody, metricsBody } = loadTokens();

const out = `/* СГЕНЕРИРОВАНО из apps/desktop/src/styles/tokens.css — не редактировать.
 * Пересобрать: pnpm design:tokens
 * Те же значения, что в приложении, но на .t-light/.t-dark вместо :root —
 * ThemePair.vue показывает обе темы рядом независимо от темы зрителя.
 */

.t-light {${lightBody}}

.t-dark {${darkBody}}

:root {${metricsBody}}
`;

const outPath = join(APPS_DESIGN, '.vitepress', 'theme', 'preview-tokens.css');
writeFileSync(outPath, out, 'utf8');
console.log('apps/design/.vitepress/theme/preview-tokens.css — обновлён');
