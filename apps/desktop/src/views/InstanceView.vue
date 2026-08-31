<script setup lang="ts">

import { ArrowLeft } from '@lucide/vue';
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';
import { openPath } from '@tauri-apps/plugin-opener';

import EmptyNote from '../components/EmptyNote.vue';
import Field from '../components/ui/Field.vue';
import Group from '../components/ui/Group.vue';
import InstanceHeader from '../components/ui/InstanceHeader.vue';
import KeyValueList from '../components/ui/KeyValueList.vue';
import KeyValueRow from '../components/ui/KeyValueRow.vue';
import InstanceFields from '../components/InstanceFields.vue';
import LaunchControls from '../components/LaunchControls.vue';
import LaunchNotices from '../components/LaunchNotices.vue';
import LogConsole from '../components/LogConsole.vue';
import ModelsPanel from '../components/ModelsPanel.vue';
import PathText from '../components/PathText.vue';
import WorkflowPanel from '../components/WorkflowPanel.vue';
import SharedPanel from '../components/SharedPanel.vue';
import StatusPill from '../components/StatusPill.vue';
import type { InstanceEdit } from '../bindings';
import { accentVar, initial, useFormat } from '../lib/format';
import { displayStatus } from '../lib/status';
import { useSlidingTabs, withViewTransition } from '../lib/motion';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useSharedStore } from '../stores/shared';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ id: string }>();

const instances = useInstancesStore();
const run = useRunStore();
const shared = useSharedStore();
const ui = useUiStore();
const router = useRouter();
const { t } = useI18n();
const { bytes, moment } = useFormat();

type Tab = 'overview' | 'models' | 'workflows' | 'settings';
const TABS: Tab[] = ['overview', 'models', 'workflows', 'settings'];

function tabFromQuery(value: unknown): Tab {
  return TABS.includes(value as Tab) ? (value as Tab) : 'overview';
}

const tab = ref<Tab>(tabFromQuery(useRoute().query.tab));

const tabsBar = ref<HTMLElement | null>(null);
useSlidingTabs(tabsBar, tab);

function selectTab(next: Tab): void {
  if (tab.value === next) return;
  withViewTransition(() => { tab.value = next; });
}

function toggleLog(open: boolean): void {
  if (logOpen.value === open) return;
  withViewTransition(() => { logOpen.value = open; });
}

const logOpen = ref(false);

const logShown = computed(() => logOpen.value && tab.value === 'overview');
const editing = ref(false);
const confirming = ref(false);
const measuring = ref(false);
const draft = ref<InstanceEdit>({
  name: '',
  description: '',
  accent: 'teal',
  preferredPort: 8188,
});

const instance = computed(() => instances.byId(props.id));
const status = computed(() => run.statusOf(props.id));
const state = computed(() =>
  instance.value ? displayStatus(instance.value, status.value) : 'stopped',
);
const lines = computed(() => run.logs[props.id] ?? []);
const profiles = computed(() => run.profiles[props.id] ?? []);
const custom = computed(() => instance.value?.customProfiles ?? []);

const sizeText = computed(() => bytes(instance.value?.sizeBytes));
const sizeMeasuredText = computed(() => {
  const when = moment(instance.value?.sizeMeasuredAt);
  return when ? t('instances.field.sizeMeasuredAt', { when }) : '';
});

const sharedText = computed(() => {
  if (instance.value?.shared?.enabled !== true) return '';
  const mode = instance.value.shared.applyMode;
  return `${shared.root?.path ?? ''} · ${t(`shared.mode.${mode}.short`)}`;
});

const lastRunText = computed(() => {
  const when = moment(instance.value?.lastStartedAt);
  return when
    ? t('instances.field.lastRun', { when })
    : t('instances.field.lastRunNever');
});

onMounted(async () => {
  if (!instances.loaded) await instances.load();

  if (!shared.loaded) await shared.load();
  await run.loadProfiles(props.id);
  await run.loadLog(props.id);
  await measureIfNeeded();
});

watch(instance, () => void measureIfNeeded());

watch(state, (now, before) => {
  if (now === 'starting' && before !== 'starting') logOpen.value = true;
});

watch(tab, () => {
  logOpen.value = false;
});

async function measureIfNeeded(): Promise<void> {
  const it = instance.value;
  if (!it || !it.available || it.sizeBytes !== null || measuring.value) return;
  await measure();
}

async function measure(): Promise<void> {
  measuring.value = true;
  try {
    await instances.measureSize(props.id);
  } finally {
    measuring.value = false;
  }
}

