<script setup lang="ts">

import { RouterLink } from 'vue-router';

withDefaults(defineProps<{
  as?: 'div' | typeof RouterLink;

  accent?: string;
  gone?: boolean;
}>(), { as: 'div', accent: undefined, gone: false });
</script>

<template>
  <component :is="as" class="card" :class="{ gone }">
    <div
      class="card-accent"
      :style="accent ? { '--instance-accent': accent } : undefined"
    ></div>
    <div class="card-in">
      <slot />
    </div>
  </component>
</template>

<style src="./Card.css" />
