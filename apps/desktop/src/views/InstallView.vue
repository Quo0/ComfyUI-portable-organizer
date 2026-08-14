<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import InstallForks from '../components/InstallForks.vue';

const { t } = useI18n();

// Раздел постоянный, а не только для первого запуска. Первая дорожка —
// регистрация существующей папки — работает с Фазы 1; мастер распаковки
// придёт в Фазе 1.5.
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('install.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad wide">
        <InstallForks />
      </div>
    </div>
  </section>
</template>