function startEdit(): void {
  const it = instance.value;
  if (!it) return;
  draft.value = {
    name: it.name,
    description: it.description,
    accent: it.accent,
    preferredPort: it.preferredPort,
  };
  editing.value = true;
}

async function save(): Promise<void> {
  const saved = await instances.update(props.id, draft.value);
  if (!saved) return;
  editing.value = false;
  ui.pushOk(t('instances.toast.updated', { name: saved.name }));
}

async function remove(): Promise<void> {
  const name = instance.value?.name ?? '';
  if (!(await instances.remove(props.id))) return;
  ui.pushOk(t('instances.toast.removed', { name }));
  await router.push('/instances');
}

function openFolder(): void {
  if (instance.value) void openPath(instance.value.path);
}
</script>

<template>
  <var class="InstanceView">
    <section v-if="instance" class="screen inst-screen">
      <InstanceHeader>
        <RouterLink class="btn ghost" to="/instances">
          <ArrowLeft class="ico" />
          {{ t('common.back') }}
        </RouterLink>
        <span
          class="chip"
          :style="{ '--instance-accent': accentVar(instance.accent) }"
        >{{ initial(instance.name) }}</span>
        <h1 class="t-lg inst-name">{{ instance.name }}</h1>
        <span class="spacer"></span>

        <StatusPill :status="state" />
        <span v-if="status?.port" class="t-mono">:{{ status.port }}</span>
        <LaunchControls :instance="instance" />
      </InstanceHeader>

      <div v-if="state === 'starting'" class="track indet starting-track"><i></i></div>

      <LaunchNotices :instance="instance" />

      <div v-if="!instance.available" class="notices">
        <div class="banner bad">
          <div>
            <b>{{ t('instances.unavailable.title') }}</b>
            <p class="t-sm">{{ t('instances.unavailable.body', { path: instance.path }) }}</p>
          </div>
        </div>
      </div>

      <nav ref="tabsBar" class="tabs" role="tablist">
        <span class="tabs-pill" aria-hidden="true"></span>
        <button
          v-for="item in TABS"
          :key="item"
          type="button"
          role="tab"
          :aria-selected="tab === item"
          @click="selectTab(item)"
        >
          {{ t(`instances.tab.${item}`) }}
        </button>
      </nav>

      <div v-if="logShown" class="log-area">
        <div class="log-head">
          <span class="t-label">{{ t('run.console') }}</span>
          <span class="hint">{{ t('run.lines', lines.length) }}</span>
          <span class="spacer"></span>
          <button type="button" class="btn secondary" @click="toggleLog(false)">
            {{ t('run.hideLog') }}
          </button>
        </div>
        <div class="log">
          <LogConsole v-if="lines.length" :lines="lines" />
          <EmptyNote v-else>{{ t('run.empty') }}</EmptyNote>
        </div>
      </div>

      <div v-else class="screen-body">
        <div class="screen-pad wide">

          <div v-if="tab === 'overview'" class="cols">

            <div>
              <Field>
                <label>{{ t('instances.field.lastRunTitle') }}</label>
                <KeyValueList>
                  <KeyValueRow>
                    <span class="lbl">{{ lastRunText }}</span>
                    <span class="val">
                      {{ status?.readySecs ? t('run.readyIn', { secs: status.readySecs }) : '' }}
                    </span>
                  </KeyValueRow>

                  <KeyValueRow>
                    <span class="lbl">{{ t('instances.field.port') }}</span>
                    <span class="val">{{ instance.preferredPort }}</span>
                  </KeyValueRow>
                  <KeyValueRow>
                    <span class="lbl">{{ t('instances.field.profiles') }}</span>
                    <span class="val">{{ status?.profileId ?? profiles[0]?.id ?? '—' }}</span>
                  </KeyValueRow>
                </KeyValueList>
              </Field>

              <Field>
                <label>{{ t('run.console') }}</label>
                <div class="row">
                  <button type="button" class="btn secondary" @click="toggleLog(true)">
                    {{ t('run.showLog') }}
                  </button>

                  <span class="hint">
                    {{ t('run.lines', lines.length) }} · {{ t('run.logExpands') }}
                  </span>
                </div>
              </Field>

              <Field v-if="instance.description">
                <label>{{ t('instances.field.description') }}</label>
                <p class="t-sm">{{ instance.description }}</p>
              </Field>
            </div>

            <div>
              <Field>
                <label>{{ t('instances.field.folder') }}</label>
                <div class="path-row">
                  <div class="input mono"><span>{{ instance.path }}</span></div>
                  <button
                    type="button"
                    class="btn secondary"
                    :disabled="!instance.available"
                    @click="openFolder"
                  >
                    {{ t('common.openFolder') }}
                  </button>
                </div>
              </Field>

              <KeyValueList>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.comfyVersion') }}</span>
                  <span class="val">{{ instance.comfyVersion ?? t('common.unknown') }}</span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.pythonVersion') }}</span>
                  <span class="val">{{ instance.pythonVersion ?? t('common.unknown') }}</span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.sizeOnDisk') }}</span>
                  <span class="val">
                    {{ sizeText || (measuring ? t('instances.field.sizeMeasuring') : '—') }}
                  </span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.profiles') }}</span>

                  <span class="val">
                    {{ profiles.length
                    }}{{
                      custom.length
                        ? ` + ${t('instances.field.profilesCustom', custom.length)}`
                        : ''
                    }}
                  </span>
                </KeyValueRow>

                <KeyValueRow>
                  <span class="lbl">{{ t('shared.instance.label') }}</span>
                  <span class="val">
                    <template v-if="sharedText">
                      <PathText :path="sharedText" />
                    </template>
                    <template v-else>—</template>
                  </span>
                </KeyValueRow>
              </KeyValueList>

              <div class="row">
                <span v-if="sizeMeasuredText" class="hint">{{ sizeMeasuredText }}</span>
                <button
                  type="button"
                  class="btn ghost"
                  :disabled="measuring || !instance.available"
                  @click="measure"
                >
                  {{ t('instances.field.recalculate') }}
                </button>
                <RouterLink class="btn ghost" :to="`/instances/${instance.id}/args`">
                  {{ t('args.edit') }}
                </RouterLink>
              </div>

              <div v-if="instance.source" class="src">
                {{
                  t('instances.field.source', {
                    archive: instance.source.archiveLabel,
                    when: moment(instance.source.installedAt),
                  })
                }}
              </div>
            </div>
          </div>

          <template v-else-if="tab === 'models'">
            <SharedPanel :instance="instance" />
            <ModelsPanel :instance="instance" />
          </template>

          <WorkflowPanel v-else-if="tab === 'workflows'" :instance="instance" />

          <template v-else>
            <template v-if="editing">
              <InstanceFields v-model="draft" />
              <div class="row">
                <button
                  type="button"
                  class="btn primary"
                  :disabled="draft.name.trim() === ''"
                  @click="save"
                >
                  {{ t('common.save') }}
                </button>
                <button type="button" class="btn ghost" @click="editing = false">
                  {{ t('common.cancel') }}
                </button>
              </div>
            </template>
            <Group v-else>
              <KeyValueList>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.name') }}</span>
                  <span class="val">{{ instance.name }}</span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.description') }}</span>
                  <span class="val">{{ instance.description || '—' }}</span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('instances.field.port') }}</span>
                  <span class="val">{{ instance.preferredPort }}</span>
                </KeyValueRow>
              </KeyValueList>
              <div class="row">
                <button type="button" class="btn secondary" @click="startEdit">
                  {{ t('common.edit') }}
                </button>
              </div>
            </Group>

            <Group danger>
              <button
                v-if="!confirming"
                type="button"
                class="btn danger"
                @click="confirming = true"
              >
                {{ t('instances.remove.action') }}
              </button>
              <template v-else>
                <p class="t-md">{{ t('instances.remove.title', { name: instance.name }) }}</p>
                <p class="t-sm">{{ t('instances.remove.body') }}</p>
                <div class="row">
                  <button type="button" class="btn danger" @click="remove">
                    {{ t('instances.remove.confirm') }}
                  </button>
                  <button type="button" class="btn ghost" @click="confirming = false">
                    {{ t('common.cancel') }}
                  </button>
                </div>
              </template>
            </Group>
          </template>
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>

.inst-screen {
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.inst-screen > .screen-body,
.log-area {
  flex: 1;
}

.inst-name {
  min-width: 0;
  overflow-wrap: anywhere;
}

.starting-track {
  margin: 0 var(--space-5) var(--space-2);
}

.tabs {
  padding: 0 var(--space-5);
  margin-top: var(--space-2);
}

.notices {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: 0 var(--space-5);
}

.log-area {
  min-height: 0;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: var(--space-2);
  padding: var(--space-3) var(--space-5) var(--space-4);
}

.log-head {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
}
</style>
