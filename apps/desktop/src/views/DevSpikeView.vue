<script setup lang="ts">
// Стенд Фазы 0, доступный по /dev/spike. В рейле его нет: это не раздел
// приложения, а единственный способ до Фазы 3 увидеть встроенную вкладку.
//
// Строки здесь намеренно не переведены — экран не входит в интерфейс
// и до релиза будет удалён. Всё остальное, что видит пользователь,
// обязано идти через t().
//
// Ценность стенда для Фазы 0.5 в том, что он показывает пересчёт границ:
// свернуть рейл при открытой вкладке — и она обязана переехать следом.
import { ref, onMounted, onBeforeUnmount, useTemplateRef, nextTick } from 'vue';
import type { UnlistenFn } from '@tauri-apps/api/event';

import { commands, events, type SpikeLog } from '../bindings';
import { errorText } from '../lib/errors';
import { useUiStore } from '../stores/ui';

const ui = useUiStore();

const lines = ref<SpikeLog[]>([]);
const status = ref('Остановлен');
const busy = ref(false);
const embedded = ref(false);

const slot = useTemplateRef<HTMLDivElement>('slot');
const unlisten: UnlistenFn[] = [];
let observer: ResizeObserver | null = null;
let port = 0;

onMounted(async () => {
  unlisten.push(
    await events.spikeLog.listen((e) => {
      lines.value.push(e.payload);
      if (lines.value.length > 2000) lines.value.splice(0, 500);
    }),
  );
  unlisten.push(
    await events.spikeReady.listen(async (e) => {
      status.value = `Работает на :${e.payload.port}, готов за ${e.payload.secs} с`;
      await embed(e.payload.port);
    }),
  );
});

onBeforeUnmount(() => {
  unlisten.forEach((off) => off());
  observer?.disconnect();
  window.removeEventListener('resize', sync);
  // Вкладку прячем, а не закрываем: процесс продолжает работать,
  // уход в другой раздел его не прерывает.
  if (embedded.value) void commands.hideComfy();
});

async function start(): Promise<void> {
  busy.value = true;
  lines.value = [];
  status.value = 'Стартует';
  try {
    const started = await commands.startComfy();
    if (started.status === 'error') {
      status.value = errorText(started.error);
      ui.pushError(started.error);
      return;
    }
    const ready = await commands.waitReady(started.data, 300);
    if (ready.status === 'error') {
      status.value = errorText(ready.error);
      ui.pushError(ready.error);
      return;
    }
    status.value = `Работает на :${started.data}, готов за ${ready.data} с`;
    await embed(started.data);
  } finally {
    busy.value = false;
  }
}

async function embed(readyPort: number): Promise<void> {
  port = readyPort;
  // Контейнер надо показать ДО замера: у элемента под v-show
  // getBoundingClientRect возвращает нули, и вебвью получил бы нулевой
  // размер — то есть остался бы невидимым без единой ошибки.
  embedded.value = true;
  await nextTick();
  await sync();

  // Автолейаута у дочернего вебвью нет: он нативное окно поверх нашего
  // HTML. Прямоугольник пересчитывается на каждое изменение — ресайз окна,
  // сворачивание рейла, навигацию.
  observer?.disconnect();
  if (slot.value) {
    observer = new ResizeObserver(() => void sync());
    observer.observe(slot.value);
  }
  window.addEventListener('resize', sync);
}

async function sync(): Promise<void> {
  const el = slot.value;
  if (!el) return;
  const r = el.getBoundingClientRect();
  if (r.width < 1 || r.height < 1) return;
  const res = await commands.embedComfy(port, r.left, r.top, r.width, r.height);
  // Молча глотать нельзя: без вкладки стенд бессмыслен, а ошибка
  // встраивания никак иначе себя не проявит.
  if (res.status === 'error') ui.pushError(res.error);
}

async function stop(): Promise<void> {
  busy.value = true;
  try {
    const res = await commands.stopComfy();
    if (res.status === 'error') ui.pushError(res.error);
    embedded.value = false;
    status.value = 'Остановлен';
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">Спайк Фазы 0</h1>
      <span class="t-sm">{{ status }}</span>
      <span style="margin-left: auto"></span>
      <div class="row">
        <button type="button" class="btn primary" :disabled="busy" @click="start">
          Запустить ComfyUI
        </button>
        <button type="button" class="btn secondary" :disabled="busy" @click="stop">
          Остановить
        </button>
      </div>
    </header>

    <!-- Когда вкладка создана, лог остаётся полосой снизу: в него приходят
         строки о том, что вкладка реально увидела. -->
    <div class="spike-body" :class="{ split: embedded }">
      <div v-show="embedded" ref="slot" class="spike-slot"></div>
      <pre class="console"><span
        v-for="(l, i) in lines"
        :key="i"
        :class="{ dim: l.stream === 'stdout' }"
      >{{ l.text }}
</span></pre>
    </div>
  </section>
</template>

<style scoped>
.spike-body {
  min-height: 0;
  display: grid;
  grid-template-rows: minmax(0, 1fr);
}
.spike-body.split {
  grid-template-rows: minmax(0, 1fr) 180px;
}
.spike-slot {
  min-height: 0;
}
.console {
  min-height: 0;
  overflow: auto;
  margin: 0;
}
</style>
