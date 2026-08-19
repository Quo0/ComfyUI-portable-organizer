<script setup lang="ts">
// Кнопка с выпадающим меню.
//
// Первый всплывающий слой в приложении, кроме тостов, — и заведён он
// с оговоркой, которую нельзя потерять.
//
// **На экране вкладки сборки этот компонент ставить нельзя.** Там поверх
// нашего HTML лежит нативное окно ComfyUI, и перекрыть его разметкой
// невозможно в принципе: меню просто не будет видно. Именно поэтому
// форма вставки воркфлоу разворачивается на месте списка, а не всплывает
// над ним, — и такой должна остаться. Меню годится только для обычных
// экранов, где ниже нас ничего чужого не лежит.
//
// Пункты приходят слотом: компонент отвечает за раскрытие, закрытие
// и клавиатуру, а что именно делают пункты — дело того, кто их поставил.
import type { Component } from 'vue';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

const { label, icon } = defineProps<{
  /** Подпись на кнопке. */
  label: string;
  /** Компонент значка из `@lucide/vue`, например `Plus`. */
  icon?: Component;
}>();

const open = ref(false);
const root = useTemplateRef<HTMLElement>('root');

/**
 * Клик мимо меню закрывает его.
 *
 * `pointerdown`, а не `click`: пункт может увести фокус или открыть
 * системный диалог, и `click` до нас уже не дойдёт. Фаза перехвата —
 * чтобы успеть раньше обработчиков на самих элементах страницы.
 */
function onPointerDown(event: PointerEvent): void {
  if (!open.value) return;
  if (root.value?.contains(event.target as Node)) return;
  open.value = false;
}

/** Escape закрывает меню, не трогая ничего вокруг. */
function onKeyDown(event: KeyboardEvent): void {
  if (!open.value || event.key !== 'Escape') return;
  event.stopPropagation();
  open.value = false;
}

onMounted(() => {
  document.addEventListener('pointerdown', onPointerDown, true);
  document.addEventListener('keydown', onKeyDown);
});

onUnmounted(() => {
  document.removeEventListener('pointerdown', onPointerDown, true);
  document.removeEventListener('keydown', onKeyDown);
});
</script>

<template>
  <div ref="root" class="menu">
    <button
      type="button"
      class="btn secondary"
      aria-haspopup="menu"
      :aria-expanded="open"
      @click="open = !open"
    >
      <component :is="icon" v-if="icon" class="ico" />
      {{ label }}
    </button>

    <!-- Закрытие висит на самом списке, а не на каждом пункте: выбор
         пункта всплывает сюда, и повторять `@click` в каждой разметке,
         которая этим меню пользуется, значило бы однажды забыть. -->
    <div v-if="open" class="menu-pop" role="menu" @click="open = false">
      <slot />
    </div>
  </div>
</template>
