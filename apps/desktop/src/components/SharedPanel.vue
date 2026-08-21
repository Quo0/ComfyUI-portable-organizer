<script setup lang="ts">
// Подключение инстанса к общим моделям.
//
// Всё раскрывается на месте, включая сравнение с чужим конфигом: модалок
// в приложении нет по дисциплине z-order, а решение «заменить чужой файл»
// требует показать, что именно заменяется.
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import type { ApplyMode, Instance, InstanceFileInfo } from '../bindings';
import { commands } from '../bindings';
import OpenFolderButton from './OpenFolderButton.vue';
import Group from './ui/Group.vue';
import Pane from './ui/Pane.vue';
import Toggle from './ui/Toggle.vue';
import ToggleRow from './ui/ToggleRow.vue';
import { withViewTransitionAsync } from '../lib/motion';
import { displayStatus } from '../lib/status';
import { useRunStore } from '../stores/run';
import { useSharedStore } from '../stores/shared';
import { useUiStore } from '../stores/ui';

const props = defineProps<{ instance: Instance }>();

const shared = useSharedStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

const mode = ref<ApplyMode>(props.instance.shared?.applyMode ?? 'flag');
/** Чужой конфиг, который придётся подвинуть. Пусто — вопроса нет. */
const foreign = ref<InstanceFileInfo | null>(null);
const backup = ref<string | null>(null);
const busy = ref(false);

/** Перехлёст ползунка тумблера играет только с первого клика, не с отрисовки. */
const toggleTouched = ref(false);

const enabled = computed(() => props.instance.shared?.enabled === true);

/**
 * Настройка прочитается только при следующем старте: аргументы командной
 * строки у работающего процесса поменять нечем. Сказать это обязательно —
 * иначе пользователь сочтёт, что функция сломана.
 */
const running = computed(() =>
  ['starting', 'running', 'stopping', 'detached'].includes(
    displayStatus(props.instance, run.statusOf(props.instance.id)),
  ),
);
const needsRestart = ref(false);

onMounted(() => {
  if (!shared.loaded) void shared.load();
});

/**
 * Блок «путь и способ» и панель режима под тумблером появляются
 * и пропадают вместе с `enabled` — с переходом (transitions.dev). Само
 * состояние переписывает стор уже после ответа бэкенда, поэтому обёртка
 * асинхронная (`withViewTransitionAsync`): синхронной здесь нечего ловить.
 */
async function toggle(): Promise<void> {
  if (busy.value) return;
  toggleTouched.value = true;
  busy.value = true;
  try {
    await withViewTransitionAsync(async () => {
      if (enabled.value) {
        if (await shared.disconnect(props.instance.id)) {
          foreign.value = null;
          backup.value = null;
          needsRestart.value = running.value;
        }
        return;
      }
      await connect(false);
    });
  } finally {
    busy.value = false;
  }
}

async function connect(confirmOverwrite: boolean): Promise<void> {
  const result = await shared.connect(props.instance.id, mode.value, confirmOverwrite);
  if (result === false) {
    // Отказ по чужому файлу — единственный случай, когда стор молчит:
    // это развилка, а не ошибка. Показываем, что там лежит.
    const res = await commands.inspectInstanceConfig(props.instance.id);
    if (res.status === 'ok' && res.data.state === 'foreign') foreign.value = res.data;
    return;
  }
  foreign.value = null;
  backup.value = result;
  needsRestart.value = running.value;
  if (result) ui.pushOk(t('shared.instance.backupMade', { path: result }));
}

