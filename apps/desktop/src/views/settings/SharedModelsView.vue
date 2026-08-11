<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';

import { useFormat } from '../../lib/format';
import { useInstancesStore } from '../../stores/instances';
import { useSharedStore } from '../../stores/shared';

const shared = useSharedStore();
const instances = useInstancesStore();
const { t } = useI18n();
const { bytes } = useFormat();

/** Предпросмотр YAML свёрнут: он длинный и нужен не всем. */
const showYaml = ref(false);

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

async function pick(): Promise<void> {
  const picked = await open({ directory: true, multiple: false });
  if (typeof picked !== 'string') return;

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
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('shared.title') }}</h1>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <div class="field">
          <label class="t-label" for="shared-root">{{ t('shared.root.label') }}</label>
          <div class="path-row">
            <div id="shared-root" class="input mono">
              <span>{{ shared.root?.path ?? t('shared.root.empty') }}</span>
            </div>
            <button class="btn secondary" type="button" @click="pick">
              {{ t('common.browse') }}
            </button>
          </div>

          <!-- Пока идёт обход дерева, показываем движение: на сотнях
               гигабайт это заметная пауза, и тишина читается как зависание. -->
          <div v-if="shared.scanning" class="bar indet"><i></i></div>

          <p v-else-if="!shared.configured" class="hint">{{ t('shared.root.howto') }}</p>

          <p v-else-if="!shared.available" class="hint">
            {{ t('shared.root.unavailable') }}
          </p>

          <p v-else class="hint">
            {{ t('shared.summary.categories', shared.recognized.length) }} ·
            {{ bytes(shared.scan?.totalBytes) }} ·
            {{ t('shared.summary.connected', shared.connected) }}
          </p>

          <!-- Корень внутри сборки. Не запрет, а предупреждение: удаление
               этой сборки унесёт общие модели всех остальных. -->
          <p v-if="insideInstance" class="hint bad">
            {{ t('shared.root.insideInstance', { name: insideInstance.name }) }}
          </p>
        </div>

        <!-- Смена корня действует сразу на все подключённые сборки,
             поэтому спрашиваем до, а не после. -->
        <div v-if="pending" class="group danger-zone">
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
        </div>

        <template v-if="shared.configured && shared.available">
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
          <p v-else class="empty">{{ t('shared.root.noCategories') }}</p>

          <p v-if="shared.blocked.length" class="hint">{{ t('shared.cat.whyBlocked') }}</p>

          <div v-if="shared.scan?.missing.length" class="group">
            <p class="hint">
              {{ t('shared.missing.hint', { list: shared.scan.missing.join(', ') }) }}
            </p>
            <button class="btn secondary" type="button" @click="shared.createMissing()">
              {{ t('shared.missing.create') }}
            </button>
          </div>

          <div class="toggle-row">
            <button
              class="toggle"
              :class="{ off: !shared.settings.makeDefaultTarget }"
              type="button"
              role="switch"
              :aria-checked="shared.settings.makeDefaultTarget"
              @click="shared.setDefaultTarget(!shared.settings.makeDefaultTarget)"
            ></button>
            <div>
              <div class="t-base">{{ t('shared.default.label') }}</div>
              <div class="hint">{{ t('shared.default.hint') }}</div>
              <!-- Настройка читается сборкой при старте. Молчать об этом
                   нельзя: переключивший тумблер у работающей сборки решит,
                   что она сломана. -->
              <div class="hint">{{ t('shared.default.restartNote') }}</div>
            </div>
          </div>

          <div class="group">
            <button class="btn ghost" type="button" @click="showYaml = !showYaml">
              {{ showYaml ? t('shared.yaml.hide') : t('shared.yaml.show') }}
            </button>
            <div v-if="showYaml" class="pane">
              <div class="pane-head">
                <span class="title">{{ t('shared.yaml.title') }}</span>
              </div>
              <div class="scroll">
                <pre class="console">{{ shared.yaml }}</pre>
              </div>
            </div>
          </div>
        </template>
      </div>
    </div>
  </section>
</template>
