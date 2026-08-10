<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

// Библиотека воркфлоу — Фаза 2.6.
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('workflows.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <p class="t-sm">{{ t('workflows.lead') }}</p>
        <p class="placeholder">{{ t('common.notReady') }}</p>
      </div>
    </div>
  </section>
</template>
