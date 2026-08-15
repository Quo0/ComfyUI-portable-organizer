<script setup lang="ts">
// Кнопка «показать в проводнике» — одна на всё приложение.
//
// Она была скопирована по экранам вместе со всей обвязкой: проверка
// на пустой путь, `title`, `aria-label` и значок. Обвязка здесь не
// украшение — значок без подписи обязан нести доступное имя, иначе
// программа чтения с экрана объявит кнопку безымянной, — и забыть его
// в очередной копии было вопросом времени.
//
// Открывает не саму папку, а показывает её в проводнике выделенной.
// Так устроены все такие кнопки в приложении, и расходиться поведением
// у одинакового значка нельзя.
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

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
  if (props.path) void revealItemInDir(props.path);
}
</script>

<template>
  <button
    type="button"
    class="btn ghost"
    :disabled="disabled || !path"
    :title="title ?? name"
    :aria-label="name"
    @click="open"
  >
    <svg class="ico"><use href="#i-folder" /></svg>
  </button>
</template>
