import { computed, readonly, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  events,
  type ApplyMode,
  type ArchiveInfo,
  type ArchiveRecord,
  type Instance,
  type InstallProgress,
  type InstallTarget,
  type TargetCheck,
} from '../bindings';
import { withViewTransition } from '../lib/motion';
import { useInstancesStore } from './instances';
import { useSharedStore } from './shared';
import { useUiStore } from './ui';

export type WizardStep = 'archive' | 'targets' | 'shared' | 'running' | 'done';

const ACCENTS = ['teal', 'indigo', 'ember', 'moss', 'azure', 'orchid', 'rose', 'amber'];

export const useInstallerStore = defineStore('installer', () => {
  const ui = useUiStore();
  const instances = useInstancesStore();

  const step = ref<WizardStep>('archive');
  const history = ref<ArchiveRecord[]>([]);
  const info = ref<ArchiveInfo | null>(null);
  const targets = ref<InstallTarget[]>([]);
  const checks = ref<TargetCheck[]>([]);

  const draft = ref<InstallTarget>(blankDraft());
  const draftCheck = ref<TargetCheck | null>(null);

  const draftSubmitted = ref(false);

  const editIndex = ref<number | null>(null);
  const editDraft = ref<InstallTarget | null>(null);

  function blankDraft(): InstallTarget {
    return {
      path: '',
      name: '',
      description: '',
      accent: ACCENTS[targets.value.length % ACCENTS.length],

      preferredPort: 8188 + targets.value.length,
    };
  }

  const draftReady = computed(
    () =>
      draft.value.path.trim() !== ''
      && draft.value.name.trim() !== ''
      && (draftCheck.value?.errors.length ?? 0) === 0,
  );

  const draftProblems = computed(
    () => draft.value.path.trim() !== '' || draftSubmitted.value,
  );

  const editCheck = computed(() =>
    editIndex.value === null ? null : checks.value[editIndex.value] ?? null,
  );

  const editReady = computed(
    () =>
      editDraft.value !== null
      && editDraft.value.path.trim() !== ''
      && editDraft.value.name.trim() !== ''
      && (editCheck.value?.errors.length ?? 0) === 0,
  );
  const progress = ref<InstallProgress | null>(null);
  const running = ref(false);
  const created = ref<Instance[]>([]);

  const connectShared = ref(false);
  const sharedMode = ref<ApplyMode>('flag');

  const reading = ref(false);

  let listening = false;

  const blocked = computed(
    () => targets.value.length === 0 || checks.value.some((c) => c.errors.length > 0),
  );

  async function listen(): Promise<void> {
    if (listening) return;
    listening = true;
    await events.installProgress.listen((e) => {
      progress.value = e.payload;
    });
  }

  async function loadHistory(): Promise<void> {
    const res = await commands.archiveHistory();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    history.value = res.data;
  }

  async function forget(path: string): Promise<void> {
    const res = await commands.forgetArchive(path);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    await loadHistory();
  }

  async function chooseArchive(path: string): Promise<boolean> {
    reading.value = true;
    try {
      const res = await commands.probeArchive(path);
      if (res.status === 'error') {
        ui.pushError(res.error);
        return false;
      }
      info.value = res.data;
      return true;
    } finally {
      reading.value = false;
    }
  }

  async function recheck(): Promise<void> {
    if (!info.value) return;
    const edited = targets.value.map((target, i) =>
      i === editIndex.value && editDraft.value ? editDraft.value : target,
    );
    const res = await commands.checkTargets(info.value, [...edited, draft.value]);
    if (res.status !== 'ok') {
      checks.value = [];
      draftCheck.value = null;
      return;
    }
    checks.value = res.data.slice(0, targets.value.length);
    draftCheck.value = res.data[targets.value.length] ?? null;
  }

  async function addDraft(): Promise<void> {
    draftSubmitted.value = true;
    await recheck();
    if (!draftReady.value) return;

    targets.value.push({ ...draft.value });
    draft.value = blankDraft();
    draftCheck.value = null;
    draftSubmitted.value = false;
    await recheck();
  }

  async function removeTarget(index: number): Promise<void> {
    if (editIndex.value === index) cancelEdit();
    else if (editIndex.value !== null && editIndex.value > index) editIndex.value -= 1;

    targets.value.splice(index, 1);
    await recheck();
  }

  async function startEdit(index: number): Promise<void> {
    editIndex.value = index;
    editDraft.value = { ...targets.value[index] };
    await recheck();
  }

  function cancelEdit(): void {
    editIndex.value = null;
    editDraft.value = null;
    void recheck();
  }

  async function saveEdit(): Promise<void> {
    if (editIndex.value === null || !editDraft.value || !editReady.value) return;
    targets.value[editIndex.value] = { ...editDraft.value };
    editIndex.value = null;
    editDraft.value = null;
    await recheck();
  }

  function setStep(next: WizardStep): void {
    if (step.value === next) return;
    withViewTransition(() => { step.value = next; });
  }

  function reset(): void {
    setStep('archive');
    info.value = null;
    targets.value = [];
    checks.value = [];
    draft.value = blankDraft();
    draftCheck.value = null;
    draftSubmitted.value = false;
    editIndex.value = null;
    editDraft.value = null;
    progress.value = null;
    created.value = [];
    connectShared.value = false;
    sharedMode.value = 'flag';
  }

  async function start(): Promise<void> {
    if (!info.value) return;
    running.value = true;
    setStep('running');
    progress.value = null;
    await listen();

    const res = await commands.runInstall(info.value, targets.value);
    running.value = false;

    if (res.status === 'error') {
      ui.pushError(res.error);

      setStep('targets');
      await recheck();
      return;
    }

    created.value = res.data;

    if (connectShared.value) {
      const shared = useSharedStore();
      for (const instance of created.value) {
        await shared.connect(instance.id, sharedMode.value);
      }
    }

    setStep('done');
    await instances.load();
    await loadHistory();
  }

  async function cancel(): Promise<void> {
    await commands.cancelInstall();
  }

  return {
    step: readonly(step),
    setStep,
    history,
    info,
    targets,
    checks,
    draft,
    draftCheck,
    draftReady,
    draftProblems,
    editIndex,
    editDraft,
    editCheck,
    editReady,
    progress,
    running,
    created,
    connectShared,
    sharedMode,
    reading,
    blocked,
    loadHistory,
    forget,
    chooseArchive,
    recheck,
    addDraft,
    removeTarget,
    startEdit,
    cancelEdit,
    saveEdit,
    reset,
    start,
    cancel,
  };
});
