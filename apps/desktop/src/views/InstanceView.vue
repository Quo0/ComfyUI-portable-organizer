<script setup lang="ts">
// Экран сборки.
//
// До ревизии это была одна лента: запуск, общие модели, модели сборки,
// воркфлоу, путь, версии, размер, профили, источник, форма редактирования
// и удаление — подряд, примерно на тысячу строк вёрстки. Найти нужное
// можно было только прокруткой.
//
// Вкладки разводят четыре независимых набора данных. Шапка со сборкой
// и кнопкой запуска закреплена: она нужна на любой вкладке.
import { computed, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { revealItemInDir } from '@tauri-apps/plugin-opener';

import InstanceFields from '../components/InstanceFields.vue';
import LaunchControls from '../components/LaunchControls.vue';
import LaunchNotices from '../components/LaunchNotices.vue';
import LogConsole from '../components/LogConsole.vue';
import ModelsPanel from '../components/ModelsPanel.vue';
import PathText from '../components/PathText.vue';
import WorkflowPanel from '../components/WorkflowPanel.vue';
import SharedPanel from '../components/SharedPanel.vue';
import StatusPill from '../components/StatusPill.vue';
import type { InstanceEdit } from '../bindings';
import { accentVar, initial, useFormat } from '../lib/format';
import { displayStatus } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';
import { useSharedStore } from '../stores/shared';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ id: string }>();

const instances = useInstancesStore();
const run = useRunStore();
const shared = useSharedStore();
const ui = useUiStore();
const router = useRouter();
const { t } = useI18n();
const { bytes, moment } = useFormat();

type Tab = 'overview' | 'models' | 'workflows' | 'settings';
const TABS: Tab[] = ['overview', 'models', 'workflows', 'settings'];

const tab = ref<Tab>('overview');
/** Лог занимает всю область данных, а не висит куском посреди свитка. */
const logOpen = ref(false);
const editing = ref(false);
const confirming = ref(false);
const measuring = ref(false);
const draft = ref<InstanceEdit>({
  name: '',
  description: '',
  accent: 'teal',
  preferredPort: 8188,
});

const instance = computed(() => instances.byId(props.id));
const status = computed(() => run.statusOf(props.id));
const state = computed(() =>
  instance.value ? displayStatus(instance.value, status.value) : 'stopped',
);
const lines = computed(() => run.logs[props.id] ?? []);
const profiles = computed(() => run.profiles[props.id] ?? []);
const custom = computed(() => instance.value?.customProfiles ?? []);

/** Пустая строка означает «ещё не посчитано»: показывать нечего. */
const sizeText = computed(() => bytes(instance.value?.sizeBytes));
const sizeMeasuredText = computed(() => {
  const when = moment(instance.value?.sizeMeasuredAt);
  return when ? t('instances.field.sizeMeasuredAt', { when }) : '';
});
/**
 * Строка про общие модели для обзора. Пустая означает «не подключены»:
 * подключение — не ошибка и не пропуск, поэтому в строке стоит прочерк,
 * а не предложение что-то сделать.
 */
const sharedText = computed(() => {
  if (instance.value?.shared?.enabled !== true) return '';
  const mode = instance.value.shared.applyMode;
  return `${shared.root?.path ?? ''} · ${t(`shared.mode.${mode}.short`)}`;
});

const lastRunText = computed(() => {
  const when = moment(instance.value?.lastStartedAt);
  return when
    ? t('instances.field.lastRun', { when })
    : t('instances.field.lastRunNever');
});

onMounted(async () => {
  if (!instances.loaded) await instances.load();
  // Путь общей папки нужен уже на обзоре, а не только на вкладке «Модели».
  if (!shared.loaded) await shared.load();
  await run.loadProfiles(props.id);
  await run.loadLog(props.id);
  await measureIfNeeded();
});

// Прямой переход по ссылке отдаёт экран раньше, чем загрузился реестр.
watch(instance, () => void measureIfNeeded());

