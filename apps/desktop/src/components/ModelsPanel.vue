<script setup lang="ts">

//

//

import { ChevronRight, RotateCw } from '@lucide/vue';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import {
  commands,
  events,
  type Instance,
  type MigrateOutcome,
  type MigrateProgress,
  type ModelCategory,
  type ModelEntry,
  type ModelsScan,
} from '../bindings';
import EmptyNote from './EmptyNote.vue';
import OpenFolderButton from './OpenFolderButton.vue';
import Group from './ui/Group.vue';
import Toggle from './ui/Toggle.vue';
import { displayStatus } from '../lib/status';
import { useFormat } from '../lib/format';
import { useToggleTouch } from '../lib/motion';
import { useRunStore } from '../stores/run';
import { useSharedStore } from '../stores/shared';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ instance: Instance }>();

const shared = useSharedStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();
const { bytes } = useFormat();

const scan = ref<ModelsScan | null>(null);
const loading = ref(false);
const progress = ref<MigrateProgress | null>(null);
const outcome = ref<MigrateOutcome | null>(null);
const busy = ref(false);
const cleaning = ref(false);

function keyOf(folder: string, name: string): string {
  return `${folder}/${name}`;
}

function selectable(entry: ModelEntry): boolean {
  return entry.sameName !== 'different';
}

const chosen = ref<Set<string>>(new Set());

const collapsed = ref<Set<string>>(new Set());

const { isTouched, touch } = useToggleTouch();

let unlisten: (() => void) | null = null;

const running = computed(() =>
  ['starting', 'running', 'stopping', 'detached'].includes(
    displayStatus(props.instance, run.statusOf(props.instance.id)),
  ),
);

onMounted(async () => {
  if (!shared.loaded) await shared.load();
  unlisten = await events.migrateProgress.listen((e) => (progress.value = e.payload));
  await refresh();
});
onUnmounted(() => unlisten?.());

watch(running, () => void refresh());

async function refresh(): Promise<void> {
  if (!shared.configured) return;
  loading.value = true;
  try {
    const res = await commands.scanInstanceModels(props.instance.id);
    if (res.status === 'error') {
      if (res.error.code !== 'shared.noRoots') ui.pushError(res.error);
      return;
    }
    scan.value = res.data;

    chosen.value = new Set(
      res.data.categories.flatMap((category) =>
        category.entries
          .filter(selectable)
          .map((entry) => keyOf(category.folder, entry.name)),
      ),
    );
  } finally {
    loading.value = false;
  }
}

function toggle(key: string): void {
  touch(key);
  const next = new Set(chosen.value);
  if (!next.delete(key)) next.add(key);
  chosen.value = next;
}

function keysOf(category: ModelCategory): string[] {
  return category.entries.filter(selectable).map((entry) => keyOf(category.folder, entry.name));
}

function catState(category: ModelCategory): 'on' | 'off' | 'mixed' {
  const keys = keysOf(category);
  const on = keys.filter((key) => chosen.value.has(key)).length;
  if (on === 0) return 'off';
  return on === keys.length ? 'on' : 'mixed';
}

function toggleCategory(category: ModelCategory): void {
  touch(category.folder);
  const keys = keysOf(category);
  const next = new Set(chosen.value);

  if (catState(category) === 'on') keys.forEach((key) => next.delete(key));
  else keys.forEach((key) => next.add(key));
  chosen.value = next;
}

function toggleCollapse(folder: string): void {
  const next = new Set(collapsed.value);
  if (!next.delete(folder)) next.add(folder);
  collapsed.value = next;
}

type Chosen = { category: string; name: string; entry: ModelEntry };

const chosenEntries = computed<Chosen[]>(() =>
  (scan.value?.categories ?? []).flatMap((category) =>
    category.entries
      .filter((entry) => chosen.value.has(keyOf(category.folder, entry.name)))
      .map((entry) => ({ category: category.folder, name: entry.name, entry })),
  ),
);

const moving = computed(() => chosenEntries.value.filter((item) => item.entry.sameName === null));

const plan = computed(() => ({
  files: moving.value.reduce((sum, item) => sum + item.entry.files, 0),

  size: moving.value.reduce((sum, item) => sum + (item.entry.sizeBytes ?? 0), 0),
}));

async function migrate(): Promise<void> {
  busy.value = true;
  outcome.value = null;
  progress.value = null;
  try {
    const res = await commands.migrateModels(
      props.instance.id,
      moving.value.map((item) => [item.category, item.name] as [string, string]),
    );
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    outcome.value = res.data;
    await refresh();
    await shared.rescan();
  } finally {
    busy.value = false;
    progress.value = null;
  }
}

const occupied = computed(() =>
  (scan.value?.categories ?? []).flatMap((category) =>
    category.entries
      .filter((entry) => entry.sameName !== null)
      .map((entry) => ({ category: category.folder, entry })),
  ),
);

const duplicates = computed(() =>
  occupied.value.filter((item) => item.entry.sameName !== 'different'),
);

const different = computed(() =>
  occupied.value.filter((item) => item.entry.sameName === 'different'),
);

function dupCount(folder: string): number {
  return duplicates.value.filter((item) => item.category === folder).length;
}

const picked = computed(() =>
  chosenEntries.value.filter(
    (item) => item.entry.sameName !== null && item.entry.sameName !== 'different',
  ),
);

const freeable = computed(() =>
  picked.value.reduce((sum, item) => sum + (item.entry.sizeBytes ?? 0), 0),
);

