<script setup lang="ts">
// Оболочка приложения: рейл слева виден всегда, включая экран работающего
// инстанса, справа — сменяемая контентная область. Из этого ограничения
// выведено всё остальное: контентный прямоугольник это окно минус рейл,
// и именно его получает дочерний вебвью с ComfyUI.
import IconSprite from './shell/IconSprite.vue';
import NavRail from './shell/NavRail.vue';
import ToastHost from './shell/ToastHost.vue';
</script>

<template>
  <IconSprite />
  <div class="shell">
    <NavRail />
    <main class="content">
      <RouterView />
      <ToastHost />
    </main>
  </div>
</template>
