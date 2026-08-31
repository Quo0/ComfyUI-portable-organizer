<script setup lang="ts">

import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';

import StatusPill from '../components/StatusPill.vue';
import Card from '../components/ui/Card.vue';
import ScreenHeader from '../components/ui/ScreenHeader.vue';
import { commands } from '../bindings';
import { accentVar } from '../lib/format';
import { displayStatus } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useUiStore } from '../stores/ui';

const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const router = useRouter();
const { t } = useI18n();

const busy = ref(false);

const running = computed(() =>
  run.active
    .filter((status) => status.state !== 'detached')
    .map((status) => ({
      status,
      instance: instances.byId(status.instanceId),
    }))
    .filter((row) => row.instance),
);

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  await run.load();

  if (running.value.length === 0) await router.replace('/instances');
});

async function stopAndExit(): Promise<void> {
  busy.value = true;
  const res = await commands.stopAllAndQuit();

  busy.value = false;
  if (res.status === 'error') ui.pushError(res.error);
}

async function toTray(): Promise<void> {
  const res = await commands.hideToTray();
  if (res.status === 'error') {
    ui.pushError(res.error);
    return;
  }

  await router.replace('/instances');
}
</script>

<template>
  <var class="QuitView">
    <section class="screen">
      <ScreenHeader>
        <h1 class="t-lg">{{ t('quit.title') }}</h1>
      </ScreenHeader>

      <div class="screen-body">
        <div class="screen-pad">
          <p class="t-sm">{{ t('quit.body') }}</p>

          <div class="cards">
            <Card
              v-for="row in running"
              :key="row.status.instanceId"
              :accent="accentVar(row.instance!.accent)"
            >
              <div class="card-top">
                <span class="card-name">{{ row.instance!.name }}</span>
                <StatusPill
                  :status="displayStatus(row.instance!, row.status)"
                />
                <span v-if="row.status.port" class="t-mono">
                  :{{ row.status.port }}
                </span>
              </div>
            </Card>
          </div>

          <div class="row">
            <button type="button" class="btn danger lg" :disabled="busy" @click="stopAndExit">
              {{ t('quit.stopAndExit') }}
            </button>
            <button type="button" class="btn secondary lg" :disabled="busy" @click="toTray">
              {{ t('quit.toTray') }}
            </button>
          </div>
          <p class="hint">{{ t('quit.trayHint') }}</p>
        </div>
      </div>
    </section>
  </var>
</template>
