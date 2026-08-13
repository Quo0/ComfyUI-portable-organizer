<script setup lang="ts">
// Органы управления запуском: кнопка, выбор профиля, остановка.
//
// Живут в шапке экрана сборки, а не внутри вкладки: запустить и остановить
// нужно с любой из них, и прятать это за переключением вкладок значило бы
// заставлять искать главное действие.
//
// Выпадашка профилей рендерится, пока вебвью ещё не создан, поэтому
// конфликта с ним нет — так и записано в дисциплине z-order.
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

// Профили грузит экран, и приехать они могут позже нас.
watch(profiles, (list) => {
  if (!chosen.value) chosen.value = list[0]?.id ?? null;
});

function openInBrowser(): void {
  const port = status.value?.port;
  if (port) void openUrl(`http://127.0.0.1:${port}`);
}
</script>

<template>
  <template v-if="!active">
    <!-- Сплит-кнопка: основное действие слева, выбор профиля справа.
         Профилей у портабл-сборки обычно четыре, и прятать их
         в настройки значило бы прятать выбор видеокарты. -->
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
      <!-- Имена профилей — имена файлов, они не переводятся. -->
      <option v-for="p in profiles" :key="p.id" :value="p.id">
        {{ p.name }}{{ p.advanced ? ' · ' + t('instances.field.profilesAdvanced') : '' }}
      </option>
    </select>
  </template>

  <template v-else>
    <!-- Ради вкладки всё и затевалось, поэтому она — главное действие
         работающей сборки, а не пункт где-то сбоку. -->
    <RouterLink
      v-if="state === 'running' || state === 'starting'"
      class="btn primary lg"
      :to="`/instances/${instance.id}/tab`"
    >
      {{ t('tab.open') }}
    </RouterLink>
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
    <button
      v-if="state === 'running'"
      type="button"
      class="btn ghost"
      @click="openInBrowser"
    >
      {{ t('run.openInBrowser') }}
    </button>
  </template>
</template>
