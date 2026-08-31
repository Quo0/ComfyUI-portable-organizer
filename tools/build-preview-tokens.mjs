// Builds apps/ui-design/.vitepress/theme/preview-tokens.css from
// apps/desktop/src/styles/tokens.css — the source now lives in the app, and the
// showcase only projects the same values onto .t-light/.t-dark.
//
// Why this exists at all instead of a direct import: the ThemePair.vue panels
// show both app themes at once, independently of the viewer's own theme.
// `:root[data-theme="dark"]` will not fire inside a page — `:root` matches only
// `<html>`, not an arbitrary `<div>`. Metrics (spacing, radii, typography,
// timings) do not depend on the theme and go onto `:root` the same way as in the
// app itself.
//
// Run: node tools/build-preview-tokens.mjs

import { writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { loadTokens, APPS_UI_DESIGN } from './lib/style-tokens.mjs';

const { lightBody, darkBody, metricsBody } = loadTokens();

const out = `/* GENERATED from apps/desktop/src/styles/tokens.css — do not edit.
 * Rebuild: pnpm ui-design:tokens
 * The same values as in the app, but on .t-light/.t-dark instead of :root —
 * ThemePair.vue shows both themes side by side regardless of the viewer's theme.
 */

.t-light {${lightBody}}

.t-dark {${darkBody}}

:root {${metricsBody}}
`;

const outPath = join(APPS_UI_DESIGN, '.vitepress', 'theme', 'preview-tokens.css');
writeFileSync(outPath, out, 'utf8');
console.log('apps/ui-design/.vitepress/theme/preview-tokens.css — updated');
