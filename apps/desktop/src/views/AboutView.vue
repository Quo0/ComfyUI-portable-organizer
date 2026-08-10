<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const { t } = useI18n();

// Путь не переводится и не сокращается: по нему пользователь идёт
// разбираться руками, и обрезанный путь бесполезен.
function openDataDir(): void {
  if (ui.appDataDir) void revealItemInDir(ui.appDataDir);
}
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('about.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <div class="group">
          <span class="t-label">{{ t('about.version') }}</span>
          <span class="t-mono">{{ ui.version }}</span>
        </div>

        <div class="group">
          <span class="t-label">{{ t('about.paths.title') }}</span>
          <div class="field">
            <label>{{ t('about.paths.appData') }}</label>
            <div class="path-row">
              <div class="input mono"><span>{{ ui.appDataDir }}</span></div>
              <button type="button" class="btn secondary" @click="openDataDir">
                {{ t('common.openFolder') }}
              </button>
            </div>
          </div>
        </div>

        <div class="group">
          <span class="t-label">{{ t('about.uninstall.title') }}</span>
          <p class="t-sm">{{ t('about.uninstall.body') }}</p>
        </div>
      </div>
    </div>
  </section>
</template>
