<script setup lang="ts">
withDefaults(
  defineProps<{
    light?: string;
    dark?: string;
    /** .app.flush — без внутреннего отступа, когда содержимое само задаёт поля. */
    flush?: boolean;
  }>(),
  { light: 'Светлая', dark: 'Тёмная', flush: false },
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
