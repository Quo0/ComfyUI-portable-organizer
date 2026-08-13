<script setup lang="ts">
// Оболочка приложения: рейл слева виден всегда, включая экран работающего
// инстанса, справа — сменяемая контентная область. Из этого ограничения
// выведено всё остальное: контентный прямоугольник это окно минус рейл,
// и именно его получает дочерний вебвью с ComfyUI.
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';

import IconSprite from './shell/IconSprite.vue';
import NavRail from './shell/NavRail.vue';
import ToastHost from './shell/ToastHost.vue';
import { events } from './bindings';

const router = useRouter();

// Закрытие окна при работающих серверах. Rust к этому моменту уже отменил
// закрытие и спрятал вкладки — остаётся привести на экран вопроса.
// Роутом владеет фронт, поэтому навигация здесь, а не в Rust.
onMounted(async () => {
  await events.quitRequested.listen(() => {
    void router.push('/quit');
  });
});
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
