<script setup lang="ts">

//

//

import { ArrowLeft, X } from '@lucide/vue';
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import EmptyNote from '../components/EmptyNote.vue';
import Field from '../components/ui/Field.vue';
import Group from '../components/ui/Group.vue';
import StepBar from '../components/ui/StepBar.vue';
import { commands, type CustomProfile, type LaunchProfile } from '../bindings';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ id: string }>();

const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

const instance = computed(() => instances.byId(props.id));
const all = computed<LaunchProfile[]>(() => run.profiles[props.id] ?? []);

const bases = computed(() => all.value.filter((p) => !p.id.startsWith('custom:')));
const mine = computed(() => instance.value?.customProfiles ?? []);

const draft = ref<CustomProfile>({ id: '', name: '', baseId: '', args: [] });
const preview = ref<string[]>([]);

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  await run.loadProfiles(props.id);
  if (!draft.value.baseId) reset(bases.value[0]?.id ?? '');
});

watch(bases, (list) => {
  if (!draft.value.baseId && list.length) reset(list[0].id);
});

function reset(baseId: string): void {
  const base = all.value.find((p) => p.id === baseId);
  draft.value = {
    id: draft.value.id,
    name: draft.value.name || (base ? `${base.name} +` : ''),
    baseId,
    args: [...(base?.args ?? [])],
  };
  void refreshPreview();
}

function edit(profile: CustomProfile): void {
  draft.value = { ...profile, args: [...profile.args] };
  void refreshPreview();
}

function addArg(): void {
  draft.value.args.push('');
}

function removeArg(index: number): void {
  draft.value.args.splice(index, 1);
  void refreshPreview();
}

async function refreshPreview(): Promise<void> {
  const source = draft.value.baseId;
  if (!source) return;
  const res = await commands.previewCommand(props.id, source);
  if (res.status !== 'ok') {
    preview.value = [];
    return;
  }
  const base = run.profiles[props.id]?.find((p) => p.id === source);
  const added = res.data.slice(1 + (base?.args.length ?? 0));
  preview.value = [
    res.data[0],
    ...draft.value.args.filter((a) => a.trim() !== ''),
    ...added,
  ];
}

watch(
  () => [draft.value.args.join(' '), draft.value.baseId],
  () => void refreshPreview(),
  { deep: true },
);

async function save(): Promise<void> {
  const res = await commands.saveCustomProfile(props.id, {
    ...draft.value,
    args: draft.value.args.filter((a) => a.trim() !== ''),
  });
  if (res.status === 'error') {
    ui.pushError(res.error);
    return;
  }
  instances.replace(res.data);
  await run.loadProfiles(props.id);
  ui.pushOk(t('args.saved', { name: draft.value.name }));
  draft.value = { id: '', name: '', baseId: draft.value.baseId, args: draft.value.args };
  await refreshPreview();
}

async function remove(profile: CustomProfile): Promise<void> {
  const res = await commands.removeCustomProfile(props.id, profile.id);
  if (res.status === 'error') {
    ui.pushError(res.error);
    return;
  }
  instances.replace(res.data);
  await run.loadProfiles(props.id);
  ui.pushOk(t('args.removed', { name: profile.name }));
  if (draft.value.id === profile.id) reset(draft.value.baseId);
}
</script>

<template>
  <var class="ArgsEditorView">
    <section v-if="instance" class="screen">

      <StepBar>
        <h2 class="title">{{ t('args.title') }}</h2>

        <span class="hint">{{ instance.name }}</span>
        <span class="spacer"></span>
        <span class="acts">
          <RouterLink class="btn ghost" :to="`/instances/${instance.id}`">
            <ArrowLeft class="ico" />
            {{ t('common.back') }}
          </RouterLink>
          <button
            type="button"
            class="btn primary lg"
            :disabled="draft.name.trim() === '' || !draft.baseId"
            @click="save"
          >
            {{ t('common.save') }}
          </button>
        </span>
      </StepBar>

      <div class="screen-body">
        <div class="screen-pad">
          <p class="t-sm">{{ t('args.lead') }}</p>

          <Group>
            <span class="t-label">{{ t('args.mine') }}</span>
            <div v-if="mine.length" class="wf-list">
              <div v-for="p in mine" :key="p.id" class="wf-row">
                <span class="nm">{{ p.name }}</span>
                <span class="hint">{{ p.baseId }}</span>
                <button type="button" class="btn ghost" @click="edit(p)">
                  {{ t('common.edit') }}
                </button>
                <button type="button" class="btn ghost" @click="remove(p)">
                  {{ t('instances.remove.action') }}
                </button>
              </div>
            </div>
            <EmptyNote v-else>{{ t('args.none') }}</EmptyNote>
          </Group>

          <div class="two">
            <Field>
              <label for="base">{{ t('args.base') }}</label>

              <select
                id="base"
                class="input"
                :value="draft.baseId"
                @change="reset(($event.target as HTMLSelectElement).value)"
              >
                <option v-for="p in bases" :key="p.id" :value="p.id">{{ p.id }}</option>
              </select>
            </Field>
            <Field>
              <label for="pname">{{ t('args.name') }}</label>
              <input id="pname" v-model="draft.name" class="input" type="text" />
            </Field>
          </div>

          <Group>
            <span class="t-label">{{ t('args.list') }}</span>
            <div v-for="(_, index) in draft.args" :key="index" class="path-row">

              <input v-model="draft.args[index]" class="input mono" type="text" />

              <span class="acts">
                <button
                  type="button"
                  class="act"
                  :title="t('args.remove')"
                  :aria-label="t('args.remove')"
                  @click="removeArg(index)"
                >
                  <X class="ico" />
                </button>
              </span>
            </div>
            <div class="row">
              <button type="button" class="btn secondary" @click="addArg">
                {{ t('args.add') }}
              </button>
              <button type="button" class="btn ghost" @click="reset(draft.baseId)">
                {{ t('args.reset') }}
              </button>
            </div>
          </Group>

          <Group>
            <span class="t-label">{{ t('args.preview') }}</span>

            <pre class="console preview">{{ preview.join(' ') }}</pre>
            <p class="hint">{{ t('args.previewHint') }}</p>
          </Group>
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>

.ArgsEditorView :deep(.step-bar) {
  padding-top: var(--space-4);
}
</style>
