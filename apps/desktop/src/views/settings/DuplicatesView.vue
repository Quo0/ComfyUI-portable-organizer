<script setup lang="ts">
// Отчёт о дубликатах моделей.
//
// Только отчёт. Ни одной кнопки, что-либо делающей с файлами: уборка
// дублей живёт своей командой на экране сборки, где виден перечень
// и понятно, из какой именно установки уйдёт копия.
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { UnlistenFn } from '@tauri-apps/api/event';

import { commands, events, type DuplicatesReport } from '../../bindings';
import { useFormat } from '../../lib/format';
import { useUiStore } from '../../stores/ui';

const ui = useUiStore();
const { t } = useI18n();
const { bytes } = useFormat();

const report = ref<DuplicatesReport | null>(null);
const scanning = ref(false);
const progress = ref({ done: 0, total: 0, place: '' });
const unlisten: UnlistenFn[] = [];

const percent = computed(() =>
  progress.value.total > 0
    ? Math.round((progress.value.done / progress.value.total) * 100)
    : 0,
);

onMounted(async () => {
  unlisten.push(
    await events.dupProgress.listen((e) => {
      progress.value = e.payload;
    }),
  );
});

onBeforeUnmount(() => {
  unlisten.forEach((off) => off());
  // Уход с экрана прекращает обход: держать диск занятым ради отчёта,
  // который никто не увидит, незачем.
  if (scanning.value) void commands.cancelDuplicatesScan();
});

async function scan(): Promise<void> {
  scanning.value = true;
  progress.value = { done: 0, total: 0, place: '' };
  try {
    const res = await commands.scanDuplicates();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    report.value = res.data;
  } finally {
    scanning.value = false;
  }
}

async function cancel(): Promise<void> {
  await commands.cancelDuplicatesScan();
}
</script>

<template>
  <div class="screen">
    <!-- Экран называется отчётом, а не «дубликатами»: дубликаты — это
         то, что он нашёл, а не то, чем он является. -->
    <header class="screen-head">
      <h1 class="t-lg">{{ t('settings.section.report') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad wide">
        <p class="t-sm">{{ t('dups.lead') }}</p>

        <div class="row">
          <button type="button" class="btn primary" :disabled="scanning" @click="scan">
            {{ t('dups.scan') }}
          </button>
          <button v-if="scanning" type="button" class="btn ghost" @click="cancel">
            {{ t('common.cancel') }}
          </button>
        </div>

        <!-- Пауза без подписи читается как зависание, поэтому здесь и полоса,
             и название места, которое обходим прямо сейчас. -->
        <div v-if="scanning" class="group">
          <div class="bar"><i :style="{ width: `${percent}%` }"></i></div>
          <span class="hint">{{ t('dups.scanning', { place: progress.place }) }}</span>
        </div>

        <template v-if="report">
          <p v-if="report.cancelled" class="hint bad">{{ t('dups.cancelled') }}</p>

          <div v-if="report.duplicates.length" class="group">
            <span class="t-label">
              {{ t('dups.wasted', { size: bytes(report.wastedBytes) }) }}
            </span>
            <!-- Строка отчёта: имя, категория, где лежит, сколько впустую.
                 Список «подпись — значение» здесь не годится — колонок
                 четыре, и места, где лежат копии, важнее размера. -->
            <div class="dup-list">
              <div
                v-for="group in report.duplicates"
                :key="group.category + group.name"
                class="dup-row"
              >
                <!-- Имена файлов и категорий не переводятся. -->
                <span class="nm">{{ group.name }}</span>
                <span class="tag">{{ group.category }}</span>
                <span class="t-mono">{{ bytes(group.wastedBytes) }}</span>
                <span class="where hint">
                  {{ group.copies.map((c) => c.source).join(' · ') }}
                </span>
              </div>
            </div>
            <p class="hint">{{ t('dups.noAction') }}</p>
          </div>
          <p v-else-if="!scanning" class="hint">{{ t('dups.empty') }}</p>

          <div v-if="report.nameClashes.length" class="group">
            <span class="t-label">{{ t('dups.clashes') }}</span>
            <p class="hint">{{ t('dups.clashesHint') }}</p>
            <div class="dup-list">
              <div
                v-for="group in report.nameClashes"
                :key="group.category + group.name"
                class="dup-row"
              >
                <span class="nm">{{ group.name }}</span>
                <span class="tag">{{ group.category }}</span>
                <span class="t-mono"></span>
                <span class="where hint">
                  {{ group.copies.map((c) => `${c.source} · ${bytes(c.sizeBytes)}`).join(' · ') }}
                </span>
              </div>
            </div>
          </div>

          <p v-if="report.skipped.length" class="hint bad">
            {{ t('dups.skipped', { list: report.skipped.join(', ') }) }}
          </p>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Отступы у строк обязательны: без них содержимое упирается в рамку
   и читается как обрезанное. */
.dup-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  background: var(--line);
  border: 1px solid var(--line);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.dup-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto minmax(0, 1.2fr);
  gap: var(--space-3);
  align-items: center;
  background: var(--surface);
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
}

.dup-row .nm,
.dup-row .where {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dup-row .t-mono {
  font-variant-numeric: tabular-nums;
}
</style>
