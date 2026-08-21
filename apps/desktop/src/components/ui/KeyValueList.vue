<script setup lang="ts">
// Список пар «подпись — значение»: зазор между строками в 1px красит сам
// список (тёмный фон), а не рамка у каждой строки, — того же приёма
// держится `KeyValueRow`.
withDefaults(defineProps<{
  /** У каждой строки есть действия (кнопка, значок): своя сетка колонок
   *  и вид подписи у `KeyValueRow`. Модификатор стоит на списке, а не
   *  на строке, — одна настройка для всех строк разом. */
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
