<script setup lang="ts">
// Запуск инстанса: выбор профиля, кнопка, живая консоль.
//
// Выпадашка профилей рендерится, пока вебвью ещё не создан, поэтому
// конфликта с ним нет — так и записано в дисциплине z-order.
import { computed, nextTick, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { openUrl } from '@tauri-apps/plugin-opener';

import type { Instance } from '../bindings';
import { displayStatus } from '../lib/status';
import { useRunStore } from '../stores/run';

const props = defineProps<{ instance: Instance }>();

const run = useRunStore();
const { t } = useI18n();

const chosen = ref<string | null>(null);
const console = ref<HTMLPreElement | null>(null);
/** Прокрутка следует за логом, пока пользователь сам не отмотал вверх. */
const follow = ref(true);

const status = computed(() => run.statusOf(props.instance.id));
const state = computed(() => displayStatus(props.instance, status.value));
const profiles = computed(() => run.profiles[props.instance.id] ?? []);
const lines = computed(() => run.logs[props.instance.id] ?? []);
const busy = computed(() => run.busy[props.instance.id] === true);

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
  scrollDown();
});

watch(
  () => lines.value.length,
  () => scrollDown(),
);

async function scrollDown(): Promise<void> {
  if (!follow.value) return;
  await nextTick();
  const el = console.value;
  if (el) el.scrollTop = el.scrollHeight;
}

function onScroll(): void {
  const el = console.value;
  if (!el) return;
  // Отступ в пару строк: точное сравнение ломается на дробной высоте строки.
  follow.value = el.scrollHeight - el.scrollTop - el.clientHeight < 40;
}

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
        <button
          type="button"
          class="btn danger lg"
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
        <pre ref="console" class="console" @scroll="onScroll"><span
          v-for="(line, i) in lines"
          :key="i"
          :class="{ dim: line.stream === 'stdout' }"
        >{{ line.text }}
</span></pre>
      </div>
    </div>
  </div>
</template>

<style scoped>
.log {
  height: 260px;
}
.console {
  margin: 0;
  overflow: auto;
}
</style>
