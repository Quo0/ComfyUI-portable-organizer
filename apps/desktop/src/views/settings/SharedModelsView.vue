<script setup lang="ts">
// Общие модели: корень снаружи сборок и то, что в нём лежит.
//
// Экран устроен как мастер-детейл, а не лентой блоков: корень задаёт адрес
// всему, что ниже, а тумблер загрузок и предпросмотр конфига — это ответ
// на вопрос «что сборка от этого получит». Пока всё стояло одной лентой,
// список категорий утаскивал вниз и корень, и тумблер: на двадцати пяти
// папках до тумблера надо было доскроллить, а он относится не к списку,
// а к настройке целиком.
//
// Две области прокрутки — оговорённое исключение из правила «одна на экран»,
// то же, что у библиотеки воркфлоу: слева список категорий, справа конфиг,
// и скроллятся они порознь. Всё остальное закреплено и видно всегда.
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import EmptyNote from '../../components/EmptyNote.vue';
import PathPicker from '../../components/PathPicker.vue';
import Field from '../../components/ui/Field.vue';
import Group from '../../components/ui/Group.vue';
import Pane from '../../components/ui/Pane.vue';
import ScreenHeader from '../../components/ui/ScreenHeader.vue';
import Toggle from '../../components/ui/Toggle.vue';
import ToggleRow from '../../components/ui/ToggleRow.vue';
import { useFormat } from '../../lib/format';
import { withViewTransition } from '../../lib/motion';
import { useInstancesStore } from '../../stores/instances';
import { useSharedStore } from '../../stores/shared';

const shared = useSharedStore();
const instances = useInstancesStore();
const { t } = useI18n();
const { bytes } = useFormat();

/** Предпросмотр YAML свёрнут: он длинный и нужен не всем. */
const showYaml = ref(false);

/** Перехлёст ползунка тумблера играет только с первого клика, не с отрисовки. */
const defaultTargetTouched = ref(false);

function toggleDefaultTarget(): void {
  defaultTargetTouched.value = true;
  shared.setDefaultTarget(!shared.settings.makeDefaultTarget);
}

/** Показ и скрытие конфига — с переходом (transitions.dev): блок длинный, и без перехода появлялся рывком. */
function toggleYaml(): void {
  withViewTransition(() => { showYaml.value = !showYaml.value; });
}

/**
 * Выбранная, но ещё не применённая папка. Смена корня действует сразу на
 * все подключённые инстансы, поэтому спрашиваем до, а не после.
 */
const pending = ref<string | null>(null);

onMounted(async () => {
  if (!shared.loaded) await shared.load();
  if (!instances.loaded) await instances.load();
});

const sameCase = (a: string, b: string): boolean =>
  a.replace(/\\/g, '/').toLowerCase().startsWith(b.replace(/\\/g, '/').toLowerCase());

/**
 * Корень лежит внутри зарегистрированной сборки.
 *
 * Не запрещаем — бывает осмысленно, — но последствия неочевидны:
 * удаление этой сборки унесёт с собой общие модели всех остальных.
 */
const insideInstance = computed(() => {
  const path = shared.root?.path;
  if (!path) return null;
  return instances.items.find((i) => sameCase(path, i.path)) ?? null;
});

async function pick(picked: string): Promise<void> {
  // Первый выбор применяем сразу: менять нечего и предупреждать не о чем.
  if (!shared.configured || shared.connected === 0) {
    await shared.setRoot(picked);
    return;
  }
  pending.value = picked;
}

async function applyPending(): Promise<void> {
  if (!pending.value) return;
  await shared.setRoot(pending.value);
  pending.value = null;
}
</script>

