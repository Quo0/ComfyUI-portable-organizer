<script setup lang="ts">

import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import type { InstanceEdit } from '../bindings';
import { accentVar, isCustomAccent } from '../lib/format';
import Field from './ui/Field.vue';

const model = defineModel<InstanceEdit>({ required: true });

const { idPrefix = 'instance', showProblems = true } = defineProps<{
  idPrefix?: string;

  showProblems?: boolean;
}>();

const { t } = useI18n();

const accents = computed(() => [
  { value: 'teal' as const, label: t('accent.teal') },
  { value: 'indigo' as const, label: t('accent.indigo') },
  { value: 'ember' as const, label: t('accent.ember') },
  { value: 'moss' as const, label: t('accent.moss') },
  { value: 'azure' as const, label: t('accent.azure') },
  { value: 'orchid' as const, label: t('accent.orchid') },
  { value: 'rose' as const, label: t('accent.rose') },
  { value: 'amber' as const, label: t('accent.amber') },
]);

const nameEmpty = () => showProblems && model.value.name.trim() === '';
</script>

<template>
  <var class="InstanceFields">
    <Field>
      <label :for="`${idPrefix}-name`">{{ t('instances.field.name') }}</label>
      <input
        :id="`${idPrefix}-name`"
        v-model="model.name"
        class="input"
        :class="{ bad: nameEmpty() }"
        type="text"
        maxlength="80"
      />
      <p v-if="nameEmpty()" class="hint bad">{{ t('errors.instances.emptyName') }}</p>
    </Field>

    <Field>
      <label :for="`${idPrefix}-desc`">{{ t('instances.field.description') }}</label>
      <input
        :id="`${idPrefix}-desc`"
        v-model="model.description"
        class="input"
        type="text"
        maxlength="200"
      />
    </Field>

    <Field>
      <span class="t-label">{{ t('instances.field.accent') }}</span>
      <div class="picker">
        <button
          v-for="accent in accents"
          :key="accent.value"
          type="button"
          :style="{ background: accentVar(accent.value) }"
          :aria-pressed="model.accent === accent.value"
          :aria-label="accent.label"
          :title="accent.label"
          @click="model.accent = accent.value"
        ></button>

        <label
          class="swatch-custom"
          :class="{ on: isCustomAccent(model.accent) }"
          :title="t('instances.field.accentCustom')"
        >
          <input
            type="color"
            :value="isCustomAccent(model.accent) ? model.accent : '#4db6a5'"
            :aria-label="t('instances.field.accentCustom')"
            @input="model.accent = ($event.target as HTMLInputElement).value"
          />
        </label>
      </div>
    </Field>

    <Field>
      <label :for="`${idPrefix}-port`">{{ t('instances.field.port') }}</label>
      <input
        :id="`${idPrefix}-port`"
        v-model.number="model.preferredPort"
        class="input num"
        type="number"
        min="1024"
        max="65535"
      />
      <p class="hint">{{ t('instances.field.portHint') }}</p>
    </Field>
  </var>
</template>

