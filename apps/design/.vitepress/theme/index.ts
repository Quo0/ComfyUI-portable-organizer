import DefaultTheme from 'vitepress/theme';
import type { Theme } from 'vitepress';

// Компоненты приложения — источник правды в apps/desktop, читаются отсюда
// напрямую, без копий: правка исходника видна сразу на дев-сервере.
// Токены на .t-light/.t-dark — производные (см. Layout.vue и
// tools/build-preview-tokens.mjs), пересобираются командой `design:tokens`.
// Порядок важен только для читаемости, все три файла объявляют независимые
// селекторы, пересечений нет.
import '../../../../apps/desktop/src/styles/components.css';
// Точечные стили Layout/UI-компонентов приложения — переехали из
// components.css в <style src="./Name.css"> рядом с каждым SFC.
// Без `scoped`: витрина подключает эти файлы как обычный CSS, минуя
// компилятор Vue SFC, а `:slotted()`/`:deep()` — конструкции именно
// этого компилятора и молча не матчатся вне его. Часть этих компонентов
// (ScreenHeader, StepBar, KeyValueRow) стилизует содержимое своего
// слота — оно и есть причина не заводить `scoped` тут вовсе, а не только
// повод для витрины.
// Витрина по-прежнему читает исходники напрямую, просто теперь их
// несколько, а не один общий файл. Список пополняется по мере миграции.
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
  // Свой каркас страницы — см. комментарий в Layout.vue. От темы по
  // умолчанию остаются только типографика `.vp-doc` и страница 404.
  Layout,
  enhanceApp({ app }) {
    app.component('ThemePair', ThemePair);
    app.component('Window', Window);
    app.component('Roles', Roles);
    app.component('Swatches', Swatches);
  },
} satisfies Theme;