// Старт — это когда лог и есть содержимое экрана: минуты холодного старта
// без него выглядят как зависание.
watch(state, (now, before) => {
  if (now === 'starting' && before !== 'starting') logOpen.value = true;
});

/**
 * Размер считается сам при первом открытии и кэшируется. Обход 52 ГБ
 * занимает минуты — без кэша экран показывал бы «считаю…» каждый раз.
 */
async function measureIfNeeded(): Promise<void> {
  const it = instance.value;
  if (!it || !it.available || it.sizeBytes !== null || measuring.value) return;
  await measure();
}

async function measure(): Promise<void> {
  measuring.value = true;
  try {
    await instances.measureSize(props.id);
  } finally {
    measuring.value = false;
  }
}

function startEdit(): void {
  const it = instance.value;
  if (!it) return;
  draft.value = {
    name: it.name,
    description: it.description,
    accent: it.accent,
    preferredPort: it.preferredPort,
  };
  editing.value = true;
}

async function save(): Promise<void> {
  const saved = await instances.update(props.id, draft.value);
  if (!saved) return;
  editing.value = false;
  ui.pushOk(t('instances.toast.updated', { name: saved.name }));
}

async function remove(): Promise<void> {
  const name = instance.value?.name ?? '';
  if (!(await instances.remove(props.id))) return;
  ui.pushOk(t('instances.toast.removed', { name }));
  await router.push('/instances');
}

function openFolder(): void {
  if (instance.value) void revealItemInDir(instance.value.path);
}
</script>

