<script setup lang="ts">
// Где что лежит и что исчезнет при удалении.
//
// Экран отвечает на вопрос, который иначе пришлось бы проверять
// экспериментом: приложение удаляется штатно, но модели и библиотека
// весят сотни гигабайт, и бояться за них пользователь не должен.
//
// Всё содержимое собрано в списки «подпись — путь — папка», а не в поля
// с кнопками: полей было шесть, у каждого своя рамка и своя подпись,
// и экран читался одним полотном одинаковых строк. Блоков теперь три,
// и каждый обведён: подпись, список строк, одна строка объяснения.
//
// Цветом уцелевшее не помечено. Зелёный в этой палитре — цвет работающего
// процесса, и на списке папок он обещал бы состояние, которого у папки
// нет вовсе. Границу между «нашим» и «вашим» держат заголовки блоков
// и строка объяснения под каждым списком.
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import OpenFolderButton from '../components/OpenFolderButton.vue';
import PathText from '../components/PathText.vue';
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
</script>

<template>
  <section class="screen">
    <!-- Версия — один факт, а не блок: в шапке она рядом с названием
         раздела, а место на экране отдано тому, что о папках. -->
    <header class="screen-head">
      <h1 class="t-lg">{{ t('about.title') }}</h1>
      <span class="lead">
        {{ t('about.version') }} <span class="t-mono">{{ ui.version }}</span>
      </span>
    </header>

    <div class="screen-body">
      <!-- Две колонки по смыслу: слева наше, справа ваше. Одним столбцом
           это была простыня одинаковых полей, в которой не видно границы
           между тем, что приложение вправе удалить, и тем, что не вправе. -->
      <div class="screen-pad wide cols">
        <div>
          <div class="group">
            <span class="t-label">{{ t('about.uninstall.title') }}</span>
            <!-- Папки две, и удаляются они обе. Показывать одну значило бы
                 оставить вторую сюрпризом. -->
            <div class="paths with-acts">
              <div class="path-item">
                <span class="lbl">
                  {{ t('about.paths.appData') }}
                  <span class="hint"><PathText :path="ui.appDataDir" /></span>
                </span>
                <span class="acts">
                  <OpenFolderButton :path="ui.appDataDir" :title="ui.appDataDir" />
                </span>
              </div>
              <div class="path-item">
                <span class="lbl">
                  {{ t('about.paths.localData') }}
                  <span class="hint"><PathText :path="ui.appLocalDataDir" /></span>
                </span>
                <span class="acts">
                  <OpenFolderButton
                    :path="ui.appLocalDataDir"
                    :title="ui.appLocalDataDir"
                  />
                </span>
              </div>
            </div>
            <p class="hint">{{ t('about.uninstall.body') }}</p>
          </div>

          <!-- Отдельный блок, а не строка среди уцелевшего: тот отвечает
               на вопрос «где моё», а этот — «что вы тронули в моём».
               Два вида файлов названы по одному, строками: абзацем они
               читались как оговорка, которую можно пролистать. -->
          <div class="group">
            <span class="t-label">{{ t('about.written.title') }}</span>
            <div class="paths">
              <div class="path-item">
                <span class="lbl">{{ t('about.written.yaml') }}</span>
                <span class="val">{{ t('about.written.yamlWhen') }}</span>
              </div>
              <div class="path-item">
                <span class="lbl">{{ t('about.written.workflows') }}</span>
                <span class="val">{{ t('about.written.workflowsWhen') }}</span>
              </div>
            </div>
            <p class="hint">{{ t('about.written.body') }}</p>
          </div>
        </div>

        <div>
          <div class="group">
            <span class="t-label">{{ t('about.content.title') }}</span>

            <!-- Ради этого блока экран и существует: сотни гигабайт моделей
                 не должны зависеть от веры в то, что деинсталлятор поступит
                 правильно. Отвечает за это заголовок блока и строка под
                 списком, а не подкраска строк. -->
            <div class="paths with-acts">
              <div class="path-item">
                <span class="lbl">
                  {{ t('about.content.shared') }}
                  <span class="hint">
                    <PathText v-if="sharedRoot" :path="sharedRoot" />
                    <template v-else>{{ t('about.content.notSet') }}</template>
                  </span>
                </span>
                <span class="acts">
                  <OpenFolderButton :path="sharedRoot" :title="sharedRoot || undefined" />
                </span>
              </div>
              <div class="path-item">
                <span class="lbl">
                  {{ t('about.content.library') }}
                  <span class="hint">
                    <PathText v-if="libraryPath" :path="libraryPath" />
                    <template v-else>{{ t('about.content.notSet') }}</template>
                  </span>
                </span>
                <span class="acts">
                  <OpenFolderButton
                    :path="libraryPath"
                    :title="libraryPath || undefined"
                  />
                </span>
              </div>
            </div>

            <!-- Сборки отдельным списком под своей подписью: их бывает
                 восемь, и вперемешку с общими папками имена сборок
                 читались бы как ещё два вида общего добра. -->
            <template v-if="instances.items.length">
              <span class="t-label">{{ t('about.content.instances') }}</span>
              <div class="paths with-acts">
                <div
                  v-for="instance in instances.items"
                  :key="instance.id"
                  class="path-item"
                >
                  <!-- Имя сборки не переводится и не сокращается, путь
                       под ним — тоже: по нему идут разбираться руками. -->
                  <span class="lbl">
                    {{ instance.name }}
                    <span class="hint"><PathText :path="instance.path" /></span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton
                      :path="instance.path"
                      :disabled="!instance.available"
                      :title="instance.path"
                    />
                  </span>
                </div>
              </div>
            </template>

            <p class="hint">{{ t('about.content.body') }}</p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
