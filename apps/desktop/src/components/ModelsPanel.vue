<script setup lang="ts">
// Перенос моделей этой сборки в общую папку и уборка дубликатов.
//
// Зеркало панели воркфлоу, но с двумя отличиями, вытекающими из того, что
// здесь двигаются десятки гигабайт и удаляются файлы: перечень показывается
// до начала, а уборка — отдельное действие, хоть и по тому же списку.
//
// Список ровно один: категория раскрывается, и модели внутри показаны
// со своими вердиктами. Раньше их было три — категории, дубликаты
// и «совпало имя», — и одна и та же модель попадала то в один перечень,
// то в другой, а связь с категорией приходилось восстанавливать по имени
// в строке.
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import {
  commands,
  events,
  type Instance,
  type MigrateOutcome,
  type MigrateProgress,
  type ModelCategory,
  type ModelEntry,
  type ModelsScan,
} from '../bindings';
import OpenFolderButton from './OpenFolderButton.vue';
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
const progress = ref<MigrateProgress | null>(null);
const outcome = ref<MigrateOutcome | null>(null);
const busy = ref(false);
const cleaning = ref(false);

/** Ключ модели в наборах выбора: категория и имя внутри неё. */
function keyOf(folder: string, name: string): string {
  return `${folder}/${name}`;
}

/**
 * Чужое содержимое под тем же именем не трогается ничем: переносить некуда
 * (имя в общей папке занято), убирать нельзя (это не дубликат). Тумблера
 * у такой строки нет вовсе, и в выбор она не попадает никогда.
 */
function selectable(entry: ModelEntry): boolean {
  return entry.sameName !== 'different';
}

/**
 * Что тронем. Ключи помодельные, а не покатегорийные: одна лора
 * из двадцати может быть нужна сборке локально, и ради неё не должно
 * приходиться отказываться от переноса целой категории.
 */
const chosen = ref<Set<string>>(new Set());

/**
 * Свёрнутые категории — храним именно свёрнутые, потому что по умолчанию
 * раскрыто всё. Пересканирование этот набор не трогает: имена категорий
 * переживают скан, в отличие от ключей моделей.
 */
const collapsed = ref<Set<string>>(new Set());

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
    // Отмечено всё, что можно тронуть. Снятые галки между сканами
    // не помнятся: список пришёл из файловой системы, и тот же ключ
    // мог означать уже другой файл.
    chosen.value = new Set(
      res.data.categories.flatMap((category) =>
        category.entries
          .filter(selectable)
          .map((entry) => keyOf(category.folder, entry.name)),
      ),
    );
  } finally {
    loading.value = false;
  }
}

function toggle(key: string): void {
  const next = new Set(chosen.value);
  if (!next.delete(key)) next.add(key);
  chosen.value = next;
}

/** Ключи всего, чем распоряжается тумблер категории. */
function keysOf(category: ModelCategory): string[] {
  return category.entries.filter(selectable).map((entry) => keyOf(category.folder, entry.name));
}

/** Положение тумблера категории: по её моделям, а не по ней самой. */
function catState(category: ModelCategory): 'on' | 'off' | 'mixed' {
  const keys = keysOf(category);
  const on = keys.filter((key) => chosen.value.has(key)).length;
  if (on === 0) return 'off';
  return on === keys.length ? 'on' : 'mixed';
}

/**
 * `aria-checked` для тумблера категории.
 *
 * Роль у него `checkbox`, а не `switch`, как у моделей: `switch` состояния
 * «частично» не знает вовсе, и `mixed` на нём программа чтения с экрана
 * прочитает как «выключено» — то есть соврёт.
 */
function catChecked(category: ModelCategory): 'true' | 'false' | 'mixed' {
  const state = catState(category);
  if (state === 'mixed') return 'mixed';
  return state === 'on' ? 'true' : 'false';
}

function toggleCategory(category: ModelCategory): void {
  const keys = keysOf(category);
  const next = new Set(chosen.value);
  // Частичный выбор нажатие доводит до полного, а не сбрасывает:
  // промежуточный тумблер читается как «ещё не всё», и жест к нему —
  // «включить остальное».
  if (catState(category) === 'on') keys.forEach((key) => next.delete(key));
  else keys.forEach((key) => next.add(key));
  chosen.value = next;
}

function toggleCollapse(folder: string): void {
  const next = new Set(collapsed.value);
  if (!next.delete(folder)) next.add(folder);
  collapsed.value = next;
}

/** Выбранные модели вместе с категорией — основа обоих действий. */
type Chosen = { category: string; name: string; entry: ModelEntry };

const chosenEntries = computed<Chosen[]>(() =>
  (scan.value?.categories ?? []).flatMap((category) =>
    category.entries
      .filter((entry) => chosen.value.has(keyOf(category.folder, entry.name)))
      .map((entry) => ({ category: category.folder, name: entry.name, entry })),
  ),
);