<template>
  <section v-if="instance" class="screen inst-screen">
    <header class="screen-head">
      <RouterLink class="btn ghost" to="/instances">
        <svg class="ico"><use href="#i-back" /></svg>
        {{ t('common.back') }}
      </RouterLink>
      <span
        class="chip"
        :style="{ '--instance-accent': accentVar(instance.accent) }"
      >{{ initial(instance.name) }}</span>
      <h1 class="t-lg">{{ instance.name }}</h1>
      <StatusPill :status="state" />
      <span v-if="status?.port" class="t-mono">:{{ status.port }}</span>
      <span class="head-spacer"></span>
      <LaunchControls :instance="instance" />
    </header>

    <div v-if="state === 'starting'" class="track indet starting-track"><i></i></div>

    <LaunchNotices :instance="instance" />

    <!-- Инстанс с исчезнувшей папкой не пропадает из списка: иначе
         выглядит так, будто приложение потеряло чужую сборку. -->
    <div v-if="!instance.available" class="notices">
      <div class="banner bad">
        <div>
          <b>{{ t('instances.unavailable.title') }}</b>
          <p class="t-sm">{{ t('instances.unavailable.body', { path: instance.path }) }}</p>
        </div>
      </div>
    </div>

    <nav class="tabs" role="tablist">
      <button
        v-for="item in TABS"
        :key="item"
        type="button"
        role="tab"
        :aria-selected="tab === item"
        @click="tab = item"
      >
        {{ t(`instances.tab.${item}`) }}
      </button>
    </nav>

    <!-- Лог занимает область данных целиком и сворачивается кнопкой.
         Поверх лечь он не может: у работающей сборки поверх нашего HTML
         лежит нативное окно вкладки. -->
    <div v-if="logOpen" class="log-area">
      <div class="log-head">
        <span class="t-label">{{ t('run.console') }}</span>
        <span class="hint">{{ t('run.lines', lines.length) }}</span>
        <span class="head-spacer"></span>
        <button type="button" class="btn secondary" @click="logOpen = false">
          {{ t('run.hideLog') }}
        </button>
      </div>
      <div class="log">
        <LogConsole v-if="lines.length" :lines="lines" />
        <p v-else class="hint">{{ t('run.empty') }}</p>
      </div>
    </div>

    <div v-else class="screen-body">
      <div class="screen-pad wide">
        <!-- ------------------------------------------------------- обзор -->
        <div v-if="tab === 'overview'" class="cols">
          <!-- Подписи здесь те же `.field > label`, что и в правой колонке:
               две разные подписи в одной форме читаются как два разных
               уровня вложенности, которых нет. -->
          <div>
            <div class="field">
              <label>{{ t('instances.field.lastRunTitle') }}</label>
              <div class="paths">
                <div class="path-item keep">
                  <span class="lbl">{{ lastRunText }}</span>
                  <span class="val">
                    {{ status?.readySecs ? t('run.readyIn', { secs: status.readySecs }) : '' }}
                  </span>
                </div>
                <!-- Именно предпочитаемый: настоящий порт выдаётся при
                     старте и стоит в шапке рядом с состоянием. Показывать
                     под этой подписью выданный значило бы врать в тот
                     единственный момент, когда они разошлись. -->
                <div class="path-item">
                  <span class="lbl">{{ t('instances.field.port') }}</span>
                  <span class="val">{{ instance.preferredPort }}</span>
                </div>
                <div class="path-item">
                  <span class="lbl">{{ t('instances.field.profiles') }}</span>
                  <span class="val">{{ status?.profileId ?? profiles[0]?.id ?? '—' }}</span>
                </div>
              </div>
            </div>

            <div class="field">
              <label>{{ t('run.console') }}</label>
              <div class="row">
                <button type="button" class="btn secondary" @click="logOpen = true">
                  {{ t('run.showLog') }}
                </button>
                <!-- Про всю область сказано заранее: кнопка не открывает
                     панельку сбоку, а забирает экран целиком. -->
                <span class="hint">
                  {{ t('run.lines', lines.length) }} · {{ t('run.logExpands') }}
                </span>
              </div>
            </div>

            <div v-if="instance.description" class="field">
              <label>{{ t('instances.field.description') }}</label>
              <p class="t-sm">{{ instance.description }}</p>
            </div>
          </div>

          <div>
            <div class="field">
              <label>{{ t('instances.field.folder') }}</label>
              <div class="path-row">
                <div class="input mono"><span>{{ instance.path }}</span></div>
                <button
                  type="button"
                  class="btn secondary"
                  :disabled="!instance.available"
                  @click="openFolder"
                >
                  {{ t('common.openFolder') }}
                </button>
              </div>
            </div>

            <div class="paths">
              <div class="path-item keep">
                <span class="lbl">{{ t('instances.field.comfyVersion') }}</span>
                <span class="val">{{ instance.comfyVersion ?? t('common.unknown') }}</span>
              </div>
              <div class="path-item keep">
                <span class="lbl">{{ t('instances.field.pythonVersion') }}</span>
                <span class="val">{{ instance.pythonVersion ?? t('common.unknown') }}</span>
              </div>
              <div class="path-item">
                <span class="lbl">{{ t('instances.field.sizeOnDisk') }}</span>
                <span class="val">
                  {{ sizeText || (measuring ? t('instances.field.sizeMeasuring') : '—') }}
                </span>
              </div>
              <div class="path-item">
                <span class="lbl">{{ t('instances.field.profiles') }}</span>
                <!-- «4 + 1» не говорит, что за плюс один: свои профили
                     заводит пользователь, и найденные в папке .bat с ними
                     не смешиваются. -->
                <span class="val">
                  {{ profiles.length
                  }}{{
                    custom.length
                      ? ` + ${t('instances.field.profilesCustom', custom.length)}`
                      : ''
                  }}
                </span>
              </div>
              <!-- Подключение к общим моделям — факт про сборку, а не только
                   про вкладку «Модели»: почему у двух сборок разный набор
                   чекпоинтов, спрашивают на обзоре. Здесь он только
                   показан — тумблер живёт в одном месте, на своей вкладке. -->
              <div class="path-item">
                <span class="lbl">{{ t('shared.instance.label') }}</span>
                <span class="val">
                  <template v-if="sharedText">
                    <PathText :path="sharedText" />
                  </template>
                  <template v-else>—</template>
                </span>
              </div>
            </div>

            <div class="row">
              <span v-if="sizeMeasuredText" class="hint">{{ sizeMeasuredText }}</span>
              <button
                type="button"
                class="btn ghost"
                :disabled="measuring || !instance.available"
                @click="measure"
              >
                {{ t('instances.field.recalculate') }}
              </button>
              <RouterLink class="btn ghost" :to="`/instances/${instance.id}/args`">
                {{ t('args.edit') }}
              </RouterLink>
            </div>

            <div v-if="instance.source" class="src">
              {{
                t('instances.field.source', {
                  archive: instance.source.archiveLabel,
                  when: moment(instance.source.installedAt),
                })
              }}
            </div>
          </div>
        </div>

        <!-- ------------------------------------------------------ модели -->
        <template v-else-if="tab === 'models'">
          <SharedPanel :instance="instance" />
          <ModelsPanel :instance="instance" />
        </template>

        <!-- ---------------------------------------------------- воркфлоу -->
        <WorkflowPanel v-else-if="tab === 'workflows'" :instance="instance" />

        <!-- --------------------------------------------------- параметры -->
        <template v-else>
          <template v-if="editing">
            <InstanceFields v-model="draft" />
            <div class="row">
              <button
                type="button"
                class="btn primary"
                :disabled="draft.name.trim() === ''"
                @click="save"
              >
                {{ t('common.save') }}
              </button>
              <button type="button" class="btn ghost" @click="editing = false">
                {{ t('common.cancel') }}
              </button>
            </div>
          </template>
          <div v-else class="group">
            <div class="paths">
              <div class="path-item">
                <span class="lbl">{{ t('instances.field.name') }}</span>
                <span class="val">{{ instance.name }}</span>
              </div>
              <div class="path-item">
                <span class="lbl">{{ t('instances.field.description') }}</span>
                <span class="val">{{ instance.description || '—' }}</span>
              </div>
              <div class="path-item">
                <span class="lbl">{{ t('instances.field.port') }}</span>
                <span class="val">{{ instance.preferredPort }}</span>
              </div>
            </div>
            <div class="row">
              <button type="button" class="btn secondary" @click="startEdit">
                {{ t('common.edit') }}
              </button>
            </div>
          </div>

          <!-- Подтверждение раскрывается на месте, а не всплывает диалогом:
               модалок в приложении нет, и объяснение важнее, чем вопрос. -->
          <div class="group danger-zone">
            <button
              v-if="!confirming"
              type="button"
              class="btn danger"
              @click="confirming = true"
            >
              {{ t('instances.remove.action') }}
            </button>
            <template v-else>
              <p class="t-md">{{ t('instances.remove.title', { name: instance.name }) }}</p>
              <p class="t-sm">{{ t('instances.remove.body') }}</p>
              <div class="row">
                <button type="button" class="btn danger" @click="remove">
                  {{ t('instances.remove.confirm') }}
                </button>
                <button type="button" class="btn ghost" @click="confirming = false">
                  {{ t('common.cancel') }}
                </button>
              </div>
            </template>
          </div>
        </template>
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Колонка, а не сетка с посчитанными строками: над областью данных стоят
   шапка, полоса старта, два независимых блока уведомлений и вкладки, и все,
   кроме шапки и вкладок, появляются и исчезают. Заранее выписанные строки
   разъезжались с их настоящим числом, и область данных теряла свою долю. */
.inst-screen {
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.inst-screen > .screen-body,
.log-area {
  flex: 1;
}

/* Старт длится минуты, доля выполненного неизвестна. Полоса не сообщает
   прогресс, а показывает, что процесс жив: без неё пауза читается
   как зависание. */
.starting-track {
  margin: 0 var(--space-5) var(--space-2);
}

.tabs {
  padding: 0 var(--space-5);
  margin-top: var(--space-2);
}

.notices {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: 0 var(--space-5);
}

/* Лог на всё оставшееся место: прокрутка внутри него, а не снаружи. */
.log-area {
  min-height: 0;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  gap: var(--space-2);
  padding: var(--space-3) var(--space-5) var(--space-4);
}

.log-head {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
}
</style>
