import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import { router } from './router';
import { i18n } from './i18n';
import { useUiStore } from './stores/ui';

// Порядок важен: токены объявляют переменные, компоненты их используют,
// каркас окна опирается и на то, и на другое.
import './styles/tokens.css';
import './styles/components.css';
import './styles/shell.css';

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(i18n);

// Тема и язык применяются до монтирования: иначе первый кадр успевает
// показаться светлым у того, кто выбрал тёмную, и на английском у того,
// кто выбрал русский.
const ui = useUiStore();
ui.init().finally(() => app.mount('#app'));
