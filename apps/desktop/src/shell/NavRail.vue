<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import { accentVar, initial } from '../lib/format';
import { displayStatus, needsAttention, STATE_DOT } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const instances = useInstancesStore();
const run = useRunStore();
const { t } = useI18n();

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  // Подписка на события живёт в сторе и заводится один раз: рейл виден
  // всегда, значит и слушать он обязан всегда.
  await run.load();
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
 * пользователя, обязаны быть видны независимо от открытого раздела:
 * упавший процесс, самоперезапуск от ComfyUI-Manager, исчезнувшая папка.
 */
const attention = computed(() =>
  instances.items
    .map((instance) => ({
      instance,
      status: displayStatus(instance, run.statusOf(instance.id)),
    }))
    .filter((row) => needsAttention(row.status)),
);

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
          v-for="row in attention"
          :key="row.instance.id"
          class="nav-run"
          :class="{ alert: row.status === 'crashed' || row.status === 'detached' }"
          :to="`/instances/${row.instance.id}`"
          :title="row.instance.name"
        >
          <span
            class="chip"
            :style="{ '--instance-accent': accentVar(row.instance.accent) }"
          >
            {{ initial(row.instance.name) }}
          </span>
          <em>{{ row.instance.name }}</em>
          <span v-if="row.status === 'crashed'" class="badge">!</span>
          <i v-else class="dot" :style="{ background: STATE_DOT[row.status] }"></i>
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
