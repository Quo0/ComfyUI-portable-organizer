<script setup lang="ts">
// Строка «путь + Обзор»: поле показывает выбранную папку, менять её можно
// только системным диалогом — руками путь не набирают.
//
// Пара верстались на каждом экране заново, и одна копия успела разойтись:
// в шапке библиотеки воркфлоу поле осталось без `.path-row`, а обрезка
// с многоточием живёт именно там — длинный путь вылезал за край раздела
// и обрывался без единого признака, что он продолжается. Разметка,
// обрезка и подсказка с полным путём теперь приезжают вместе.
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';

const { path = null, empty = '', id = undefined } = defineProps<{
  /** Текущий путь. Пусто — на его месте стоит `empty`. */
  path?: string | null;
  /** Что показывать, пока путь не выбран. */
  empty?: string;
  /** Идентификатор поля — для внешнего `<label for>`. */
  id?: string;
}>();

const emit = defineEmits<{
  /** Пользователь выбрал папку. Что с ней делать — решает экран. */
  pick: [path: string];
}>();

const { t } = useI18n();

/**
 * Диалог выбора папки.
 *
 * Отмену не отличаем от «ничего не выбрано»: в обоих случаях делать
 * нечего, и сообщать не о чем.
 */
async function browse(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked === 'string') emit('pick', picked);
}
</script>

<template>
  <!-- Ряд растягивается на всю ширину `.field`, а в ряду с другими
       действиями его тянет класс `grow` снаружи. -->
  <div class="path-row">
    <!-- `title` обязателен: строка обрезается многоточием, и полный путь
         иначе взять негде. У пустого поля подсказки нет — подсказывать
         нечего. -->
    <div :id="id" class="input mono" :title="path || undefined">
      <span>{{ path || empty }}</span>
    </div>
    <button class="btn secondary" type="button" @click="browse">
      {{ t('common.browse') }}
    </button>
  </div>
</template>
