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
      <span class="nav-item" aria-disabled="true">
        <span>{{ t('settings.section.library') }}</span>
      </span>
      <span class="nav-item" aria-disabled="true">
        <span>{{ t('settings.section.archives') }}</span>
      </span>
    </nav>

    <RouterView />
  </div>
</template>
