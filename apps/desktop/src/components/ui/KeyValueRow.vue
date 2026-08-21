<script setup lang="ts">
// Строка `KeyValueList`: подпись и значение в сетке из двух колонок.
//
// `editable` — строка с полями редактирования вместо статичного значения:
// своя сетка (подпись — два поля — действия). Модификатор `with-acts` сюда
// не входит: он живёт на списке (`KeyValueList`), а не на строке, — одна
// настройка сразу на все строки.
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
