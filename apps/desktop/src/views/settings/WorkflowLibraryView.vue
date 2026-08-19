<script setup lang="ts">
// Библиотека воркфлоу: папка и её содержимое на одном экране.
//
// Раньше это были два места — путь в настройках, список в разделе рейла, —
// и на вопрос «куда это складывается» нельзя было ответить, не уходя
// с экрана. Библиотека устроена ровно как общие модели: одна папка снаружи
// сборок, поэтому и живёт там же, где они.
//
// Две области прокрутки на экране — оговорённое исключение из правила
// «одна на экран»: список слева и детали справа скроллятся порознь, иначе
// панель деталей уезжала бы вместе с двумя сотнями воркфлоу.
import { Check, ExternalLink, ListChecks, Plus, RotateCw } from '@lucide/vue';
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { useI18n } from 'vue-i18n';

import type { LibItem } from '../../stores/workflows';
import CheckGlyph from '../../components/CheckGlyph.vue';
import EmptyNote from '../../components/EmptyNote.vue';
import MenuButton from '../../components/MenuButton.vue';
import PathPicker from '../../components/PathPicker.vue';
import WorkflowPasteForm from '../../components/WorkflowPasteForm.vue';
import { commands } from '../../bindings';
import { accentVar } from '../../lib/format';
import { errorText } from '../../lib/errors';
import { useSlidingTabs } from '../../lib/motion';
import { useRunStore } from '../../stores/run';
import { useInstancesStore } from '../../stores/instances';
import { useUiStore } from '../../stores/ui';
import { useWorkflowsStore } from '../../stores/workflows';

const library = useWorkflowsStore();
const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

/** Подсказка «положить рядом с общими моделями». Может и не быть. */
const suggested = ref<string | null>(null);

/** Черновик заметки и тегов: пишем в манифест по кнопке, а не на каждый ввод. */
const noteDraft = ref('');
const tagsDraft = ref('');
const editing = ref(false);

/**
 * Вкладки правой панели. Триста пикселей на совместимость, заметку и теги
 * разом — это столбик, в котором ничего не видно целиком.
 */
type Side = 'where' | 'note' | 'tags';
const SIDES: Side[] = ['where', 'note', 'tags'];
const side = ref<Side>('where');

/** Плашка скользит к выбранной вкладке сама, замером в DOM. */
const sideTabsBar = ref<HTMLElement | null>(null);
useSlidingTabs(sideTabsBar, side);

onMounted(async () => {
  if (!library.loaded) await library.load();
  if (!instances.loaded) await instances.load();
  const res = await commands.suggestLibraryPath();
  if (res.status === 'ok') suggested.value = res.data;
});

/** Недоступную сборку целью не предлагаем: писать некуда. */
const targets = computed(() => instances.items.filter((i) => i.available));

/**
 * Имя сборки по опознавателю — для отчёта о массовой записи.
 *
 * Запасной вариант — сам опознаватель: сборку могли убрать из реестра,
 * пока шла запись, и промолчать о такой строке отчёта хуже, чем показать
 * её нечитаемо.
 */
function instanceName(id: string): string {
  return instances.items.find((i) => i.id === id)?.name ?? id;
}

/**
 * Клик по строке списка выбирает воркфлоу.
 *
 * В режиме отметок строка не делает ничего: отмечает только сам квадратик.
 * Раньше отмечала вся строка, и промах по любому её месту менял отбор —
 * а отбор здесь ведёт к записи файлов в сборки, и случайно попасть в него
 * мимо квадратика не должно быть возможно.
 */
function onRow(item: LibItem): void {
  if (library.multi) return;
  void library.select(item.path);
}

/** Отмечены все доступные сборки. */
const allTargets = computed(
  () => targets.value.length > 0 && targets.value.every((i) => library.markedTargets.has(i.id)),
);

/** Отмечена часть — шапке списка нужен третий вид, а не «снято». */
const someTargets = computed(
  () => library.markedTargets.size > 0 && !allTargets.value,
);

