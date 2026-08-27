<script setup lang="ts">

//

import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';

const { path = null, empty = '', id = undefined } = defineProps<{
  path?: string | null;

  empty?: string;

  id?: string;
}>();

const emit = defineEmits<{
  pick: [path: string];
}>();

const { t } = useI18n();

async function browse(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked === 'string') emit('pick', picked);
}
</script>

<template>
  <var class="PathPicker">

    <div class="path-row">

      <div :id="id" class="input mono" :title="path || undefined">
        <span>{{ path || empty }}</span>
      </div>
      <button class="btn secondary" type="button" @click="browse">
        {{ t('common.browse') }}
      </button>
    </div>
  </var>
</template>
