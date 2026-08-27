import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import { router } from './router';
import { i18n } from './i18n';
import { useUiStore } from './stores/ui';

import './styles/tokens.css';
import './styles/components.css';
import './styles/shell.css';

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(i18n);

const ui = useUiStore();
ui.init().finally(() => app.mount('#app'));
