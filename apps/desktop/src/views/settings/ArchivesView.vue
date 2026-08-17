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

import EmptyNote from '../../components/EmptyNote.vue';
import OpenFolderButton from '../../components/OpenFolderButton.vue';
import PathText from '../../components/PathText.vue';
import { useFormat } from '../../lib/format';
import { useInstallerStore } from '../../stores/installer';

const installer = useInstallerStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

onMounted(() => void installer.loadHistory());
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('settings.section.archives') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <p class="t-sm">{{ t('archives.lead') }}</p>

        <!-- `with-acts`: у каждой строки есть действия, и сетка у списка
             своя. Модификатор на списке, а не на строке — он один на все
             строки разом. -->
        <div v-if="installer.history.length" class="paths with-acts">
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
              <OpenFolderButton :path="record.path" :disabled="!record.available" />
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

        <EmptyNote v-else>{{ t('archives.empty') }}</EmptyNote>
      </div>
    </div>
  </section>
</template>
