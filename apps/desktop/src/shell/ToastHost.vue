<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const { t } = useI18n();

/** Раскрытые подробности — по одной ошибке за раз хватает. */
const opened = ref<number | null>(null);
const copied = ref<number | null>(null);

function toggle(id: number): void {
  opened.value = opened.value === id ? null : id;
}

async function copy(id: number, text: string): Promise<void> {
  try {
    await navigator.clipboard.writeText(text);
    copied.value = id;
    window.setTimeout(() => {
      if (copied.value === id) copied.value = null;
    }, 2000);
  } catch {
    // Буфер обмена может быть недоступен. Показывать ошибку про ошибку
    // бессмысленно: текст и так на экране, его можно выделить.
  }
}
</script>

<template>
  <var class="ToastHost">
    <!-- Тосты появляются только на экранах оболочки: на экране работающего
         инстанса пользователь внутри ComfyUI, и перекрывать его нечем. -->
    <div class="toasts toast-host" role="status" aria-live="polite">
      <div
        v-for="toast in ui.toasts"
        :key="toast.id"
        class="toast"
        :class="toast.kind === 'ok' ? 'ok' : 'err'"
      >
        <i></i>
        <div class="toast-in">
          <div class="toast-head">
            <b>{{ toast.text }}</b>
            <span
              v-if="toast.count > 1"
              class="badge"
              :title="t('toast.repeated', { n: toast.count })"
            >
              ×{{ toast.count }}
            </span>
            <button
              type="button"
              class="close"
              :aria-label="t('toast.dismiss')"
              @click="ui.dismiss(toast.id)"
            >
              ✕
            </button>
          </div>

          <div v-if="toast.details" class="row">
            <button type="button" class="btn ghost" @click="toggle(toast.id)">
              {{ t('common.details') }}
            </button>
            <button
              type="button"
              class="btn ghost"
              :aria-label="t('toast.copyError')"
              @click="copy(toast.id, toast.details)"
            >
              {{ copied === toast.id ? t('common.copied') : t('common.copy') }}
            </button>
          </div>

          <pre v-if="opened === toast.id && toast.details" class="toast-details">{{
            toast.details
          }}</pre>
        </div>
      </div>
    </div>
  </var>
</template>
