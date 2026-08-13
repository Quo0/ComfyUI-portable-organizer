<script setup lang="ts">
// Перенос моделей этой сборки в общую папку.
//
// Зеркало панели воркфлоу, но с двумя отличиями, вытекающими из того, что
// здесь двигаются десятки гигабайт и удаляются файлы: перечень показывается
// до начала, а уборка дубликатов — отдельное действие с отдельным списком.
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import {
  commands,
  events,
  type Instance,
  type MigrateOutcome,
  type MigrateProgress,
  type ModelsScan,
} from '../bindings';
import { displayStatus } from '../lib/status';
import { useFormat } from '../lib/format';
import { useRunStore } from '../stores/run';
import { useSharedStore } from '../stores/shared';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ instance: Instance }>();

const shared = useSharedStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();
const { bytes } = useFormat();

const scan = ref<ModelsScan | null>(null);
const loading = ref(false);
const chosen = ref<Set<string>>(new Set());
const progress = ref<MigrateProgress | null>(null);
const outcome = ref<MigrateOutcome | null>(null);
const busy = ref(false);
const cleaning = ref(false);

let unlisten: (() => void) | null = null;

const running = computed(() =>
  ['starting', 'running', 'stopping', 'detached'].includes(
    displayStatus(props.instance, run.statusOf(props.instance.id)),
  ),
);

onMounted(async () => {
  if (!shared.loaded) await shared.load();
  unlisten = await events.migrateProgress.listen((e) => (progress.value = e.payload));
  await refresh();
});
onUnmounted(() => unlisten?.());

// Перенос меняет содержимое папки — после запуска и остановки сборки
// список мог поменяться и сам.
watch(running, () => void refresh());

async function refresh(): Promise<void> {
  if (!shared.configured) return;
  loading.value = true;
  try {
    const res = await commands.scanInstanceModels(props.instance.id);
    if (res.status === 'error') {
      // Общий корень не задан — это не ошибка сборки, молчим:
      // об этом уже сказано отдельной строкой.
      if (res.error.code !== 'shared.noRoots') ui.pushError(res.error);
      return;
    }
    scan.value = res.data;
    chosen.value = new Set(res.data.categories.map((c) => c.folder));
  } finally {
    loading.value = false;
  }
}

function toggle(folder: string): void {
  const next = new Set(chosen.value);
  if (!next.delete(folder)) next.add(folder);
  chosen.value = next;
}

/** Сколько поедет: занятые имена остаются на месте и в счёт не идут. */
const plan = computed(() => {
  let files = 0;
  let size = 0;
  for (const category of scan.value?.categories ?? []) {
    if (!chosen.value.has(category.folder)) continue;
    for (const entry of category.entries) {
      if (entry.sameName) continue;
      files += entry.files;
      // specta отдаёт f64 как `number | null`: в JSON нет NaN.
      size += entry.sizeBytes ?? 0;
    }
  }
  return { files, size };
});

async function migrate(): Promise<void> {
  busy.value = true;
  outcome.value = null;
  progress.value = null;
  try {
    const res = await commands.migrateModels(props.instance.id, [...chosen.value]);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    outcome.value = res.data;
    await refresh();
    await shared.rescan();
  } finally {
    busy.value = false;
    progress.value = null;
  }
}

/** Кандидаты в уборку: только то, что признано дубликатом. */
const duplicates = computed(
  () =>
    outcome.value?.skipped.filter(
      (s) => s.verdict === 'duplicate' || s.verdict === 'likelyDuplicate',
    ) ?? [],
);

/** Совпало имя, но не содержимое. Удалять такое нельзя. */
const different = computed(
  () => outcome.value?.skipped.filter((s) => s.verdict === 'different') ?? [],
);

const freeable = computed(() => duplicates.value.reduce((sum, s) => sum + (s.sizeBytes ?? 0), 0));

async function cleanup(): Promise<void> {
  cleaning.value = true;
  try {
    const res = await commands.removeDuplicateModels(
      props.instance.id,
      duplicates.value.map((s) => [s.category, s.name] as [string, string]),
    );
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    ui.pushOk(t('migrate.cleanup.done', { size: bytes(res.data.freedBytes) }));
    outcome.value = null;
    await refresh();
  } finally {
    cleaning.value = false;
  }
}
</script>

