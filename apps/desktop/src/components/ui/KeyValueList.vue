<script setup lang="ts">

withDefaults(defineProps<{
  withActs?: boolean;
}>(), { withActs: false });
</script>

<template>
  <var class="KeyValueList">
    <div class="paths" :class="{ 'with-acts': withActs }">
      <slot />
    </div>
  </var>
</template>

<style src="./KeyValueList.css" />
