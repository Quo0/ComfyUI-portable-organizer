<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

// Две дорожки — существующая папка и мастер распаковки — появятся
// в Фазе 1.5. Раздел постоянный, а не только для первого запуска.
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('install.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <p class="t-sm">{{ t('install.lead') }}</p>
        <p class="placeholder">{{ t('common.notReady') }}</p>
      </div>
    </div>
  </section>
</template>
