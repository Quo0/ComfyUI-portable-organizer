<script setup lang="ts">
import { ChevronLeft, ChevronRight, FolderPlus, Info, Layers, SlidersHorizontal } from '@lucide/vue';
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import { accentVar, initial } from '../lib/format';
import { displayStatus, isLive, STATE_DOT } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';
import { useUpdatesStore } from '../stores/updates';

const ui = useUiStore();
const instances = useInstancesStore();
const run = useRunStore();
const updates = useUpdatesStore();
const { t } = useI18n();

onMounted(async () => {
  if (!instances.loaded) await instances.load();

  await run.load();
});

//

//

const sections = computed(() => [
  { to: '/instances', icon: Layers, label: t('nav.instances'), mark: false },
  { to: '/install', icon: FolderPlus, label: t('nav.install'), mark: false },
  { to: '/settings', icon: SlidersHorizontal, label: t('nav.settings'), mark: false },
  { to: '/about', icon: Info, label: t('nav.about'), mark: updates.info !== null },
]);

const live = computed(() =>
  instances.items
    .map((instance) => ({
      instance,
      status: displayStatus(instance, run.statusOf(instance.id)),
    }))
    .filter((row) => isLive(row.status)),
);

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
        <i v-if="item.mark" class="dot nav-mark" :title="t('about.update.title')"></i>
      </RouterLink>

      <template v-if="live.length">
        <div class="nav-sep"></div>
        <div class="nav-note">{{ t('nav.active') }} · {{ live.length }}</div>

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
