<script setup lang="ts">
// Поля инстанса, общие для добавления и правки. Один компонент, два места
// показа: иначе валидация и подписи разойдутся при первой же правке.
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import type { InstanceEdit } from '../bindings';
import { accentVar, isCustomAccent } from '../lib/format';

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
  <div class="field">
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
  </div>

  <div class="field">
    <label :for="`${idPrefix}-desc`">{{ t('instances.field.description') }}</label>
    <input
      :id="`${idPrefix}-desc`"
      v-model="model.description"
      class="input"
      type="text"
      maxlength="200"
    />
  </div>

  <div class="field">
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
  </div>

  <div class="field">
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
  </div>
</template>

<style scoped>
/* Квадрат выбора своего цвета: тот же размер, что у образцов палитры.
   Радуга вместо заливки — потому что «любой цвет» нечем показать одним. */
.swatch-custom {
  width: 22px;
  height: 22px;
  border-radius: var(--radius-sm);
  display: block;
  cursor: pointer;
  box-shadow: 0 0 0 1px var(--line-strong) inset;
  background: conic-gradient(
    #e5534b,
    #d9a441,
    #6fbf73,
    #4db6a5,
    #5b8def,
    #a77bd6,
    #e5534b
  );
}

.swatch-custom.on {
  outline: 2px solid var(--ink);
  outline-offset: 2px;
}

/* Родное поле цвета скрыто, но остаётся кликабельным и фокусируемым:
   именно оно открывает системную палитру. */
.swatch-custom input {
  opacity: 0;
  width: 100%;
  height: 100%;
  display: block;
  cursor: pointer;
}

.swatch-custom:focus-within {
  outline: 2px solid var(--focus-ring);
  outline-offset: 2px;
}
</style>