/** Что поедет: занятые имена остаются на месте и в перенос не идут. */
const moving = computed(() => chosenEntries.value.filter((item) => item.entry.sameName === null));

/** Сколько поедет — числами, которые видит пользователь до начала. */
const plan = computed(() => ({
  files: moving.value.reduce((sum, item) => sum + item.entry.files, 0),
  // specta отдаёт f64 как `number | null`: в JSON нет NaN.
  size: moving.value.reduce((sum, item) => sum + (item.entry.sizeBytes ?? 0), 0),
}));

async function migrate(): Promise<void> {
  busy.value = true;
  outcome.value = null;
  progress.value = null;
  try {
    const res = await commands.migrateModels(
      props.instance.id,
      moving.value.map((item) => [item.category, item.name] as [string, string]),
    );
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

/**
 * Занятые имена — все, независимо от выбора.
 *
 * Раньше это было видно только в отчёте после переноса, а до него сборка
 * с уже перенесёнными моделями выглядела тупиком: категории на месте,
 * «перенесётся 0 файлов», кнопка погашена и сделать нечего. Теперь то же
 * самое видно сразу из скана — переносить и не требовалось, требовалось
 * убрать локальные копии.
 *
 * Держится на этом счёте всё, что говорится словами: строки-пояснения
 * под списком и метка на свёрнутой категории.
 */
const occupied = computed(() =>
  (scan.value?.categories ?? []).flatMap((category) =>
    category.entries
      .filter((entry) => entry.sameName !== null)
      .map((entry) => ({ category: category.folder, entry })),
  ),
);

/** Признано дубликатом — такое можно убрать. */
const duplicates = computed(() =>
  occupied.value.filter((item) => item.entry.sameName !== 'different'),
);

/** Совпало имя, но не содержимое. Удалять такое нельзя. */
const different = computed(() =>
  occupied.value.filter((item) => item.entry.sameName === 'different'),
);

/** Сколько дубликатов в категории — метка нужна свёрнутой строке. */
function dupCount(folder: string): number {
  return duplicates.value.filter((item) => item.category === folder).length;
}

/**
 * Что именно убираем: выбранное среди дубликатов. Каждую строку можно
 * снять — одна модель из двадцати может быть нужна в сборке локально,
 * и ради неё не должно приходиться отказываться от уборки целиком.
 */
const picked = computed(() =>
  chosenEntries.value.filter(
    (item) => item.entry.sameName !== null && item.entry.sameName !== 'different',
  ),
);
// specta отдаёт f64 как `number | null`: в JSON нет NaN.
const freeable = computed(() =>
  picked.value.reduce((sum, item) => sum + (item.entry.sizeBytes ?? 0), 0),
);

async function cleanup(): Promise<void> {
  cleaning.value = true;
  try {
    const res = await commands.removeDuplicateModels(
      props.instance.id,
      picked.value.map((item) => [item.category, item.name] as [string, string]),
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
      <!-- Подсказкой идёт сам путь: в этой панели он больше нигде
           не показан, а идти разбираться руками приходится именно туда. -->
      <OpenFolderButton
        :path="scan?.path"
        :title="scan?.path"
        :disabled="!scan?.available"
      />
      <span class="spacer"></span>
      <button type="button" class="btn ghost" :disabled="loading || busy" @click="refresh">
        <svg class="ico"><use href="#i-reload" /></svg>
        {{ t('library.refresh') }}
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
        <!-- Один список на всё: категория раскрывается, и модели внутри
             показаны со своими вердиктами. Раскрытием управляет отдельная
             кнопка, а не вся строка, — в той же строке стоит тумблер,
             а кнопка внутри кнопки невалидна. -->
        <div class="cats">
          <template v-for="category in scan.categories" :key="category.folder">
            <div class="cat" :class="{ marked: dupCount(category.folder) > 0 }">
              <button
                type="button"
                class="disclose"
                :aria-expanded="!collapsed.has(category.folder)"
                @click="toggleCollapse(category.folder)"
              >
                <svg class="ico"><use href="#i-expand" /></svg>
                <code>{{ category.folder }}</code>
              </button>
              <span class="n">
                {{ t('migrate.entries', category.entries.length) }} ·
                {{ bytes(category.sizeBytes) }}
              </span>
              <!-- Метка нужна свёрнутой строке: иначе про дубликаты внутри
                   приходилось догадываться по нулю в сводке переноса. -->
              <span v-if="dupCount(category.folder) > 0" class="tag warn">
                {{ t('migrate.dup.inCategory', dupCount(category.folder)) }}
              </span>
              <button
                type="button"
                class="toggle"
                :class="{
                  off: catState(category) === 'off',
                  mixed: catState(category) === 'mixed',
                }"
                role="checkbox"
                :aria-checked="catChecked(category)"
                :aria-label="category.folder"
                :disabled="busy || cleaning"
                @click="toggleCategory(category)"
              ></button>
            </div>

            <template v-if="!collapsed.has(category.folder)">
              <div
                v-for="entry in category.entries"
                :key="entry.name"
                class="cat model"
                :class="{ marked: entry.sameName !== null }"
              >
                <code>{{ entry.name }}</code>
                <span class="n">{{ bytes(entry.sizeBytes) }}</span>
                <!-- Вердикт носит подсказку: из двух слов не видно, на чём
                     он основан, а разница существенная. У файла сверены
                     байты, у папки — только объём и число файлов внутри,
                     и «похоже, тот же» ровно об этом. -->
                <span
                  v-if="entry.sameName"
                  class="tag"
                  :class="{
                    warn: entry.sameName === 'likelyDuplicate',
                    stop: entry.sameName === 'different',
                  }"
                  :title="t(`migrate.verdict.why.${entry.sameName}`)"
                >
                  {{ t(`migrate.verdict.${entry.sameName}`) }}
                </span>
                <!-- У чужого содержимого под тем же именем тумблера нет:
                     его не переносят и не убирают никогда. Место под него
                     остаётся, иначе вердикты в соседних строках разъедутся. -->
                <button
                  v-if="selectable(entry)"
                  type="button"
                  class="toggle"
                  :class="{ off: !chosen.has(keyOf(category.folder, entry.name)) }"
                  role="switch"
                  :aria-checked="chosen.has(keyOf(category.folder, entry.name))"
                  :aria-label="keyOf(category.folder, entry.name)"
                  :disabled="busy || cleaning"
                  @click="toggle(keyOf(category.folder, entry.name))"
                ></button>
                <span v-else class="no-toggle"></span>
              </div>
            </template>
          </template>
        </div>

        <!-- Действия к этому списку не относится: чужое содержимое
             под тем же именем не переносят и не убирают. -->
        <p v-if="different.length" class="hint">
          {{ t('migrate.diff.line', { n: different.length }) }}
        </p>

        <!-- Файлы забираются из-под ComfyUI — у работающей сборки этого
             делать нельзя, и сказать надо прямо, а не гасить кнопки молча. -->
        <p v-if="running" class="hint bad">{{ t('migrate.mustStop') }}</p>
        <!-- Без подключения уборка лишила бы сборку моделей вовсе. -->
        <p v-if="duplicates.length && !instance.shared?.enabled" class="hint bad">
          {{ t('migrate.dup.needsConnection') }}
        </p>

        <!-- Каждая кнопка стоит рядом со своим описанием: одна удаляет
             файлы, другая переносит, и путать их нельзя. Уборка идёт
             первой — она про то, что уже перенесено, а перенос про то,
             что ещё нет.

             Сетка одна на оба ряда, а обёрток на ряд нет намеренно:
             от этого зависит, что кнопки одной ширины. Подробности
             в `.acts` в `design/styles/app.css`. -->
        <div class="act-grid">
          <template v-if="duplicates.length">
            <button
              type="button"
              class="btn danger"
              :disabled="cleaning || running || !picked.length || !instance.shared?.enabled"
              @click="cleanup"
            >
              {{ t('migrate.dup.remove') }}
            </button>
            <p class="hint">
              {{ t('migrate.dup.line', { n: duplicates.length, size: bytes(freeable) }) }}
            </p>
          </template>

          <!-- Во время переноса ряд уступает место полосе прогресса ниже. -->
          <template v-if="!progress">
            <button
              type="button"
              class="btn primary"
              :disabled="busy || running || plan.files === 0"
              @click="migrate"
            >
              {{ t('migrate.action') }}
            </button>
            <p class="hint">
              {{ t('migrate.summary', { files: plan.files, size: bytes(plan.size) }) }}
            </p>
          </template>
        </div>

        <div v-if="progress" class="group">
          <p class="t-sm">{{ progress.category }}/{{ progress.name }}</p>
          <div class="bar">
            <i :style="{ width: `${(progress.done / progress.total) * 100}%` }"></i>
          </div>
          <button type="button" class="btn danger" @click="commands.cancelMigrate()">
            {{ t('common.cancel') }}
          </button>
        </div>
      </template>

      <p v-else class="empty">{{ t('migrate.empty') }}</p>

      <!-- Отчёт о переносе. Что не поехало и почему — уже в списках выше,
           здесь только итог и сбои. -->
      <template v-if="outcome">
        <p class="hint">
          {{ t('migrate.moved', { n: outcome.moved.length, size: bytes(outcome.movedBytes) }) }}
        </p>
        <p v-if="outcome.failed.length" class="hint bad">
          {{ t('migrate.failed', outcome.failed.length) }}:
          {{ outcome.failed.map((f) => `${f.category}/${f.name}`).join(', ') }}
        </p>
      </template>
    </template>
  </div>
</template>
