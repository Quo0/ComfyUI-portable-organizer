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
import KeyValueList from '../../components/ui/KeyValueList.vue';
import KeyValueRow from '../../components/ui/KeyValueRow.vue';
import ScreenHeader from '../../components/ui/ScreenHeader.vue';
import { useFormat } from '../../lib/format';
import { useInstallerStore } from '../../stores/installer';

const installer = useInstallerStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

onMounted(() => void installer.loadHistory());
</script>

<template>
  <var class="ArchivesView">
    <section class="screen">
      <ScreenHeader>
        <h1 class="t-lg">{{ t('settings.section.archives') }}</h1>
      </ScreenHeader>

      <div class="screen-body">
        <div class="screen-pad">
          <p class="t-sm">{{ t('archives.lead') }}</p>

          <KeyValueList v-if="installer.history.length" with-acts>
            <KeyValueRow v-for="record in installer.history" :key="record.path">
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
            </KeyValueRow>
          </KeyValueList>

          <EmptyNote v-else>{{ t('archives.empty') }}</EmptyNote>
        </div>
      </div>
    </section>
  </var>
</template>
