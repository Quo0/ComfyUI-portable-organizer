<script setup lang="ts">
// Мастер установки — отдельный роут с шагами внутри, а не череда модалок.
// Шаги: архив → назначения → общие ресурсы → выполнение → итог.
// Шаг общих ресурсов собран из тех же компонентов, что экран настроек,
// и пользуется тем же стором: логика подключения не дублируется.
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';

import type { ArchiveRecord, InstallTarget } from '../bindings';
import { errorText } from '../lib/errors';
import { accentVar, isCustomAccent, useFormat } from '../lib/format';
import type { WizardStep } from '../stores/installer';
import { useInstallerStore } from '../stores/installer';
import { useSharedStore } from '../stores/shared';
import { useWorkflowsStore } from '../stores/workflows';

const wizard = useInstallerStore();
const shared = useSharedStore();
const library = useWorkflowsStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

/** Порядок шагов. Он же порядок показа в полосе состояния мастера. */
const STEPS: WizardStep[] = ['archive', 'targets', 'shared', 'running', 'done'];
const current = computed(() => STEPS.indexOf(wizard.step));

/**
 * Название текущего шага для ряда с кнопками перехода.
 *
 * У трёх шагов оно своё и говорит о деле («Куда распаковать»), у двух
 * совпадает с названием в полосе шагов — придумывать им второе имя
 * значило бы говорить об одном и том же двумя словами.
 */
const stepTitle = computed(() => {
  switch (wizard.step) {
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

const ACCENTS = [
  'teal',
  'indigo',
  'ember',
  'moss',
  'azure',
  'orchid',
  'rose',
  'amber',
] as const;

onMounted(() => {
  if (!wizard.info) void wizard.loadHistory();
  if (!shared.loaded) void shared.load();
  if (!library.loaded) void library.load();
});

/** Общий корень задаётся прямо здесь — уходить в настройки не нужно. */
async function pickSharedRoot(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked === 'string') await shared.setRoot(picked);
}

/** То же для библиотеки воркфлоу: она независима от общих моделей. */
async function pickLibrary(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked === 'string') await library.setPath(picked);
}

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
  if (wizard.targets.length === 0) await addTarget();
}

async function useRecent(record: ArchiveRecord): Promise<void> {
  if (!record.available) return;
  if (await wizard.chooseArchive(record.path)) wizard.step = 'targets';
  if (wizard.targets.length === 0) await addTarget();
}

async function addTarget(): Promise<void> {
  wizard.targets.push({
    path: '',
    name: '',
    description: '',
    accent: ACCENTS[wizard.targets.length % ACCENTS.length],
    preferredPort: 8188 + wizard.targets.length,
  });
  await wizard.recheck();
}

async function removeTarget(index: number): Promise<void> {
  wizard.targets.splice(index, 1);
  await wizard.recheck();
}

async function pickTargetFolder(target: InstallTarget): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked !== 'string') return;
  target.path = picked;
  // Имя папки — разумное значение по умолчанию, но только пока пользователь
  // не вписал своё: затирать введённое было бы грубо.
  if (!target.name.trim()) {
    target.name = picked.split(/[\\/]/).filter(Boolean).pop() ?? '';
  }
  await wizard.recheck();
}

/** Сколько места нужно с учётом того, что цели могут быть на одном диске. */
const needed = computed(() =>
  bytes((wizard.info?.totalUncompressed ?? 0) * wizard.targets.length),
);
</script>

