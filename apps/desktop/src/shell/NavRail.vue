<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import { accentVar, initial } from '../lib/format';
import { statusOf, useInstancesStore } from '../stores/instances';
import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const instances = useInstancesStore();
const { t } = useI18n();

onMounted(() => {
  if (!instances.loaded) void instances.load();
});

// Порядок фиксирован планом: инстансы, установка, библиотека, настройки,
// сведения.
const sections = computed(() => [
  { to: '/instances', icon: 'i-inst', label: t('nav.instances') },
  { to: '/install', icon: 'i-install', label: t('nav.install') },
  { to: '/workflows', icon: 'i-wf', label: t('nav.workflows') },
  { to: '/settings', icon: 'i-set', label: t('nav.settings') },
  { to: '/about', icon: 'i-about', label: t('nav.about') },
]);

/**
 * Второй блок рейла. Смысл его в том, что события, не вызванные действием
 * пользователя, обязаны быть видны независимо от открытого раздела.
 *
 * Пока нет супервизора, это инстансы с исчезнувшей папкой. В Фазе 2 сюда же
 * придут работающие, стартующие и аварийно завершённые — отсюда нейтральный
 * заголовок вместо «Запущены»: он остаётся верным в обоих случаях.
 */
const attention = computed(() => instances.needsAttention);

const dotColor: Record<string, string> = {
  running: 'var(--state-running)',
  starting: 'var(--state-starting)',
  crashed: 'var(--state-crashed)',
  unavailable: 'var(--state-unavailable)',
  stopped: 'var(--state-stopped)',
};

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

    <template v-if="attention.length">
      <div class="nav-sep"></div>
      <div class="nav-note">{{ t('nav.active') }} · {{ attention.length }}</div>
      <!-- Собственная прокрутка: восемь инстансов не должны выталкивать
           кнопку сворачивания за пределы окна. -->
      <div class="nav-runs">
        <RouterLink
          v-for="instance in attention"
          :key="instance.id"
          class="nav-run"
          :class="{ alert: statusOf(instance) === 'crashed' }"
          :to="`/instances/${instance.id}`"
          :title="instance.name"
        >
          <span
            class="chip"
            :style="{ '--instance-accent': accentVar(instance.accent) }"
          >
            {{ initial(instance.name) }}
          </span>
          <em>{{ instance.name }}</em>
          <i class="dot" :style="{ background: dotColor[statusOf(instance)] }"></i>
        </RouterLink>
      </div>
    </template>

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
