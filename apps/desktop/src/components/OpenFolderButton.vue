<script setup lang="ts">
// Кнопка «показать в проводнике» — одна на всё приложение.
//
// Она была скопирована по экранам вместе со всей обвязкой: проверка
// на пустой путь, `title`, `aria-label` и значок. Обвязка здесь не
// украшение — значок без подписи обязан нести доступное имя, иначе
// программа чтения с экрана объявит кнопку безымянной, — и забыть его
// в очередной копии было вопросом времени.
//
// `path` — всегда папка, а не файл. `openPath` открывает её содержимым
// в проводнике; `revealItemInDir` вместо этого открыл бы родителя
// с выделенной папкой внутри — путь пришлось бы проходить вручную
// ещё на шаг, каждый раз одним и тем же лишним Enter.
import { FolderOpen } from '@lucide/vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { openPath } from '@tauri-apps/plugin-opener';

import { useUiStore } from '../stores/ui';

const props = withDefaults(
  defineProps<{
    /** Что показать. Пусто — кнопка погашена, звать проводник не с чем. */
    path?: string | null;
    /** Доступное имя. По умолчанию — «Открыть папку». */
    label?: string;
    /**
     * Подсказка, когда она полезнее имени: обычно сам путь. Путь
     * не переводится и не сокращается — по нему идут разбираться руками.
     */
    title?: string;
    /** Причина погасить кнопку помимо пустого пути: папки может не быть. */
    disabled?: boolean;
  }>(),
  { path: null, label: undefined, title: undefined, disabled: false },
);

const { t } = useI18n();
const ui = useUiStore();

const name = computed(() => props.label ?? t('common.openFolder'));

// `openPath` — команда плагина, а не наша, и ошибку она отдаёт отказом
// промиса, а не `AppError`. Брошенный `void` съедал этот отказ целиком:
// кнопка нажималась, проводник не открывался, и на экране не было ничего —
// ни папки, ни причины. Приводим отказ к обычному коду ошибки и показываем
// его тостом, как любую другую.
async function open(): Promise<void> {
  if (!props.path) return;
  try {
    await openPath(props.path);
  } catch (e) {
    ui.pushError({
      code: 'shell.openFailed',
      params: { reason: String(e), path: props.path },
    });
  }
}
</script>

<template>
  <var class="OpenFolderButton">
    <button
      type="button"
      class="btn ghost icon"
      :disabled="disabled || !path"
      :title="title ?? name"
      :aria-label="name"
      @click="open"
    >
      <FolderOpen class="ico" />
    </button>
  </var>
</template>
