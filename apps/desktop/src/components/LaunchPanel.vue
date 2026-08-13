<script setup lang="ts">
// Запуск инстанса: выбор профиля, кнопка, живая консоль.
//
// Выпадашка профилей рендерится, пока вебвью ещё не создан, поэтому
// конфликта с ним нет — так и записано в дисциплине z-order.
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { openUrl } from '@tauri-apps/plugin-opener';

import LogConsole from './LogConsole.vue';
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
const lines = computed(() => run.logs[props.instance.id] ?? []);
const busy = computed(() => run.busy[props.instance.id] === true);
const warning = computed(() => run.sharedWarning[props.instance.id]);

const active = computed(() =>
  ['starting', 'running', 'stopping'].includes(state.value),
);

const profile = computed(() =>
  profiles.value.find((p) => p.id === (chosen.value ?? status.value?.profileId)),
);

onMounted(async () => {
  await run.loadProfiles(props.instance.id);
  await run.loadLog(props.instance.id);
  chosen.value = profiles.value[0]?.id ?? null;
});

function openInBrowser(): void {
  const port = status.value?.port;
  if (port) void openUrl(`http://127.0.0.1:${port}`);
}
</script>

<template>
  <div class="group">
    <div class="row">
      <template v-if="!active">
        <!-- Сплит-кнопка: основное действие слева, выбор профиля справа.
             Профилей у портабл-сборки обычно четыре, и прятать их
             в настройки значило бы прятать выбор видеокарты. -->
        <div class="split">
          <button
            type="button"
            class="btn primary lg"
            :disabled="busy || !instance.available || profiles.length === 0"
            @click="run.start(instance.id, chosen)"
          >
            {{ t('run.start') }}
          </button>
        </div>
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
          class="btn danger"
          :disabled="busy || state === 'stopping'"
          @click="run.stop(instance.id)"
        >
          {{ t('run.stop') }}
        </button>
        <button
          v-if="state === 'running'"
          type="button"
          class="btn secondary"
          @click="openInBrowser"
        >
          {{ t('run.openInBrowser') }}
        </button>
      </template>

      <span v-if="status?.port" class="hint">
        {{ t('run.onPort', { port: status.port }) }}
      </span>
      <span v-if="status?.readySecs" class="hint">
        {{ t('run.readyIn', { secs: status.readySecs }) }}
      </span>
    </div>

    <!-- Общий корень недоступен. Не ошибка запуска, а развилка: продолжить
         без общих моделей или отменить. Раскрывается на месте — модалок
         в приложении нет, а у тоста не бывает кнопок. -->
    <div v-if="warning" class="group danger-zone">
      <p class="t-md">{{ t('shared.unavailable.title') }}</p>
      <p class="t-sm">{{ t('shared.unavailable.body', { path: warning.path }) }}</p>
      <div class="row">
        <button
          type="button"
          class="btn secondary"
          @click="run.start(instance.id, warning.profileId, true)"
        >
          {{ t('shared.unavailable.startAnyway') }}
        </button>
        <button
          type="button"
          class="btn ghost"
          @click="run.dismissSharedWarning(instance.id)"
        >
          {{ t('common.cancel') }}
        </button>
      </div>
    </div>

    <p v-if="profile?.fallback" class="hint bad">{{ t('run.fallback') }}</p>

    <p v-if="state === 'crashed'" class="hint bad">
      {{ t('run.crashed', { code: status?.exitCode ?? '—' }) }}
    </p>
    <p v-if="state === 'detached'" class="hint bad">{{ t('run.detached') }}</p>

    <!-- Консоль появляется вместе с первым запуском и остаётся после него:
         именно в ней лежит трейсбек, если сборка не поднялась. -->
    <div v-if="lines.length" class="group">
      <span class="t-label">{{ t('run.console') }}</span>
      <div class="log">
        <LogConsole :lines="lines" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.log {
  height: 260px;
  display: grid;
  min-height: 0;
}
</style>
