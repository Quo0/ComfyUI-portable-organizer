<script setup lang="ts">
// Всё, что запуск говорит пользователю: развилки и объяснения.
//
// Стоит под шапкой экрана сборки и видно на любой вкладке. Это ответы
// на только что нажатую кнопку, и прятать их за переключением вкладок
// значило бы отвечать в пустоту.
//
// Развилки раскрываются на месте: модалку над областью контента положить
// нельзя (дисциплина z-order), а у тоста не бывает кнопок.
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { openUrl } from '@tauri-apps/plugin-opener';

import type { Instance } from '../bindings';
import { displayStatus } from '../lib/status';
import { useRunStore } from '../stores/run';

const props = defineProps<{ instance: Instance }>();

const run = useRunStore();
const { t } = useI18n();

const status = computed(() => run.statusOf(props.instance.id));
const state = computed(() => displayStatus(props.instance, status.value));
const busy = computed(() => run.busy[props.instance.id] === true);
const shared = computed(() => run.sharedWarning[props.instance.id]);
const other = computed(() => run.busyWarning[props.instance.id]);

const profiles = computed(() => run.profiles[props.instance.id] ?? []);
const profile = computed(() =>
  profiles.value.find((p) => p.id === status.value?.profileId),
);

function openInBrowser(): void {
  const port = status.value?.port;
  if (port) void openUrl(`http://127.0.0.1:${port}`);
}
</script>

<template>
  <div v-if="shared || other || state === 'crashed' || state === 'detached' || profile?.fallback" class="notices">
    <!-- Общий корень недоступен. Не ошибка запуска, а развилка: продолжить
         без общих моделей или отменить. -->
    <div v-if="shared" class="banner">
      <div>
        <b>{{ t('shared.unavailable.title') }}</b>
        <p class="t-sm">{{ t('shared.unavailable.body', { path: shared.path }) }}</p>
      </div>
      <span class="spacer"></span>
      <button
        type="button"
        class="btn secondary"
        @click="run.start(instance.id, shared.profileId, { withoutShared: true, allowMultiple: false })"
      >
        {{ t('shared.unavailable.startAnyway') }}
      </button>
      <button type="button" class="btn ghost" @click="run.dismissSharedWarning(instance.id)">
        {{ t('common.cancel') }}
      </button>
    </div>

    <!-- Соседняя сборка уже держит видеопамять. Запретить второй запуск
         нельзя — бывает две видеокарты, — но и молчать нельзя: вторая
         сборка упадёт посреди генерации. -->
    <div v-if="other" class="banner">
      <div>
        <b>{{ t('run.otherRunning.title') }}</b>
        <p class="t-sm">{{ t('run.otherRunning.body', { name: other.other }) }}</p>
      </div>
      <span class="spacer"></span>
      <button
        type="button"
        class="btn secondary"
        @click="run.stopOthersAndStart(instance.id, other.profileId)"
      >
        {{ t('run.otherRunning.stopIt') }}
      </button>
      <button
        type="button"
        class="btn ghost"
        @click="run.start(instance.id, other.profileId, { withoutShared: false, allowMultiple: true })"
      >
        {{ t('run.otherRunning.anyway') }}
      </button>
      <button type="button" class="btn ghost" @click="run.dismissBusyWarning(instance.id)">
        {{ t('common.cancel') }}
      </button>
    </div>

    <div v-if="state === 'crashed'" class="banner bad">
      <b>{{ t('run.crashed', { code: status?.exitCode ?? '—' }) }}</b>
    </div>

    <!-- Сервер перезапустился сам. Кнопка возвращает над ним контроль:
         PID нового процесса ищется по владельцу порта. -->
    <div v-if="state === 'detached'" class="banner">
      <div>
        <p class="t-sm">{{ t('run.detached') }}</p>
      </div>
      <span class="spacer"></span>
      <button
        type="button"
        class="btn secondary"
        :disabled="busy"
        @click="run.adopt(instance.id)"
      >
        {{ t('run.adopt') }}
      </button>
      <button type="button" class="btn ghost" @click="openInBrowser">
        {{ t('run.openInBrowser') }}
      </button>
    </div>

    <p v-if="profile?.fallback" class="hint bad">{{ t('run.fallback') }}</p>
  </div>
</template>

<style scoped>
.notices {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: 0 var(--space-5);
}

.banner .spacer {
  margin-left: auto;
}

.banner p {
  margin: 0;
}
</style>
