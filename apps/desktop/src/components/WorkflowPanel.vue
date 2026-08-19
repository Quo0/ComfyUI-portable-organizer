<script setup lang="ts">
// Воркфлоу этой сборки: что внутри и что забрать в библиотеку.
//
// Список берётся у запущенной сборки по API, у остановленной — с диска.
// Разница не косметическая: запущенная знает про то, что сохранила минуту
// назад, а у остановленной другого источника и нет.
import { RotateCw } from '@lucide/vue';
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import {
  commands,
  type Instance,
  type InstanceWorkflow,
  type InstanceWorkflowsDir,
} from '../bindings';
import EmptyNote from './EmptyNote.vue';
import OpenFolderButton from './OpenFolderButton.vue';
import { displayStatus } from '../lib/status';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';
import { useWorkflowsStore } from '../stores/workflows';

const props = defineProps<{ instance: Instance }>();

const library = useWorkflowsStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

const entries = ref<InstanceWorkflow[]>([]);
/** Папка воркфлоу сборки: в панели её путь больше нигде не показан. */
const dir = ref<InstanceWorkflowsDir | null>(null);
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
    // Библиотека уходит на бэкенд вместе с запросом: вердикт по каждому
    // воркфлоу считается там, где можно прочитать оба файла. Здесь
    // о совпадении имён известно, а о содержимом — нет.
    const res = await commands.instanceWorkflows(props.instance.id, library.path);
    entries.value = res.status === 'ok' ? res.data : [];
    if (res.status === 'error') ui.pushError(res.error);

    // Перечитывается вместе со списком: папку ComfyUI заводит при первом
    // сохранении, и до него показывать в проводнике нечего. Своей ошибки
    // не показываем — на том же обновлении её уже показал список, и второй
    // тост про то же самое лишний.
    const folder = await commands.instanceWorkflowsDir(props.instance.id);
    dir.value = folder.status === 'ok' ? folder.data : null;
  } finally {
    loading.value = false;
  }
}

const inLibrary = computed(() => new Set(library.items.map((i) => i.path)));

/**
 * Свободное имя в библиотеке: `flux/portrait.json` → `flux/portrait (2).json`.
 *
 * Нужно на разошедшихся версиях: имя занято чужой работой, а своя должна
 * лечь рядом, а не поверх. Считается здесь, а не спрашивается у пользователя,
 * — печатать путь руками в этом месте не за чем, а предложенное имя видно
 * в вопросе до нажатия.
 */
function freeName(rel: string): string {
  const dot = rel.lastIndexOf('.');
  const stem = dot === -1 ? rel : rel.slice(0, dot);
  const ext = dot === -1 ? '' : rel.slice(dot);
  for (let n = 2; ; n += 1) {
    const candidate = `${stem} (${n})${ext}`;
    if (!inLibrary.value.has(candidate)) return candidate;
  }
}

/**
 * Переносит воркфлоу в библиотеку: из сборки он уходит.
 *
 * Разошедшаяся версия забирается под свободным именем и только после
 * вопроса. Штатный `confirm` браузера, а не наш экран: вопрос простой,
 * ответов два, и оба безобидны — при отказе не двигается ничего,
 * при согласии в библиотеке ничего не заменяется.
 *
 * Список после переноса перечитывается весь, а не правится на месте:
 * файл ушёл из сборки, и перечень сборки изменился тоже.
 */
async function pull(entry: InstanceWorkflow): Promise<void> {
  let target: string | null = null;
  if (entry.library === 'diverged') {
    target = freeName(entry.path);
    if (!window.confirm(t('library.pull.underNewName', { name: entry.path, as: target }))) {
      return;
    }
  }

  busy.value = entry.path;
  try {
    const res = await commands.pullWorkflow(
      props.instance.id,
      entry.path,
      library.path,
      target,
    );
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    await library.rescan();
    await refresh();
    ui.pushOk(t('library.pull.done', { name: res.data }));
  } finally {
    busy.value = null;
  }
}
</script>

<template>
  <div class="group">
    <div class="row">
      <span class="t-label">{{ t('library.instance.title') }}</span>
      <!-- Тот же значок и то же место, что у моделей сборки: путь к папке
           воркфлоу в панели больше нигде не показан, а идти разбираться
           руками приходится именно туда. -->
      <OpenFolderButton :path="dir?.path" :title="dir?.path" :disabled="!dir?.available" />
      <span class="spacer"></span>
      <button type="button" class="btn ghost" :disabled="loading" @click="refresh">
        <RotateCw class="ico" />
        {{ t('library.refresh') }}
      </button>
    </div>

    <!-- Библиотека не задана — забирать некуда. Ведём туда, где её задают. -->
    <p v-if="!library.configured" class="hint">
      {{ t('library.instance.noLibrary') }}
      <RouterLink to="/settings/workflow-library">{{ t('library.path.setUp') }}</RouterLink>
    </p>

    <div v-if="loading" class="bar indet"><i></i></div>

    <!-- `of-instance`: в строке нет ни отметки, ни звёздочки избранного,
         и сетка у неё своя. Избранное живёт в манифесте библиотеки, а эти
         файлы в неё ещё не попали. -->
    <div v-else-if="entries.length" class="wf-list of-instance">
      <div v-for="entry in entries" :key="entry.path" class="wf-row">
        <span class="nm">{{ entry.path }}</span>
        <!-- Два вердикта, а не один: метка по одному имени врала бы.
             Правленная в сборке версия под занятым именем выглядела бы
             сохранённой — и пропала бы при первой же уборке папки. -->
        <span class="tags">
          <span v-if="entry.library === 'same'" class="tag">
            {{ t('library.instance.already') }}
          </span>
          <span v-else-if="entry.library === 'diverged'" class="tag warn">
            {{ t('library.instance.diverged') }}
          </span>
        </span>
        <!-- Погашена только у того, что уже лежит в библиотеке тем же
             файлом. У разошедшегося кнопка работает: это чужая работа
             под тем же именем, и забрать её надо под своим. -->
        <button
          type="button"
          class="btn ghost"
          :disabled="
            !library.configured ||
            !library.available ||
            busy === entry.path ||
            entry.library === 'same'
          "
          @click="pull(entry)"
        >
          {{ t('library.instance.pull') }}
        </button>
      </div>
    </div>

    <EmptyNote v-else>{{ t('library.instance.empty') }}</EmptyNote>

    <!-- Что кнопка уносит файл из сборки, сказано словами и заранее:
         узнать это по исчезнувшей строке — худший из способов. -->
    <p class="hint">{{ t('library.instance.moves') }}</p>

    <!-- Легенда меток. Слева стоит та же самая метка, что в строке, справа —
         что она значит. Одним абзацем это уже было: метку в нём приходилось
         сперва найти глазами, а объяснения двух разных вердиктов сливались
         в один текст. Показывается только со списком: без строк объяснять
         нечего. -->
    <dl v-if="entries.length" class="tag-legend">
      <dt><span class="tag">{{ t('library.instance.already') }}</span></dt>
      <dd>{{ t('library.instance.alreadyMeans') }}</dd>
      <dt><span class="tag warn">{{ t('library.instance.diverged') }}</span></dt>
      <dd>{{ t('library.instance.divergedMeans') }}</dd>
    </dl>

    <p class="hint">
      {{ running ? t('library.instance.fromRunning') : t('library.instance.fromDisk') }}
    </p>
  </div>
</template>
