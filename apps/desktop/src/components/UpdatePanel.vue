<script setup lang="ts">

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

const toggleTouched = ref(false);

const released = computed(() => moment(updates.info?.date));

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

      <div v-if="updates.info" class="banner">
        <div>
          <b>{{ t('about.update.available', { version: updates.info.version }) }}</b>

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
