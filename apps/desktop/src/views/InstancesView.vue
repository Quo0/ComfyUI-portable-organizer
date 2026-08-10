<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

// Список инстансов появится в Фазе 1. Пустое состояние написано сразу
// и остаётся: отдельного Welcome-экрана в приложении нет, его роль
// берут на себя пустые состояния «Инстансов» и «Установки».
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('instances.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <div class="group">
          <p class="t-md">{{ t('instances.empty.title') }}</p>
          <p class="t-sm">{{ t('instances.empty.body') }}</p>
          <div class="row">
            <RouterLink class="btn primary" to="/install">
              {{ t('instances.empty.action') }}
            </RouterLink>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
