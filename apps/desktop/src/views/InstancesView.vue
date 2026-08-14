<script setup lang="ts">
// Список сборок.
//
// Кнопки «Добавить» здесь нет намеренно: сборки заводит раздел «Установка»,
// и двух дверей в одно место быть не должно. Пустое состояние туда и ведёт.
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import InstallForks from '../components/InstallForks.vue';
import PathText from '../components/PathText.vue';
import StatusPill from '../components/StatusPill.vue';
import type { Instance } from '../bindings';
import { accentVar, initial, useFormat } from '../lib/format';
import { displayStatus } from '../lib/status';
import { useInstancesStore } from '../stores/instances';
import { useRunStore } from '../stores/run';

const instances = useInstancesStore();
const run = useRunStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

type Sort = 'name' | 'lastRun' | 'size';

const query = ref('');
const sort = ref<Sort>('name');

const SORTS: Sort[] = ['name', 'lastRun', 'size'];

/**
 * Поиск идёт и по пути: сборки часто называют одинаково («SDXL», «тест»),
 * а путь у каждой свой, и именно по нему их и различают в проводнике.
 */
const visible = computed(() => {
  const needle = query.value.trim().toLowerCase();
  const found = instances.items.filter((instance) =>
    needle === ''
      ? true
      : [instance.name, instance.description, instance.path]
          .join(' ')
          .toLowerCase()
          .includes(needle),
  );

  return [...found].sort((a, b) => {
    if (sort.value === 'size') return (b.sizeBytes ?? 0) - (a.sizeBytes ?? 0);
    // Ни разу не запускавшиеся уезжают вниз, а не притворяются
    // запущенными в начале эпохи.
    if (sort.value === 'lastRun') return (b.lastStartedAt ?? 0) - (a.lastStartedAt ?? 0);
    return a.name.localeCompare(b.name);
  });
});

/** Размер показывается, только когда посчитан. */
function sizeText(instance: Instance): string {
  return bytes(instance.sizeBytes);
}

function lastRun(instance: Instance): string {
  const when = moment(instance.lastStartedAt);
  return when ? t('instances.field.lastRun', { when }) : t('instances.field.lastRunNever');
}

onMounted(() => {
  if (!instances.loaded) void instances.load();
});
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('instances.title') }}</h1>
      <!-- Рядом с заголовком «Инстансы» слово «инстансов» — то же слово
           дважды. Число молча, фраза целиком остаётся подсказкой. -->
      <span
        v-if="instances.items.length"
        class="t-sm"
        :title="t('instances.count', instances.items.length)"
      >{{ instances.items.length }}</span>
      <span class="head-spacer"></span>
      <template v-if="instances.items.length > 1">
        <input
          v-model="query"
          class="input search"
          type="search"
          :placeholder="t('instances.search')"
        />
        <div class="seg" :aria-label="t('instances.sort.label')">
          <button
            v-for="option in SORTS"
            :key="option"
            type="button"
            :aria-pressed="sort === option"
            @click="sort = option"
          >
            {{ t(`instances.sort.${option}`) }}
          </button>
        </div>
      </template>
    </header>

    <!-- Единственная область прокрутки экрана: шапка с заголовком и поиском
         остаётся на месте, иначе при длинном списке непонятно, где ты. -->
    <div class="screen-body">
      <div class="screen-pad wide">
        <div v-if="visible.length" class="cards grid">
          <RouterLink
            v-for="instance in visible"
            :key="instance.id"
            class="card"
            :class="{ gone: !instance.available }"
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

              <!-- У пропавшей сборки версия, порт и размер описывают то,
                   чего на диске уже нет. Вместо них — путь: единственное,
                   с чем можно пойти разбираться. -->
              <div v-if="!instance.available" class="meta">
                <span><PathText :path="instance.path" /></span>
              </div>

              <div v-else class="meta">
                <span v-if="instance.comfyVersion">{{ instance.comfyVersion }}</span>
                <span>:{{ instance.preferredPort }}</span>
                <span v-if="sizeText(instance)">{{ sizeText(instance) }}</span>
                <!-- Подключение к общим моделям видно из списка: иначе
                     разобраться, почему у двух сборок разный набор
                     чекпоинтов, можно только зайдя в каждую. -->
                <span v-if="instance.shared?.enabled" class="tag">
                  {{ t('shared.instance.badge') }}
                </span>
              </div>

              <!-- Из какого архива развёрнута сборка, видно прямо в списке:
                   две версии рядом различаются не номером ComfyUI, а вариантом
                   архива, и без этой строки различить их можно было бы только
                   открыв папки (US-INST-07/AC-5). -->
              <div class="src">
                <div v-if="instance.source">
                  {{
                    t('instances.field.source', {
                      archive: instance.source.archiveLabel,
                      when: moment(instance.source.installedAt),
                    })
                  }}
                </div>
                <div>{{ lastRun(instance) }}</div>
              </div>
            </div>
          </RouterLink>
        </div>

        <p v-else-if="instances.items.length" class="hint">
          {{ t('instances.nothingFound') }}
        </p>

        <!-- Отдельного Welcome-экрана нет: его роль берёт это состояние.
             Поэтому здесь не ссылка на «Установку», а сама развилка: у того,
             у кого ещё ничего нет, выбор между «папка уже есть»
             и «распаковать архив» — первое же решение, и прятать его
             за лишним переходом незачем. -->
        <div v-else class="empty">
          <h4>{{ t('instances.empty.title') }}</h4>
          <p>{{ t('instances.empty.body') }}</p>
          <InstallForks />
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
/* Поиск в шапке: тянется, но не расползается на всю ширину окна. */
.search {
  flex: 1;
  max-width: 320px;
}
</style>
