<script setup lang="ts">

import { computed } from 'vue';

const { path } = defineProps<{ path: string }>();

const parts = computed(() => path.match(/[^\\/]*[\\/]+|[^\\/]+/g) ?? []);
</script>

<template>
  <var class="PathText">

    <span class="path-text"
      ><template v-for="(part, i) in parts" :key="i">{{ part }}<wbr /></template
    ></span>
  </var>
</template>
