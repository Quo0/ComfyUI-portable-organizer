<script setup lang="ts">
// Панель с фиксированными шапкой и подвалом и прокруткой между ними:
// сетка из трёх строк (голова — тело — подвал), тело тянется. Head/foot
// не отдельные слоты — та же логика, что у Group: `.pane-head`/`.pane-foot`
// просто первый и последний ребёнок в разметке, компонент их не различает.
</script>

<template>
  <var class="Pane">
    <div class="pane">
      <slot />
    </div>
  </var>
</template>

<style src="./Pane.css" />
