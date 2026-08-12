<script setup lang="ts">
// Воркфлоу этой сборки: что внутри и что забрать в библиотеку.
//
// Список берётся у запущенной сборки по API, у остановленной — с диска.
// Разница не косметическая: запущенная знает про то, что сохранила минуту
// назад, а у остановленной другого источника и нет.
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import { commands, type Instance } from '../bindings';
import { displayStatus } from '../lib/status';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';
import { useWorkflowsStore } from '../stores/workflows';

const props = defineProps<{ instance: Instance }>();

const library = useWorkflowsStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

const names = ref<string[]>([]);
const loading = ref(false);
const busy = ref<string | null>(null);

const running = computed(
  () => displayStatus(props.instance, run.statusOf(props.instance.id)) === 'running',
);

onMounted(async () => {
  if (!library.loaded) await library.load();
  await refresh();
});

// Список у запущенной и остановленной сборки берётся из разных мест,
// поэтому перечитываем при смене состояния.
watch(running, () => void refresh());

async function refresh(): Promise<void> {
  loading.value = true;
  try {
    const res = await commands.instanceWorkflows(props.instance.id);
    names.value = res.status === 'ok' ? res.data : [];
    if (res.status === 'error') ui.pushError(res.error);
  } finally {
    loading.value = false;
  }
}

/** Забирает воркфлоу в библиотеку. Исходный остаётся в сборке. */
async function pull(rel: string, overwrite = false): Promise<void> {
  busy.value = rel;
  try {
    const res = await commands.pullWorkflow(
      props.instance.id,
      rel,
      library.path,
      overwrite,
    );
    if (res.status === 'error') {
      if (res.error.code === 'workflows.nameTaken') {
        // Имя занято — спрашиваем, а не решаем за пользователя.
        if (confirmReplace(rel)) await pull(rel, true);
        return;
      }
      ui.pushError(res.error);
      return;
    }
    await library.rescan();
    ui.pushOk(t('library.pull.done', { name: rel }));
  } finally {
    busy.value = null;
  }
}

/**
 * Подтверждение замены. Штатный `confirm` браузера, а не наш экран:
 * это единственное место, где вопрос простой, ответов два и оба
 * безобидны — воркфлоу в сборке при любом ответе остаётся на месте.
 */
function confirmReplace(rel: string): boolean {
  return window.confirm(t('library.pull.replace', { name: rel }));
}

const inLibrary = computed(() => new Set(library.items.map((i) => i.path)));
</script>

<template>
  <div class="group">
    <div class="row">
      <span class="t-label">{{ t('library.instance.title') }}</span>
      <span class="head-spacer"></span>
      <button type="button" class="btn ghost" :disabled="loading" @click="refresh">
        {{ t('library.retry') }}
      </button>
    </div>

    <!-- Библиотека не задана — забирать некуда. Ведём туда, где её задают. -->
    <p v-if="!library.configured" class="hint">
      {{ t('library.instance.noLibrary') }}
      <RouterLink to="/settings/workflow-library">{{ t('library.path.setUp') }}</RouterLink>
    </p>

    <div v-if="loading" class="bar indet"><i></i></div>

    <div v-else-if="names.length" class="wf-list">
      <div v-for="rel in names" :key="rel" class="wf-row">
        <span class="star off">·</span>
        <span class="nm">{{ rel }}</span>
        <span class="tags">
          <span v-if="inLibrary.has(rel)" class="tag">{{ t('library.instance.already') }}</span>
        </span>
        <button
          type="button"
          class="btn ghost"
          :disabled="!library.configured || !library.available || busy === rel"
          @click="pull(rel)"
        >
          {{ t('library.instance.pull') }}
        </button>
      </div>
    </div>

    <p v-else class="empty">{{ t('library.instance.empty') }}</p>

    <p class="hint">
      {{ running ? t('library.instance.fromRunning') : t('library.instance.fromDisk') }}
    </p>
  </div>
</template>
