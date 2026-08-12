<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';

import { commands } from '../../bindings';
import { useWorkflowsStore } from '../../stores/workflows';

const library = useWorkflowsStore();
const { t } = useI18n();

/** Подсказка «положить рядом с общими моделями». Может и не быть. */
const suggested = ref<string | null>(null);

onMounted(async () => {
  if (!library.loaded) await library.load();
  const res = await commands.suggestLibraryPath();
  if (res.status === 'ok') suggested.value = res.data;
});

async function pick(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked === 'string') await library.setPath(picked);
}
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('library.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <div class="field">
          <label class="t-label" for="library-path">{{ t('library.path.label') }}</label>
          <div class="path-row">
            <div id="library-path" class="input mono">
              <span>{{ library.path || t('library.path.empty') }}</span>
            </div>
            <button class="btn secondary" type="button" @click="pick">
              {{ t('common.browse') }}
            </button>
          </div>

          <div v-if="library.scanning" class="bar indet"><i></i></div>

          <p v-else-if="!library.configured" class="hint">{{ t('library.path.howto') }}</p>

          <!-- Недоступная папка не выводит приложение из строя: говорим
               и продолжаем работать, остальные разделы не затронуты. -->
          <p v-else-if="!library.available" class="hint bad">
            {{ t('library.path.unavailable') }}
          </p>

          <p v-else class="hint">
            {{ t('library.summary', library.items.length) }}
          </p>

          <!-- Повреждённые теги не имеют права уносить сами воркфлоу:
               файлы на месте, и об этом надо сказать прямо. -->
          <p v-if="library.scan?.manifestBroken" class="hint bad">
            {{ t('library.manifestBroken') }}
          </p>
        </div>

        <!-- Библиотека независима от общих моделей, но лежать рядом с ними
             удобно: они обычно на просторном диске. Подсказка, не правило. -->
        <div v-if="!library.configured && suggested" class="group">
          <p class="hint">{{ t('library.path.suggested') }}</p>
          <div class="path-row">
            <div class="input mono"><span>{{ suggested }}</span></div>
            <button class="btn secondary" type="button" @click="library.setPath(suggested)">
              {{ t('library.path.useSuggested') }}
            </button>
          </div>
        </div>

        <p class="hint">{{ t('library.path.portable') }}</p>
      </div>
    </div>
  </section>
</template>
