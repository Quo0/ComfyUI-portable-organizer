<script setup lang="ts">
withDefaults(
  defineProps<{
    light?: string;
    dark?: string;
    /** .app.flush — no inner padding, for content that sets its own. */
    flush?: boolean;
  }>(),
  { light: 'Light', dark: 'Dark', flush: false },
);
</script>

<template>
  <var class="ThemePair">
    <div class="pair">
      <figure class="panel t-light">
        <figcaption>{{ light }}</figcaption>
        <div class="app" :class="{ flush }"><slot /></div>
      </figure>
      <figure class="panel t-dark">
        <figcaption>{{ dark }}</figcaption>
        <div class="app" :class="{ flush }"><slot /></div>
      </figure>
    </div>
  </var>
</template>
