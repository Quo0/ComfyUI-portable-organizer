<script setup lang="ts">
// Развилка «папка уже есть или распаковать архив».
//
// Компонент, а не два куска вёрстки, потому что показывается в двух местах:
// на экране «Добавление» и в пустом реестре, который играет роль
// приветственного экрана. Пока дорожки были описаны по месту, одна и та же
// развилка разъехалась на три формулировки — в макетах, в реестре
// и в установке.
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<template>
  <!-- Две дорожки рядом, а не одна под другой: это развилка, и обе ветки
       должны быть видны разом, чтобы выбирать было из чего. Подписи внутри
       карточек объясняют выбор целиком — вводного абзаца над ними не нужно. -->
  <div class="forks">
    <!-- Кнопка — прямой ребёнок дорожки: описание растянуто
         (`.fork p { flex: 1 }`), и кнопка сама встаёт к низу. Обёртка `.row`
         добавляла между ними лишний зазор, и в соседних дорожках кнопки
         вставали на разной высоте. -->
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
</template>
