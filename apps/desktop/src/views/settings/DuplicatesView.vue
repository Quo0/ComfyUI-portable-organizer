<script setup lang="ts">
// Отчёт о дубликатах моделей.
//
// Только отчёт. Ни одной кнопки, что-либо делающей с файлами: уборка
// дублей живёт своей командой на экране сборки, где виден перечень
// и понятно, из какой именно установки уйдёт копия.
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { UnlistenFn } from '@tauri-apps/api/event';

import EmptyNote from '../../components/EmptyNote.vue';
import Group from '../../components/ui/Group.vue';
import ScreenHeader from '../../components/ui/ScreenHeader.vue';
import { commands, events, type DuplicatesReport } from '../../bindings';
import { useFormat } from '../../lib/format';
import { withViewTransition } from '../../lib/motion';
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

/**
 * Полоса хода и итоговый отчёт подменяют друг друга целиком — с переходом
 * (transitions.dev), а не рывком. Мутация синхронна по обе стороны
 * единственного `await`: `report.value` переписывается сразу по ответу
 * бэкенда, без вложенных стора и вторых `await` между ними, — как
 * и у выбора строки в библиотеке воркфлоу, ловить здесь нечего.
 */
async function scan(): Promise<void> {
  withViewTransition(() => {
    scanning.value = true;
    progress.value = { done: 0, total: 0, place: '' };
  });
  try {
    const res = await commands.scanDuplicates();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    withViewTransition(() => { report.value = res.data; });
  } finally {
    scanning.value = false;
  }
}

async function cancel(): Promise<void> {
  await commands.cancelDuplicatesScan();
}
</script>

<template>
  <var class="DuplicatesView">
    <div class="screen">
      <!-- Экран называется отчётом, а не «дубликатами»: дубликаты — это
           то, что он нашёл, а не то, чем он является. -->
      <ScreenHeader>
        <h1 class="t-lg">{{ t('settings.section.report') }}</h1>
      </ScreenHeader>

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
          <Group v-if="scanning">
            <div class="bar"><i :style="{ width: `${percent}%` }"></i></div>
            <span class="hint">{{ t('dups.scanning', { place: progress.place }) }}</span>
          </Group>

          <template v-if="report">
            <p v-if="report.cancelled" class="hint bad">{{ t('dups.cancelled') }}</p>

            <Group v-if="report.duplicates.length">
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
            </Group>
            <EmptyNote v-else-if="!scanning">{{ t('dups.empty') }}</EmptyNote>

            <Group v-if="report.nameClashes.length">
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
            </Group>

            <p v-if="report.skipped.length" class="hint bad">
              {{ t('dups.skipped', { list: report.skipped.join(', ') }) }}
            </p>
          </template>
        </div>
      </div>
    </div>
  </var>
</template>
