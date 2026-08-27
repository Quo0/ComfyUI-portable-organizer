<script setup lang="ts">

//

withDefaults(defineProps<{
  editable?: boolean;
  title?: string;
}>(), { editable: false });
</script>

<template>
  <var class="KeyValueRow">
    <div class="path-item" :class="{ editable }" :title="title">
      <slot />
    </div>
  </var>
</template>

<style src="./KeyValueRow.css" />
