<script setup lang="ts">
// Поля инстанса, общие для добавления и правки. Один компонент, два места
// показа: иначе валидация и подписи разойдутся при первой же правке.
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import type { InstanceEdit } from '../bindings';
import { accentVar, isCustomAccent } from '../lib/format';
import Field from './ui/Field.vue';

const model = defineModel<InstanceEdit>({ required: true });

/**
 * Приставка к `id` полей.
 *
 * Нужна там, где формы на экране две сразу — в мастере это «Новое
 * назначение» и «Редактирование». Одинаковые `id` у двух полей рвут
 * связь подписи с полем: клик по подписи попадает в чужое поле,
 * и ошибку эту глазами не увидеть.
 */
const { idPrefix = 'instance', showProblems = true } = defineProps<{
  idPrefix?: string;
  /**
   * Показывать ли жалобы на незаполненное.
   *
   * По умолчанию да: на экранах добавления и правки инстанса имя уже
   * прочитано из папки, и пустым оно бывает, только если его стёрли
   * руками. В мастере иначе — там форма после каждого добавления
   * встаёт в исходное, и подсветка на чистой форме ругается на поле,
   * которого пользователь ещё не касался.
   */
  showProblems?: boolean;
}>();

const { t } = useI18n();

// Ключи выписаны литералами, а не собраны из имени цвета: типизация ключей
// от en.json проверяет только литералы, а собранная строка проскочит мимо
// и всплывёт пустой подписью в интерфейсе.
const accents = computed(() => [
  { value: 'teal' as const, label: t('accent.teal') },
  { value: 'indigo' as const, label: t('accent.indigo') },
  { value: 'ember' as const, label: t('accent.ember') },
  { value: 'moss' as const, label: t('accent.moss') },
  { value: 'azure' as const, label: t('accent.azure') },
  { value: 'orchid' as const, label: t('accent.orchid') },
  { value: 'rose' as const, label: t('accent.rose') },
  { value: 'amber' as const, label: t('accent.amber') },
]);

/**
 * Пустое имя не принимается. Подсказка появляется до сохранения,
 * но не раньше, чем есть о чём говорить: `showProblems` держит
 * нетронутую форму молчащей.
 */
const nameEmpty = () => showProblems && model.value.name.trim() === '';
</script>

<template>
  <var class="InstanceFields">
    <Field>
      <label :for="`${idPrefix}-name`">{{ t('instances.field.name') }}</label>
      <input
        :id="`${idPrefix}-name`"
        v-model="model.name"
        class="input"
        :class="{ bad: nameEmpty() }"
        type="text"
        maxlength="80"
      />
      <p v-if="nameEmpty()" class="hint bad">{{ t('errors.instances.emptyName') }}</p>
    </Field>

    <Field>
      <label :for="`${idPrefix}-desc`">{{ t('instances.field.description') }}</label>
      <input
        :id="`${idPrefix}-desc`"
        v-model="model.description"
        class="input"
        type="text"
        maxlength="200"
      />
    </Field>

    <Field>
      <span class="t-label">{{ t('instances.field.accent') }}</span>
      <div class="picker">
        <button
          v-for="accent in accents"
          :key="accent.value"
          type="button"
          :style="{ background: accentVar(accent.value) }"
          :aria-pressed="model.accent === accent.value"
          :aria-label="accent.label"
          :title="accent.label"
          @click="model.accent = accent.value"
        ></button>
        <!-- Свой цвет тем же квадратом, что и палитра: это такой же выбор,
             просто из всех цветов сразу. Значение уходит в разметку как есть
             и потому одинаково в обеих темах — за палитру ручается сборка
             дизайна, за свой цвет отвечает пользователь. -->
        <label
          class="swatch-custom"
          :class="{ on: isCustomAccent(model.accent) }"
          :title="t('instances.field.accentCustom')"
        >
          <input
            type="color"
            :value="isCustomAccent(model.accent) ? model.accent : '#4db6a5'"
            :aria-label="t('instances.field.accentCustom')"
            @input="model.accent = ($event.target as HTMLInputElement).value"
          />
        </label>
      </div>
    </Field>

    <Field>
      <label :for="`${idPrefix}-port`">{{ t('instances.field.port') }}</label>
      <input
        :id="`${idPrefix}-port`"
        v-model.number="model.preferredPort"
        class="input num"
        type="number"
        min="1024"
        max="65535"
      />
      <p class="hint">{{ t('instances.field.portHint') }}</p>
    </Field>
  </var>
</template>

<!-- Своих стилей у полей больше нет: квадрат выбора своего цвета уехал
     в дизайн-систему. Живя только здесь, он не попадал в макеты, и там
     палитра выходила на квадрат короче, чем в приложении. -->
