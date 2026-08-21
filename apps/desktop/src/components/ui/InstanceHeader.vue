<script setup lang="ts">
// Шапка сборки — один и тот же набор фактов (кнопка «назад», чип, имя,
// статус, порт), но в двух местах разной формы. По умолчанию это обычный
// `ScreenHeader`: экран сборки — такой же прокручиваемый раздел, как
// остальные, и его заголовок ничем не отличается от их.
//
// `toolbar` — вариант тулбара встроенной вкладки ComfyUI: рамка снизу
// отделяет его от нативного окна, которое сидит вплотную под ним,
// переноса строк нет и с боков не осталось свободного места — узкие
// значки вместо подписей. Непредсказуемая высота от переноса испортила бы
// расчёт прямоугольника вебвью (автолейаута у него нет, см. CLAUDE.md).
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
