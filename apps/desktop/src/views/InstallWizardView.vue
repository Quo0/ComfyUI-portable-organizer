<script setup lang="ts">

import { ArrowLeft, Ban, Check, Pencil, X } from '@lucide/vue';
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';
import { RouterLink } from 'vue-router';

import EmptyNote from '../components/EmptyNote.vue';
import Card from '../components/ui/Card.vue';
import Field from '../components/ui/Field.vue';
import PathPicker from '../components/PathPicker.vue';
import PathText from '../components/PathText.vue';
import StatusPill from '../components/StatusPill.vue';
import TargetForm from '../components/TargetForm.vue';
import Group from '../components/ui/Group.vue';
import KeyValueList from '../components/ui/KeyValueList.vue';
import KeyValueRow from '../components/ui/KeyValueRow.vue';
import Pane from '../components/ui/Pane.vue';
import StepBar from '../components/ui/StepBar.vue';
import Toggle from '../components/ui/Toggle.vue';
import ToggleRow from '../components/ui/ToggleRow.vue';
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

const count = (value: number): string => n(value, 'integer');

const STEPS: WizardStep[] = ['archive', 'targets', 'shared', 'running', 'done'];
const current = computed(() => STEPS.indexOf(wizard.step));

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

const runCounter = computed(() => {
  const p = wizard.progress;
  if (!p?.targets) return '';
  return t('install.run.counter', { done: p.target, total: p.targets });
});

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

const percent = computed(() => {
  const p = wizard.progress;
  if (!p?.totalFiles) return 0;
  return Math.min(100, (p.doneFiles / p.totalFiles) * 100);
});

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
  if (await wizard.chooseArchive(picked)) wizard.setStep('targets');
}

async function useRecent(record: ArchiveRecord): Promise<void> {
  if (!record.available) return;
  if (await wizard.chooseArchive(record.path)) wizard.setStep('targets');
}

const needed = computed(() =>
  bytes((wizard.info?.totalUncompressed ?? 0) * wizard.targets.length),
);
</script>

