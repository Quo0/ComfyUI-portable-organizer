<script setup lang="ts">

import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<template>
  <var class="InstallForks">

    <div class="forks">

      <div class="fork">
        <b>{{ t('install.existing.title') }}</b>
        <p>{{ t('install.existing.body') }}</p>
        <RouterLink class="btn secondary" to="/instances/add">
          {{ t('install.existing.action') }}
        </RouterLink>
      </div>

      <div class="fork">
        <b>{{ t('install.wizard.title') }}</b>
        <p>{{ t('install.wizard.body') }}</p>
        <RouterLink class="btn primary" to="/install/wizard">
          {{ t('install.archive.choose') }}
        </RouterLink>
      </div>
    </div>
  </var>
</template>
