<script setup lang="ts">
// Обновление приложения на экране «О приложении».
//
// Здесь, а не в «Настройках»: версия уже стоит в шапке этого экрана,
// и вопрос «что у меня установлено и есть ли новее» — один вопрос,
// а не два в разных разделах.
//
// Развилка «работают сборки» раскрывается на месте баннером, как гард
// мульти-запуска: модалку над областью контента положить нельзя
// (дисциплина z-order), а у тоста не бывает кнопок.
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import Group from './ui/Group.vue';
import Toggle from './ui/Toggle.vue';
import ToggleRow from './ui/ToggleRow.vue';
import { useFormat } from '../lib/format';
import { useUiStore } from '../stores/ui';
import { useUpdatesStore } from '../stores/updates';

const ui = useUiStore();
const updates = useUpdatesStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

/** Перехлёст ползунка тумблера играет только с первого клика, не с отрисовки. */
const toggleTouched = ref(false);

const released = computed(() => moment(updates.info?.date));

/** Доля скачанного. Без известной длины полоса индетерминантная. */
const share = computed(() => {
  const p = updates.progress;
  if (!p?.total) return null;
  return Math.min(100, Math.round((p.downloaded / p.total) * 100));
});

function toggleAuto(): void {
  toggleTouched.value = true;
  ui.setCheckUpdates(!ui.checkUpdates);
}
</script>

<template>
  <var class="UpdatePanel">
    <Group>
      <span class="t-label">{{ t('about.update.title') }}</span>

      <!-- Найдена новая версия. Номер обязателен: «доступно обновление»
           без версии не даёт решить, нужно ли оно сейчас. -->
      <div v-if="updates.info" class="banner">
        <div>
          <b>{{ t('about.update.available', { version: updates.info.version }) }}</b>
          <!-- Установленная версия здесь же, а не только в шапке экрана:
               «доступна 0.2.0» без ответа на «а у меня какая» — половина
               сведений, и вторую половину пришлось бы искать глазами. -->
          <p class="t-sm">
            {{ t('about.update.from', { current: updates.info.currentVersion }) }}
            <template v-if="released"> · {{ t('about.update.released', { date: released }) }}</template>
          </p>
        </div>
        <span class="spacer"></span>
        <button
          v-if="!updates.running"
          type="button"
          class="btn primary"
          :disabled="updates.installing"
          @click="updates.install(false)"
        >
          {{ updates.installing ? t('about.update.installing') : t('about.update.install') }}
        </button>
      </div>

      <!-- Установка закроет приложение, а Job Object унесёт с собой
           работающие сборки вместе с их очередью генерации. Решает это
           пользователь, а не мы. -->
      <div v-if="updates.running" class="banner">
        <div>
          <b>{{ t('about.update.running.title') }}</b>
          <p class="t-sm">
            {{ t('about.update.running.body', { names: updates.running }) }}
          </p>
        </div>
        <span class="spacer"></span>
        <button
          type="button"
          class="btn secondary"
          :disabled="updates.installing"
          @click="updates.install(true)"
        >
          {{ t('about.update.running.stopAndInstall') }}
        </button>
        <button type="button" class="btn ghost" @click="updates.dismissRunning()">
          {{ t('about.update.running.later') }}
        </button>
      </div>

      <div v-if="updates.installing && updates.progress" class="dl">
        <div class="track" :class="{ indet: share === null }">
          <i :style="share === null ? undefined : { width: `${share}%` }"></i>
        </div>
        <p class="hint">
          <template v-if="updates.progress.total">
            {{ t('about.update.progress', {
              done: bytes(updates.progress.downloaded),
              total: bytes(updates.progress.total),
            }) }}
          </template>
          <template v-else>
            {{ t('about.update.progressUnknown', { done: bytes(updates.progress.downloaded) }) }}
          </template>
        </p>
      </div>

      <!-- Что изменилось: тело релиза как есть. Это текст выпуска,
           а не строка интерфейса, и переводу не подлежит. -->
      <div v-if="updates.info?.notes" class="notes">
        <span class="t-label">{{ t('about.update.notes') }}</span>
        <p class="t-sm">{{ updates.info.notes }}</p>
      </div>

      <div class="row">
        <button
          type="button"
          class="btn secondary"
          :disabled="updates.checking || updates.installing"
          @click="updates.check(true)"
        >
          {{ updates.checking ? t('about.update.checking') : t('about.update.check') }}
        </button>
        <!-- «Последняя версия» отличается от «не проверяли»: молчание
             на месте ответа читается как сломанная кнопка. -->
        <span v-if="updates.checked && !updates.info" class="hint">
          {{ t('about.update.upToDate') }}
        </span>
      </div>

      <ToggleRow>
        <Toggle
          :checked="ui.checkUpdates"
          :touched="toggleTouched"
          :aria-label="t('about.update.auto')"
          @click="toggleAuto"
        />
        <div>
          <div class="t-base">{{ t('about.update.auto') }}</div>
          <p class="hint">{{ t('about.update.autoHint') }}</p>
        </div>
      </ToggleRow>
    </Group>
  </var>
</template>

<style scoped>
/* Тело релиза приходит из CHANGELOG.md, где переводы строк значимы:
   без сохранения переносов список изменений слипается в абзац. */
.notes p {
  margin: var(--space-1) 0 0;
  white-space: pre-wrap;
}

.dl {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}
</style>
