<script setup lang="ts">

withDefaults(defineProps<{
  danger?: boolean;
}>(), { danger: false });
</script>

<template>
  <var class="Group">
    <div class="group" :class="{ 'danger-zone': danger }">
      <slot />
    </div>
  </var>
</template>

<style src="./Group.css" />
