<script setup lang="ts">
// Карточка сборки: цветная полоса-акцент слева фиксированной ширины,
// содержимое справа. Форма — сетка из двух колонок — общая везде,
// а вот тег корня разный: в списке сборок карточка — ссылка, в списке
// назначений мастера и в списке работающих при выходе — обычный блок.
// Поэтому тег передаётся пропом `as`, а не выбирается самим компонентом;
// `to` и всё остальное, что нужно `RouterLink`, просто проваливается вниз
// обычным fallthrough-атрибутом — Card его не объявляет и не трогает.
import { RouterLink } from 'vue-router';

withDefaults(defineProps<{
  as?: 'div' | typeof RouterLink;
  /** Уже готовое значение CSS-переменной (`accentVar(instance.accent)`),
   *  а не имя акцента: сам расчёт — забота вызывающего экрана. */
  accent?: string;
  gone?: boolean;
}>(), { as: 'div', accent: undefined, gone: false });
</script>

<template>
  <component :is="as" class="card" :class="{ gone }">
    <div
      class="card-accent"
      :style="accent ? { '--instance-accent': accent } : undefined"
    ></div>
    <div class="card-in">
      <slot />
    </div>
  </component>
</template>

<style src="./Card.css" />
