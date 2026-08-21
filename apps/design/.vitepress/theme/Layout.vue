<script setup>
// Свой каркас страницы вместо VPNav/VPSidebar/VPContent темы по умолчанию.
// У формулы «после 1440px открывай поля» под колонку --vp-layout-max-width
// нет способа отключить точечно — она держится на брейкпоинтах 960/1440,
// продублированных в четырёх разных компонентах с разными формулами отступов.
// Своя разметка — три класса без брейкпоинтов вовсе: тулбар фиксированной
// высоты, сайдбар фиксированной ширины, контент занимает всё остальное.
//
// `.vp-doc` на <Content> — это всё, что берётся у темы по умолчанию:
// типографика заголовков/списков/кода/таблиц, без каркаса вокруг неё.
import { Content, useData, useRoute } from 'vitepress';
import { useSidebar } from 'vitepress/theme';

const { site, theme } = useData();
const route = useRoute();
const { sidebarGroups, hasSidebar } = useSidebar();

// cleanUrls снимает .html, но не хвостовой слэш у индексов разделов
// ('/styleguide/') — сравнение ссылок ведём без него.
function normalize(path) {
  return path.replace(/\/$/, '') || '/';
}

function isNavActive(link) {
  const target = normalize(link);
  return target === '/' ? normalize(route.path) === '/' : normalize(route.path).startsWith(target);
}

function isSidebarActive(link) {
  return normalize(route.path) === normalize(link);
}
</script>

<template>
  <var class="Layout">
    <div class="ds-shell">
      <header class="ds-toolbar">
        <a class="ds-brand" href="/">{{ site.title }}</a>
        <nav class="ds-toolbar-nav">
          <a
            v-for="item in theme.nav"
            :key="item.link"
            :href="item.link"
            :class="{ active: isNavActive(item.link) }"
          >{{ item.text }}</a>
        </nav>
      </header>
      <div class="ds-body">
        <aside v-if="hasSidebar" class="ds-sidebar">
          <nav v-for="group in sidebarGroups" :key="group.text" class="ds-sidebar-group">
            <p class="ds-sidebar-title">{{ group.text }}</p>
            <a
              v-for="item in group.items"
              :key="item.link"
              :href="item.link"
              :class="{ active: isSidebarActive(item.link) }"
            >{{ item.text }}</a>
          </nav>
        </aside>
        <main class="ds-content">
          <Content class="vp-doc" />
        </main>
      </div>
    </div>
  </var>
</template>
