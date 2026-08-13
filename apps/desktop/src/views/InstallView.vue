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
      <div class="screen-pad wide">
        <p class="t-sm">{{ t('install.lead') }}</p>

        <!-- Две дорожки рядом, а не одна под другой: это развилка, и обе
             ветки должны быть видны разом, чтобы выбирать было из чего. -->
        <div class="forks">
          <div class="fork">
            <b>{{ t('install.existing.title') }}</b>
            <p>{{ t('install.existing.body') }}</p>
            <div class="row">
              <RouterLink class="btn secondary" to="/instances/add">
                {{ t('install.existing.action') }}
              </RouterLink>
            </div>
          </div>

          <div class="fork">
            <b>{{ t('install.wizard.title') }}</b>
            <p>{{ t('install.wizard.body') }}</p>
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
