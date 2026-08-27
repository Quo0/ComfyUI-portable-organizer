<script setup lang="ts">
import { useI18n } from 'vue-i18n';

import InstallForks from '../components/InstallForks.vue';
import ScreenHeader from '../components/ui/ScreenHeader.vue';

const { t } = useI18n();
</script>

<template>
  <var class="InstallView">
    <section class="screen">
      <ScreenHeader>
        <h1 class="t-lg">{{ t('install.title') }}</h1>
      </ScreenHeader>

      <div class="screen-body">
        <div class="screen-pad wide">
          <InstallForks />
        </div>
      </div>
    </section>
  </var>
</template>
