<script setup lang="ts">
import { useI18n } from 'vue-i18n';

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
      <div class="screen-pad">
        <p class="t-sm">{{ t('install.lead') }}</p>

        <div class="pane">
          <div class="pane-head">
            <span class="title">{{ t('install.existing.title') }}</span>
          </div>
          <div class="scroll-pad">
            <p class="t-sm">{{ t('install.existing.body') }}</p>
            <div class="row">
              <RouterLink class="btn primary" to="/instances/add">
                {{ t('install.existing.action') }}
              </RouterLink>
            </div>
          </div>
        </div>

        <div class="pane">
          <div class="pane-head">
            <span class="title">{{ t('install.wizard.title') }}</span>
          </div>
          <div class="scroll-pad">
            <p class="t-sm">{{ t('install.wizard.body') }}</p>
            <div class="row">
              <RouterLink class="btn primary" to="/install/wizard">
                {{ t('install.archive.choose') }}
              </RouterLink>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