async function cleanup(): Promise<void> {
  cleaning.value = true;
  try {
    const res = await commands.removeDuplicateModels(
      props.instance.id,
      picked.value.map((item) => [item.category, item.name] as [string, string]),
    );
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    ui.pushOk(t('migrate.cleanup.done', { size: bytes(res.data.freedBytes) }));
    outcome.value = null;
    await refresh();
  } finally {
    cleaning.value = false;
  }
}
</script>

<template>
  <var class="ModelsPanel">
    <Group>
      <div class="row">
        <span class="t-label">{{ t('migrate.title') }}</span>

        <OpenFolderButton
          :path="scan?.path"
          :title="scan?.path"
          :disabled="!scan?.available"
        />
        <span class="spacer"></span>
        <button type="button" class="btn ghost" :disabled="loading || busy" @click="refresh">
          <RotateCw class="ico" />
          {{ t('library.refresh') }}
        </button>
      </div>

      <p v-if="!shared.configured" class="hint">
        {{ t('migrate.noRoot') }}
        <RouterLink to="/settings/shared-models">{{ t('shared.instance.setUp') }}</RouterLink>
      </p>

      <template v-else>
        <div v-if="loading" class="bar indet"><i></i></div>

        <template v-else-if="scan?.categories.length">

          <div class="cats">
            <template v-for="category in scan.categories" :key="category.folder">
              <div class="cat" :class="{ marked: dupCount(category.folder) > 0 }">
                <button
                  type="button"
                  class="disclose"
                  :aria-expanded="!collapsed.has(category.folder)"
                  @click="toggleCollapse(category.folder)"
                >
                  <ChevronRight class="ico" />
                  <code>{{ category.folder }}</code>
                </button>
                <span class="n">
                  {{ t('migrate.entries', category.entries.length) }} ·
                  {{ bytes(category.sizeBytes) }}
                </span>

                <span v-if="dupCount(category.folder) > 0" class="tag warn">
                  {{ t('migrate.dup.inCategory', dupCount(category.folder)) }}
                </span>

                <Toggle
                  role="checkbox"
                  :checked="catState(category) !== 'off'"
                  :mixed="catState(category) === 'mixed'"
                  :touched="isTouched(category.folder)"
                  :aria-label="category.folder"
                  :disabled="busy || cleaning"
                  @click="toggleCategory(category)"
                />
              </div>

              <template v-if="!collapsed.has(category.folder)">
                <div
                  v-for="entry in category.entries"
                  :key="entry.name"
                  class="cat model"
                  :class="{ marked: entry.sameName !== null }"
                >
                  <code>{{ entry.name }}</code>
                  <span class="n">{{ bytes(entry.sizeBytes) }}</span>

                  <span
                    v-if="entry.sameName"
                    class="tag"
                    :class="{
                      warn: entry.sameName === 'likelyDuplicate',
                      stop: entry.sameName === 'different',
                    }"
                    :title="t(`migrate.verdict.why.${entry.sameName}`)"
                  >
                    {{ t(`migrate.verdict.${entry.sameName}`) }}
                  </span>

                  <Toggle
                    v-if="selectable(entry)"
                    :checked="chosen.has(keyOf(category.folder, entry.name))"
                    :touched="isTouched(keyOf(category.folder, entry.name))"
                    :aria-label="keyOf(category.folder, entry.name)"
                    :disabled="busy || cleaning"
                    @click="toggle(keyOf(category.folder, entry.name))"
                  />
                  <span v-else class="no-toggle"></span>
                </div>
              </template>
            </template>
          </div>

          <p v-if="different.length" class="hint">
            {{ t('migrate.diff.line', { n: different.length }) }}
          </p>

          <p v-if="running" class="hint bad">{{ t('migrate.mustStop') }}</p>

          <p v-if="duplicates.length && !instance.shared?.enabled" class="hint bad">
            {{ t('migrate.dup.needsConnection') }}
          </p>

          <div class="act-grid">
            <template v-if="duplicates.length">
              <button
                type="button"
                class="btn danger"
                :disabled="cleaning || running || !picked.length || !instance.shared?.enabled"
                @click="cleanup"
              >
                {{ t('migrate.dup.remove') }}
              </button>
              <p class="hint">
                {{ t('migrate.dup.line', { n: duplicates.length, size: bytes(freeable) }) }}
              </p>
            </template>

            <template v-if="!progress">
              <button
                type="button"
                class="btn primary"
                :disabled="busy || running || plan.files === 0"
                @click="migrate"
              >
                {{ t('migrate.action') }}
              </button>
              <p class="hint">
                {{ t('migrate.summary', { files: plan.files, size: bytes(plan.size) }) }}
              </p>
            </template>
          </div>

          <Group v-if="progress">
            <p class="t-sm">{{ progress.category }}/{{ progress.name }}</p>
            <div class="bar">
              <i :style="{ width: `${(progress.done / progress.total) * 100}%` }"></i>
            </div>
            <button type="button" class="btn danger" @click="commands.cancelMigrate()">
              {{ t('common.cancel') }}
            </button>
          </Group>
        </template>

        <EmptyNote v-else>{{ t('migrate.empty') }}</EmptyNote>

        <template v-if="outcome">
          <p class="hint">
            {{ t('migrate.moved', { n: outcome.moved.length, size: bytes(outcome.movedBytes) }) }}
          </p>
          <p v-if="outcome.failed.length" class="hint bad">
            {{ t('migrate.failed', outcome.failed.length) }}:
            {{ outcome.failed.map((f) => `${f.category}/${f.name}`).join(', ') }}
          </p>
        </template>
      </template>
    </Group>
  </var>
</template>
