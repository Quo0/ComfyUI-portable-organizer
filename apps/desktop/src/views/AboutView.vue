<script setup lang="ts">
// Где что лежит и что исчезнет при удалении.
//
// Экран отвечает на вопрос, который иначе пришлось бы проверять
// экспериментом: приложение удаляется штатно, но модели и библиотека
// весят сотни гигабайт, и бояться за них пользователь не должен.
//
// Всё содержимое собрано в списки «подпись — путь — папка», а не в поля
// с кнопками: полей было шесть, у каждого своя рамка и своя подпись,
// и экран читался одним полотном одинаковых строк. Блоков теперь четыре,
// и каждый обведён: подпись, список строк, одна строка объяснения.
// Сборки в реестре — отдельный блок, а не список внутри «Останется на
// диске»: так на широкой панели он встаёт в свою пару с «Что приложение
// записало внутрь ваших сборок» в CSS-грид 2×2 (раскладка — в scoped
// стилях внизу файла), а не тонет во втором по счёту блоке.
//
// Цветом уцелевшее не помечено. Зелёный в этой палитре — цвет работающего
// процесса, и на списке папок он обещал бы состояние, которого у папки
// нет вовсе. Границу между «нашим» и «вашим» держат заголовки блоков
// и строка объяснения под каждым списком.
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import Group from '../components/ui/Group.vue';
import KeyValueList from '../components/ui/KeyValueList.vue';
import KeyValueRow from '../components/ui/KeyValueRow.vue';
import ScreenHeader from '../components/ui/ScreenHeader.vue';
import OpenFolderButton from '../components/OpenFolderButton.vue';
import PathText from '../components/PathText.vue';
import UpdatePanel from '../components/UpdatePanel.vue';
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
  <var class="AboutView">
    <section class="screen">
      <!-- Версия — один факт, а не блок: в шапке она рядом с названием
           раздела, а место на экране отдано тому, что о папках. -->
      <ScreenHeader>
        <h1 class="t-lg">{{ t('about.title') }}</h1>
        <span class="lead">
          {{ t('about.version') }} <span class="t-mono">{{ ui.version }}</span>
        </span>
      </ScreenHeader>

      <div class="screen-body">
        <!-- CSS-грид с именованными областями: на широкой панели — 2×2,
             «что исчезнет» в паре с «останется на диске» строкой выше,
             «что дописано в сборки» с «сборками в реестре» строкой ниже;
             на узкой — та же четвёрка одной колонкой, все блоки одной
             ширины, потому что это один грид, а не два независимых
             столбца. Порог зависит от ширины самой панели (`@container`
             на `.screen-pad`), а не окна: рейл сворачивается независимо
             от окна, и ширина панели от окна не зависит один в один. -->
        <div class="screen-pad wide">

          <div class="about-grid">
            <Group class="cell-uninstall">
              <span class="t-label">{{ t('about.uninstall.title') }}</span>
              <!-- Папки две, и удаляются они обе. Показывать одну значило бы
                   оставить вторую сюрпризом. -->
              <KeyValueList with-acts>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.paths.appData') }}
                    <span class="hint">
                      <PathText :path="ui.appDataDir" />
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="ui.appDataDir" :title="ui.appDataDir" />
                  </span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.paths.localData') }}
                    <span class="hint">
                      <PathText :path="ui.appLocalDataDir" />
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="ui.appLocalDataDir" :title="ui.appLocalDataDir" />
                  </span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.uninstall.body') }}</p>
            </Group>

            <Group class="cell-content">
              <span class="t-label">{{ t('about.content.title') }}</span>

              <!-- Ради этого блока экран и существует: сотни гигабайт моделей
                   не должны зависеть от веры в то, что деинсталлятор поступит
                   правильно. Отвечает за это заголовок блока и строка под
                   списком, а не подкраска строк. -->
              <KeyValueList with-acts>
                <KeyValueRow>
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
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">
                    {{ t('about.content.library') }}
                    <span class="hint">
                      <PathText v-if="libraryPath" :path="libraryPath" />
                      <template v-else>{{ t('about.content.notSet') }}</template>
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="libraryPath" :title="libraryPath || undefined" />
                  </span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.content.body') }}</p>
            </Group>

            <!-- Отдельный блок, а не строка среди уцелевшего: тот отвечает
                 на вопрос «где моё», а этот — «что вы тронули в моём».
                 Два вида файлов названы по одному, строками: абзацем они
                 читались как оговорка, которую можно пролистать. -->
            <Group class="cell-written">
              <span class="t-label">{{ t('about.written.title') }}</span>
              <KeyValueList>
                <KeyValueRow>
                  <span class="lbl">{{ t('about.written.yaml') }}</span>
                  <span class="val">{{ t('about.written.yamlWhen') }}</span>
                </KeyValueRow>
                <KeyValueRow>
                  <span class="lbl">{{ t('about.written.workflows') }}</span>
                  <span class="val">{{ t('about.written.workflowsWhen') }}</span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.written.body') }}</p>
            </Group>

            <!-- Сборки своим блоком, а не списком внутри «Останется на диске»:
                 их бывает восемь, и вперемешку с общими папками имена сборок
                 читались бы как ещё два вида общего добра. Отдельный блок
                 заодно встаёт в пару с «Что приложение записало» — оба про то,
                 что приходится на каждую сборку, а не на приложение целиком. -->
            <Group v-if="instances.items.length" class="cell-instances">
              <span class="t-label">{{ t('about.content.instances') }}</span>
              <KeyValueList with-acts>
                <KeyValueRow v-for="instance in instances.items" :key="instance.id">
                  <!-- Имя сборки не переводится и не сокращается, путь
                       под ним — тоже: по нему идут разбираться руками. -->
                  <span class="lbl">
                    {{ instance.name }}
                    <span class="hint">
                      <PathText :path="instance.path" />
                    </span>
                  </span>
                  <span class="acts">
                    <OpenFolderButton :path="instance.path" :disabled="!instance.available" :title="instance.path" />
                  </span>
                </KeyValueRow>
              </KeyValueList>
              <p class="hint">{{ t('about.content.instancesBody') }}</p>
            </Group>
          </div>


          <!-- Обновление — над сеткой и во всю ширину, а не пятым блоком
               в ней: остальные четыре про то, где что лежит, а этот
               про действие, и парой ни к одному из них он не встаёт. -->
          <UpdatePanel />
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>
/* Запрос смотрит на ширину самой панели (после её max-width), а не окна:
   рейл сворачивается независимо от окна, и от окна ширина панели
   не зависит один в один. Тот же приём — у мастер-детейла в shell.css,
   `.screen-body:has(> .split-master)`. Контейнер и сам грид — разные
   элементы: грид не может зависеть от собственной ширины. */
.screen-pad {
  container-type: inline-size;
}

/* Именованные области, а не auto-fit: нужна не любая раскладка в две
   колонки, а конкретная пара в каждой строке — «что исчезнет» рядом
   с «останется на диске», «что дописано в сборки» рядом со «сборками
   в реестре». Один грид, а не два независимых столбца, — поэтому
   на обычной панели, где остаётся одна колонка, все четыре блока
   получают одну и ту же ширину без отдельного правила. */
.about-grid {
  display: grid;
  grid-template-columns: 1fr;
  grid-template-areas:
    'uninstall'
    'content'
    'written'
    'instances';
  gap: var(--space-4);
}

.cell-uninstall {
  grid-area: uninstall;
}

.cell-content {
  grid-area: content;
}

.cell-written {
  grid-area: written;
}

.cell-instances {
  grid-area: instances;
}

/* Порог тот же, что у авто-грида `.cols` на экране инстанса: две колонки
   по 480px и промежуток между ними. Ниже него в паре не остаётся места
   на путь и кнопку, и разметка возвращается к одной колонке. */
@container (min-width: 980px) {
  .about-grid {
    grid-template-columns: 1fr 1fr;
    grid-template-areas:
      'uninstall content'
      'written instances';
  }
}
</style>
