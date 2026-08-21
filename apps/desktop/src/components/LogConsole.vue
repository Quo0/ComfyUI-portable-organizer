<script setup lang="ts">
// Консоль запуска. Живёт отдельным компонентом, потому что показывается
// в двух местах: на экране инстанса под кнопкой «Запустить» и на экране
// встроенной вкладки вместо самой вкладки.
//
// Второй случай и объясняет, почему консоль не может быть панелью поверх:
// вкладка — нативное окно поверх нашего HTML, и перекрыть её нечем.
// Поэтому лог занимает её место, а вкладка на это время прячется.
import { nextTick, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import type { LogLine } from '../bindings';

const props = defineProps<{ lines: LogLine[] }>();

const { t } = useI18n();

const box = ref<HTMLPreElement | null>(null);
/** Прокрутка следует за логом, пока пользователь сам не отмотал вверх. */
const follow = ref(true);
/**
 * Сколько строк пришло, пока пользователь читал верх лога. Без счётчика
 * отмотавший вверх не видит, что внизу вообще что-то происходит: холодный
 * старт пишет сотни строк, и вернуться к ним нечем, кроме как тащить
 * полосу прокрутки до упора.
 */
const pending = ref(0);

watch(
  () => props.lines.length,
  (now, before) => {
    if (follow.value) {
      pending.value = 0;
      void scrollDown();
      return;
    }
    pending.value += Math.max(0, now - (before ?? now));
  },
  { immediate: true },
);

async function scrollDown(): Promise<void> {
  await nextTick();
  const el = box.value;
  if (el) el.scrollTop = el.scrollHeight;
}

function onScroll(): void {
  const el = box.value;
  if (!el) return;
  // Отступ в пару строк: точное сравнение ломается на дробной высоте строки.
  follow.value = el.scrollHeight - el.scrollTop - el.clientHeight < 40;
  if (follow.value) pending.value = 0;
}

function toLatest(): void {
  follow.value = true;
  pending.value = 0;
  void scrollDown();
}
</script>

<template>
  <var class="LogConsole">
    <pre ref="box" class="console" @scroll="onScroll"><span
      v-for="(line, i) in lines"
      :key="i"
      :class="{ dim: line.stream === 'stdout' }"
    >{{ line.text }}
</span></pre>

    <!-- Кнопка появляется только когда есть что показать: следующему за логом
         возвращаться некуда. Позиционируется по родителю `.log`. -->
    <button v-if="!follow && pending" type="button" class="log-follow" @click="toLatest">
      {{ t('run.toLatest') }}
      <span class="n">+{{ pending }}</span>
    </button>
  </var>
</template>

<style scoped>
.console {
  margin: 0;
  min-height: 0;
  overflow: auto;
}

/* Кнопка из дизайн-системы, но там она — не кнопка, а `span`: сбрасываем
   рамку и курсор, оставляя вид. */
.log-follow {
  border: 0;
  cursor: pointer;
  font-family: inherit;
}
</style>
