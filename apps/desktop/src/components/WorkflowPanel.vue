<script setup lang="ts">

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
import Group from './ui/Group.vue';
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

watch(running, () => void refresh());

async function refresh(): Promise<void> {
  loading.value = true;
  try {
    const res = await commands.instanceWorkflows(props.instance.id, library.path);
    entries.value = res.status === 'ok' ? res.data : [];
    if (res.status === 'error') ui.pushError(res.error);

    const folder = await commands.instanceWorkflowsDir(props.instance.id);
    dir.value = folder.status === 'ok' ? folder.data : null;
  } finally {
    loading.value = false;
  }
}

const inLibrary = computed(() => new Set(library.items.map((i) => i.path)));

function freeName(rel: string): string {
  const dot = rel.lastIndexOf('.');
  const stem = dot === -1 ? rel : rel.slice(0, dot);
  const ext = dot === -1 ? '' : rel.slice(dot);
  for (let n = 2; ; n += 1) {
    const candidate = `${stem} (${n})${ext}`;
    if (!inLibrary.value.has(candidate)) return candidate;
  }
}

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
  <var class="WorkflowPanel">
    <Group>
      <div class="row">
        <span class="t-label">{{ t('library.instance.title') }}</span>

        <OpenFolderButton :path="dir?.path" :title="dir?.path" :disabled="!dir?.available" />
        <span class="spacer"></span>
        <button type="button" class="btn ghost" :disabled="loading" @click="refresh">
          <RotateCw class="ico" />
          {{ t('library.refresh') }}
        </button>
      </div>

      <p v-if="!library.configured" class="hint">
        {{ t('library.instance.noLibrary') }}
        <RouterLink to="/settings/workflow-library">{{ t('library.path.setUp') }}</RouterLink>
      </p>

      <div v-if="loading" class="bar indet"><i></i></div>

      <div v-else-if="entries.length" class="wf-list of-instance">
        <div v-for="entry in entries" :key="entry.path" class="wf-row">
          <span class="nm">{{ entry.path }}</span>

          <span class="tags">
            <span v-if="entry.library === 'same'" class="tag">
              {{ t('library.instance.already') }}
            </span>
            <span v-else-if="entry.library === 'diverged'" class="tag warn">
              {{ t('library.instance.diverged') }}
            </span>
          </span>

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

      <p class="hint">{{ t('library.instance.moves') }}</p>

      <dl v-if="entries.length" class="tag-legend">
        <dt><span class="tag">{{ t('library.instance.already') }}</span></dt>
        <dd>{{ t('library.instance.alreadyMeans') }}</dd>
        <dt><span class="tag warn">{{ t('library.instance.diverged') }}</span></dt>
        <dd>{{ t('library.instance.divergedMeans') }}</dd>
      </dl>

      <p class="hint">
        {{ running ? t('library.instance.fromRunning') : t('library.instance.fromDisk') }}
      </p>
    </Group>
  </var>
</template>
