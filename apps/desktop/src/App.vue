<script setup lang="ts">
// Спайк Фазы 0. Интерфейс намеренно черновой: его задача — не выглядеть,
// а показать четыре факта. Настоящие экраны появятся с Фазы 0.5.
import { ref, onMounted, onBeforeUnmount, useTemplateRef, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

type LogLine = { stream: 'stdout' | 'stderr' | 'webview'; text: string };

const lines = ref<LogLine[]>([]);
const status = ref('Остановлен');
const busy = ref(false);
const embedded = ref(false);
const firstLineAfter = ref<number | null>(null);

const slot = useTemplateRef<HTMLDivElement>('slot');
let unlisten: UnlistenFn | null = null;
let startedAt = 0;
let observer: ResizeObserver | null = null;

onMounted(async () => {
  unlisten = await listen<LogLine>('comfy-log', (e) => {
    // Проверка живого стриминга: засекаем, через сколько пришла первая строка.
    if (firstLineAfter.value === null && startedAt) {
      firstLineAfter.value = Math.round((performance.now() - startedAt) / 100) / 10;
    }
    lines.value.push(e.payload);
    if (lines.value.length > 2000) lines.value.splice(0, 500);
  });
});

onBeforeUnmount(() => {
  unlisten?.();
  observer?.disconnect();
});

async function start() {
  busy.value = true;
  lines.value = [];
  firstLineAfter.value = null;
  startedAt = performance.now();
  status.value = 'Стартует';
  try {
    const port = await invoke<number>('start_comfy');
    const secs = await invoke<number>('wait_ready', { port, timeoutSecs: 300 });
    status.value = `Работает на :${port}, готов за ${secs} с`;
    await embed(port);
  } catch (e) {
    status.value = `Ошибка: ${String(e)}`;
  } finally {
    busy.value = false;
  }
}

async function embed(port: number) {
  await nextTick();
  const el = slot.value;
  if (!el) return;

  // Автолейаута у дочернего вебвью нет: он нативное окно поверх нашего HTML,
  // поэтому прямоугольник пересчитывается вручную на каждое изменение.
  const sync = () => {
    const r = el.getBoundingClientRect();
    void invoke('embed_comfy', {
      port,
      x: r.left,
      y: r.top,
      w: r.width,
      h: r.height,
    });
  };
  sync();
  embedded.value = true;

  observer?.disconnect();
  observer = new ResizeObserver(sync);
  observer.observe(el);
  window.addEventListener('resize', sync);
}

async function stop() {
  busy.value = true;
  try {
    await invoke('stop_comfy');
    embedded.value = false;
    status.value = 'Остановлен';
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <div class="spike">
    <header>
      <strong>Спайк Фазы 0</strong>
      <span class="status">{{ status }}</span>
      <span v-if="firstLineAfter !== null" class="metric">
        первая строка лога через {{ firstLineAfter }} с
      </span>
      <span class="spacer"></span>
      <button :disabled="busy" @click="start">Запустить ComfyUI</button>
      <button :disabled="busy" @click="stop">Остановить</button>
    </header>

    <!-- Когда вебвью создан, лог остаётся полосой снизу: именно в него
         приходят строки о том, что вкладка реально увидела. -->
    <main :class="{ split: embedded }">
      <div v-show="embedded" ref="slot" class="slot"></div>
      <pre class="log"><span
        v-for="(l, i) in lines"
        :key="i"
        :class="l.stream"
      >{{ l.text }}
</span></pre>
    </main>
  </div>
</template>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
  overflow: hidden;
}
</style>

<style scoped>
.spike {
  height: 100%;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  font: 13px/1.45 'Segoe UI Variable Text', 'Segoe UI', system-ui, sans-serif;
  background: #171613;
  color: #f2f0eb;
}
header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-bottom: 1px solid #35322b;
  background: #201f1b;
}
.status {
  color: #a9a399;
}
.metric {
  font-family: 'Cascadia Mono', Consolas, monospace;
  font-size: 11px;
  color: #6fbf74;
}
.spacer {
  margin-left: auto;
}
button {
  font: inherit;
  padding: 4px 10px;
  border: 1px solid #4a463d;
  border-radius: 3px;
  background: #eeece6;
  color: #171613;
  cursor: pointer;
}
button:disabled {
  opacity: 0.45;
  cursor: default;
}
main {
  min-height: 0;
  display: grid;
  grid-template-rows: minmax(0, 1fr);
}
main.split {
  grid-template-rows: minmax(0, 1fr) 190px;
}
.log {
  height: 100%;
  margin: 0;
  padding: 10px 12px;
  overflow: auto;
  font-family: 'Cascadia Mono', Consolas, monospace;
  font-size: 12px;
  white-space: pre-wrap;
  word-break: break-word;
}
.log .stderr {
  color: #ddd9d1;
}
.log .stdout {
  color: #8b857a;
}
.log .webview {
  color: #559de4;
}
.slot {
  height: 100%;
}
</style>
