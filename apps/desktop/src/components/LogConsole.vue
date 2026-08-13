<script setup lang="ts">
// Консоль запуска. Живёт отдельным компонентом, потому что показывается
// в двух местах: на экране инстанса под кнопкой «Запустить» и на экране
// встроенной вкладки вместо самой вкладки.
//
// Второй случай и объясняет, почему консоль не может быть панелью поверх:
// вкладка — нативное окно поверх нашего HTML, и перекрыть её нечем.
// Поэтому лог занимает её место, а вкладка на это время прячется.
import { nextTick, ref, watch } from 'vue';

import type { LogLine } from '../bindings';

const props = defineProps<{ lines: LogLine[] }>();

const box = ref<HTMLPreElement | null>(null);
/** Прокрутка следует за логом, пока пользователь сам не отмотал вверх. */
const follow = ref(true);

watch(() => props.lines.length, () => void scrollDown(), { immediate: true });

async function scrollDown(): Promise<void> {
  if (!follow.value) return;
  await nextTick();
  const el = box.value;
  if (el) el.scrollTop = el.scrollHeight;
}

function onScroll(): void {
  const el = box.value;
  if (!el) return;
  // Отступ в пару строк: точное сравнение ломается на дробной высоте строки.
  follow.value = el.scrollHeight - el.scrollTop - el.clientHeight < 40;
}
</script>

<template>
  <pre ref="box" class="console" @scroll="onScroll"><span
    v-for="(line, i) in lines"
    :key="i"
    :class="{ dim: line.stream === 'stdout' }"
  >{{ line.text }}
</span></pre>
</template>

<style scoped>
.console {
  margin: 0;
  min-height: 0;
  overflow: auto;
}
</style>
