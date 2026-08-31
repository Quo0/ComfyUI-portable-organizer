<script setup lang="ts">

import { FolderOpen } from '@lucide/vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { openPath } from '@tauri-apps/plugin-opener';

import { useUiStore } from '../stores/ui';

const props = withDefaults(
  defineProps<{
    path?: string | null;

    label?: string;

    title?: string;

    disabled?: boolean;
  }>(),
  { path: null, label: undefined, title: undefined, disabled: false },
);

const { t } = useI18n();
const ui = useUiStore();

const name = computed(() => props.label ?? t('common.openFolder'));

async function open(): Promise<void> {
  if (!props.path) return;
  try {
    await openPath(props.path);
  } catch (e) {
    ui.pushError({
      code: 'shell.openFailed',
      params: { reason: String(e), path: props.path },
    });
  }
}
</script>

<template>
  <var class="OpenFolderButton">
    <button
      type="button"
      class="btn ghost icon"
      :disabled="disabled || !path"
      :title="title ?? name"
      :aria-label="name"
      @click="open"
    >
      <FolderOpen class="ico" />
    </button>
  </var>
</template>
