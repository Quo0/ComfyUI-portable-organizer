<script setup lang="ts">

//

//

import { Check, ExternalLink, ListChecks, Plus, RotateCw } from '@lucide/vue';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { useI18n } from 'vue-i18n';

import type { LibItem } from '../../stores/workflows';
import CheckGlyph from '../../components/CheckGlyph.vue';
import EmptyNote from '../../components/EmptyNote.vue';
import MenuButton from '../../components/MenuButton.vue';
import PathPicker from '../../components/PathPicker.vue';
import Field from '../../components/ui/Field.vue';
import Group from '../../components/ui/Group.vue';
import Pane from '../../components/ui/Pane.vue';
import ScreenHeader from '../../components/ui/ScreenHeader.vue';
import WorkflowPasteForm from '../../components/WorkflowPasteForm.vue';
import { commands } from '../../bindings';
import { accentVar } from '../../lib/format';
import { errorText } from '../../lib/errors';
import { useSlidingTabs, withViewTransition } from '../../lib/motion';
import { useRunStore } from '../../stores/run';
import { useInstancesStore } from '../../stores/instances';
import { useUiStore } from '../../stores/ui';
import { useWorkflowsStore } from '../../stores/workflows';

const library = useWorkflowsStore();
const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

const suggested = ref<string | null>(null);

const noteDraft = ref('');
const tagsDraft = ref('');
const editing = ref(false);

type Side = 'where' | 'note' | 'tags';
const SIDES: Side[] = ['where', 'note', 'tags'];
const side = ref<Side>('where');

const sideTabsBar = ref<HTMLElement | null>(null);
useSlidingTabs(sideTabsBar, side);

function selectSide(next: Side): void {
  if (side.value === next) return;
  withViewTransition(() => { side.value = next; });
}

function toggleMulti(): void {
  withViewTransition(() => { library.setMulti(!library.multi); });
}

onMounted(async () => {
  if (!library.loaded) await library.load();
  if (!instances.loaded) await instances.load();
  const res = await commands.suggestLibraryPath();
  if (res.status === 'ok') suggested.value = res.data;
});

const targets = computed(() => instances.items.filter((i) => i.available));

function instanceName(id: string): string {
  return instances.items.find((i) => i.id === id)?.name ?? id;
}

function onRow(item: LibItem): void {
  if (library.multi) return;
  if (library.selected === item.path) return;

  withViewTransition(() => { void library.select(item.path); });
}

const allTargets = computed(
  () => targets.value.length > 0 && targets.value.every((i) => library.markedTargets.has(i.id)),
);

const someTargets = computed(
  () => library.markedTargets.size > 0 && !allTargets.value,
);

function toggleAllTargets(): void {
  library.setTargets(allTargets.value ? [] : targets.value.map((i) => i.id));
}

function startEdit(item: LibItem): void {
  noteDraft.value = item.meta.note;
  tagsDraft.value = item.meta.tags.join(', ');
  withViewTransition(() => { editing.value = true; });
}

function cancelEdit(): void {
  withViewTransition(() => { editing.value = false; });
}

async function saveMeta(item: LibItem): Promise<void> {
  await library.setMeta(item.path, {
    ...item.meta,
    note: noteDraft.value,
    tags: tagsDraft.value
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean),
  });
  editing.value = false;
}

const pushing = ref<string | null>(null);

const pushed = ref<Set<string>>(new Set());

watch(() => library.selected, () => (pushed.value = new Set()));

function added(instanceId: string): boolean {
  return library.compatOf(instanceId)?.present === true || pushed.value.has(instanceId);
}

async function push(instanceId: string, overwrite = false): Promise<void> {
  const item = library.current;
  if (!item) return;
  pushing.value = instanceId;
  try {
    const res = await commands.pushWorkflow(
      instanceId,
      library.path,
      item.path,
      overwrite,
    );
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    if (res.data === 'conflict') {
      if (window.confirm(t('library.push.replace', { name: item.path }))) {
        await push(instanceId, true);
      }
      return;
    }

    pushed.value = new Set(pushed.value).add(instanceId);
    const running = run.statusOf(instanceId)?.state === 'running';
    ui.pushOk(running ? t('library.push.doneRunning') : t('library.push.done'));
  } finally {
    pushing.value = null;
  }
}

const ACCEPTED = ['.json', '.png'];

async function addFile(): Promise<void> {
  const picked = await open({
    multiple: false,

    filters: [{ name: 'JSON, PNG', extensions: ['json', 'png'] }],
  });
  if (typeof picked !== 'string') return;
  if (await library.addFile(picked)) ui.pushOk(t('library.add.done'));
}

const pasting = ref(false);

function onPasted(name: string): void {
  pasting.value = false;
  ui.pushOk(t('library.paste.done', { name }));
}

const dragging = ref(false);

