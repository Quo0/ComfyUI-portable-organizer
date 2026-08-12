<script setup lang="ts">
// Библиотека воркфлоу: мастер-детейл.
//
// Две области прокрутки на экране — оговорённое исключение из правила
// «одна на экран»: список слева и детали справа скроллятся порознь, иначе
// панель деталей уезжала бы вместе с двумя сотнями воркфлоу.
import { computed, onMounted, ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from 'vue-i18n';

import type { LibItem } from '../stores/workflows';
import { commands } from '../bindings';
import { accentVar } from '../lib/format';
import { useRunStore } from '../stores/run';
import { useInstancesStore } from '../stores/instances';
import { useUiStore } from '../stores/ui';
import { useWorkflowsStore } from '../stores/workflows';

const library = useWorkflowsStore();
const instances = useInstancesStore();
const run = useRunStore();
const ui = useUiStore();
const { t } = useI18n();

/** Черновик заметки и тегов: пишем в манифест по кнопке, а не на каждый ввод. */
const noteDraft = ref('');
const tagsDraft = ref('');
const editing = ref(false);

onMounted(async () => {
  if (!library.loaded) await library.load();
  if (!instances.loaded) await instances.load();
});

/** Недоступную сборку целью не предлагаем: писать некуда. */
const targets = computed(() => instances.items.filter((i) => i.available));

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
    const running = run.statusOf(instanceId)?.state === 'running';
    ui.pushOk(running ? t('library.push.doneRunning') : t('library.push.done'));
  } finally {
    pushing.value = null;
  }
}

async function addFile(): Promise<void> {
  const picked = await open({
    multiple: false,
    filters: [{ name: 'JSON', extensions: ['json'] }],
  });
  if (typeof picked !== 'string') return;
  if (await library.addFile(picked)) ui.pushOk(t('library.add.done'));
}
</script>

<template>
  <section class="screen">
    <header class="screen-head">
      <h1 class="t-lg">{{ t('workflows.title') }}</h1>
      <span class="head-spacer"></span>
      <button
        type="button"
        class="btn secondary"
        :disabled="!library.configured || !library.available"
        @click="addFile"
      >
        {{ t('library.add.file') }}
      </button>
    </header>

    <div class="screen-body">
      <!-- Библиотека не задана: вести надо туда, где её задают,
           а не показывать пустой список. -->
      <div v-if="!library.configured" class="screen-pad">
        <p class="empty">{{ t('library.path.empty') }}</p>
        <p class="hint">{{ t('library.path.howto') }}</p>
        <RouterLink class="btn primary" to="/settings/workflow-library">
          {{ t('library.path.setUp') }}
        </RouterLink>
      </div>

      <div v-else-if="!library.available" class="screen-pad">
        <p class="empty">{{ t('library.path.unavailable') }}</p>
        <div class="input mono"><span>{{ library.path }}</span></div>
        <button type="button" class="btn secondary" @click="library.rescan()">
          {{ t('library.retry') }}
        </button>
      </div>

      <div v-else class="split-master">
        <div class="pane">
          <div class="pane-head">
            <input
              v-model="library.query"
              class="input"
              type="search"
              :placeholder="t('library.search')"
            />
            <button
              type="button"
              class="btn ghost"
              :aria-pressed="library.favoritesOnly"
              @click="library.favoritesOnly = !library.favoritesOnly"
            >
              {{ t('library.favoritesOnly') }}
            </button>
          </div>

          <div class="scroll">
            <div class="scroll-pad">
              <div v-if="library.visible.length" class="wf-list">
                <button
                  v-for="item in library.visible"
                  :key="item.path"
                  type="button"
                  class="wf-row"
                  :class="{ on: item.path === library.selected, lost: item.lost }"
                  @click="library.select(item.path)"
                >
                  <span class="star" :class="{ off: !item.meta.favorite }">★</span>
                  <span class="nm">{{ item.name }}</span>
                  <span class="tags">
                    <span v-for="tag in item.meta.tags" :key="tag" class="tag">{{ tag }}</span>
                  </span>
                  <span v-if="item.lost" class="tag stop">{{ t('library.lost') }}</span>
                  <span v-else-if="item.broken" class="tag warn">{{ t('library.broken') }}</span>
                  <span v-else class="n"></span>
                </button>
              </div>
              <p v-else class="empty">{{ t('library.nothingFound') }}</p>
            </div>
          </div>
        </div>

        <div class="pane">
          <div v-if="library.current" class="pane-head">
            <span class="title">{{ library.current.name }}</span>
            <button
              type="button"
              class="btn ghost"
              @click="library.toggleFavorite(library.current)"
            >
              {{ library.current.meta.favorite ? t('library.unstar') : t('library.star') }}
            </button>
          </div>

          <div class="scroll">
            <div class="scroll-pad">
              <p v-if="!library.current" class="empty">{{ t('library.pickOne') }}</p>

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
                  <div class="group">
                    <span class="t-label">{{ t('library.compat') }}</span>
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
                        </div>
                        <div
                          v-if="(library.compatOf(instance.id)?.missing.length ?? 0) > 0"
                          class="missing"
                        >
                          {{ library.compatOf(instance.id)!.missing.join(' · ') }}
                        </div>
                        <!-- Нехватка нод предупреждает, но не запрещает:
                             пользователь вправе положить воркфлоу и
                             доустановить ноды потом. -->
                        <div class="missing">
                          <button
                            type="button"
                            class="btn ghost"
                            :disabled="pushing === instance.id"
                            @click="push(instance.id)"
                          >
                            {{ t('library.push.action') }}
                          </button>
                        </div>
                      </template>
                    </div>
                    <p v-else class="hint">{{ t('library.noInstances') }}</p>
                  </div>

                  <div class="group">
                    <span class="t-label">{{ t('library.note') }}</span>
                    <template v-if="editing">
                      <input
                        v-model="tagsDraft"
                        class="input"
                        :placeholder="t('library.tagsPlaceholder')"
                      />
                      <textarea v-model="noteDraft" class="input" rows="3"></textarea>
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
                      <p class="t-sm">{{ library.current.meta.note || t('library.noNote') }}</p>
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
