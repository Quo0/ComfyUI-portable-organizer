<script setup lang="ts">
// Форма назначения мастера установки. Один компонент, два места показа:
// «Новое назначение» слева и «Редактирование», разворачивающееся прямо
// в списке. Выписанная дважды, она разошлась бы при первой же правке —
// ровно та причина, по которой существует InstanceFields.
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';

import InstanceFields from './InstanceFields.vue';
import Field from './ui/Field.vue';
import Pane from './ui/Pane.vue';
import type { InstallTarget, TargetCheck } from '../bindings';
import { errorText } from '../lib/errors';

const target = defineModel<InstallTarget>({ required: true });

const {
  title,
  check = null,
  showProblems = true,
  idPrefix,
} = defineProps<{
  /** Заголовок панели: «Новое назначение» или «Редактирование». */
  title: string;
  check?: TargetCheck | null;
  /**
   * Показывать ли жалобы проверки. Пустая форма, к которой ещё
   * не притрагивались, молчать обязана: ругаться на незаполненное поле
   * раньше, чем пользователь до него дошёл, — врать.
   */
  showProblems?: boolean;
  /** Приставка к `id` полей: форм на экране бывает две разом. */
  idPrefix: string;
}>();

const { t } = useI18n();

const emit = defineEmits<{ change: [] }>();

async function pickFolder(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked !== 'string') return;
  target.value.path = picked;
  // Имя папки — разумное значение по умолчанию, но только пока пользователь
  // не вписал своё: затирать введённое было бы грубо.
  if (!target.value.name.trim()) {
    target.value.name = picked.split(/[\\/]/).filter(Boolean).pop() ?? '';
  }
  emit('change');
}
</script>

<template>
  <var class="TargetForm">
    <Pane>
      <div class="pane-head">
        <span class="title">{{ title }}</span>
        <slot name="acts"></slot>
      </div>

      <div class="scroll-pad">
        <Field>
          <span class="t-label">{{ t('instances.field.folder') }}</span>
          <div class="path-row">
            <!-- Путь не переводится и не сокращается. -->
            <div class="input mono"><span>{{ target.path }}</span></div>
            <button type="button" class="btn secondary" @click="pickFolder">
              {{ t('install.targets.choose') }}
            </button>
          </div>

          <!-- Ошибки и предупреждения разделены: с предупреждением
               распаковать можно, с ошибкой — нет. Оба относятся к пути,
               поэтому стоят под ним. -->
          <template v-if="showProblems">
            <p v-for="(problem, i) in check?.errors ?? []" :key="`e${i}`" class="hint bad">
              {{ errorText(problem) }}
            </p>
            <p v-for="(problem, i) in check?.warnings ?? []" :key="`w${i}`" class="hint">
              {{ errorText(problem) }}
            </p>
          </template>
        </Field>

        <InstanceFields
          v-model="target"
          :id-prefix="idPrefix"
          :show-problems="showProblems"
        />
      </div>
    </Pane>
  </var>
</template>
