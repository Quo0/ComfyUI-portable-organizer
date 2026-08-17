<script setup lang="ts">
// Мастер установки — отдельный роут с шагами внутри, а не череда модалок.
// Шаги: архив → назначения → общие ресурсы → выполнение → итог.
// Шаг общих ресурсов собран из тех же компонентов, что экран настроек,
// и пользуется тем же стором: логика подключения не дублируется.
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';

import EmptyNote from '../components/EmptyNote.vue';
import PathPicker from '../components/PathPicker.vue';
import PathText from '../components/PathText.vue';
import StatusPill from '../components/StatusPill.vue';
import TargetForm from '../components/TargetForm.vue';
import type { ArchiveRecord } from '../bindings';
import { errorText } from '../lib/errors';
import { accentVar, initial, useFormat } from '../lib/format';
import { displayStatus } from '../lib/status';
import type { WizardStep } from '../stores/installer';
import { useInstallerStore } from '../stores/installer';
import { useRunStore } from '../stores/run';
import { useSharedStore } from '../stores/shared';
import { useWorkflowsStore } from '../stores/workflows';

const wizard = useInstallerStore();
const run = useRunStore();
const shared = useSharedStore();
const library = useWorkflowsStore();
const { t, n } = useI18n();
const { bytes, moment } = useFormat();

/**
 * Количество файлов через форматтер локали.
 *
 * В сообщение о числе файлов подставлялось сырое число, и на экране
 * стояло «56128 файлов» — пять цифр подряд без разделителя разрядов
 * не читаются. Формат `integer` для этого уже заведён в `i18n/index.ts`,
 * и там же записано, что числа идут только через `n()`.
 */
const count = (value: number): string => n(value, 'integer');

/** Порядок шагов. Он же порядок показа в полосе состояния мастера. */
const STEPS: WizardStep[] = ['archive', 'targets', 'shared', 'running', 'done'];
const current = computed(() => STEPS.indexOf(wizard.step));

/**
 * Название текущего шага для ряда с кнопками перехода.
 *
 * У трёх шагов оно своё и говорит о деле («Куда распаковать»), у двух
 * совпадает с названием в полосе шагов — придумывать им второе имя
 * значило бы говорить об одном и том же двумя словами. Запасного
 * варианта тут быть не должно: раньше он подписывал первый шаг
 * «Распаковкой», и не всплыло это лишь потому, что на первом шаге
 * ряда шага не было вовсе.
 */
const stepTitle = computed(() => {
  switch (wizard.step) {
    case 'archive':
      return t('install.wizard.step.archive');
    case 'targets':
      return t('install.targets.title');
    case 'shared':
      return t('install.shared.title');
    case 'done':
      return t('install.done.title');
    default:
      return t('install.wizard.step.running');
  }
});

/** Сколько назначений уже пройдено — для счётчика в ряду шага. */
const runCounter = computed(() => {
  const p = wizard.progress;
  if (!p?.targets) return '';
  return t('install.run.counter', { done: p.target, total: p.targets });
});

/**
 * Состояние назначения на шаге распаковки.
 *
 * Считается из индекса: `progress.target` — номер текущего, начиная
 * с единицы. Всё до него сделано, всё после ждёт очереди. Отдельного
 * события на каждое назначение Rust не шлёт, да и незачем — порядок
 * известен заранее.
 */
type RunState = 'done' | 'now' | 'queued';

function runStateOf(index: number): RunState {
  const at = wizard.progress?.target ?? 1;
  if (index + 1 < at) return 'done';
  if (index + 1 > at) return 'queued';
  return 'now';
}

onMounted(() => {
  if (!wizard.info) void wizard.loadHistory();
  if (!shared.loaded) void shared.load();
  if (!library.loaded) void library.load();
});

/**
 * Полоса идёт по файлам, а не по байтам.
 *
 * Хвост сборки — это `site-packages` с десятками тысяч файлов по паре
 * килобайт: на отметке 98% байт сделано меньше половины файлов, и полоса
 * замирает ровно там, где работы остаётся больше всего. Время уходит
 * не на байты, а на создание файлов и проверку каждого антивирусом.
 */
const percent = computed(() => {
  const p = wizard.progress;
  if (!p?.totalFiles) return 0;
  return Math.min(100, (p.doneFiles / p.totalFiles) * 100);
});