onMounted(async () => {
  unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
    if (event.payload.type === 'over') {
      dragging.value = library.configured && library.available;
      return;
    }
    if (event.payload.type === 'leave') {
      dragging.value = false;
      return;
    }
    dragging.value = false;
    if (!library.configured || !library.available) return;

    let added = 0;
    for (const file of event.payload.paths) {
      const lower = file.toLowerCase();
      if (!ACCEPTED.some((ext) => lower.endsWith(ext))) continue;
      if (await library.addFile(file)) added += 1;
    }
    if (added > 0) ui.pushOk(t('library.add.dropped', added));
  });
});

let unlisten: (() => void) | null = null;
onUnmounted(() => unlisten?.());
</script>

<template>
  <var class="WorkflowLibraryView">
    <section class="screen">

      <ScreenHeader>
        <h1 class="t-lg">{{ t('library.title') }}</h1>
        <span class="spacer"></span>
        <template v-if="library.configured">
          <PathPicker class="grow" :path="library.path" @pick="library.setPath($event)" />
          <span v-if="dragging" class="tag">{{ t('library.add.drop') }}</span>

          <MenuButton v-if="library.available" class="spacer" :icon="Plus" :label="t('library.add.action')">
            <button type="button" role="menuitem" @click="addFile">
              {{ t('library.add.file') }}
            </button>
            <button v-if="!pasting" type="button" role="menuitem" @click="pasting = true">
              {{ t('library.paste.action') }}
            </button>
          </MenuButton>
        </template>
      </ScreenHeader>

      <div v-if="!library.configured" class="screen-body">
        <div class="screen-pad">
          <div class="empty">
            <p>{{ t('library.path.howto') }}</p>

            <div v-if="library.scanning" class="bar indet"><i></i></div>

            <Field>
              <label class="t-label" for="library-path">{{ t('library.path.label') }}</label>
              <PathPicker id="library-path" :path="library.path" :empty="t('library.path.empty')"
                @pick="library.setPath($event)" />

              <p v-if="suggested" class="hint">{{ t('library.path.suggested') }}</p>
            </Field>

            <button v-if="suggested" class="btn primary" type="button" @click="library.setPath(suggested)">
              {{ t('library.path.useSuggested') }}
            </button>

            <p class="hint">{{ t('library.path.portable') }}</p>
          </div>
        </div>
      </div>

      <div v-else-if="!library.available" class="screen-body">
        <div class="screen-pad">

          <div class="empty">
            <p>{{ t('library.path.unavailable') }}</p>
            <button type="button" class="btn secondary" @click="library.rescan()">
              <RotateCw class="ico" />
              {{ t('library.refresh') }}
            </button>
          </div>
        </div>
      </div>

      <div v-else-if="pasting" class="screen-body">
        <div class="screen-pad">
          <WorkflowPasteForm :save="library.addText" @done="onPasted" @cancel="pasting = false" />
        </div>
      </div>

      <div v-else class="screen-body">
        <div class="split-master">
          <Pane>
            <div class="pane-head">

              <button type="button" class="btn ghost icon" :aria-pressed="library.multi"
                :title="t('library.multi.action')" :aria-label="t('library.multi.action')" @click="toggleMulti()">
                <ListChecks class="ico" />
              </button>

              <input v-model="library.query" class="input search" type="search" :placeholder="t('library.search')" />

              <button type="button" class="btn ghost icon" :aria-pressed="library.favoritesOnly"
                :title="t('library.favoritesOnly')" :aria-label="t('library.favoritesOnly')"
                @click="library.favoritesOnly = !library.favoritesOnly">
                <span class="star" :class="{ off: !library.favoritesOnly }">★</span>
              </button>
            </div>

            <div class="scroll">
              <div class="scroll-pad">

                <p v-if="library.scan?.manifestBroken" class="hint bad">
                  {{ t('library.manifestBroken') }}
                </p>

                <TransitionGroup v-if="library.visible.length" name="wf-row" tag="div" class="wf-list"
                  :class="{ picking: library.multi }">

                  <component :is="library.multi ? 'div' : 'button'" v-for="item in library.visible" :key="item.path"
                    :type="library.multi ? undefined : 'button'" class="wf-row" :class="{
                      on: library.multi
                        ? library.marked.has(item.path)
                        : item.path === library.selected,
                      lost: item.lost,
                    }" @click="onRow(item)">

                    <button v-if="library.multi" type="button" class="check" role="checkbox"
                      :class="{ on: library.marked.has(item.path) }" :aria-checked="library.marked.has(item.path)"
                      :aria-label="item.name" @click="library.toggleMark(item.path)">
                      <CheckGlyph />
                    </button>
                    <span class="star" :class="{ off: !item.meta.favorite }"
                      :title="item.meta.favorite ? t('library.unstar') : t('library.star')"
                      @click.stop="library.toggleFavorite(item)">★</span>
                    <span class="nm">{{ item.name }}</span>

                    <span class="tags">
                      <span v-for="tag in item.meta.tags" :key="tag" class="tag">{{ tag }}</span>
                      <span v-if="item.lost" class="tag stop">{{ t('library.lost') }}</span>
                      <span v-else-if="item.broken" class="tag warn">{{ t('library.broken') }}</span>
                    </span>
                  </component>
                </TransitionGroup>
                <EmptyNote v-else>{{ t('library.nothingFound') }}</EmptyNote>
              </div>
            </div>

            <div class="pane-foot">
              <span class="t-label">{{ t('library.summary', library.items.length) }}</span>
            </div>
          </Pane>

          <Pane v-if="library.multi">

            <div class="pane-head">
              <span class="title">{{ t('library.multi.title') }}</span>
              <button v-if="!library.bulk" type="button" class="btn primary"
                :disabled="!library.marked.size || !library.markedTargets.size"
                @click="library.pushMany([...library.markedTargets])">
                {{ t('library.multi.push') }}
              </button>
            </div>

            <div class="scroll">
              <div class="scroll-pad">

                <Group v-if="library.bulk">

                  <p class="t-sm">
                    {{ t('library.bulk.progress', {
                      done: library.bulk.ok.length,
                      total: library.bulk.total,
                    }) }}
                  </p>
                  <div class="bar">
                    <i :style="{ width: `${(library.bulk.done / library.bulk.total) * 100}%` }"></i>
                  </div>

                  <template v-if="library.bulk.failed.length">
                    <p class="hint bad">
                      {{ t('library.bulk.failed', library.bulk.failed.length) }}
                    </p>
                    <div class="fails">
                      <div v-for="fail in library.bulk.failed" :key="`${fail.workflow}/${fail.instanceId}`"
                        class="fail">
                        <span class="fail-pair">
                          {{ fail.workflow }} → {{ instanceName(fail.instanceId) }}
                        </span>
                        <span class="fail-why">
                          {{ fail.error ? errorText(fail.error) : t('library.bulk.nameTaken') }}
                        </span>
                      </div>
                    </div>

                    <p v-if="library.bulk.failed.some((f) => !f.error)" class="hint">
                      {{ t('library.bulk.replaceHint') }}
                    </p>
                  </template>

                  <p v-else-if="!library.bulk.running" class="hint">
                    {{ library.bulk.done === library.bulk.total
                      ? t('library.bulk.done')
                      : t('library.bulk.stopped') }}
                  </p>

                  <div class="row">
                    <button v-if="library.bulk.running" type="button" class="btn danger" @click="library.cancel()">
                      {{ t('common.cancel') }}
                    </button>
                    <button v-else type="button" class="btn ghost" @click="library.clearBulk()">
                      {{ t('common.close') }}
                    </button>
                  </div>
                </Group>

                <template v-else>
                  <div v-if="targets.length" class="pick-list">
                    <button type="button" class="pick-head" @click="toggleAllTargets">
                      <span class="check" :class="{ on: allTargets, mixed: someTargets }">
                        <CheckGlyph />
                      </span>
                      <span>{{ t('library.pick.all') }}</span>
                    </button>
                    <button v-for="instance in targets" :key="instance.id" type="button" class="pick-row"
                      @click="library.toggleTarget(instance.id)">
                      <span class="check" :class="{ on: library.markedTargets.has(instance.id) }">
                        <CheckGlyph />
                      </span>
                      <span class="chip" :style="{ '--instance-accent': accentVar(instance.accent) }"></span>
                      <span class="nm">{{ instance.name }}</span>
                    </button>
                  </div>
                  <EmptyNote v-else>{{ t('library.noInstances') }}</EmptyNote>
                </template>
              </div>
            </div>

            <div class="pane-foot">
              <span class="t-label">{{ t('library.bulk.title', library.marked.size) }}</span>
            </div>
          </Pane>

          <Pane v-else>
            <div v-if="library.current" class="pane-head">
              <span class="title">{{ library.current.name }}</span>

              <button type="button" class="star lg" :class="{ off: !library.current.meta.favorite }"
                :aria-pressed="library.current.meta.favorite"
                :title="library.current.meta.favorite ? t('library.unstar') : t('library.star')"
                @click="library.toggleFavorite(library.current)">★</button>
            </div>

            <nav v-if="library.current" ref="sideTabsBar" class="tabs" role="tablist">
              <span class="tabs-pill" aria-hidden="true"></span>
              <button v-for="item in SIDES" :key="item" type="button" role="tab" :aria-selected="side === item"
                @click="selectSide(item)">
                {{ t(`library.side.${item}`) }}
                <span v-if="item === 'tags' && library.current.meta.tags.length" class="n">
                  {{ library.current.meta.tags.length }}
                </span>
              </button>
            </nav>

            <div class="scroll">
              <div class="scroll-pad">
                <EmptyNote v-if="!library.current">{{ t('library.pickOne') }}</EmptyNote>

                <template v-else>

                  <Group v-if="library.current.lost">
                    <p class="t-md">{{ t('library.lostTitle') }}</p>
                    <p class="t-sm">{{ t('library.lostBody', { path: library.current.path }) }}</p>
                    <button type="button" class="btn danger" @click="library.forget(library.current.path)">
                      {{ t('library.forget') }}
                    </button>
                  </Group>

                  <p v-else-if="library.current.broken" class="hint bad">
                    {{ t('library.brokenBody') }}
                  </p>

                  <template v-else>
                    <Group v-if="side === 'where'">
                      <div v-if="targets.length" class="compat">
                        <template v-for="instance in targets" :key="instance.id">
                          <div class="compat-row" :class="{
                            ok: library.compatOf(instance.id)?.missing.length === 0
                              && library.compatOf(instance.id)?.source !== 'unknown',
                            warn: (library.compatOf(instance.id)?.missing.length ?? 0) > 0,
                          }">
                            <span class="chip" :style="{ '--instance-accent': accentVar(instance.accent) }"></span>
                            <span class="nm">{{ instance.name }}</span>

                            <RouterLink class="act open" :to="`/instances/${instance.id}?tab=workflows`"
                              :title="t('library.openInInstance', { name: instance.name })"
                              :aria-label="t('library.openInInstance', { name: instance.name })">
                              <ExternalLink class="ico" />
                            </RouterLink>

                            <span class="compat-note">
                              {{
                                library.compatOf(instance.id)?.source === 'unknown'
                                  ? t('library.compatUnknown')
                                  : (library.compatOf(instance.id)?.missing.length ?? 0) > 0
                                    ? t('library.compatMissing', library.compatOf(instance.id)!.missing.length)
                                    : library.compatOf(instance.id)?.source === 'cached'
                                      ? t('library.compatCached')
                                      : t('library.compatOk')
                              }}
                            </span>

                            <button type="button" class="act" :class="{ on: added(instance.id) }"
                              :disabled="pushing === instance.id" :title="added(instance.id)
                                ? t('library.push.again')
                                : t('library.push.action')" @click="push(instance.id)">
                              <span class="icon-swap" :data-state="added(instance.id) ? 'b' : 'a'">
                                <Plus class="ico" data-icon="a" />
                                <Check class="ico" data-icon="b" />
                              </span>
                            </button>
                          </div>
                          <div v-if="(library.compatOf(instance.id)?.missing.length ?? 0) > 0" class="missing">
                            {{ library.compatOf(instance.id)!.missing.join(' · ') }}
                          </div>
                        </template>
                      </div>
                      <EmptyNote v-else>{{ t('library.noInstances') }}</EmptyNote>
                    </Group>

                    <Group v-else-if="side === 'note'">
                      <template v-if="editing">
                        <textarea v-model="noteDraft" class="input area" rows="6"></textarea>
                        <div class="row">
                          <button type="button" class="btn primary" @click="saveMeta(library.current)">
                            {{ t('common.save') }}
                          </button>
                          <button type="button" class="btn ghost" @click="cancelEdit()">
                            {{ t('common.cancel') }}
                          </button>
                        </div>
                      </template>
                      <template v-else>

                        <p v-if="library.current.meta.note" class="t-sm">
                          {{ library.current.meta.note }}
                        </p>
                        <EmptyNote v-else>{{ t('library.noNote') }}</EmptyNote>
                        <button type="button" class="btn ghost" @click="startEdit(library.current)">
                          {{ t('common.edit') }}
                        </button>
                      </template>
                    </Group>

                    <Group v-else>
                      <template v-if="editing">
                        <input v-model="tagsDraft" class="input" :placeholder="t('library.tagsPlaceholder')" />
                        <div class="row">
                          <button type="button" class="btn primary" @click="saveMeta(library.current)">
                            {{ t('common.save') }}
                          </button>
                          <button type="button" class="btn ghost" @click="cancelEdit()">
                            {{ t('common.cancel') }}
                          </button>
                        </div>
                      </template>
                      <template v-else>
                        <div v-if="library.current.meta.tags.length" class="row">
                          <span v-for="tag in library.current.meta.tags" :key="tag" class="tag">{{ tag }}</span>
                        </div>
                        <EmptyNote v-else>{{ t('library.noTags') }}</EmptyNote>
                        <button type="button" class="btn ghost" @click="startEdit(library.current)">
                          {{ t('common.edit') }}
                        </button>
                      </template>
                    </Group>
                  </template>
                </template>
              </div>
            </div>
          </Pane>
        </div>
      </div>
    </section>
  </var>
</template>
