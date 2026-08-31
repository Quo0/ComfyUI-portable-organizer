<script setup>
// Our own page shell instead of the theme's VPNav/VPSidebar/VPContent. The
// theme's "open up the margins past 1440px" rule cannot be switched off in
// one place: it rests on the 960/1440 breakpoints, repeated in four
// components with four different padding formulas. This markup has no
// breakpoints at all — a fixed-height toolbar, a fixed-width sidebar, and
// content taking the rest.
//
// `.vp-doc` on <Content> is everything we take from the default theme: the
// typography, without the shell around it.
import { Content, useData, useRoute } from 'vitepress';
import { useSidebar } from 'vitepress/theme';

const { site, theme } = useData();
const route = useRoute();
const { sidebarGroups, hasSidebar } = useSidebar();

// cleanUrls drops the .html but not the trailing slash on section indexes
// ('/styleguide/'), so links are compared without it.
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
