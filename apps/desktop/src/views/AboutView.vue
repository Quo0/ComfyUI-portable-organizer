<script setup lang="ts">
// Где что лежит и что исчезнет при удалении.
//
// Экран отвечает на вопрос, который иначе пришлось бы проверять
// экспериментом: приложение удаляется штатно, но модели и библиотека
// весят сотни гигабайт, и бояться за них пользователь не должен.
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

import { useInstancesStore } from '../stores/instances';
import { useSharedStore } from '../stores/shared';
import { useWorkflowsStore } from '../stores/workflows';
import { useUiStore } from '../stores/ui';

const ui = useUiStore();
const instances = useInstancesStore();
const shared = useSharedStore();
const workflows = useWorkflowsStore();
const { t } = useI18n();

/** Общий корень моделей: первый из настроенных, MVP их и заводит один. */
const sharedRoot = computed(() => shared.root?.path ?? '');
const libraryPath = computed(() => workflows.path);

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  if (!shared.loaded) await shared.load();
  if (!workflows.loaded) await workflows.load();
});

// Путь не переводится и не сокращается: по нему пользователь идёт
// разбираться руками, и обрезанный путь бесполезен.
function open(path: string): void {
  if (path) void revealItemInDir(path);
}
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('about.title') }}</h1>
    </header>

    <div class="screen-body">
      <!-- Две колонки по смыслу: слева наше, справа ваше. Одним столбцом
           это была простыня одинаковых полей, в которой не видно границы
           между тем, что приложение вправе удалить, и тем, что не вправе. -->
      <div class="screen-pad wide cols">
        <div>
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
              <button type="button" class="btn secondary" @click="open(ui.appDataDir)">
                {{ t('common.openFolder') }}
              </button>
            </div>
          </div>

          <!-- Папки две, и удаляются они обе. Показывать одну значило бы
               оставить вторую сюрпризом. -->
          <div class="field">
            <label>{{ t('about.paths.localData') }}</label>
            <div class="path-row">
              <div class="input mono"><span>{{ ui.appLocalDataDir }}</span></div>
              <button
                type="button"
                class="btn secondary"
                @click="open(ui.appLocalDataDir)"
              >
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

        <div>
        <div class="group">
          <span class="t-label">{{ t('about.content.title') }}</span>
          <p class="t-sm">{{ t('about.content.body') }}</p>

          <div class="field">
            <label>{{ t('about.content.shared') }}</label>
            <div class="path-row">
              <div class="input mono">
                <span>{{ sharedRoot || t('about.content.notSet') }}</span>
              </div>
              <button
                type="button"
                class="btn secondary"
                :disabled="!sharedRoot"
                @click="open(sharedRoot)"
              >
                {{ t('common.openFolder') }}
              </button>
            </div>
          </div>

          <div class="field">
            <label>{{ t('about.content.library') }}</label>
            <div class="path-row">
              <div class="input mono">
                <span>{{ libraryPath || t('about.content.notSet') }}</span>
              </div>
              <button
                type="button"
                class="btn secondary"
                :disabled="!libraryPath"
                @click="open(libraryPath)"
              >
                {{ t('common.openFolder') }}
              </button>
            </div>
          </div>

          <div class="field">
            <label>{{ t('about.content.instances') }}</label>
            <div v-for="instance in instances.items" :key="instance.id" class="path-row">
              <div class="input mono"><span>{{ instance.path }}</span></div>
              <button
                type="button"
                class="btn secondary"
                :disabled="!instance.available"
                @click="open(instance.path)"
              >
                {{ t('common.openFolder') }}
              </button>
            </div>
          </div>
        </div>

        <div class="group">
          <span class="t-label">{{ t('about.written.title') }}</span>
          <p class="t-sm">{{ t('about.written.body') }}</p>
        </div>
        </div>
      </div>
    </div>
  </section>
</template>