<template>
  <div class="group">
    <div class="row">
      <span class="t-label">{{ t('migrate.title') }}</span>
      <span class="head-spacer"></span>
      <button type="button" class="btn ghost" :disabled="loading || busy" @click="refresh">
        {{ t('library.retry') }}
      </button>
    </div>

    <!-- Общий корень не задан — переносить некуда. -->
    <p v-if="!shared.configured" class="hint">
      {{ t('migrate.noRoot') }}
      <RouterLink to="/settings/shared-models">{{ t('shared.instance.setUp') }}</RouterLink>
    </p>

    <template v-else>
      <div v-if="loading" class="bar indet"><i></i></div>

      <template v-else-if="scan?.categories.length">
        <div class="cats">
          <div v-for="category in scan.categories" :key="category.folder" class="cat">
            <code>{{ category.folder }}</code>
            <span class="n">
              {{ t('migrate.entries', category.entries.length) }} ·
              {{ bytes(category.sizeBytes) }}
            </span>
            <button
              type="button"
              class="toggle"
              :class="{ off: !chosen.has(category.folder) }"
              role="switch"
              :aria-checked="chosen.has(category.folder)"
              :disabled="busy"
              @click="toggle(category.folder)"
            ></button>
          </div>
        </div>

        <p class="hint">
          {{ t('migrate.summary', { files: plan.files, size: bytes(plan.size) }) }}
        </p>

        <!-- Файлы забираются из-под ComfyUI — у работающей сборки этого
             делать нельзя, и сказать надо прямо, а не гасить кнопку молча. -->
        <p v-if="running" class="hint bad">{{ t('migrate.mustStop') }}</p>

        <div v-if="progress" class="group">
          <p class="t-sm">{{ progress.category }}/{{ progress.name }}</p>
          <div class="bar">
            <i :style="{ width: `${(progress.done / progress.total) * 100}%` }"></i>
          </div>
          <button type="button" class="btn danger" @click="commands.cancelMigrate()">
            {{ t('common.cancel') }}
          </button>
        </div>

        <div v-else class="row">
          <button
            type="button"
            class="btn primary"
            :disabled="busy || running || plan.files === 0"
            @click="migrate"
          >
            {{ t('migrate.action') }}
          </button>
        </div>
      </template>

      <p v-else class="empty">{{ t('migrate.empty') }}</p>

      <!-- Отчёт. Дубликаты и разные файлы разведены намеренно: это ровно
           тот случай, когда «удалить всё выделенное» уносит не то. -->
      <template v-if="outcome">
        <p class="hint">
          {{ t('migrate.moved', { n: outcome.moved.length, size: bytes(outcome.movedBytes) }) }}
        </p>

        <div v-if="duplicates.length" class="group danger-zone">
          <p class="t-md">{{ t('migrate.dup.title', duplicates.length) }}</p>
          <p class="t-sm">{{ t('migrate.dup.body', { size: bytes(freeable) }) }}</p>
          <div class="cats">
            <div v-for="item in duplicates" :key="`${item.category}/${item.name}`" class="cat">
              <code>{{ item.category }}/{{ item.name }}</code>
              <span class="n">{{ bytes(item.sizeBytes) }}</span>
              <span class="tag" :class="{ warn: item.verdict === 'likelyDuplicate' }">
                {{ t(`migrate.verdict.${item.verdict}`) }}
              </span>
            </div>
          </div>
          <button
            type="button"
            class="btn danger"
            :disabled="cleaning || running || !instance.shared?.enabled"
            @click="cleanup"
          >
            {{ t('migrate.dup.remove') }}
          </button>
          <!-- Без подключения удаление лишило бы сборку моделей вовсе. -->
          <p v-if="!instance.shared?.enabled" class="hint bad">
            {{ t('migrate.dup.needsConnection') }}
          </p>
        </div>

        <div v-if="different.length" class="group">
          <p class="t-md">{{ t('migrate.diff.title', different.length) }}</p>
          <p class="t-sm">{{ t('migrate.diff.body') }}</p>
          <div class="cats">
            <div v-for="item in different" :key="`${item.category}/${item.name}`" class="cat">
              <code>{{ item.category }}/{{ item.name }}</code>
              <span class="n">{{ bytes(item.sizeBytes) }}</span>
              <span class="tag stop">{{ t('migrate.verdict.different') }}</span>
            </div>
          </div>
        </div>

        <p v-if="outcome.failed.length" class="hint bad">
          {{ t('migrate.failed', outcome.failed.length) }}:
          {{ outcome.failed.map((f) => `${f.category}/${f.name}`).join(', ') }}
        </p>
      </template>

      <p class="hint">{{ t('migrate.pickedUpOnStart') }}</p>
    </template>
  </div>
</template>
