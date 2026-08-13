<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

// Канонический дом настроек общих ресурсов — именно этот раздел. Мастер
// установки в Фазе 1.5 встроит те же компоненты отдельным шагом, чтобы
// первый запуск был цельным; логика при этом не дублируется.
//
// Разделы, кроме «Внешнего вида», появятся вместе со своими фазами.
</script>

<template>
  <div class="settings-split">
    <nav class="settings-sections">
      <RouterLink class="nav-item" active-class="on" to="/settings/appearance">
        <span>{{ t('settings.section.appearance') }}</span>
      </RouterLink>
      <RouterLink class="nav-item" active-class="on" to="/settings/shared-models">
        <span>{{ t('settings.section.sharedModels') }}</span>
      </RouterLink>
      <RouterLink class="nav-item" active-class="on" to="/settings/workflow-library">
        <span>{{ t('settings.section.library') }}</span>
      </RouterLink>
      <!-- Отчёт по диску — не настройка, но и не раздел рейла: рейл
           фиксирован планом, а отчёт про то же, про что общие модели. -->
      <RouterLink class="nav-item" active-class="on" to="/settings/duplicates">
        <span>{{ t('settings.section.report') }}</span>
      </RouterLink>
      <RouterLink class="nav-item" active-class="on" to="/settings/archives">
        <span>{{ t('settings.section.archives') }}</span>
      </RouterLink>
    </nav>

    <RouterView />
  </div>
</template>