/** Смена способа применения у уже подключённого инстанса. */
async function setMode(next: ApplyMode): Promise<void> {
  mode.value = next;
  if (!enabled.value || busy.value) return;
  busy.value = true;
  try {
    // Сначала снимаем прежний способ, иначе файл в папке инстанса остался
    // бы лежать после перехода на флаг.
    await shared.disconnect(props.instance.id);
    await connect(false);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <var class="SharedPanel">
    <Group>
      <!-- Вкладка это два отдела: общая папка и модели самой сборки.
           Заголовок у них одной формы — подпись, значок своей папки, дальше
           содержимое: иначе верх вкладки читался набором разрозненных строк
           без границы между отделами. -->
      <div class="row">
        <span class="t-label">{{ t('shared.instance.label') }}</span>
        <!-- Тот же значок папки, что у моделей сборки ниже, но ведёт в общую:
             разбираться руками с тем, что уже перенесено, приходится именно
             там, а пути к ней на этой вкладке больше нет нигде — в строке
             под тумблером он показан только у подключённой сборки.
             Доступное имя своё: две одинаково подписанные кнопки-папки
             на одном экране неразличимы на слух. -->
        <OpenFolderButton
          :path="shared.root?.path"
          :title="shared.root?.path"
          :label="t('shared.root.open')"
          :disabled="!shared.available"
        />
      </div>

      <!-- Тумблер отдельной строкой, а подпись справа от него говорит, что он
           делает. Название отдела ушло в заголовок, и повторять его здесь
           незачем. -->
      <ToggleRow>
        <Toggle
          :checked="enabled"
          :touched="toggleTouched"
          :aria-label="t('shared.instance.label')"
          :disabled="busy || !shared.configured"
          @click="toggle"
        />
        <div>
          <div class="t-base">{{ t('shared.instance.hint') }}</div>

          <!-- Корень не задан — подключать не к чему. Вместо мёртвого
               тумблера ведём туда, где его задают. -->
          <div v-if="!shared.configured" class="hint">
            {{ t('shared.instance.noRoot') }}
            <RouterLink to="/settings/shared-models">
              {{ t('shared.instance.setUp') }}
            </RouterLink>
          </div>

          <div v-else-if="enabled" class="hint">
            {{ shared.root?.path }} ·
            {{ t(`shared.mode.${instance.shared?.applyMode ?? 'flag'}.short`) }}
          </div>
        </div>
      </ToggleRow>

      <Group v-if="shared.configured && enabled">
        <span class="t-label">{{ t('shared.mode.label') }}</span>
        <div class="seg">
          <button
            type="button"
            :aria-pressed="mode === 'flag'"
            :disabled="busy"
            @click="setMode('flag')"
          >
            {{ t('shared.mode.flag.title') }}
          </button>
          <button
            type="button"
            :aria-pressed="mode === 'instanceFile'"
            :disabled="busy"
            @click="setMode('instanceFile')"
          >
            {{ t('shared.mode.instanceFile.title') }}
          </button>
        </div>
        <p class="hint">{{ t(`shared.mode.${mode}.hint`) }}</p>
      </Group>

      <!-- Плашка про рестарт. Живёт до перезапуска, а не гаснет по таймеру:
           пользователь может уйти в другой раздел и вернуться. -->
      <p v-if="needsRestart && running" class="hint bad">
        {{ t('shared.instance.needsRestart') }}
      </p>

      <!-- В папке инстанса уже лежит чужой файл. Показываем его целиком:
           решение «заменить» принимается не вслепую. -->
      <Group v-if="foreign" danger>
        <p class="t-md">{{ t('shared.foreign.title') }}</p>
        <p class="t-sm">{{ t('shared.foreign.body', { path: foreign.path }) }}</p>
        <Pane>
          <div class="pane-head">
            <span class="title">{{ t('shared.foreign.whatsThere') }}</span>
          </div>
          <div class="scroll">
            <pre class="console">{{ foreign.content }}</pre>
          </div>
        </Pane>
        <div class="row">
          <button type="button" class="btn danger" :disabled="busy" @click="connect(true)">
            {{ t('shared.foreign.replace') }}
          </button>
          <button
            type="button"
            class="btn secondary"
            :disabled="busy"
            @click="((mode = 'flag'), connect(false))"
          >
            {{ t('shared.foreign.useFlag') }}
          </button>
          <button type="button" class="btn ghost" @click="foreign = null">
            {{ t('common.cancel') }}
          </button>
        </div>
      </Group>
    </Group>
  </var>
</template>
