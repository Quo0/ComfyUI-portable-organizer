<script setup lang="ts">

//

//

import { ArrowLeft, ExternalLink, FolderOpen, RotateCw, ScrollText } from '@lucide/vue';
import { computed, nextTick, onBeforeUnmount, onMounted, ref, useTemplateRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { openPath, openUrl } from '@tauri-apps/plugin-opener';

import EmptyNote from '../components/EmptyNote.vue';
import InstanceHeader from '../components/ui/InstanceHeader.vue';
import LogConsole from '../components/LogConsole.vue';
import StatusPill from '../components/StatusPill.vue';
import { commands, type AppError } from '../bindings';
import { errorText } from '../lib/errors';
import { accentVar, initial } from '../lib/format';
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

  void commands.hideComfy();
});

watch(() => props.id, async () => {
  problem.value = '';
  showLog.value = false;
  await run.loadLog(props.id);
  await refreshOutput();
  await apply();
});

watch([state, showLog], () => void apply());

async function apply(): Promise<void> {
  if (!instance.value) return;

  if (state.value === 'stopped' || state.value === 'crashed' || state.value === 'unavailable') {
    await commands.hideComfy();
    await router.replace(`/instances/${props.id}`);
    return;
  }

  if (!embedded.value) {
    await commands.hideComfy();
    return;
  }

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
  if (outputDir.value) void openPath(outputDir.value);
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

  await run.load();
  await refreshOutput();
}

async function stop(): Promise<void> {
  problem.value = '';
  await run.stop(props.id);
}

function fail(error: AppError): void {
  problem.value = errorText(error);

  ui.pushError(error);
}
</script>

<template>
  <var class="InstanceTabView">
    <section v-if="instance" class="screen tab-screen">
      <InstanceHeader toolbar>
        <RouterLink class="btn ghost" :to="`/instances/${instance.id}`">
          <ArrowLeft class="ico" />
          {{ t('common.back') }}
        </RouterLink>
        <span
          class="chip"
          :style="{ '--instance-accent': accentVar(instance.accent) }"
        >{{ initial(instance.name) }}</span>

        <span class="name">{{ instance.name }}</span>
        <span v-if="status?.port" class="port">127.0.0.1:{{ status.port }}</span>
        <StatusPill :status="state" />
        <span class="spacer"></span>

        <div class="tools">
          <button
            type="button"
            class="btn ghost icon"
            :aria-pressed="showLog"
            :title="showLog ? t('tab.canvas') : t('tab.logs')"
            :aria-label="showLog ? t('tab.canvas') : t('tab.logs')"
            @click="showLog = !showLog"
          >
            <ScrollText class="ico" />
          </button>
          <button
            type="button"
            class="btn ghost icon"
            :disabled="!outputDir"
            :title="outputDir ?? t('tab.outputMissing')"
            :aria-label="t('tab.output')"
            @click="openOutput"
          >
            <FolderOpen class="ico" />
          </button>
          <button
            type="button"
            class="btn ghost icon"
            :title="t('run.openInBrowser')"
            :aria-label="t('run.openInBrowser')"
            @click="openInBrowser"
          >
            <ExternalLink class="ico" />
          </button>
          <button
            type="button"
            class="btn ghost icon"
            :disabled="state !== 'running'"
            :title="t('tab.reload')"
            :aria-label="t('tab.reload')"
            @click="reload"
          >
            <RotateCw class="ico" />
          </button>
          <button type="button" class="btn secondary" :disabled="busy" @click="restart">
            {{ t('run.restart') }}
          </button>
          <button type="button" class="btn danger" :disabled="busy" @click="stop">
            {{ t('run.stop') }}
          </button>
        </div>
      </InstanceHeader>

      <div v-if="problem" class="banner bad">
        <b>{{ problem }}</b>
        <span class="spacer"></span>

        <button type="button" class="btn ghost" @click="apply">
          <RotateCw class="ico" />
          {{ t('tab.reload') }}
        </button>
      </div>

      <div v-if="state === 'starting' && !showLog" class="waiting">
        <p class="t-sm">{{ t('tab.starting') }}</p>
        <div class="bar indet"><i></i></div>
      </div>

      <div v-show="embedded" ref="slot" class="tab-slot"></div>

      <div v-if="showLog" class="log tab-log">
        <LogConsole v-if="lines.length" :lines="lines" />
        <EmptyNote v-else>{{ t('run.empty') }}</EmptyNote>
      </div>
    </section>
  </var>
</template>

<style scoped>

.tab-screen {
  min-height: 0;
  display: flex;
  flex-direction: column;
}

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

.waiting {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-4);
}
</style>