/**
 * Подпись к текущей фазе.
 *
 * Прежде здесь стоял запасной вариант «Регистрация» на случай отсутствия
 * прогресса — и именно он показывался всю подготовку, то есть врал. Теперь
 * о подготовке сообщает Rust отдельным событием, и запасной вариант нужен
 * только на мгновение до первого события.
 */
const phaseText = computed(() => {
  const p = wizard.progress;
  if (!p) return t('install.run.preparing');
  const name = p.targetName;
  switch (p.phase) {
    case 'preparing':
      return t('install.run.preparing');
    case 'cleaning':
      return t('install.run.cleaning');
    case 'registering':
      return t('install.run.registering');
    case 'copying':
      return t('install.run.copying', { name });
    default:
      return t('install.run.extracting', { name });
  }
});

/**
 * Фазы без доли выполненного. Показывать нечего, кроме того, что работа
 * идёт, — полоса бежит вместо того, чтобы стоять на нуле.
 */
const indeterminate = computed(() => {
  const phase = wizard.progress?.phase;
  return !phase || phase === 'preparing' || phase === 'cleaning' || phase === 'registering';
});

async function pickArchive(): Promise<void> {
  const picked = await open({
    multiple: false,
    filters: [{ name: '7z', extensions: ['7z'] }],
  });
  if (typeof picked !== 'string') return;
  if (await wizard.chooseArchive(picked)) wizard.step = 'targets';
}

async function useRecent(record: ArchiveRecord): Promise<void> {
  if (!record.available) return;
  if (await wizard.chooseArchive(record.path)) wizard.step = 'targets';
}

/** Сколько места нужно с учётом того, что цели могут быть на одном диске. */
const needed = computed(() =>
  bytes((wizard.info?.totalUncompressed ?? 0) * wizard.targets.length),
);
</script>

