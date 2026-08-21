<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

import Group from '../../components/ui/Group.vue';
import Pane from '../../components/ui/Pane.vue';
import ScreenHeader from '../../components/ui/ScreenHeader.vue';
import type { ThemeChoice } from '../../bindings';
import { LOCALE_NAMES, LOCALES, type Locale } from '../../i18n';
import { useFormat } from '../../lib/format';
import { useUiStore } from '../../stores/ui';

const ui = useUiStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

const themeOptions = computed<{ value: ThemeChoice; label: string }[]>(() => [
  { value: 'dark', label: t('settings.appearance.theme.dark') },
  { value: 'light', label: t('settings.appearance.theme.light') },
  { value: 'system', label: t('settings.appearance.theme.system') },
]);

/** Что сейчас выбрано в Windows — видно только в режиме следования системе. */
const systemNow = computed(() =>
  t('settings.appearance.theme.systemNow', {
    mode: ui.systemDark
      ? t('settings.appearance.theme.dark')
      : t('settings.appearance.theme.light'),
  }),
);

// Предпросмотр на выдуманных, но правдоподобных данных: он показывает
// не оформление, а то, как выбранный язык влияет на числа, даты
// и формы слов. Проверить это иначе можно только запустив четыре сборки.
const PREVIEW_SIZE = 41.7 * 1024 ** 3;
const PREVIEW_COUNT = 3;
const previewDate = new Date(2026, 7, 9, 21, 42);

function onLocale(event: Event): void {
  ui.setLocale((event.target as HTMLSelectElement).value as Locale);
}
</script>

<template>
  <var class="AppearanceView">
    <section class="screen">
      <ScreenHeader>
        <h1 class="t-lg">{{ t('settings.appearance.title') }}</h1>
      </ScreenHeader>

      <div class="screen-body">
        <div class="screen-pad">
          <Group>
            <span class="t-label">{{ t('settings.appearance.theme.label') }}</span>
            <div class="row">
              <div class="seg">
                <button
                  v-for="option in themeOptions"
                  :key="option.value"
                  type="button"
                  :aria-pressed="ui.theme === option.value"
                  @click="ui.setTheme(option.value)"
                >
                  {{ option.label }}
                </button>
              </div>
              <!-- Рядом с кнопкой «Как в системе», а не строкой под всем
                   переключателем: факт относится к этому варианту
                   и должен читаться как его продолжение, а не как общая
                   подпись ко всем трём кнопкам. -->
              <span v-if="ui.theme === 'system'" class="hint">{{ systemNow }}</span>
            </div>
          </Group>

          <Group>
            <label class="t-label" for="locale">
              {{ t('settings.appearance.language.label') }}
            </label>
            <select id="locale" class="input" :value="ui.locale" @change="onLocale">
              <!-- Подписи языков всегда на самом языке: искать свой
                   в чужом переводе неудобно. -->
              <option v-for="code in LOCALES" :key="code" :value="code">
                {{ LOCALE_NAMES[code] }}
              </option>
            </select>
          </Group>

          <Group>
            <span class="t-label">{{ t('settings.appearance.preview.title') }}</span>
            <Pane>
              <div class="pane-head">
                <span class="title">{{ t('settings.appearance.preview.hint') }}</span>
              </div>
              <div class="scroll">
                <div class="scroll-pad">
                  <div class="row">
                    <span class="pill running"><i></i>{{
                      t('settings.appearance.preview.running')
                    }}</span>
                    <span class="pill stopped"><i></i>{{
                      t('settings.appearance.preview.stopped')
                    }}</span>
                  </div>
                  <div class="meta">
                    <span>{{ t('instances.count', PREVIEW_COUNT) }}</span>
                    <span>
                      {{ t('settings.appearance.preview.size') }}:
                      {{ bytes(PREVIEW_SIZE) }}
                    </span>
                    <span>
                      {{ t('settings.appearance.preview.lastRun') }}:
                      {{ moment(previewDate.getTime()) }}
                    </span>
                  </div>
                </div>
              </div>
            </Pane>
          </Group>

          <p class="hint">{{ t('settings.appearance.comfyNote') }}</p>
        </div>
      </div>
    </section>
  </var>
</template>