function toggleAllTargets(): void {
  library.setTargets(allTargets.value ? [] : targets.value.map((i) => i.id));
}

function startEdit(item: LibItem): void {
  noteDraft.value = item.meta.note;
  tagsDraft.value = item.meta.tags.join(', ');
  editing.value = true;
}

async function saveMeta(item: LibItem): Promise<void> {
  await library.setMeta(item.path, {
    ...item.meta,
    note: noteDraft.value,
    tags: tagsDraft.value
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean),
  });
  editing.value = false;
}

const pushing = ref<string | null>(null);

/**
 * Куда положили прямо сейчас, нашим же нажатием.
 *
 * Наложение поверх ответа бэкенда, а не замена ему: даёт мгновенный отклик,
 * не дожидаясь повторного опроса. Правду про «уже там» знает `present`
 * из `workflow_compat` — он считается по файловой системе и переживает
 * перезаход на экран.
 */
const pushed = ref<Set<string>>(new Set());

// Отметки относятся к выбранному воркфлоу и вместе с ним и сбрасываются.
watch(() => library.selected, () => (pushed.value = new Set()));

/** Воркфлоу уже в этой сборке. */
function added(instanceId: string): boolean {
  return library.compatOf(instanceId)?.present === true || pushed.value.has(instanceId);
}

/**
 * Кладёт выбранный воркфлоу в сборку.
 *
 * Конфликт имён возвращается развилкой, а не ошибкой, и молчаливой
 * перезаписи не происходит ни при запущенной сборке (409 от ComfyUI),
 * ни при остановленной (наша проверка).
 */
async function push(instanceId: string, overwrite = false): Promise<void> {
  const item = library.current;
  if (!item) return;
  pushing.value = instanceId;
  try {
    const res = await commands.pushWorkflow(
      instanceId,
      library.path,
      item.path,
      overwrite,
    );
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    if (res.data === 'conflict') {
      if (window.confirm(t('library.push.replace', { name: item.path }))) {
        await push(instanceId, true);
      }
      return;
    }
    // Запущенная сборка перечитывает список воркфлоу при обновлении
    // страницы, а не сама. Сказать это обязательно: иначе пользователь
    // решит, что добавление не сработало.
    pushed.value = new Set(pushed.value).add(instanceId);
    const running = run.statusOf(instanceId)?.state === 'running';
    ui.pushOk(running ? t('library.push.doneRunning') : t('library.push.done'));
  } finally {
    pushing.value = null;
  }
}

/** Что библиотека принимает: сам граф и картинку, которая носит его в себе. */
const ACCEPTED = ['.json', '.png'];

async function addFile(): Promise<void> {
  const picked = await open({
    multiple: false,
    // Картинка из папки output носит граф в текстовом чанке, и «перетащить
    // удачную генерацию» — самый частый способ к ней вернуться.
    filters: [{ name: 'JSON, PNG', extensions: ['json', 'png'] }],
  });
  if (typeof picked !== 'string') return;
  if (await library.addFile(picked)) ui.pushOk(t('library.add.done'));
}

/** Открыта форма вставки текстом. */
const pasting = ref(false);

/**
 * Вставленное сохранено. Возврат к списку обязателен: воркфлоу уже выбран
 * стором, и оставаться в форме значило бы прятать результат собственного
 * действия.
 */
function onPasted(name: string): void {
  pasting.value = false;
  ui.pushOk(t('library.paste.done', { name }));
}

/** Файл тащат на окно. */
const dragging = ref(false);

/**
 * Перетаскивание идёт через событие Tauri, а не через `ondrop`.
 *
 * У главного окна свой нативный обработчик дропа, и на Windows он
 * перехватывает событие раньше HTML — обычный `ondrop` просто не сработает.
 * Конфликта с холстом ComfyUI нет: у дочернего вебвью дроп отключён
 * через `disable_drag_drop_handler()`, там он принадлежит канвасу.
 */
