<script setup lang="ts">

//

import ScreenHeader from './ScreenHeader.vue';

withDefaults(defineProps<{
  toolbar?: boolean;
}>(), { toolbar: false });
</script>

<template>
  <var class="InstanceHeader">
    <ScreenHeader v-if="!toolbar">
      <slot />
    </ScreenHeader>
    <div v-else class="inst-toolbar">
      <slot />
    </div>
  </var>
</template>

<style src="./InstanceHeader.css" />
