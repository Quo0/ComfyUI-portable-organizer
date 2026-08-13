<script setup lang="ts">
// Встроенная вкладка: ComfyUI внутри нашего окна.
//
// Ради этого экрана всё и затевалось, и он же — единственный, где
// прокрутки нет вовсе. Область вкладки занимает нативное окно поверх
// нашего HTML: прокрутись контейнер, разметка уехала бы, а окно осталось
// на месте, и они разошлись бы навсегда.
//
// Отсюда же и остальные решения: тулбар полосой сверху, ошибки баннером
// под ним, консоль логов не поверх вкладки, а вместо неё.
import { computed, nextTick, onBeforeUnmount, onMounted, ref, useTemplateRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { openUrl, revealItemInDir } from '@tauri-apps/plugin-opener';

import LogConsole from '../components/LogConsole.vue';
import StatusPill from '../components/StatusPill.vue';
import { commands, type AppError } from '../bindings';
import { errorText } from '../lib/errors';
import { accentVar } from '../lib/format';
import { displayStatus } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ id: string }>();

const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const router = useRouter();
const { t } = useI18n();

const slot = useTemplateRef<HTMLDivElement>('slot');

/** Ошибки этого экрана. В тост нельзя: он ушёл бы под нативное окно. */
const problem = ref('');
const showLog = ref(false);
const outputDir = ref<string | null>(null);
let observer: ResizeObserver | null = null;

const instance = computed(() => instances.byId(props.id));
const status = computed(() => run.statusOf(props.id));
const state = computed(() =>
  instance.value ? displayStatus(instance.value, status.value) : 'stopped',
);
const lines = computed(() => run.logs[props.id] ?? []);
const busy = computed(() => run.busy[props.id] === true);

/** Вкладка показывается только у готового сервера: раньше её просто нет. */
const embedded = computed(() => state.value === 'running' && !showLog.value);

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  await run.load();
  await run.loadLog(props.id);
  await refreshOutput();
  await apply();
  window.addEventListener('resize', place);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', place);
  observer?.disconnect();
  // Прячем, а не закрываем: процесс продолжает работать, а несохранённый
  // граф остаётся в живой странице до самой остановки сборки.
  void commands.hideComfy();
});

// Переключение между двумя работающими сборками не пересоздаёт компонент —
// меняется только параметр роута.
watch(() => props.id, async () => {
  problem.value = '';
  showLog.value = false;
  await run.loadLog(props.id);
  await refreshOutput();
  await apply();
});

// Сюда же приходит и приход сборки в готовность: вкладка появляется сама,
// без единого действия пользователя.
watch([state, showLog], () => void apply());

/** Показывает или прячет вкладку по текущему состоянию экрана. */
async function apply(): Promise<void> {
  if (!instance.value) return;

  // Остановленной сборке показывать нечего, а тулбар над пустотой врёт.
  if (state.value === 'stopped' || state.value === 'crashed' || state.value === 'unavailable') {
    await commands.hideComfy();
    await router.replace(`/instances/${props.id}`);
    return;
  }

  if (!embedded.value) {
    await commands.hideComfy();
    return;
  }

  // Контейнер обязан быть на экране до замера: у элемента под v-show
  // getBoundingClientRect возвращает нули, и вебвью получил бы нулевой
  // размер — то есть остался бы невидимым без единой ошибки.
  await nextTick();
  const rect = measure();
  if (!rect) return;

  const res = await commands.showComfy(props.id, rect);
  if (res.status === 'error') {
    problem.value = errorText(res.error);
    return;
  }
  problem.value = '';
  watchSize();
}

function measure(): { x: number; y: number; w: number; h: number } | null {
  const el = slot.value;
  if (!el) return null;
  const r = el.getBoundingClientRect();
  if (r.width < 1 || r.height < 1) return null;
  return { x: r.left, y: r.top, w: r.width, h: r.height };
}

/**
 * Автолейаута у дочернего вебвью нет. Прямоугольник пересчитывается
 * на каждое изменение: ресайз окна, сворачивание рейла, появление баннера.
 */
function watchSize(): void {
  if (observer || !slot.value) return;
  observer = new ResizeObserver(() => void place());
  observer.observe(slot.value);
}

async function place(): Promise<void> {
  if (!embedded.value) return;
  const rect = measure();
  if (!rect) return;
  const res = await commands.placeComfy(props.id, rect);
  if (res.status === 'error') problem.value = errorText(res.error);
}

async function refreshOutput(): Promise<void> {
  const res = await commands.instanceOutputDir(props.id, status.value?.profileId ?? null);
  outputDir.value = res.status === 'ok' ? res.data : null;
}

function openOutput(): void {
  if (outputDir.value) void revealItemInDir(outputDir.value);
}

function openInBrowser(): void {
  const port = status.value?.port;
  if (port) void openUrl(`http://127.0.0.1:${port}`);
}

async function reload(): Promise<void> {
  const res = await commands.reloadComfy(props.id);
  if (res.status === 'error') problem.value = errorText(res.error);
}

