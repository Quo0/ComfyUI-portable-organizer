<script setup lang="ts">

import { ArrowLeft } from '@lucide/vue';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/plugin-dialog';

import InstanceFields from '../components/InstanceFields.vue';
import Field from '../components/ui/Field.vue';
import Group from '../components/ui/Group.vue';
import KeyValueList from '../components/ui/KeyValueList.vue';
import KeyValueRow from '../components/ui/KeyValueRow.vue';
import StepBar from '../components/ui/StepBar.vue';
import type { AppError, InstanceEdit, ProbeResult } from '../bindings';
import { commands } from '../bindings';
import { errorText } from '../lib/errors';
import { useInstancesStore } from '../stores/instances';
import { useUiStore } from '../stores/ui';

const instances = useInstancesStore();
const ui = useUiStore();
const router = useRouter();
const { t } = useI18n();

const probing = ref(false);
const saving = ref(false);
const probe = ref<ProbeResult | null>(null);

const problem = ref<AppError | null>(null);

const edit = ref<InstanceEdit>({
  name: '',
  description: '',
  accent: 'teal',
  preferredPort: 8188,
});

async function chooseFolder(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked !== 'string') return;

  probing.value = true;
  probe.value = null;
  problem.value = null;
  try {
    const res = await commands.probeFolder(picked);
    if (res.status === 'error') {
      problem.value = res.error;
      return;
    }
    probe.value = res.data;
    edit.value = {
      name: res.data.suggestedName,
      description: '',
      accent: await instances.suggestAccent(),
      preferredPort: res.data.suggestedPort,
    };
  } finally {
    probing.value = false;
  }
}

async function submit(): Promise<void> {
  if (!probe.value) return;
  saving.value = true;
  try {
    const added = await instances.add(probe.value.probe.path, edit.value);
    if (!added) return;
    ui.pushOk(t('instances.toast.added', { name: added.name }));
    await router.push(`/instances/${added.id}`);
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <var class="AddInstanceView">
    <section class="screen">

      <StepBar>
        <h2 class="title">{{ t('instances.add.title') }}</h2>
        <span class="spacer"></span>
        <span class="acts">
          <RouterLink class="btn ghost" to="/install">
            <ArrowLeft class="ico" />
            {{ t('common.back') }}
          </RouterLink>

          <button
            v-if="!probe?.existingId"
            type="button"
            class="btn primary lg"
            :disabled="!probe || saving || edit.name.trim() === ''"
            @click="submit"
          >
            {{ t('instances.add.submit') }}
          </button>
        </span>
      </StepBar>

      <div class="screen-body">
        <div class="screen-pad wide">
          <p class="t-sm">{{ t('instances.add.lead') }}</p>

          <Field>
            <span class="t-label">{{ t('instances.field.folder') }}</span>
            <div class="path-row">
              <div class="input mono">

                <span>{{ probe?.probe.path ?? '' }}</span>
              </div>
              <button type="button" class="btn secondary" @click="chooseFolder">
                {{ t('instances.add.chooseFolder') }}
              </button>
            </div>
            <p v-if="probing" class="hint">{{ t('instances.add.checking') }}</p>
            <p v-if="problem" class="hint bad">{{ errorText(problem) }}</p>
          </Field>

          <template v-if="probe">

            <Group v-if="probe.existingId">
              <RouterLink class="btn primary" :to="`/instances/${probe.existingId}`">
                {{ t('instances.add.openExisting') }}
              </RouterLink>
            </Group>

            <div v-else class="cols">
              <div>
                <KeyValueList>
                  <KeyValueRow>
                    <span class="lbl">{{ t('instances.field.comfyVersion') }}</span>
                    <span class="val">
                      {{ probe.probe.comfyVersion ?? t('common.unknown') }}
                    </span>
                  </KeyValueRow>
                  <KeyValueRow>
                    <span class="lbl">{{ t('instances.field.pythonVersion') }}</span>
                    <span class="val">
                      {{ probe.probe.pythonVersion ?? t('common.unknown') }}
                    </span>
                  </KeyValueRow>
                  <KeyValueRow>
                    <span class="lbl">{{ t('instances.field.profiles') }}</span>
                    <span class="val">{{ probe.probe.profiles.length }}</span>
                  </KeyValueRow>
                </KeyValueList>

                <Group>
                  <div v-if="probe.probe.profiles.length" class="row">
                    <span
                      v-for="profile in probe.probe.profiles"
                      :key="profile.id"
                      class="pill stopped"
                      :title="profile.id"
                    >
                      {{ profile.name }}
                      <em v-if="profile.advanced" class="advanced">
                        {{ t('instances.field.profilesAdvanced') }}
                      </em>
                    </span>
                  </div>
                  <p v-else class="hint">{{ t('instances.field.profilesNone') }}</p>
                </Group>
              </div>

              <div>
                <InstanceFields v-model="edit" />
              </div>
            </div>
          </template>
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>

.AddInstanceView :deep(.step-bar) {
  padding-top: var(--space-4);
}
</style>
