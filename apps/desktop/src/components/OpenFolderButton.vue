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

const name = computed(() => props.label ?? t('common.openFolder'));

function open(): void {
  if (props.path) void openPath(props.path);
}
</script>

<template>
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
</template>