<template>
  <var class="InstallWizardView">
    <section class="screen wizard-screen">

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

      <StepBar>
        <h2 class="title">{{ stepTitle }}</h2>
        <span v-if="wizard.step === 'done'" class="t-label">
          {{ t('install.done.added', wizard.created.length) }}
        </span>

        <span v-else-if="wizard.step === 'running' && runCounter" class="t-label">
          {{ runCounter }}
        </span>
        <span class="spacer"></span>

        <span v-if="wizard.step === 'archive'" class="acts">
          <RouterLink class="btn ghost" to="/install">
            <ArrowLeft class="ico" />
            {{ t('common.back') }}
          </RouterLink>
        </span>

        <span v-else-if="wizard.step === 'targets'" class="acts">
          <button type="button" class="btn ghost" @click="wizard.setStep('archive')">
            <ArrowLeft class="ico" />
            {{ t('common.back') }}
          </button>

          <button
            type="button"
            class="btn primary lg"
            :disabled="wizard.blocked"
            @click="wizard.setStep('shared')"
          >
            {{ t('install.wizard.next') }}
          </button>
        </span>

        <span v-else-if="wizard.step === 'shared'" class="acts">
          <button type="button" class="btn ghost" @click="wizard.setStep('targets')">
            <ArrowLeft class="ico" />
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
      </StepBar>

      <div v-if="wizard.step === 'targets' && wizard.info" class="pinned">
        <div class="meta">
          <span>{{ wizard.info.label }}</span>

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

          <template v-if="wizard.step === 'archive'">

            <p class="t-sm">{{ t('install.archive.lead') }}</p>

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

                <span class="bar indet grow"><i></i></span>
              </template>
            </div>

            <Group v-if="wizard.history.length">
              <span class="t-label">{{ t('install.archive.history') }}</span>

              <div class="cards grid">
                <Card
                  v-for="record in wizard.history"
                  :key="record.path"
                  :gone="!record.available"
                >
                  <div class="card-top">
                    <div class="card-name">{{ record.label }}</div>
                    <span v-if="!record.available" class="pill gone">
                      {{ t('install.archive.missing') }}
                    </span>
                  </div>

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
                </Card>
              </div>
            </Group>
          </template>

          <template v-else-if="wizard.step === 'targets' && wizard.info">
            <div class="cols targets">

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

              <Field>
                <span class="t-label">{{ t('install.targets.list') }}</span>
                <KeyValueList v-if="wizard.targets.length">
                  <template v-for="(target, index) in wizard.targets" :key="index">

                    <KeyValueRow editable :title="target.description || undefined">

                      <span class="lbl"><PathText :path="target.path" /></span>
                      <span class="val">{{ target.name }}</span>

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
                          <Pencil class="ico" />
                        </button>
                        <button
                          type="button"
                          class="act"
                          :title="t('install.targets.remove')"
                          :aria-label="t('install.targets.remove')"
                          @click="wizard.removeTarget(index)"
                        >
                          <X class="ico" />
                        </button>
                      </span>
                    </KeyValueRow>

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

                          <button
                            type="button"
                            class="act"
                            :title="t('common.cancel')"
                            :aria-label="t('common.cancel')"
                            @click="wizard.cancelEdit()"
                          >
                            <Ban class="ico" />
                          </button>
                          <button
                            type="button"
                            class="act"
                            :title="t('common.save')"
                            :aria-label="t('common.save')"
                            :disabled="!wizard.editReady"
                            @click="wizard.saveEdit()"
                          >
                            <Check class="ico" />
                          </button>
                        </span>
                      </template>
                    </TargetForm>
                  </template>
                </KeyValueList>
                <EmptyNote v-else>{{ t('install.targets.empty') }}</EmptyNote>

                <template v-for="(check, index) in wizard.checks" :key="`c${index}`">
                  <p
                    v-for="(problem, i) in check.errors"
                    :key="`ce${index}-${i}`"
                    class="hint bad"
                  >
                    {{ errorText(problem) }}
                  </p>
                </template>
              </Field>
            </div>
          </template>

          <template v-else-if="wizard.step === 'shared'">

            <Pane>
            <div class="pane-head">
              <span class="title">{{ t('install.shared.models') }}</span>
            </div>
            <div class="scroll-pad">
            <Field>
              <span class="t-label">{{ t('shared.root.label') }}</span>

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
            </Field>

            <ToggleRow>
              <Toggle
                :checked="wizard.connectShared"
                :disabled="!shared.configured"
                @click="wizard.connectShared = !wizard.connectShared"
              />
              <div>
                <div class="t-base">
                  {{ t('install.shared.connect', wizard.targets.length) }}
                </div>
                <div class="hint">{{ t('shared.default.hint') }}</div>
              </div>
            </ToggleRow>

            <Group v-if="wizard.connectShared">
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
            </Group>
            </div>
            </Pane>

            <Pane>
            <div class="pane-head">
              <span class="title">{{ t('install.shared.workflows') }}</span>
            </div>
            <div class="scroll-pad">
            <Field>
              <span class="t-label">{{ t('library.path.label') }}</span>

              <PathPicker
                :path="library.path"
                :empty="t('library.path.empty')"
                @pick="library.setPath($event)"
              />
              <p v-if="library.configured && library.available" class="hint">
                {{ t('library.summary', library.items.length) }}
              </p>
              <p v-else-if="!library.configured" class="hint">{{ t('library.path.howto') }}</p>
            </Field>
            </div>
            </Pane>
          </template>

          <template v-else-if="wizard.step === 'running'">
            <div
              v-for="(target, index) in wizard.targets"
              :key="index"
              class="prog"
            >
              <div class="prog-head">

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

              <p v-if="runStateOf(index) === 'now'" class="prog-file">
                <template v-if="indeterminate">{{ phaseText }}</template>
                <template v-else>{{ wizard.progress?.current }}</template>
              </p>

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

          <template v-else>
            <div class="cards grid">
              <Card
                v-for="instance in wizard.created"
                :key="instance.id"
                :as="RouterLink"
                :accent="accentVar(instance.accent)"
                :to="`/instances/${instance.id}`"
              >
                <div class="card-top">
                  <span
                    class="chip"
                    :style="{ '--instance-accent': accentVar(instance.accent) }"
                  >{{ initial(instance.name) }}</span>
                  <div class="card-name">{{ instance.name }}</div>
                  <StatusPill :status="displayStatus(instance, run.statusOf(instance.id))" />
                </div>

                <div class="card-desc">{{ instance.description }}</div>

                <div class="meta">
                  <span v-if="instance.comfyVersion">{{ instance.comfyVersion }}</span>
                  <span>:{{ instance.preferredPort }}</span>
                  <span v-if="instance.shared?.enabled" class="tag">
                    {{ t('shared.instance.badge') }}
                  </span>
                </div>

                <div class="src"><PathText :path="instance.path" /></div>
              </Card>
            </div>
          </template>
        </div>
      </div>
    </section>
  </var>
</template>

<style scoped>

.wizard-screen {
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.wizard-screen .screen-body {
  flex: 1;
}

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
</style>
