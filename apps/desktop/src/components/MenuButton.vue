<script setup lang="ts">

//

//

//

import type { Component } from 'vue';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

const { label, icon } = defineProps<{
  label: string;

  icon?: Component;
}>();

const open = ref(false);
const root = useTemplateRef<HTMLElement>('root');

function onPointerDown(event: PointerEvent): void {
  if (!open.value) return;
  if (root.value?.contains(event.target as Node)) return;
  open.value = false;
}

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
  <var class="MenuButton">
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

      <div v-if="open" class="menu-pop" role="menu" @click="open = false">
        <slot />
      </div>
    </div>
  </var>
</template>