<template>
  <section class="screen wizard-screen">
    <!-- Шапки экрана здесь нет намеренно: её «Назад» и заголовок
         повторяли то, что и так стоит в ряду шага, — два выхода
         и два заголовка на одном экране. -->

    <!-- Шаги видны целиком: мастер, который не говорит, сколько ещё
         впереди, читается как бесконечный. -->
    <nav class="steps">
      <template v-for="(name, index) in STEPS" :key="name">
        <span v-if="index" class="step-sep"></span>
        <span
          class="step"
          :class="{ now: name === wizard.step, done: index < current }"
        >
          <u>{{ index < current ? '✓' : index + 1 }}</u>
          {{ t(`install.wizard.step.${name}`) }}
        </span>
      </template>
    </nav>

    <!-- Ряд шага: название слева, переходы справа. Кнопки стоят здесь,
         а не в подвале и не в области прокрутки: при нескольких
         назначениях «Дальше» уезжала из виду ровно тогда, когда
         становилась нужна. «Назад» всегда левее кнопки действия. -->
    <div class="step-bar">
      <h2 class="title">{{ stepTitle }}</h2>
      <span v-if="wizard.step === 'done'" class="t-label">
        {{ t('install.done.added', wizard.created.length) }}
      </span>
      <!-- Какое назначение идёт из скольких: без счётчика при шести целях
           непонятно, распаковка в разгаре или почти закончилась. -->
      <span v-else-if="wizard.step === 'running' && runCounter" class="t-label">
        {{ runCounter }}
      </span>
      <span class="spacer"></span>

      <!-- На первом шаге «Назад» выводит из мастера, а не переключает
           шаг, — поэтому ссылка, а не кнопка. Состояние мастера при этом
           не теряется: стор живёт отдельно от экрана ровно затем, чтобы
           уход и возврат ничего не стирали. -->
      <span v-if="wizard.step === 'archive'" class="acts">
        <RouterLink class="btn ghost" to="/install">
          <svg class="ico"><use href="#i-back" /></svg>
          {{ t('common.back') }}
        </RouterLink>
      </span>

      <span v-else-if="wizard.step === 'targets'" class="acts">
        <button type="button" class="btn ghost" @click="wizard.step = 'archive'">
          <svg class="ico"><use href="#i-back" /></svg>
          {{ t('common.back') }}
        </button>
        <!-- Проверка имени осталась в форме: без имени назначение
             в список не попадает, и здесь её повторять уже нечего. -->
        <button
          type="button"
          class="btn primary lg"
          :disabled="wizard.blocked"
          @click="wizard.step = 'shared'"
        >
          {{ t('install.wizard.next') }}
        </button>
      </span>

      <span v-else-if="wizard.step === 'shared'" class="acts">
        <button type="button" class="btn ghost" @click="wizard.step = 'targets'">
          <svg class="ico"><use href="#i-back" /></svg>
          {{ t('common.back') }}
        </button>
        <button type="button" class="btn primary lg" @click="wizard.start()">
          {{ t('install.run.start') }}
        </button>
      </span>

      <span v-else-if="wizard.step === 'running'" class="acts">
        <button type="button" class="btn danger" @click="wizard.cancel()">
          {{ t('install.run.cancel') }}
        </button>
      </span>

      <span v-else class="acts">
        <button type="button" class="btn ghost" @click="wizard.reset()">
          {{ t('install.done.again') }}
        </button>
        <RouterLink class="btn primary lg" to="/instances">
          {{ t('install.done.toInstances') }}
        </RouterLink>
      </span>
    </div>

    <!-- Сводка по архиву закреплена под рядом шага: она нужна, пока
         выбирают назначения, и уезжать вместе с ними не должна. -->
    <div v-if="wizard.step === 'targets' && wizard.info" class="pinned">
      <div class="meta">
        <span>{{ wizard.info.label }}</span>
        <!-- Число форматируется отдельно: множественное число выбирается
             по количеству, а печатается уже готовая строка с разделителем
             разрядов. -->
        <span>
          {{ t('install.archive.files', { n: count(wizard.info.files) }, wizard.info.files) }}
        </span>
        <span>
          {{ t('install.archive.unpacked', {
            size: bytes(wizard.info.totalUncompressed),
          }) }}
        </span>
        <span>{{ t('install.targets.needed', { size: needed }) }}</span>
      </div>
      <p v-if="wizard.info.singleRoot" class="hint">
        {{ t('install.archive.root', { name: wizard.info.singleRoot }) }}
      </p>
      <p v-else class="hint">{{ t('install.archive.noRoot') }}</p>
    </div>

    <div class="screen-body">
      <div class="screen-pad wide">
        <!-- ------------------------------------------------ шаг «архив» -->
        <template v-if="wizard.step === 'archive'">
          <!-- Сказано сразу: кнопки «скачать» здесь не будет и не появится.
               Без этой строки первое, что ищут на экране, — именно она. -->
          <p class="t-sm">{{ t('install.archive.lead') }}</p>

          <!-- Разбор оглавления на 56 тысяч записей занимает больше секунды,
               и подпись о нём стоит рядом с кнопкой, которую только что
               нажали: отдельным блоком ниже она выглядела ответом
               на что-то другое. -->
          <div class="row">
            <button
              type="button"
              class="btn primary"
              :disabled="wizard.reading"
              @click="pickArchive"
            >
              {{ t('install.archive.choose') }}
            </button>
            <template v-if="wizard.reading">
              <span class="hint">{{ t('install.archive.reading') }}</span>
              <!-- Полоса тянется до края блока: короткий индикатор рядом
                   с кнопкой читается как значок, а не как «идёт работа». -->
              <span class="bar indet grow"><i></i></span>
            </template>
          </div>

          <div v-if="wizard.history.length" class="group">
            <span class="t-label">{{ t('install.archive.history') }}</span>
            <!-- Сетка, а не колонка: карточка архива с полным путём
                 занимает всю ширину широкого экрана, и четыре архива
                 давали экран прокрутки на пустом месте. Список сборок
                 в такой же ситуации давно в сетке. -->
            <div class="cards grid">
              <div
                v-for="record in wizard.history"
                :key="record.path"
                class="card"
                :class="{ gone: !record.available }"
              >
                <div class="card-accent"></div>
                <div class="card-in">
                  <div class="card-top">
                    <div class="card-name">{{ record.label }}</div>
                    <span v-if="!record.available" class="pill gone">
                      {{ t('install.archive.missing') }}
                    </span>
                  </div>
                  <!-- Путь не переводится и не сокращается: переносится
                       по разделителям папок, а не срезается краем. -->
                  <div class="src"><code><PathText :path="record.path" /></code></div>
                  <div class="meta">
                    <span>{{ bytes(record.sizeBytes) }}</span>
                    <span>{{ moment(record.lastUsedAt) }}</span>
                  </div>
                  <div class="row">
                    <button
                      type="button"
                      class="btn secondary"
                      :disabled="!record.available"
                      @click="useRecent(record)"
                    >
                      {{ t('install.wizard.next') }}
                    </button>
                    <button
                      type="button"
                      class="btn ghost"
                      @click="wizard.forget(record.path)"
                    >
                      {{ t('install.archive.forget') }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>

        <!-- --------------------------------------------- шаг «назначения» -->
        <!-- Форма слева накидывает назначения в список справа. Панель
             со всеми полями на каждую цель разом занимала по экрану
             прокрутки на цель, а целей бывает шесть. -->
        <template v-else-if="wizard.step === 'targets' && wizard.info">
          <div class="cols targets">
            <!-- Кнопка не выключается, а проверяет: выключенная, она ничего
                 не говорит о том, чего не хватает, и нажатие на пустой
                 форме выглядит поломкой. -->
            <TargetForm
              v-model="wizard.draft"
              :title="t('install.targets.form')"
              :check="wizard.draftCheck"
              :show-problems="wizard.draftProblems"
              id-prefix="target-new"
              @change="wizard.recheck()"
            >
              <template #acts>
                <button type="button" class="btn primary" @click="wizard.addDraft()">
                  {{ t('install.targets.add') }}
                </button>
              </template>
            </TargetForm>

            <div class="field">
              <span class="t-label">{{ t('install.targets.list') }}</span>
              <div v-if="wizard.targets.length" class="paths">
                <template v-for="(target, index) in wizard.targets" :key="index">
                  <!-- Описание в строку не помещается и в неё не лезет,
                       но и пропасть не должно: оно всплывает подсказкой.
                       Атрибута нет вовсе, когда описания нет, — пустая
                       подсказка мигала бы рамкой ни о чём. -->
                  <div
                    class="path-item editable"
                    :title="target.description || undefined"
                  >
                    <!-- Путь не переводится и не сокращается. -->
                    <span class="lbl"><PathText :path="target.path" /></span>
                    <span class="val">{{ target.name }}</span>
                    <!-- Цвет виден прямо в строке: две сборки с похожими
                         путями различают по нему, а не по пути. Стоит между
                         именем и кнопками — там он выстраивается в колонку,
                         а не гуляет вслед за длиной имени. -->
                    <span
                      class="chip sm"
                      :style="{ '--instance-accent': accentVar(target.accent) }"
                    ></span>
                    <span class="acts">
                      <button
                        type="button"
                        class="act"
                        :title="t('common.edit')"
                        :aria-label="t('common.edit')"
                        :aria-pressed="wizard.editIndex === index"
                        @click="
                          wizard.editIndex === index
                            ? wizard.cancelEdit()
                            : wizard.startEdit(index)
                        "
                      >
                        <svg class="ico"><use href="#i-edit" /></svg>
                      </button>
                      <button
                        type="button"
                        class="act"
                        :title="t('install.targets.remove')"
                        :aria-label="t('install.targets.remove')"
                        @click="wizard.removeTarget(index)"
                      >
                        <svg class="ico"><use href="#i-close" /></svg>
                      </button>
                    </span>
                  </div>

                  <!-- Правка разворачивается под своей строкой, а не уводит
                       в отдельный экран: список остаётся на месте, и видно,
                       что именно правится. -->
                  <TargetForm
                    v-if="wizard.editIndex === index && wizard.editDraft"
                    v-model="wizard.editDraft"
                    :title="t('install.targets.editing')"
                    :check="wizard.editCheck"
                    id-prefix="target-edit"
                    @change="wizard.recheck()"
                  >
                    <template #acts>
                      <span class="acts">
                        <!-- Перечёркнутый круг, а не крестик: крестиком
                             рядом убирают строку, и одинаковые значки
                             в одном месте значили бы разное. -->
                        <button
                          type="button"
                          class="act"
                          :title="t('common.cancel')"
                          :aria-label="t('common.cancel')"
                          @click="wizard.cancelEdit()"
                        >
                          <svg class="ico"><use href="#i-ban" /></svg>
                        </button>
                        <button
                          type="button"
                          class="act"
                          :title="t('common.save')"
                          :aria-label="t('common.save')"
                          :disabled="!wizard.editReady"
                          @click="wizard.saveEdit()"
                        >
                          <svg class="ico"><use href="#i-check" /></svg>
                        </button>
                      </span>
                    </template>
                  </TargetForm>
                </template>
              </div>
              <EmptyNote v-else>{{ t('install.targets.empty') }}</EmptyNote>

              <!-- Ошибка на уже добавленном назначении: место на диске
                   могло кончиться после того, как его добавили. -->
              <template v-for="(check, index) in wizard.checks" :key="`c${index}`">
                <p
                  v-for="(problem, i) in check.errors"
                  :key="`ce${index}-${i}`"
                  class="hint bad"
                >
                  {{ errorText(problem) }}
                </p>
              </template>
            </div>
          </div>
        </template>

        <!-- ---------------------------------------- шаг «общие ресурсы» -->
        <template v-else-if="wizard.step === 'shared'">
          <!-- Ресурса два, и они про разное: модели и воркфлоу. Раньше всё
               шло одним списком полей, и понять, к чему относится очередной
               путь, можно было только по подписи. -->
          <div class="pane">
          <div class="pane-head">
            <span class="title">{{ t('install.shared.models') }}</span>
          </div>
          <div class="scroll-pad">
          <div class="field">
            <span class="t-label">{{ t('shared.root.label') }}</span>
            <!-- Тот же выбор, что в настройках: US-SHARED-01/AC-4 требует
                 не выгонять пользователя из мастера ради одной папки. -->
            <PathPicker
              :path="shared.root?.path"
              :empty="t('shared.root.empty')"
              @pick="shared.setRoot($event)"
            />
            <div v-if="shared.scanning" class="bar indet"><i></i></div>
            <p v-else-if="!shared.configured" class="hint">{{ t('shared.root.howto') }}</p>
            <p v-else-if="!shared.available" class="hint">
              {{ t('shared.root.unavailable') }}
            </p>
            <p v-else class="hint">
              {{ t('shared.summary.categories', shared.recognized.length) }} ·
              {{ bytes(shared.scan?.totalBytes) }}
            </p>
          </div>

          <div class="toggle-row">
            <button
              class="toggle"
              :class="{ off: !wizard.connectShared }"
              type="button"
              role="switch"
              :aria-checked="wizard.connectShared"
              :disabled="!shared.configured"
              @click="wizard.connectShared = !wizard.connectShared"
            ></button>
            <div>
              <div class="t-base">
                {{ t('install.shared.connect', wizard.targets.length) }}
              </div>
              <div class="hint">{{ t('shared.default.hint') }}</div>
            </div>
          </div>

          <div v-if="wizard.connectShared" class="group">
            <span class="t-label">{{ t('shared.mode.label') }}</span>
            <div class="seg">
              <button
                type="button"
                :aria-pressed="wizard.sharedMode === 'flag'"
                @click="wizard.sharedMode = 'flag'"
              >
                {{ t('shared.mode.flag.title') }}
              </button>
              <button
                type="button"
                :aria-pressed="wizard.sharedMode === 'instanceFile'"
                @click="wizard.sharedMode = 'instanceFile'"
              >
                {{ t('shared.mode.instanceFile.title') }}
              </button>
            </div>
            <p class="hint">{{ t(`shared.mode.${wizard.sharedMode}.hint`) }}</p>
          </div>
          </div>
          </div>

          <!-- Библиотека воркфлоу — второй общий ресурс, и подключается
               тем же шагом: уводить за ней в настройки посреди установки
               незачем. -->
          <div class="pane">
          <div class="pane-head">
            <span class="title">{{ t('install.shared.workflows') }}</span>
          </div>
          <div class="scroll-pad">
          <div class="field">
            <span class="t-label">{{ t('library.path.label') }}</span>
            <!-- Библиотека независима от общих моделей, но задаётся тут же. -->
            <PathPicker
              :path="library.path"
              :empty="t('library.path.empty')"
              @pick="library.setPath($event)"
            />
            <p v-if="library.configured && library.available" class="hint">
              {{ t('library.summary', library.items.length) }}
            </p>
            <p v-else-if="!library.configured" class="hint">{{ t('library.path.howto') }}</p>
          </div>
          </div>
          </div>

        </template>

        <!-- -------------------------------------------- шаг «выполнение» -->
        <!-- Полоса на каждое назначение, а не одна общая: при шести целях
             общая полоса три раза проходит путь от нуля до ста, и понять,
             сколько работы осталось, по ней невозможно. -->
        <template v-else-if="wizard.step === 'running'">
          <div
            v-for="(target, index) in wizard.targets"
            :key="index"
            class="prog"
          >
            <div class="prog-head">
              <!-- Путь не переводится. -->
              <span>{{ target.path }}</span>
              <span class="count">
                <template v-if="runStateOf(index) === 'done'">
                  {{ t('install.run.finished') }}
                </template>
                <template v-else-if="runStateOf(index) === 'queued'">
                  {{ t('install.run.queued') }}
                </template>
                <template v-else-if="indeterminate">—</template>
                <template v-else>{{ Math.round(percent) }}%</template>
              </span>
            </div>

            <div
              class="track"
              :class="{ indet: runStateOf(index) === 'now' && indeterminate }"
            >
              <i
                :style="{
                  width:
                    runStateOf(index) === 'done'
                      ? '100%'
                      : runStateOf(index) === 'queued'
                        ? '0'
                        : `${percent}%`,
                }"
              ></i>
            </div>

            <!-- У текущего назначения — что именно сейчас происходит.
                 В фазах без доли выполненного вместо пути стоит название
                 фазы: пути там ещё нет, а тишина читается как зависание. -->
            <p v-if="runStateOf(index) === 'now'" class="prog-file">
              <template v-if="indeterminate">{{ phaseText }}</template>
              <template v-else>{{ wizard.progress?.current }}</template>
            </p>

            <!-- Байты остаются подписью: они понятны и полезны, просто
                 мерой прогресса быть не могут. -->
            <p
              v-if="runStateOf(index) === 'now' && wizard.progress && !indeterminate"
              class="hint"
            >
              {{
                t('install.run.files', {
                  done: count(wizard.progress.doneFiles),
                  total: count(wizard.progress.totalFiles),
                })
              }}
              ·
              {{
                t('install.run.progress', {
                  done: bytes(wizard.progress.doneBytes),
                  total: bytes(wizard.progress.totalBytes),
                })
              }}
            </p>
          </div>
        </template>

        <!-- -------------------------------------------------- шаг «итог» -->
        <!-- Карточка ровно та же, что в списке инстансов: итог показывает
             то, что появилось, и узнавать это в списке пользователь должен
             без перевода взгляда. -->
        <template v-else>
          <div class="cards grid">
            <RouterLink
              v-for="instance in wizard.created"
              :key="instance.id"
              class="card"
              :to="`/instances/${instance.id}`"
            >
              <div
                class="card-accent"
                :style="{ '--instance-accent': accentVar(instance.accent) }"
              ></div>
              <div class="card-in">
                <div class="card-top">
                  <span
                    class="chip"
                    :style="{ '--instance-accent': accentVar(instance.accent) }"
                  >{{ initial(instance.name) }}</span>
                  <div class="card-name">{{ instance.name }}</div>
                  <StatusPill :status="displayStatus(instance, run.statusOf(instance.id))" />
                </div>

                <!-- Строка есть всегда, даже пустая: иначе строки версий
                     в соседних карточках встают на разной высоте. -->
                <div class="card-desc">{{ instance.description }}</div>

                <div class="meta">
                  <span v-if="instance.comfyVersion">{{ instance.comfyVersion }}</span>
                  <span>:{{ instance.preferredPort }}</span>
                  <span v-if="instance.shared?.enabled" class="tag">
                    {{ t('shared.instance.badge') }}
                  </span>
                </div>

                <!-- Путь, а не «последний запуск»: сборку только что
                     распаковали, и запускать её ещё не запускали. -->
                <div class="src"><PathText :path="instance.path" /></div>
              </div>
            </RouterLink>
          </div>
        </template>
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Колонка, а не сетка с посчитанными строками: закреплённая сводка
   по архиву есть только на шаге назначений, и числу строк меняться
   вместе с шагом нельзя. */
.wizard-screen {
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.wizard-screen .screen-body {
  flex: 1;
}

/* Полоса шагов встала на место шапки экрана и берёт её верхнее поле:
   без него она прилипает к заголовку окна. */
.steps {
  padding: var(--space-4) var(--space-5) var(--space-2);
}

.pinned {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: 0 var(--space-5) var(--space-3);
  border-bottom: 1px solid var(--line);
}

/* Копия полей назначения — палитра со своим цветом в том числе —
   уехала в TargetForm поверх InstanceFields, и стили ушли вместе с ней. */
</style>
