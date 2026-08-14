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
        <!-- Две дорожки рядом, а не одна под другой: это развилка, и обе
             ветки должны быть видны разом, чтобы выбирать было из чего.
             Подписи внутри карточек объясняют выбор целиком — вводного
             абзаца над ними не нужно. -->
        <div class="forks">
          <!-- Кнопка — прямой ребёнок дорожки: описание растянуто
               (`.fork p { flex: 1 }`), и кнопка сама встаёт к низу.
               Обёртка `.row` добавляла между ними лишний зазор,
               и в соседних дорожках кнопки вставали на разной высоте. -->
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
      </div>
    </div>
  </section>
</template>