<template>
  <var class="SharedModelsView">
    <section class="screen">
      <ScreenHeader>
        <h1 class="t-lg">{{ t('shared.title') }}</h1>
      </ScreenHeader>

      <!-- Закреплённая полоса под шапкой. Корень и то, что о нём известно,
           обязаны быть видны всегда: без них список категорий — просто набор
           имён папок неизвестно откуда. -->
      <div class="pinned">
        <Field>
          <label class="t-label" for="shared-root">{{ t('shared.root.label') }}</label>
          <PathPicker
            id="shared-root"
            :path="shared.root?.path"
            :empty="t('shared.root.empty')"
            @pick="pick"
          />

          <!-- Пока идёт обход дерева, показываем движение: на сотнях
               гигабайт это заметная пауза, и тишина читается как зависание. -->
          <div v-if="shared.scanning" class="bar indet"><i></i></div>

          <!-- Сводка о корне, а не о списке: она про папку, поэтому стоит
               под путём, а не в шапке панели с категориями. Что делать,
               когда корня нет или он недоступен, сказано на месте списка —
               там для этого есть целая область, а здесь только строка. -->
          <p v-else-if="shared.configured && shared.available" class="hint">
            {{ t('shared.summary.categories', shared.recognized.length) }} ·
            {{ bytes(shared.scan?.totalBytes) }} ·
            {{ t('shared.summary.connected', shared.connected) }}
          </p>

          <!-- Корень внутри сборки. Не запрет, а предупреждение: удаление
               этой сборки унесёт общие модели всех остальных. -->
          <p v-if="insideInstance" class="hint bad">
            {{ t('shared.root.insideInstance', { name: insideInstance.name }) }}
          </p>
        </Field>

        <!-- Смена корня действует сразу на все подключённые сборки,
             поэтому спрашиваем до, а не после. Вопрос стоит здесь же,
             у пути, который меняют, и уехать из виду не может. -->
        <Group v-if="pending" danger>
          <p class="t-md">{{ t('shared.root.changeTitle') }}</p>
          <p class="t-sm">
            {{ t('shared.root.changeBody', shared.connected) }}
          </p>
          <div class="input mono"><span>{{ pending }}</span></div>
          <div class="row">
            <button type="button" class="btn danger" @click="applyPending">
              {{ t('shared.root.changeConfirm') }}
            </button>
            <button type="button" class="btn ghost" @click="pending = null">
              {{ t('common.cancel') }}
            </button>
          </div>
        </Group>
      </div>

      <!-- Корня ещё нет: выбор стоит выше, а здесь сказано, что выбирать. -->
      <div v-if="!shared.configured" class="screen-body">
        <div class="screen-pad">
          <div class="empty">
            <p>{{ t('shared.root.howto') }}</p>
          </div>
        </div>
      </div>

      <!-- Первый обход ещё идёт: показывать пустой список рано, движение
           показывает полоса в закреплённой полосе выше. -->
      <div v-else-if="shared.scanning && !shared.scan" class="screen-body"></div>

      <!-- Папка недоступна — не ошибка приложения: говорим и продолжаем
           работать, остальные разделы не затронуты. -->
      <div v-else-if="!shared.available" class="screen-body">
        <div class="screen-pad">
          <div class="empty">
            <p>{{ t('shared.root.unavailable') }}</p>
          </div>
        </div>
      </div>

      <div v-else class="screen-body">
        <!-- Модификатор `shared` включает схлопывание колонок в строки
             на узком экране: имена папок категорий моноширинные и не
             обрезаются, а на минимальном окне списку остаётся 168 пикселей.
             Порог и поведение — в дизайн-системе, рядом с `.split-master`. -->
        <div class="split-master shared">
          <!-- Слева то, что растёт: категорий бывает двадцать пять, и только
               этот список имеет право скроллиться. -->
          <Pane>
            <div class="pane-head">
              <span class="title">{{ t('shared.cats.title') }}</span>
            </div>

            <div class="scroll">
              <div class="scroll-pad">
                <div v-if="shared.scan?.categories.length" class="cats">
                  <div
                    v-for="cat in shared.scan.categories"
                    :key="cat.folder"
                    class="cat"
                    :class="{ unknown: cat.status === 'unknown', blocked: cat.status === 'blocked' }"
                  >
                    <code>{{ cat.folder }}</code>
                    <span class="n">
                      <template v-if="cat.status === 'blocked'">{{ t('shared.cat.notShared') }}</template>
                      <template v-else-if="cat.files === 0">{{ t('shared.cat.empty') }}</template>
                      <template v-else>
                        {{ t('shared.cat.files', cat.files) }} · {{ bytes(cat.sizeBytes) }}
                      </template>
                    </span>
                    <span
                      class="tag"
                      :class="{ warn: cat.status === 'unknown', stop: cat.status === 'blocked' }"
                    >
                      {{ t(`shared.cat.status.${cat.status}`) }}
                    </span>
                  </div>
                </div>

                <!-- Пустой корень — не ошибка: папку задают заранее, до того
                     как в ней появятся модели. -->
                <EmptyNote v-else>{{ t('shared.root.noCategories') }}</EmptyNote>

                <p v-if="shared.blocked.length" class="hint">{{ t('shared.cat.whyBlocked') }}</p>

                <Group v-if="shared.scan?.missing.length">
                  <p class="hint">
                    {{ t('shared.missing.hint', { list: shared.scan.missing.join(', ') }) }}
                  </p>
                  <button class="btn secondary" type="button" @click="shared.createMissing()">
                    {{ t('shared.missing.create') }}
                  </button>
                </Group>
              </div>
            </div>
          </Pane>

          <!-- Справа то, что список настраивает. Тумблер и кнопка конфига
               закреплены: они относятся к настройке целиком, а не к строке
               списка, и доскролливаться до них незачем. -->
          <div class="side">
            <ToggleRow>
              <Toggle
                :checked="shared.settings.makeDefaultTarget"
                :touched="defaultTargetTouched"
                @click="toggleDefaultTarget"
              />
              <div>
                <div class="t-base">{{ t('shared.default.label') }}</div>
                <div class="hint">{{ t('shared.default.hint') }}</div>
                <!-- Настройка читается сборкой при старте. Молчать об этом
                     нельзя: переключивший тумблер у работающей сборки решит,
                     что она сломана. -->
                <div class="hint">{{ t('shared.default.restartNote') }}</div>
              </div>
            </ToggleRow>

            <div class="row">
              <button
                class="btn ghost"
                type="button"
                :aria-pressed="showYaml"
                @click="toggleYaml()"
              >
                {{ showYaml ? t('shared.yaml.hide') : t('shared.yaml.show') }}
              </button>
            </div>

            <!-- Конфиг длиннее, чем колонка: он занимает всё, что осталось
                 под кнопкой, и прокручивается внутри себя. -->
            <Pane v-if="showYaml">
              <div class="pane-head">
                <span class="title">{{ t('shared.yaml.title') }}</span>
              </div>
              <div class="scroll">
                <pre class="console">{{ shared.yaml }}</pre>
              </div>
            </Pane>
          </div>
        </div>
      </div>
    </section>
  </var>
</template>