<template>
  <section class="screen wizard-screen">
    <header class="screen-head">
      <RouterLink class="btn ghost" to="/install">
        <svg class="ico"><use href="#i-back" /></svg>
        {{ t('common.back') }}
      </RouterLink>
      <h1 class="t-lg">{{ t('install.wizard.title') }}</h1>
    </header>

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
    <div v-if="wizard.step !== 'archive'" class="step-bar">
      <h2 class="title">{{ stepTitle }}</h2>
      <span v-if="wizard.step === 'done'" class="t-label">
        {{ t('install.done.added', wizard.created.length) }}
      </span>
      <span class="spacer"></span>

      <span v-if="wizard.step === 'targets'" class="acts">
        <button type="button" class="btn ghost" @click="wizard.step = 'archive'">
          <svg class="ico"><use href="#i-back" /></svg>
          {{ t('common.back') }}
        </button>
        <button
          type="button"
          class="btn primary lg"
          :disabled="wizard.blocked || wizard.targets.some((x) => !x.name.trim())"
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
        <span>{{ t('install.archive.files', wizard.info.files) }}</span>
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
            <div class="cards">
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
                  <!-- Путь не переводится и не сокращается. -->
                  <div class="src"><code>{{ record.path }}</code></div>
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
        <template v-else-if="wizard.step === 'targets' && wizard.info">

          <div
            v-for="(target, index) in wizard.targets"
            :key="index"
            class="pane target"
          >
            <div class="pane-head">
              <span class="title">{{ target.name || t('install.targets.title') }}</span>
              <button
                v-if="wizard.targets.length > 1"
                type="button"
                class="btn ghost"
                @click="removeTarget(index)"
              >
                {{ t('install.targets.remove') }}
              </button>
            </div>

            <div class="scroll-pad">
              <div class="field">
                <label>{{ t('instances.field.folder') }}</label>
                <div class="path-row">
                  <div class="input mono"><span>{{ target.path }}</span></div>
                  <button
                    type="button"
                    class="btn secondary"
                    @click="pickTargetFolder(target)"
                  >
                    {{ t('install.targets.choose') }}
                  </button>
                </div>
              </div>

              <!-- Ошибки и предупреждения разделены: с предупреждением
                   распаковать можно, с ошибкой — нет. -->
              <p
                v-for="(problem, i) in wizard.checks[index]?.errors ?? []"
                :key="`e${i}`"
                class="hint bad"
              >
                {{ errorText(problem) }}
              </p>
              <p
                v-for="(problem, i) in wizard.checks[index]?.warnings ?? []"
                :key="`w${i}`"
                class="hint"
              >
                {{ errorText(problem) }}
              </p>

              <div class="field">
                <label>{{ t('instances.field.name') }}</label>
                <input
                  v-model="target.name"
                  class="input"
                  type="text"
                  maxlength="80"
                  @blur="wizard.recheck()"
                />
              </div>

              <div class="field">
                <label>{{ t('instances.field.description') }}</label>
                <input
                  v-model="target.description"
                  class="input"
                  type="text"
                  maxlength="200"
                />
              </div>

              <div class="field">
                <span class="t-label">{{ t('instances.field.accent') }}</span>
                <div class="picker">
                  <button
                    v-for="accent in ACCENTS"
                    :key="accent"
                    type="button"
                    :style="{ background: accentVar(accent) }"
                    :aria-pressed="target.accent === accent"
                    @click="target.accent = accent"
                  ></button>
                  <!-- Свой цвет — там же, где палитра, и на том же экране,
                       где заводят сборку: возвращаться за ним потом
                       в редактирование незачем. -->
                  <label
                    class="swatch-custom"
                    :class="{ on: isCustomAccent(target.accent) }"
                    :title="t('instances.field.accentCustom')"
                  >
                    <input
                      type="color"
                      :value="isCustomAccent(target.accent) ? target.accent : '#4db6a5'"
                      :aria-label="t('instances.field.accentCustom')"
                      @input="target.accent = ($event.target as HTMLInputElement).value"
                    />
                  </label>
                </div>
              </div>

              <div class="field">
                <label>{{ t('instances.field.port') }}</label>
                <input
                  v-model.number="target.preferredPort"
                  class="input num"
                  type="number"
                  min="1024"
                  max="65535"
                />
              </div>
            </div>
          </div>

          <div class="row">
            <button type="button" class="btn secondary" @click="addTarget">
              {{ t('install.targets.add') }}
            </button>
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
            <div class="path-row">
              <div class="input mono">
                <span>{{ shared.root?.path ?? t('shared.root.empty') }}</span>
              </div>
              <!-- Тот же выбор, что в настройках: US-SHARED-01/AC-4 требует
                   не выгонять пользователя из мастера ради одной папки. -->
              <button class="btn secondary" type="button" @click="pickSharedRoot">
                {{ t('common.browse') }}
              </button>
            </div>
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
            <div class="path-row">
              <div class="input mono">
                <span>{{ library.path || t('library.path.empty') }}</span>
              </div>
              <button class="btn secondary" type="button" @click="pickLibrary">
                {{ t('common.browse') }}
              </button>
            </div>
            <p v-if="library.configured && library.available" class="hint">
              {{ t('library.summary', library.items.length) }}
            </p>
            <p v-else-if="!library.configured" class="hint">{{ t('library.path.howto') }}</p>
          </div>
          </div>
          </div>

        </template>

        <!-- -------------------------------------------- шаг «выполнение» -->
        <template v-else-if="wizard.step === 'running'">
          <div class="group">
            <p class="t-md">{{ phaseText }}</p>
            <div class="bar" :class="{ indet: indeterminate }">
              <i :style="indeterminate ? undefined : { width: `${percent}%` }"></i>
            </div>

            <!-- Счётчики только там, где есть что считать: в фазах подготовки
                 они показали бы «0 из 0» и сбили бы с толку сильнее тишины. -->
            <template v-if="wizard.progress && !indeterminate">
              <!-- Байты остаются подписью: они понятны и полезны, просто
                   мерой прогресса быть не могут. -->
              <p class="hint">
                {{
                  t('install.run.files', {
                    done: wizard.progress.doneFiles,
                    total: wizard.progress.totalFiles,
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
              <!-- Текущий файл не переводится: это путь. -->
              <p class="t-mono current">{{ wizard.progress.current }}</p>
            </template>
          </div>

        </template>

        <!-- -------------------------------------------------- шаг «итог» -->
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
                  <div class="card-name">{{ instance.name }}</div>
                </div>
                <div class="meta">
                  <span v-if="instance.comfyVersion">
                    ComfyUI {{ instance.comfyVersion }}
                  </span>
                  <span>:{{ instance.preferredPort }}</span>
                </div>
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

.steps {
  padding: 0 var(--space-5) var(--space-2);
}

.pinned {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: 0 var(--space-5) var(--space-3);
  border-bottom: 1px solid var(--line);
}

.target {
  display: block;
}

/* Квадрат выбора своего цвета — тот же, что в полях инстанса.
   Повторён здесь, а не вынесен в дизайн-систему, потому что это
   единственные два места, и оба знают о нём всё. */
.swatch-custom {
  width: 22px;
  height: 22px;
  border-radius: var(--radius-sm);
  display: block;
  cursor: pointer;
  box-shadow: 0 0 0 1px var(--line-strong) inset;
  background: conic-gradient(
    #e5534b,
    #d9a441,
    #6fbf73,
    #4db6a5,
    #5b8def,
    #a77bd6,
    #e5534b
  );
}

.swatch-custom.on {
  outline: 2px solid var(--ink);
  outline-offset: 2px;
}

.swatch-custom input {
  opacity: 0;
  width: 100%;
  height: 100%;
  display: block;
  cursor: pointer;
}

.swatch-custom:focus-within {
  outline: 2px solid var(--focus-ring);
  outline-offset: 2px;
}
.current {
  color: var(--ink-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
