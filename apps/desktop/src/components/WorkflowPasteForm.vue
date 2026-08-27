<script setup lang="ts">

//

//

import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import type { AppError } from '../bindings';
import { errorText } from '../lib/errors';
import Field from './ui/Field.vue';

const props = defineProps<{
  save: (name: string, content: string) => Promise<AppError | null>;
}>();

const emit = defineEmits<{ done: [name: string]; cancel: [] }>();

const { t } = useI18n();

const name = ref('');
const content = ref('');
const saving = ref(false);
const failure = ref<AppError | null>(null);
const area = ref<HTMLTextAreaElement | null>(null);
const nameInput = ref<HTMLInputElement | null>(null);

const graph = computed<{ ok: boolean; nodes: number; types: string[] } | null>(() => {
  const text = content.value.trim();
  if (!text) return null;
  try {
    const value: unknown = JSON.parse(text);
    const nodes = (value as { nodes?: unknown }).nodes;
    if (!Array.isArray(nodes)) return { ok: false, nodes: 0, types: [] };
    const types = [
      ...new Set(
        nodes
          .map((node: unknown) => (node as { type?: unknown }).type)
          .filter((kind): kind is string => typeof kind === 'string'),
      ),
    ];
    return { ok: true, nodes: nodes.length, types };
  } catch {
    return { ok: false, nodes: 0, types: [] };
  }
});

function looksLikeWorkflow(text: string): boolean {
  try {
    return Array.isArray((JSON.parse(text) as { nodes?: unknown }).nodes);
  } catch {
    return false;
  }
}

const fileName = computed(() => {
  const stem = name.value.trim().replace(/\.json$/i, '').trimEnd();
  return stem ? `${stem}.json` : '';
});

const canSave = computed(() => !saving.value && !!fileName.value && graph.value?.ok === true);

onMounted(async () => {
  area.value?.focus();
  try {
    const text = await navigator.clipboard.readText();
    if (text.trim() && looksLikeWorkflow(text)) content.value = text;
  } catch {
  }
});

async function submit(): Promise<void> {
  if (!canSave.value) return;
  saving.value = true;
  failure.value = null;
  try {
    const error = await props.save(name.value, content.value.trim());
    if (error) {
      failure.value = error;

      if (nameFailure.value) shakeName();
      return;
    }
    emit('done', fileName.value);
  } finally {
    saving.value = false;
  }
}

const nameFailure = computed(() =>
  failure.value?.code === 'workflows.nameTaken' || failure.value?.code === 'workflows.badName'
    ? errorText(failure.value)
    : null,
);

const otherFailure = computed(() =>
  failure.value && !nameFailure.value ? errorText(failure.value) : null,
);

function shakeName(): void {
  const el = nameInput.value;
  if (!el) return;
  el.classList.remove('is-shaking');
  void el.offsetWidth;
  el.classList.add('is-shaking');
}
</script>

<template>
  <var class="WorkflowPasteForm">
    <form class="paste" @submit.prevent="submit">
      <Field>
        <label class="t-label" for="paste-name">{{ t('library.paste.name') }}</label>
        <input
          id="paste-name"
          ref="nameInput"
          v-model="name"
          class="input"
          :class="{ bad: !!nameFailure }"
          type="text"
          autocomplete="off"
          :placeholder="t('library.paste.namePlaceholder')"
        />

        <p v-if="nameFailure" class="hint bad">{{ nameFailure }}</p>
        <p v-else-if="fileName" class="hint">
          {{ t('library.paste.willSaveAs', { name: fileName }) }}
        </p>
      </Field>

      <Field>
        <label class="t-label" for="paste-json">{{ t('library.paste.json') }}</label>
        <textarea
          id="paste-json"
          ref="area"
          v-model="content"
          class="input area mono"
          :class="{ bad: graph?.ok === false }"
          spellcheck="false"
          :placeholder="t('library.paste.jsonPlaceholder')"
        ></textarea>

        <p v-if="graph?.ok" class="hint">
          {{ t('library.paste.parsed', graph.nodes) }}
          <template v-if="graph.types.length">
            · {{ t('library.paste.types', graph.types.length) }}
          </template>
        </p>
        <p v-else-if="graph" class="hint bad">{{ t('library.paste.notAWorkflow') }}</p>
        <p v-else class="hint">{{ t('library.paste.hint') }}</p>
      </Field>

      <p v-if="otherFailure" class="hint bad">{{ otherFailure }}</p>

      <div class="row">
        <button type="submit" class="btn primary" :disabled="!canSave">
          {{ t('library.paste.save') }}
        </button>
        <button type="button" class="btn ghost" @click="emit('cancel')">
          {{ t('common.cancel') }}
        </button>
      </div>
    </form>
  </var>
</template>
