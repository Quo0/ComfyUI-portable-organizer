<script setup lang="ts">
// Подпись и поле (или любое другое содержимое) одной колонкой:
// `label`/`t-label` первой строкой, дальше — что угодно (ввод, подсказка,
// ошибка). Слота под подпись отдельно нет — тот же принцип, что у Group.
</script>

<template>
  <var class="Field">
    <div class="field">
      <slot />
    </div>
  </var>
</template>

<style src="./Field.css" />
