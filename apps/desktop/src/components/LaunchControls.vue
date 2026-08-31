<script setup lang="ts">

import { ExternalLink } from '@lucide/vue';
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { openUrl } from '@tauri-apps/plugin-opener';

import type { Instance } from '../bindings';
import { displayStatus } from '../lib/status';
import { useRunStore } from '../stores/run';

const props = defineProps<{ instance: Instance }>();

const run = useRunStore();
const { t } = useI18n();

const chosen = ref<string | null>(null);

const status = computed(() => run.statusOf(props.instance.id));
const state = computed(() => displayStatus(props.instance, status.value));
const profiles = computed(() => run.profiles[props.instance.id] ?? []);
const busy = computed(() => run.busy[props.instance.id] === true);

const active = computed(() =>
  ['starting', 'running', 'stopping'].includes(state.value),
);

onMounted(() => {
  chosen.value = profiles.value[0]?.id ?? null;
});

watch(profiles, (list) => {
  if (!chosen.value) chosen.value = list[0]?.id ?? null;
});

function openInBrowser(): void {
  const port = status.value?.port;
  if (port) void openUrl(`http://127.0.0.1:${port}`);
}
</script>

<template>
  <var class="LaunchControls">
    <template v-if="!active">

      <button
        type="button"
        class="btn primary lg"
        :disabled="busy || !instance.available || profiles.length === 0"
        @click="run.start(instance.id, chosen)"
      >
        {{ t('run.start') }}
      </button>
      <select
        v-if="profiles.length > 1"
        v-model="chosen"
        class="input"
        :aria-label="t('run.profile')"
      >

        <option v-for="p in profiles" :key="p.id" :value="p.id">
          {{ p.name }}{{ p.advanced ? ' · ' + t('instances.field.profilesAdvanced') : '' }}
        </option>
      </select>
    </template>

    <template v-else>

      <RouterLink
        v-if="state === 'running' || state === 'starting'"
        class="btn primary lg"
        :to="`/instances/${instance.id}/tab`"
      >
        {{ t('tab.open') }}
      </RouterLink>
      <button
        v-if="state === 'running'"
        type="button"
        class="btn ghost icon"
        :title="t('run.openInBrowser')"
        :aria-label="t('run.openInBrowser')"
        @click="openInBrowser"
      >
        <ExternalLink class="ico" />
      </button>
      <button
        type="button"
        class="btn secondary"
        :disabled="busy || state !== 'running'"
        @click="run.restart(instance.id)"
      >
        {{ t('run.restart') }}
      </button>
      <button
        type="button"
        class="btn danger"
        :disabled="busy || state === 'stopping'"
        @click="run.stop(instance.id)"
      >
        {{ t('run.stop') }}
      </button>
    </template>
  </var>
</template>
