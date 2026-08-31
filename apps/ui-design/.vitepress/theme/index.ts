import DefaultTheme from 'vitepress/theme';
import type { Theme } from 'vitepress';

// The app's styles are the source of truth and are imported straight from
// apps/desktop, with no copy here: an edit to the source shows up on the dev
// server at once. The `.t-light`/`.t-dark` tokens are the one derived file,
// rebuilt by `pnpm ui-design:tokens`.
import '../../../../apps/desktop/src/styles/components.css';
// The per-component styles of the app's Layout/UI components, one file each.
// The showcase imports them as plain CSS, bypassing the Vue SFC compiler —
// which is why those components carry no `scoped` style; see CLAUDE.md. The
// list grows as components are migrated.
import '../../../../apps/desktop/src/components/ui/Group.css';
import '../../../../apps/desktop/src/components/ui/StepBar.css';
import '../../../../apps/desktop/src/components/ui/InstanceHeader.css';
import '../../../../apps/desktop/src/components/ui/KeyValueList.css';
import '../../../../apps/desktop/src/components/ui/KeyValueRow.css';
import '../../../../apps/desktop/src/components/ui/Toggle.css';
import '../../../../apps/desktop/src/components/ui/ToggleRow.css';
import '../../../../apps/desktop/src/components/ui/Field.css';
import '../../../../apps/desktop/src/components/ui/Pane.css';
import '../../../../apps/desktop/src/components/ui/Card.css';
import './preview-tokens.css';
import './style.css';

import Layout from './Layout.vue';
import ThemePair from '../../components/ThemePair.vue';
import Window from '../../components/Window.vue';
import Roles from '../../components/Roles.vue';
import Swatches from '../../components/Swatches.vue';

export default {
  extends: DefaultTheme,
  // Our own page shell — see Layout.vue. All that is left of the default
  // theme is the `.vp-doc` typography and the 404 page.
  Layout,
  enhanceApp({ app }) {
    app.component('ThemePair', ThemePair);
    app.component('Window', Window);
    app.component('Roles', Roles);
    app.component('Swatches', Swatches);
  },
} satisfies Theme;
