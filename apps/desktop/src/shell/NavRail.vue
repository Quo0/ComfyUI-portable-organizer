<script setup lang="ts">
import { ChevronLeft, ChevronRight, FolderPlus, Info, Layers, SlidersHorizontal } from '@lucide/vue';
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import { accentVar, initial } from '../lib/format';
import { displayStatus, isLive, STATE_DOT } from '../lib/status';
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

// Порядок фиксирован планом: инстансы, установка, настройки, сведения.
//
// Библиотеки воркфлоу здесь нет намеренно: это папка снаружи сборок,
// устроенная ровно как общие модели, и её место — в «Настройках», рядом
// с ними. Пока она была разделом рейла, путь к папке жил в настройках,
// а её содержимое — в рейле, и на вопрос «куда это складывается» нельзя
// было ответить, не уходя с экрана.
const sections = computed(() => [
  { to: '/instances', icon: Layers, label: t('nav.instances') },
  { to: '/install', icon: FolderPlus, label: t('nav.install') },
  { to: '/settings', icon: SlidersHorizontal, label: t('nav.settings') },
  { to: '/about', icon: Info, label: t('nav.about') },
]);

/**
 * Второй блок рейла: всё, что сейчас не лежит в покое. Смысл его в том,
 * что события, не вызванные действием пользователя, обязаны быть видны
 * независимо от открытого раздела: упавший процесс, самоперезапуск
 * от ComfyUI-Manager, исчезнувшая папка.
 */
const live = computed(() =>
  instances.items
    .map((instance) => ({
      instance,
      status: displayStatus(instance, run.statusOf(instance.id)),
    }))
    .filter((row) => isLive(row.status)),
);

/**
 * Работающая сборка ведёт прямо на свою вкладку: ради неё пользователь
 * сюда и жмёт. У упавшей и недоступной вкладки нет — там нужен экран
 * инстанса с логом и кнопкой запуска.
 */
function target(row: (typeof live.value)[number]): string {
  const inTab = row.status === 'running' || row.status === 'starting';
  return `/instances/${row.instance.id}${inTab ? '/tab' : ''}`;
}

const toggleLabel = computed(() =>
  ui.railCollapsed ? t('nav.expand') : t('nav.collapse'),
);
</script>

<template>
  <var class="NavRail">
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
        <component :is="item.icon" class="ico" />
        <span>{{ item.label }}</span>
      </RouterLink>

      <template v-if="live.length">
        <div class="nav-sep"></div>
        <div class="nav-note">{{ t('nav.active') }} · {{ live.length }}</div>
        <!-- Собственная прокрутка: восемь инстансов не должны выталкивать
             кнопку сворачивания за пределы окна. -->
        <div class="nav-runs">
          <RouterLink
            v-for="row in live"
            :key="row.instance.id"
            class="nav-run"
            :class="{ alert: row.status === 'crashed' || row.status === 'detached' }"
            :to="target(row)"
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
        <component :is="ui.railCollapsed ? ChevronRight : ChevronLeft" class="ico" />
        <span>{{ toggleLabel }}</span>
      </button>
    </nav>
  </var>
</template>
