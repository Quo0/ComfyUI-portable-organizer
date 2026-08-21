<script setup lang="ts">
// Добавление инстанса — отдельный роут, а не модалка: дочерний вебвью
// с ComfyUI это нативное окно поверх нашего HTML, и модалка над ним
// физически невозможна. Правило применяется всюду, а не только там,
// где вебвью уже есть, — иначе оно расползётся исключениями.
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
/** Ошибка проверки показывается на месте, а не тостом: она про эту папку
    и должна остаться на экране, пока пользователь не выберет другую. */
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
      <!-- Ряд шага, как в мастере: обе дорожки «Добавления» начинаются
           одинаково. «Назад» ведёт в «Добавление», а не в список сборок:
           сюда приходят только оттуда, и «Назад» всегда левее действия. -->
      <StepBar>
        <h2 class="title">{{ t('instances.add.title') }}</h2>
        <span class="spacer"></span>
        <span class="acts">
          <RouterLink class="btn ghost" to="/install">
            <ArrowLeft class="ico" />
            {{ t('common.back') }}
          </RouterLink>
          <!-- Главное действие экрана стоит там же, где во всём разделе, —
               в ряду шага. Внизу, под формой, оно уезжало из виду вместе
               с прочитанным о папке.

               Пока папку не выбрали, добавлять нечего, и кнопка выключена,
               а не спрятана: пропадающая кнопка не говорит, чем экран
               закончится. Исчезает она только там, где действие другое, —
               когда папка уже в списке. -->
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
                <!-- Путь не переводится и не сокращается: по нему пользователь
                     идёт разбираться руками. -->
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
            <!-- Повторная регистрация той же папки не создаёт второй инстанс:
                 ведём к уже существующему. -->
            <Group v-if="probe.existingId">
              <RouterLink class="btn primary" :to="`/instances/${probe.existingId}`">
                {{ t('instances.add.openExisting') }}
              </RouterLink>
            </Group>

            <!-- Две колонки: слева то, что прочитали в папке, справа то,
                 что заполняет пользователь. Одним свитком прочитанное
                 оттесняло форму вниз, за край экрана. -->
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

                <!-- Кнопок под формой нет: действие уехало в ряд шага,
                     «Отмена» ушла как второй выход. -->
              </div>
            </div>
          </template>
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>
/* Ряд шага встал на место шапки экрана и берёт её верхнее поле:
   без него заголовок прилипает к заголовку окна. `:deep()` обязателен:
   корень `StepBar` — обёртка `<var>`, а не сам `.step-bar`, и обычный
   скоуп-атрибут родителя до вложенного div не достаёт.

   Якорь `.AddInstanceView` перед `:deep()` обязателен тоже: голый
   `:deep(.step-bar)` компилируется без scope-атрибута вовсе и утекает
   на все `StepBar` в приложении, а не только на этот экран. */
.AddInstanceView :deep(.step-bar) {
  padding-top: var(--space-4);
}
</style>
