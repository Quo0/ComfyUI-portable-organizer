<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const { t } = useI18n();

// Порядок фиксирован планом: инстансы, установка, библиотека, настройки,
// сведения. Второй блок рейла — запущенные инстансы — появится в Фазе 2.
const sections = computed(() => [
  { to: '/instances', icon: 'i-inst', label: t('nav.instances') },
  { to: '/install', icon: 'i-install', label: t('nav.install') },
  { to: '/workflows', icon: 'i-wf', label: t('nav.workflows') },
  { to: '/settings', icon: 'i-set', label: t('nav.settings') },
  { to: '/about', icon: 'i-about', label: t('nav.about') },
]);

const toggleLabel = computed(() =>
  ui.railCollapsed ? t('nav.expand') : t('nav.collapse'),
);
</script>

<template>
  <nav class="nav in-window" :class="{ collapsed: ui.railCollapsed }">
    <RouterLink
      v-for="item in sections"
      :key="item.to"
      class="nav-item"
      active-class="on"
      :to="item.to"
      :title="item.label"
      :aria-label="item.label"
    >
      <svg class="ico"><use :href="`#${item.icon}`" /></svg>
      <span>{{ item.label }}</span>
    </RouterLink>

    <!-- Сворачивание внизу: наверху оно перетягивало бы внимание с разделов,
         а нажимают его редко и осознанно. -->
    <button
      type="button"
      class="nav-item rail-toggle"
      :title="toggleLabel"
      :aria-label="toggleLabel"
      :aria-expanded="!ui.railCollapsed"
      @click="ui.toggleRail()"
    >
      <svg class="ico">
        <use :href="ui.railCollapsed ? '#i-expand' : '#i-collapse'" />
      </svg>
      <span>{{ toggleLabel }}</span>
    </button>
  </nav>
</template>
