<script setup lang="ts">
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import StatusPill from '../components/StatusPill.vue';
import type { Instance } from '../bindings';
import { accentVar, useFormat } from '../lib/format';
import { displayStatus } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';

const instances = useInstancesStore();
const run = useRunStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

/** Размер показывается, только когда посчитан. */
function sizeText(instance: Instance): string {
  return bytes(instance.sizeBytes);
}

onMounted(() => {
  if (!instances.loaded) void instances.load();
});
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('instances.title') }}</h1>
      <span v-if="instances.items.length" class="t-sm">
        {{ t('instances.count', instances.items.length) }}
      </span>
      <span class="head-spacer"></span>
      <RouterLink class="btn secondary" to="/instances/add">
        {{ t('instances.add.title') }}
      </RouterLink>
    </header>

    <!-- Единственная область прокрутки экрана: шапка с заголовком и кнопкой
         остаётся на месте, иначе при длинном списке непонятно, где ты. -->
    <div class="screen-body">
      <div class="screen-pad">
        <div v-if="instances.items.length" class="cards">
          <RouterLink
            v-for="instance in instances.items"
            :key="instance.id"
            class="card"
            :class="{ gone: !instance.available }"
            :to="`/instances/${instance.id}`"
          >
            <div
              class="card-accent"
              :style="{ '--instance-accent': accentVar(instance.accent) }"
            ></div>
            <div class="card-in">
              <div class="card-top">
                <div class="card-name">{{ instance.name }}</div>
                <StatusPill :status="displayStatus(instance, run.statusOf(instance.id))" />
              </div>
              <div v-if="instance.description" class="card-desc">
                {{ instance.description }}
              </div>
              <div class="meta">
                <span v-if="instance.comfyVersion">
                  ComfyUI {{ instance.comfyVersion }}
                </span>
                <span>:{{ instance.preferredPort }}</span>
                <span v-if="sizeText(instance)">{{ sizeText(instance) }}</span>
              </div>
              <div v-if="instance.source" class="src">
                {{
                  t('instances.field.source', {
                    archive: instance.source.archiveLabel,
                    when: moment(instance.source.installedAt),
                  })
                }}
              </div>
            </div>
          </RouterLink>
        </div>

        <!-- Отдельного Welcome-экрана нет: его роль берёт это состояние. -->
        <div v-else class="group">
          <p class="t-md">{{ t('instances.empty.title') }}</p>
          <p class="t-sm">{{ t('instances.empty.body') }}</p>
          <div class="row">
            <RouterLink class="btn primary" to="/install">
              {{ t('instances.empty.action') }}
            </RouterLink>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