onMounted(async () => {
  unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
    if (event.payload.type === 'over') {
      dragging.value = library.configured && library.available;
      return;
    }
    if (event.payload.type === 'leave') {
      dragging.value = false;
      return;
    }
    dragging.value = false;
    if (!library.configured || !library.available) return;

    let added = 0;
    for (const file of event.payload.paths) {
      // Отсев по расширению до чтения: тащат папками, и объяснять про
      // каждый посторонний файл, что он не воркфлоу, — не помощь, а шум.
      // Картинка без графа внутри отсеется уже с объяснением: её притащили
      // осознанно, и промолчать здесь было бы хуже.
      const lower = file.toLowerCase();
      if (!ACCEPTED.some((ext) => lower.endsWith(ext))) continue;
      if (await library.addFile(file)) added += 1;
    }
    if (added > 0) ui.pushOk(t('library.add.dropped', added));
  });
});

let unlisten: (() => void) | null = null;
onUnmounted(() => unlisten?.());
</script>

<template>
  <section class="screen">
    <!-- Путь стоит в шапке раздела, а не отдельным полем над списком:
         это одна строка, которую задают один раз, и отводить под неё блок
         в полэкрана значило бы говорить о ней громче, чем о самой
         библиотеке. -->
    <header class="screen-head">
      <h1 class="t-lg">{{ t('library.title') }}</h1>

      <template v-if="library.configured">
        <PathPicker class="grow" :path="library.path" @pick="library.setPath($event)" />
        <span v-if="dragging" class="tag">{{ t('library.add.drop') }}</span>

        <!-- Оба способа пополнить библиотеку стоят здесь, над списком:
             в подвале панели их не видно, когда список длинный, а он
             бывает на две сотни строк.

             Одной кнопкой с меню, а не двумя подряд: их два, они об одном,
             и рядом с путём читались как три равноправных органа. Место,
             которое они занимали, отдано пути — ради него ряд и существует.
             Пока открыта форма вставки, предлагать открыть её ещё раз
             незачем. -->
        <MenuButton
          v-if="library.available"
          class="spacer"
          :icon="Plus"
          :label="t('library.add.action')"
        >
          <button type="button" role="menuitem" @click="addFile">
            {{ t('library.add.file') }}
          </button>
          <button v-if="!pasting" type="button" role="menuitem" @click="pasting = true">
            {{ t('library.paste.action') }}
          </button>
        </MenuButton>
      </template>
    </header>

    <!-- Папки нет: настройка и есть этот раздел, уводить некуда — выбор
         стоит здесь же, вместе с объяснением, зачем он. -->
    <div v-if="!library.configured" class="screen-body">
      <div class="screen-pad">
        <div class="empty">
          <p>{{ t('library.path.howto') }}</p>

          <div v-if="library.scanning" class="bar indet"><i></i></div>

          <div class="field">
            <label class="t-label" for="library-path">{{ t('library.path.label') }}</label>
            <PathPicker
              id="library-path"
              :path="library.path"
              :empty="t('library.path.empty')"
              @pick="library.setPath($event)"
            />
            <!-- Библиотека независима от общих моделей, но лежать рядом
                 с ними удобно: они обычно на просторном диске. Подсказка,
                 не правило. -->
            <p v-if="suggested" class="hint">{{ t('library.path.suggested') }}</p>
          </div>

          <button
            v-if="suggested"
            class="btn primary"
            type="button"
            @click="library.setPath(suggested)"
          >
            {{ t('library.path.useSuggested') }}
          </button>

          <p class="hint">{{ t('library.path.portable') }}</p>
        </div>
      </div>
    </div>

    <!-- Недоступная папка не выводит приложение из строя: говорим
         и продолжаем работать, остальные разделы не затронуты. -->
    <div v-else-if="!library.available" class="screen-body">
      <div class="screen-pad">
        <!-- Блок `.empty`, а не строка: это целый экран, и работа у него
             та же, что у экрана без сборок — сказать, что делать дальше.
             Класс стоял на самом абзаце, а кнопка лежала снаружи, поэтому
             правило `.empty > p` до текста не доставало и он выводился
             размером с обычный. -->
        <div class="empty">
          <p>{{ t('library.path.unavailable') }}</p>
          <button type="button" class="btn secondary" @click="library.rescan()">
            <RotateCw class="ico" />
            {{ t('library.refresh') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Вставка занимает место списка, а не всплывает над ним: тот же
         компонент встанет на вкладку «Воркфлоу» у сборки, где поверх
         нашего HTML лежит нативное окно ComfyUI. -->
    <div v-else-if="pasting" class="screen-body">
      <div class="screen-pad">
        <WorkflowPasteForm
          :save="library.addText"
          @done="onPasted"
          @cancel="pasting = false"
        />
      </div>
    </div>

    <div v-else class="screen-body">
      <div class="split-master">
        <div class="pane">
          <div class="pane-head">
            <!-- Множественный выбор объявляется отсюда, а не заводится сам
                 от первой отметки. Пока чекбоксы висели в каждой строке
                 всегда, попасть в этот режим можно было только случайно,
                 ткнув в маленький квадратик.
                 Переключатель стоит до поля поиска, а не после: он меняет,
                 чем список является — набором для выбора или для отметок, —
                 и относится к нему целиком. Поиск и фильтр избранного лишь
                 сужают показанное, и потому стоят вместе по другую сторону
                 поля. -->
            <button
              type="button"
              class="btn ghost"
              :aria-pressed="library.multi"
              :title="t('library.multi.action')"
              :aria-label="t('library.multi.action')"
              @click="library.setMulti(!library.multi)"
            >
              <ListChecks class="ico" />
            </button>
            <!-- Поиск тянется на всю ширину панели: в поле на 190 пикселей
                 обрезалась даже подсказка. -->
            <input
              v-model="library.query"
              class="input search"
              type="search"
              :placeholder="t('library.search')"
            />
            <!-- Нажатое состояние красится по `aria-pressed`: правило
                 добавлено в дизайн-систему, потому что без него включённый
                 фильтр ничем не отличался от выключенного. -->
            <button
              type="button"
              class="btn ghost"
              :aria-pressed="library.favoritesOnly"
              :title="t('library.favoritesOnly')"
              :aria-label="t('library.favoritesOnly')"
              @click="library.favoritesOnly = !library.favoritesOnly"
            >
              <span class="star" :class="{ off: !library.favoritesOnly }">★</span>
            </button>
          </div>

          <div class="scroll">
            <div class="scroll-pad">
              <!-- Повреждённые теги не имеют права уносить сами воркфлоу:
                   файлы на месте, и об этом надо сказать прямо — над
                   списком, которого это касается целиком. -->
              <p v-if="library.scan?.manifestBroken" class="hint bad">
                {{ t('library.manifestBroken') }}
              </p>

              <div
                v-if="library.visible.length"
                class="wf-list"
                :class="{ picking: library.multi }"
              >
                <!-- В обычном режиме строка сама и есть орган: она
                     выбирает воркфлоу, поэтому она кнопка. В режиме
                     отметок отмечает только квадратик, а строка не делает
                     ничего — и кнопкой быть перестаёт. Оставить её кнопкой
                     значило бы держать на экране орган, который ни на что
                     не отвечает: он ловил бы фокус и Enter вхолостую.
                     Заодно это разрешает вложенность — кнопка внутри
                     кнопки невалидна, а внутри `div` отметка становится
                     настоящей кнопкой со своей клавиатурой. -->
                <component
                  :is="library.multi ? 'div' : 'button'"
                  v-for="item in library.visible"
                  :key="item.path"
                  :type="library.multi ? undefined : 'button'"
                  class="wf-row"
                  :class="{
                    on: library.multi
                      ? library.marked.has(item.path)
                      : item.path === library.selected,
                    lost: item.lost,
                  }"
                  @click="onRow(item)"
                >
                  <!-- Отметка и избранное — разные органы. Раньше это был
                       один значок: показывал избранное, а по клику ставил
                       отметку, то есть отвечал не на тот вопрос, который
                       задавал.
                       Роль `checkbox`, а не нажатая кнопка: это состояние
                       строки, а не действие над ней. Подписью служит имя
                       воркфлоу — переводить его нельзя, а «✓» внутри
                       не говорит читающему с экрана ничего. -->
                  <button
                    v-if="library.multi"
                    type="button"
                    class="check"
                    role="checkbox"
                    :class="{ on: library.marked.has(item.path) }"
                    :aria-checked="library.marked.has(item.path)"
                    :aria-label="item.name"
                    @click="library.toggleMark(item.path)"
                  ><CheckGlyph /></button>
                  <span
                    class="star"
                    :class="{ off: !item.meta.favorite }"
                    :title="item.meta.favorite ? t('library.unstar') : t('library.star')"
                    @click.stop="library.toggleFavorite(item)"
                  >★</span>
                  <span class="nm">{{ item.name }}</span>
                  <!-- Метка состояния лежит внутри `.tags`, а не рядом
                       четвёртым ребёнком: колонок у строки три, и всё,
                       что не поместилось, грид переносил во вторую строку
                       вместе с отступом между строками. Строка от этого
                       была выше на пустое место, которого не видно. -->
                  <span class="tags">
                    <span v-for="tag in item.meta.tags" :key="tag" class="tag">{{ tag }}</span>
                    <span v-if="item.lost" class="tag stop">{{ t('library.lost') }}</span>
                    <span v-else-if="item.broken" class="tag warn">{{ t('library.broken') }}</span>
                  </span>
                </component>
              </div>
              <EmptyNote v-else>{{ t('library.nothingFound') }}</EmptyNote>
            </div>
          </div>

          <!-- Счётчик внизу списка, а не в его шапке: шапка занята поиском,
               а число отвечает на вопрос о том, что под ним. -->
          <div class="pane-foot">
            <span class="t-label">{{ t('library.summary', library.items.length) }}</span>
          </div>
        </div>

        <!-- Множественный выбор заменяет панель целиком, а не дописывает
             блок под разбором одного воркфлоу. Раньше панель жила в двух
             режимах разом: заголовок со звёздочкой говорил про выбранный
             воркфлоу, а тело под ним — про все отмеченные. -->
        <div v-if="library.multi" class="pane">
          <!-- Кнопка стоит в шапке, а счётчик отмеченного — в подвале,
               как и счётчик списка слева: обе панели отвечают на вопрос
               «сколько тут» внизу, а действие держат наверху, где его
               видно и при пустом списке сборок.
               Пока на экране отчёт, предлагать начать ещё одну запись
               поверх идущей незачем. -->
          <div class="pane-head">
            <span class="title">{{ t('library.multi.title') }}</span>
            <button
              v-if="!library.bulk"
              type="button"
              class="btn primary"
              :disabled="!library.marked.size || !library.markedTargets.size"
              @click="library.pushMany([...library.markedTargets])"
            >
              {{ t('library.multi.push') }}
            </button>
          </div>

          <div class="scroll">
            <div class="scroll-pad">
              <!-- Ход операции занимает место списка: пока идёт перенос
                   двадцати, выбирать сборки не во что. -->
              <div v-if="library.bulk" class="group">
                <!-- В счётчике — удавшиеся операции, а не пройденные пары.
                     Считать попытки и называть их сделанным значит врать
                     ровно там, где пользователь ищет правду: четыре пары,
                     каждая упёрлась в занятое имя, и «скопировано 4 из 4»
                     говорило о записи, которой не было ни одной.
                     Полоса при этом идёт по пройденному: её вопрос —
                     «сколько ещё ждать», и на отказах она обязана двигаться,
                     иначе операция снова выглядит зависшей. -->
                <p class="t-sm">
                  {{ t('library.bulk.progress', {
                    done: library.bulk.ok.length,
                    total: library.bulk.total,
                  }) }}
                </p>
                <div class="bar">
                  <i :style="{ width: `${(library.bulk.done / library.bulk.total) * 100}%` }"></i>
                </div>
                <!-- Отказы перечислены по одному и с причиной. Раньше это
                     была одна строка из склеенных запятой пар «файл →
                     опознаватель сборки», без единого слова о том, почему
                     не прошло: пользователь видел, что что-то не удалось,
                     и не мог узнать ни что делать, ни с какой сборкой это
                     случилось — опознаватель вида `i1786962802438` ему
                     нигде больше не показывают. -->
                <template v-if="library.bulk.failed.length">
                  <p class="hint bad">
                    {{ t('library.bulk.failed', library.bulk.failed.length) }}
                  </p>
                  <div class="fails">
                    <div
                      v-for="fail in library.bulk.failed"
                      :key="`${fail.workflow}/${fail.instanceId}`"
                      class="fail"
                    >
                      <span class="fail-pair">
                        {{ fail.workflow }} → {{ instanceName(fail.instanceId) }}
                      </span>
                      <span class="fail-why">
                        {{ fail.error ? errorText(fail.error) : t('library.bulk.nameTaken') }}
                      </span>
                    </div>
                  </div>
                  <!-- Совет один на весь список, а не в каждой строке:
                       причина у отказов часто одна и та же, и на четырёх
                       отказах он повторялся четыре раза подряд. -->
                  <p v-if="library.bulk.failed.some((f) => !f.error)" class="hint">
                    {{ t('library.bulk.replaceHint') }}
                  </p>
                </template>
                <!-- Чем кончилось, сказано словами, а не оставлено читаться
                     из двух чисел. Полоса и счётчик отвечают «сколько», но
                     на вопрос «прошло или нет» не отвечают вовсе: остановка
                     на середине и успешный конец выглядели одинаково —
                     цифрами, которые перестали меняться. -->
                <p v-else-if="!library.bulk.running" class="hint">
                  {{ library.bulk.done === library.bulk.total
                    ? t('library.bulk.done')
                    : t('library.bulk.stopped') }}
                </p>
                <!-- Кнопка смотрит на признак «идёт», а не на «сделано
                     меньше, чем всего». По сравнению прерванная операция
                     навсегда оставалась идущей, и отчёт после отмены
                     нечем было закрыть. -->
                <div class="row">
                  <button
                    v-if="library.bulk.running"
                    type="button"
                    class="btn danger"
                    @click="library.cancel()"
                  >
                    {{ t('common.cancel') }}
                  </button>
                  <button v-else type="button" class="btn ghost" @click="library.clearBulk()">
                    {{ t('common.close') }}
                  </button>
                </div>
              </div>

              <!-- Сборки отмечаются, а не запускают запись по клику:
                   файлы пишутся на диск необратимо, и начинать это
                   попаданием мышью в строку нельзя. Кладёт кнопка внизу,
                   которая так и подписана. -->
              <template v-else>
                <div v-if="targets.length" class="pick-list">
                  <button type="button" class="pick-head" @click="toggleAllTargets">
                    <span
                      class="check"
                      :class="{ on: allTargets, mixed: someTargets }"
                    ><CheckGlyph /></span>
                    <span>{{ t('library.pick.all') }}</span>
                  </button>
                  <button
                    v-for="instance in targets"
                    :key="instance.id"
                    type="button"
                    class="pick-row"
                    @click="library.toggleTarget(instance.id)"
                  >
                    <span
                      class="check"
                      :class="{ on: library.markedTargets.has(instance.id) }"
                    ><CheckGlyph /></span>
                    <span
                      class="chip"
                      :style="{ '--instance-accent': accentVar(instance.accent) }"
                    ></span>
                    <span class="nm">{{ instance.name }}</span>
                  </button>
                </div>
                <EmptyNote v-else>{{ t('library.noInstances') }}</EmptyNote>
              </template>
            </div>
          </div>

          <div class="pane-foot">
            <span class="t-label">{{ t('library.bulk.title', library.marked.size) }}</span>
          </div>
        </div>

        <div v-else class="pane">
          <div v-if="library.current" class="pane-head">
            <span class="title">{{ library.current.name }}</span>
            <!-- Значок, а не подпись: по надписи «В избранное» не понять,
                 текущее это состояние или предлагаемое действие. Цвет
                 звёздочки отвечает на вопрос сразу, а что произойдёт
                 по нажатию — говорит подсказка. -->
            <button
              type="button"
              class="star lg"
              :class="{ off: !library.current.meta.favorite }"
              :aria-pressed="library.current.meta.favorite"
              :title="library.current.meta.favorite ? t('library.unstar') : t('library.star')"
              @click="library.toggleFavorite(library.current)"
            >★</button>
          </div>

          <!-- Три набора данных в панели на 320 пикселей не помещаются
               столбиком: совместимость сама по себе длинная, а заметка
               и теги под ней оказывались за краем экрана. -->
          <nav v-if="library.current" ref="sideTabsBar" class="tabs" role="tablist">
            <span class="tabs-pill" aria-hidden="true"></span>
            <button
              v-for="item in SIDES"
              :key="item"
              type="button"
              role="tab"
              :aria-selected="side === item"
              @click="side = item"
            >
              {{ t(`library.side.${item}`) }}
              <span v-if="item === 'tags' && library.current.meta.tags.length" class="n">
                {{ library.current.meta.tags.length }}
              </span>
            </button>
          </nav>

          <div class="scroll">
            <div class="scroll-pad">
              <EmptyNote v-if="!library.current">{{ t('library.pickOne') }}</EmptyNote>

              <template v-else>
                <!-- Файла нет, а запись осталась. Единственное разумное
                     действие — убрать запись, файлов это не касается. -->
                <div v-if="library.current.lost" class="group">
                  <p class="t-md">{{ t('library.lostTitle') }}</p>
                  <p class="t-sm">{{ t('library.lostBody', { path: library.current.path }) }}</p>
                  <button
                    type="button"
                    class="btn danger"
                    @click="library.forget(library.current.path)"
                  >
                    {{ t('library.forget') }}
                  </button>
                </div>

                <p v-else-if="library.current.broken" class="hint bad">
                  {{ t('library.brokenBody') }}
                </p>

                <template v-else>
                  <div v-if="side === 'where'" class="group">
                    <div v-if="targets.length" class="compat">
                      <template v-for="instance in targets" :key="instance.id">
                        <div
                          class="compat-row"
                          :class="{
                            ok: library.compatOf(instance.id)?.missing.length === 0
                              && library.compatOf(instance.id)?.source !== 'unknown',
                            warn: (library.compatOf(instance.id)?.missing.length ?? 0) > 0,
                          }"
                        >
                          <span
                            class="chip"
                            :style="{ '--instance-accent': accentVar(instance.accent) }"
                          ></span>
                          <span class="nm">{{ instance.name }}</span>
                          <!-- Уход на страницу сборки, сразу в её раздел
                               «Воркфлоу». Ссылка стоит в строке сборки,
                               а не в шапке панели: в шапке показан воркфлоу,
                               и какая из сборок там имеется в виду — вопрос
                               без ответа. Только значок: подпись вытеснила бы
                               из строки то, ради чего строка есть. -->
                          <RouterLink
                            class="act open"
                            :to="`/instances/${instance.id}?tab=workflows`"
                            :title="t('library.openInInstance', { name: instance.name })"
                            :aria-label="t('library.openInInstance', { name: instance.name })"
                          >
                            <ExternalLink class="ico" />
                          </RouterLink>
                          <!-- Три состояния, и «неизвестно» не выдаётся
                               за «всё хорошо»: зелёная галочка без оснований
                               хуже её отсутствия. -->
                          <span class="compat-note">
                            {{
                              library.compatOf(instance.id)?.source === 'unknown'
                                ? t('library.compatUnknown')
                                : (library.compatOf(instance.id)?.missing.length ?? 0) > 0
                                  ? t('library.compatMissing', library.compatOf(instance.id)!.missing.length)
                                  : library.compatOf(instance.id)?.source === 'cached'
                                    ? t('library.compatCached')
                                    : t('library.compatOk')
                            }}
                          </span>
                          <!-- Нехватка нод предупреждает, но не запрещает:
                               пользователь вправе положить воркфлоу и
                               доустановить ноды потом. Значок вместо
                               подписи: состояние видно цветом и формой,
                               а что произойдёт — говорит подсказка. -->
                          <button
                            type="button"
                            class="act"
                            :class="{ on: added(instance.id) }"
                            :disabled="pushing === instance.id"
                            :title="added(instance.id)
                              ? t('library.push.again')
                              : t('library.push.action')"
                            @click="push(instance.id)"
                          >
                            <span class="icon-swap" :data-state="added(instance.id) ? 'b' : 'a'">
                              <Plus class="ico" data-icon="a" />
                              <Check class="ico" data-icon="b" />
                            </span>
                          </button>
                        </div>
                        <div
                          v-if="(library.compatOf(instance.id)?.missing.length ?? 0) > 0"
                          class="missing"
                        >
                          {{ library.compatOf(instance.id)!.missing.join(' · ') }}
                        </div>
                      </template>
                    </div>
                    <EmptyNote v-else>{{ t('library.noInstances') }}</EmptyNote>
                  </div>

                  <!-- Заметка и теги правятся одной формой: они лежат
                       в одной записи манифеста, и сохранять их порознь
                       значило бы писать файл дважды подряд. -->
                  <div v-else-if="side === 'note'" class="group">
                    <template v-if="editing">
                      <textarea v-model="noteDraft" class="input area" rows="6"></textarea>
                      <div class="row">
                        <button
                          type="button"
                          class="btn primary"
                          @click="saveMeta(library.current)"
                        >
                          {{ t('common.save') }}
                        </button>
                        <button type="button" class="btn ghost" @click="editing = false">
                          {{ t('common.cancel') }}
                        </button>
                      </div>
                    </template>
                    <template v-else>
                      <!-- Заметка и её отсутствие — разные вещи, и вид
                           у них разный: сама заметка написана человеком
                           и читается как текст, а «заметки пока нет» —
                           сообщение о пустоте, вполголоса. Раньше это была
                           одна строка через `||`, и пустота выглядела
                           заметкой. -->
                      <p v-if="library.current.meta.note" class="t-sm">
                        {{ library.current.meta.note }}
                      </p>
                      <EmptyNote v-else>{{ t('library.noNote') }}</EmptyNote>
                      <button
                        type="button"
                        class="btn ghost"
                        @click="startEdit(library.current)"
                      >
                        {{ t('common.edit') }}
                      </button>
                    </template>
                  </div>

                  <div v-else class="group">
                    <template v-if="editing">
                      <input
                        v-model="tagsDraft"
                        class="input"
                        :placeholder="t('library.tagsPlaceholder')"
                      />
                      <div class="row">
                        <button
                          type="button"
                          class="btn primary"
                          @click="saveMeta(library.current)"
                        >
                          {{ t('common.save') }}
                        </button>
                        <button type="button" class="btn ghost" @click="editing = false">
                          {{ t('common.cancel') }}
                        </button>
                      </div>
                    </template>
                    <template v-else>
                      <div v-if="library.current.meta.tags.length" class="row">
                        <span
                          v-for="tag in library.current.meta.tags"
                          :key="tag"
                          class="tag"
                        >{{ tag }}</span>
                      </div>
                      <EmptyNote v-else>{{ t('library.noTags') }}</EmptyNote>
                      <button
                        type="button"
                        class="btn ghost"
                        @click="startEdit(library.current)"
                      >
                        {{ t('common.edit') }}
                      </button>
                    </template>
                  </div>
                </template>
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>
