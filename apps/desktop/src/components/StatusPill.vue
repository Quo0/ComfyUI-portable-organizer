<script setup lang="ts">
// Пилюля состояния. Ключи выписаны литералами, чтобы типизация от en.json
// их проверяла: собранная из имени состояния строка проскочила бы мимо.
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import type { InstanceStatus } from '../stores/instances';

const props = defineProps<{ status: InstanceStatus }>();

const { t } = useI18n();

const label = computed(() => {
  switch (props.status) {
    case 'starting':
      return t('status.starting');
    case 'running':
      return t('status.running');
    case 'crashed':
      return t('status.crashed');
    case 'unavailable':
      return t('status.unavailable');
    default:
      return t('status.stopped');
  }
});

/** Недоступный инстанс рисуется пунктиром: он не сломан, его просто нет. */
const kind = computed(() =>
  props.status === 'unavailable' ? 'gone' : props.status,
);
</script>

<template>
  <span class="pill" :class="kind"><i></i>{{ label }}</span>
</template>
