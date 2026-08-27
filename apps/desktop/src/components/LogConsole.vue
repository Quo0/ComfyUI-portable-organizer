<script setup lang="ts">

//

import { nextTick, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import type { LogLine } from '../bindings';

const props = defineProps<{ lines: LogLine[] }>();

const { t } = useI18n();

const box = ref<HTMLPreElement | null>(null);

const follow = ref(true);

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

.log-follow {
  border: 0;
  cursor: pointer;
  font-family: inherit;
}
</style>
