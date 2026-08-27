<script setup lang="ts">

import { onMounted } from 'vue';
import { useRouter } from 'vue-router';

import NavRail from './shell/NavRail.vue';
import ToastHost from './shell/ToastHost.vue';
import { events } from './bindings';
import { useUiStore } from './stores/ui';
import { useUpdatesStore } from './stores/updates';

const router = useRouter();
const ui = useUiStore();
const updates = useUpdatesStore();

onMounted(async () => {
  await events.quitRequested.listen(() => {
    void router.push('/quit');
  });

  if (ui.checkUpdates) void updates.check(false);
});
</script>

<template>
  <var class="App">
    <div class="shell">
      <NavRail />
      <main class="content">
        <RouterView />
        <ToastHost />
      </main>
    </div>
  </var>
</template>
