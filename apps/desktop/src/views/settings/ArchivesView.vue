<script setup lang="ts">
// История архивов установщика.
//
// Раздел был серым пунктом-заглушкой: команды `archive_history`
// и `forget_archive` существовали с Фазы 1.5, но экрана у них не было,
// и посмотреть, что приложение помнит, было негде.
//
// Список ничего не удаляет с диска: «забыть» убирает запись, а сам
// архив остаётся там, куда его скачали.
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

import PathText from '../../components/PathText.vue';
import { useFormat } from '../../lib/format';
import { useInstallerStore } from '../../stores/installer';

const installer = useInstallerStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

onMounted(() => void installer.loadHistory());

function reveal(path: string): void {
  void revealItemInDir(path);
}
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('settings.section.archives') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <p class="t-sm">{{ t('archives.lead') }}</p>

        <div v-if="installer.history.length" class="paths">
          <div
            v-for="record in installer.history"
            :key="record.path"
            class="path-item"
          >
            <span class="lbl">
              <!-- Имя файла и путь не переводятся никогда, а путь ещё
                   и не сокращается: он переносится по разделителям
                   папок, а не срезается краем списка. -->
              {{ record.label }}
              <span class="hint"><PathText :path="record.path" /></span>
            </span>
            <span class="val">
              <template v-if="record.available">
                {{ bytes(record.sizeBytes) }}
                <template v-if="record.lastUsedAt">
                  · {{ moment(record.lastUsedAt) }}
                </template>
              </template>
              <template v-else>{{ t('install.archive.missing') }}</template>
            </span>
            <span class="acts">
              <button
                type="button"
                class="btn ghost"
                :disabled="!record.available"
                :title="t('common.openFolder')"
                :aria-label="t('common.openFolder')"
                @click="reveal(record.path)"
              >
                <svg class="ico"><use href="#i-folder" /></svg>
              </button>
              <button
                type="button"
                class="btn ghost"
                @click="installer.forget(record.path)"
              >
                {{ t('install.archive.forget') }}
              </button>
            </span>
          </div>
        </div>

        <p v-else class="hint">{{ t('archives.empty') }}</p>
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Третья колонка под действия: у списка «подпись — значение» их нет,
   а здесь строка кликабельна в двух местах. */
.path-item {
  grid-template-columns: minmax(0, 1fr) auto auto;
}

.path-item .lbl {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.path-item .lbl .hint {
  font-family: var(--font-mono);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.acts {
  display: flex;
  gap: var(--space-2);
}
</style>