async function restart(): Promise<void> {
  problem.value = '';
  const res = await commands.restartInstance(props.id);
  if (res.status === 'error') {
    fail(res.error);
    return;
  }
  // Порт после рестарта другой, и вкладка уже закрыта бэкендом:
  // новую создаст `apply`, когда сервер придёт в готовность.
  await run.load();
  await refreshOutput();
}

async function stop(): Promise<void> {
  problem.value = '';
  await run.stop(props.id);
}

function fail(error: AppError): void {
  problem.value = errorText(error);
  // Тост тут не годится, но и терять ошибку насовсем нельзя: в очереди
  // она останется после ухода с экрана.
  ui.pushError(error);
}
</script>

<template>
  <section v-if="instance" class="screen tab-screen">
    <div class="inst-toolbar">
      <RouterLink class="btn ghost" :to="`/instances/${instance.id}`">
        <svg class="ico"><use href="#i-back" /></svg>
        {{ t('common.back') }}
      </RouterLink>
      <span
        class="chip"
        :style="{ '--instance-accent': accentVar(instance.accent) }"
      ></span>
      <!-- Имя инстанса и адрес не переводятся никогда. -->
      <span class="name">{{ instance.name }}</span>
      <span v-if="status?.port" class="port">127.0.0.1:{{ status.port }}</span>
      <StatusPill :status="state" />
      <span class="spacer"></span>
      <!-- Значки, а не подписи: шесть текстовых кнопок в ряд не помещались
           на узком окне. Словами остались только остановка и перезапуск —
           у них цена ошибки выше, чем экономия места. -->
      <div class="tools">
        <button
          type="button"
          class="btn ghost"
          :aria-pressed="showLog"
          :title="showLog ? t('tab.canvas') : t('tab.logs')"
          :aria-label="showLog ? t('tab.canvas') : t('tab.logs')"
          @click="showLog = !showLog"
        >
          <svg class="ico"><use href="#i-log" /></svg>
        </button>
        <button
          type="button"
          class="btn ghost"
          :disabled="!outputDir"
          :title="outputDir ?? t('tab.outputMissing')"
          :aria-label="t('tab.output')"
          @click="openOutput"
        >
          <svg class="ico"><use href="#i-folder" /></svg>
        </button>
        <button
          type="button"
          class="btn ghost"
          :title="t('run.openInBrowser')"
          :aria-label="t('run.openInBrowser')"
          @click="openInBrowser"
        >
          <svg class="ico"><use href="#i-external" /></svg>
        </button>
        <button
          type="button"
          class="btn ghost"
          :disabled="state !== 'running'"
          :title="t('tab.reload')"
          :aria-label="t('tab.reload')"
          @click="reload"
        >
          <svg class="ico"><use href="#i-reload" /></svg>
        </button>
        <button type="button" class="btn secondary" :disabled="busy" @click="restart">
          {{ t('run.restart') }}
        </button>
        <button type="button" class="btn danger" :disabled="busy" @click="stop">
          {{ t('run.stop') }}
        </button>
      </div>
    </div>

    <!-- Баннер стоит над областью вкладки и просто уменьшает её
         прямоугольник: ResizeObserver пересчитает его сам. Плавающий тост
         на этом экране оказался бы под нативным окном, то есть невидим. -->
    <div v-if="problem" class="banner bad">
      <b>{{ problem }}</b>
      <span class="spacer"></span>
      <button type="button" class="btn ghost" @click="apply">{{ t('tab.retry') }}</button>
    </div>

    <div v-if="state === 'starting' && !showLog" class="waiting">
      <p class="t-sm">{{ t('tab.starting') }}</p>
      <div class="bar indet"><i></i></div>
    </div>

    <!-- Место вкладки. Пусто по построению: сюда встаёт нативное окно. -->
    <div v-show="embedded" ref="slot" class="tab-slot"></div>

    <!-- Лог занимает место вкладки, а не ложится поверх: положить что-либо
         поверх нативного окна невозможно. -->
    <div v-if="showLog" class="tab-log">
      <LogConsole v-if="lines.length" :lines="lines" />
      <p v-else class="hint">{{ t('run.empty') }}</p>
    </div>
  </section>
</template>

<style scoped>
/* Прокрутки нет ни у одного элемента этого экрана — она развела бы
   разметку с нативным окном.
   Колонка, а не сетка с фиксированными строками: блоки над вкладкой
   появляются и исчезают, и подсчитанные заранее строки разъехались бы
   с реальным их числом. */
.tab-screen {
  min-height: 0;
  display: flex;
  flex-direction: column;
}

/* Место вкладки: всё, что осталось от тулбара и баннера. */
.tab-slot {
  flex: 1;
  min-height: 0;
}

.tab-log {
  flex: 1;
  min-height: 0;
  display: grid;
  padding: var(--space-3);
}

.banner {
  margin: var(--space-2) var(--space-3);
}

.banner .spacer {
  margin-left: auto;
}

.waiting {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
}
</style>
