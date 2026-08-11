<script setup lang="ts">
// Мастер установки — отдельный роут с шагами внутри, а не череда модалок.
// Шаги: архив → назначения → выполнение → итог. Шаг общих ресурсов придёт
// в Фазе 2.5 теми же компонентами, что в «Настройках».
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';

import type { ArchiveRecord, InstallTarget } from '../bindings';
import { errorText } from '../lib/errors';
import { accentVar, useFormat } from '../lib/format';
import { useInstallerStore } from '../stores/installer';

const wizard = useInstallerStore();
const { t } = useI18n();
const { bytes, moment } = useFormat();

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

const phaseText = computed(() => {
  const p = wizard.progress;
  if (!p) return t('install.run.registering');
  const name = p.targetName;
  return p.phase === 'copying'
    ? t('install.run.copying', { name })
    : t('install.run.extracting', { name });
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
  <section class="screen">
    <header class="screen-head">
      <RouterLink class="btn ghost" to="/install">{{ t('common.back') }}</RouterLink>
      <h1 class="t-lg">{{ t('install.wizard.title') }}</h1>
      <span class="t-sm">{{
        wizard.step === 'archive'
          ? t('install.wizard.step.archive')
          : wizard.step === 'targets'
            ? t('install.wizard.step.targets')
            : wizard.step === 'running'
              ? t('install.wizard.step.running')
              : t('install.wizard.step.done')
      }}</span>
    </header>

    <div class="screen-body">
      <div class="screen-pad">
        <!-- ------------------------------------------------ шаг «архив» -->
        <template v-if="wizard.step === 'archive'">
          <div class="row">
            <button type="button" class="btn primary" @click="pickArchive">
              {{ t('install.archive.choose') }}
            </button>
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
          <div class="group">
            <div class="meta">
              <span>{{ wizard.info.label }}</span>
              <span>{{ t('install.archive.files', wizard.info.files) }}</span>
              <span>
                {{ t('install.archive.unpacked', {
                  size: bytes(wizard.info.totalUncompressed),
                }) }}
              </span>
            </div>
            <p v-if="wizard.info.singleRoot" class="hint">
              {{ t('install.archive.root', { name: wizard.info.singleRoot }) }}
            </p>
            <p v-else class="hint">{{ t('install.archive.noRoot') }}</p>
          </div>

          <div class="group">
            <span class="t-label">{{ t('install.targets.title') }}</span>
            <p class="hint">{{ t('install.targets.hint') }}</p>
            <p class="hint">{{ t('install.targets.needed', { size: needed }) }}</p>
          </div>

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

          <p class="hint">{{ t('install.run.note') }}</p>

          <div class="row">
            <button
              type="button"
              class="btn primary lg"
              :disabled="wizard.blocked || wizard.targets.some((x) => !x.name.trim())"
              @click="wizard.start()"
            >
              {{ t('install.run.start') }}
            </button>
          </div>
        </template>

        <!-- -------------------------------------------- шаг «выполнение» -->
        <template v-else-if="wizard.step === 'running'">
          <div class="group">
            <p class="t-md">{{ phaseText }}</p>
            <div class="bar">
              <i :style="{ width: `${percent}%` }"></i>
            </div>
            <!-- Байты остаются подписью: они понятны и полезны, просто мерой
                 прогресса быть не могут. -->
            <p v-if="wizard.progress" class="hint">
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
            <p v-if="wizard.progress" class="t-mono current">
              {{ wizard.progress.current }}
            </p>
          </div>

          <div class="row">
            <button type="button" class="btn danger" @click="wizard.cancel()">
              {{ t('install.run.cancel') }}
            </button>
          </div>
        </template>

        <!-- -------------------------------------------------- шаг «итог» -->
        <template v-else>
          <div class="group">
            <p class="t-md">{{ t('install.done.title') }}</p>
            <p class="t-sm">
              {{ t('install.done.added', wizard.created.length) }}
            </p>
          </div>

          <div class="cards">
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

          <div class="row">
            <RouterLink class="btn secondary" to="/instances">
              {{ t('nav.instances') }}
            </RouterLink>
            <button type="button" class="btn ghost" @click="wizard.reset()">
              {{ t('install.wizard.title') }}
            </button>
          </div>
        </template>
      </div>
    </div>
  </section>
</template>

<style scoped>
.target {
  display: block;
}
.current {
  color: var(--ink-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
