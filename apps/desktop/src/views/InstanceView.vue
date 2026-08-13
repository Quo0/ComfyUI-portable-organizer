<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

import InstanceFields from '../components/InstanceFields.vue';
import LaunchPanel from '../components/LaunchPanel.vue';
import ModelsPanel from '../components/ModelsPanel.vue';
import WorkflowPanel from '../components/WorkflowPanel.vue';
import SharedPanel from '../components/SharedPanel.vue';
import StatusPill from '../components/StatusPill.vue';
import type { InstanceEdit } from '../bindings';
import { accentVar, useFormat } from '../lib/format';
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
const { bytes, moment } = useFormat();

const instance = computed(() => instances.byId(props.id));

/** Пустая строка означает «ещё не посчитано»: показывать нечего. */
const sizeText = computed(() => bytes(instance.value?.sizeBytes));

const sizeMeasuredText = computed(() => {
  const when = moment(instance.value?.sizeMeasuredAt);
  return when ? t('instances.field.sizeMeasuredAt', { when }) : '';
});
const editing = ref(false);
const confirming = ref(false);
const measuring = ref(false);
const draft = ref<InstanceEdit>({
  name: '',
  description: '',
  accent: 'teal',
  preferredPort: 8188,
});

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  await measureIfNeeded();
});

// Прямой переход по ссылке отдаёт экран раньше, чем загрузился реестр.
watch(instance, () => void measureIfNeeded());

/**
 * Размер считается сам при первом открытии и кэшируется. Обход 52 ГБ
 * занимает минуты — без кэша экран показывал бы «считаю…» каждый раз.
 */
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
  if (instance.value) void revealItemInDir(instance.value.path);
}
</script>

<template>
  <section v-if="instance" class="screen">
    <header class="screen-head">
      <RouterLink class="btn ghost" to="/instances">{{ t('common.back') }}</RouterLink>
      <span
        class="chip"
        :style="{ '--instance-accent': accentVar(instance.accent) }"
      ></span>
      <h1 class="t-lg">{{ instance.name }}</h1>
      <StatusPill :status="displayStatus(instance, run.statusOf(instance.id))" />
      <span class="head-spacer"></span>
      <button v-if="!editing" type="button" class="btn secondary" @click="startEdit">
        {{ t('common.edit') }}
      </button>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <!-- Инстанс с исчезнувшей папкой не пропадает из списка: иначе
             выглядит так, будто приложение потеряло чужую сборку. -->
        <div v-if="!instance.available" class="group">
          <p class="t-md">{{ t('instances.unavailable.title') }}</p>
          <p class="t-sm">
            {{ t('instances.unavailable.body', { path: instance.path }) }}
          </p>
        </div>

        <LaunchPanel :instance="instance" />

        <SharedPanel :instance="instance" />

        <ModelsPanel :instance="instance" />

        <WorkflowPanel :instance="instance" />

        <div v-if="instance.description" class="card-desc">
          {{ instance.description }}
        </div>

        <div class="field">
          <span class="t-label">{{ t('instances.field.folder') }}</span>
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
        </div>

        <div class="meta">
          <span>
            {{ t('instances.field.comfyVersion') }}:
            {{ instance.comfyVersion ?? t('common.unknown') }}
          </span>
          <span>
            {{ t('instances.field.pythonVersion') }}:
            {{ instance.pythonVersion ?? t('common.unknown') }}
          </span>
          <span>:{{ instance.preferredPort }}</span>
        </div>

        <div class="group">
          <span class="t-label">{{ t('instances.field.sizeOnDisk') }}</span>
          <div class="row">
            <span v-if="sizeText" class="t-mono">{{ sizeText }}</span>
            <span v-else-if="measuring" class="t-sm">
              {{ t('instances.field.sizeMeasuring') }}
            </span>
            <span v-if="sizeMeasuredText" class="hint">{{ sizeMeasuredText }}</span>
            <button
              type="button"
              class="btn ghost"
              :disabled="measuring || !instance.available"
              @click="measure"
            >
              {{ t('instances.field.recalculate') }}
            </button>
          </div>
        </div>

        <div class="group">
          <span class="t-label">{{ t('instances.field.profiles') }}</span>
          <div v-if="instance.profiles.length" class="row">
            <span
              v-for="profile in instance.profiles"
              :key="profile.id"
              class="pill stopped"
              :title="profile.id"
            >
              {{ profile.name }}
              <em v-if="profile.advanced" class="advanced">
                {{ t('instances.field.profilesAdvanced') }}
              </em>
            </span>
          </div>
          <p v-else class="hint">{{ t('instances.field.profilesNone') }}</p>
        </div>

        <div v-if="instance.source" class="src">
          {{
            t('instances.field.source', {
              archive: instance.source.archiveLabel,
              when: moment(instance.source.installedAt),
            })
          }}
        </div>

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

        <!-- Подтверждение раскрывается на месте, а не всплывает диалогом:
             модалок в приложении нет, и объяснение важнее, чем вопрос. -->
        <div class="group danger-zone">
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
        </div>
      </div>
    </div>
  </section>
</template>
