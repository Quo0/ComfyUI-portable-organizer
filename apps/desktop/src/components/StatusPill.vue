<script setup lang="ts">

import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import type { DisplayStatus } from '../lib/status';

const props = defineProps<{ status: DisplayStatus }>();

const { t } = useI18n();

const label = computed(() => {
  switch (props.status) {
    case 'starting':
      return t('status.starting');
    case 'running':
      return t('status.running');
    case 'stopping':
      return t('status.stopping');
    case 'crashed':
      return t('status.crashed');
    case 'detached':
      return t('status.detached');
    case 'unavailable':
      return t('status.unavailable');
    default:
      return t('status.stopped');
  }
});

const kind = computed(() => {
  switch (props.status) {
    case 'starting':
    case 'stopping':
      return 'starting';
    case 'running':
      return 'running';
    case 'crashed':
    case 'detached':
      return 'crashed';
    case 'unavailable':
      return 'gone';
    default:
      return 'stopped';
  }
});
</script>

<template>
  <var class="StatusPill">
    <span class="pill" :class="kind"><i></i>{{ label }}</span>
  </var>
</template>
