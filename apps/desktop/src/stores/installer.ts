import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  events,
  type ArchiveInfo,
  type ArchiveRecord,
  type Instance,
  type InstallProgress,
  type InstallTarget,
  type TargetCheck,
} from '../bindings';
import { useInstancesStore } from './instances';
import { useUiStore } from './ui';

export type WizardStep = 'archive' | 'targets' | 'running' | 'done';

/**
 * Состояние мастера живёт в сторе, а не в экране, по одной причине:
 * распаковка идёт минуты, и уход в другой раздел не должен её терять.
 * Процесс всё равно продолжается в Rust — терялся бы только прогресс.
 */
export const useInstallerStore = defineStore('installer', () => {
  const ui = useUiStore();
  const instances = useInstancesStore();

  const step = ref<WizardStep>('archive');
  const history = ref<ArchiveRecord[]>([]);
  const info = ref<ArchiveInfo | null>(null);
  const targets = ref<InstallTarget[]>([]);
  const checks = ref<TargetCheck[]>([]);
  const progress = ref<InstallProgress | null>(null);
  const running = ref(false);
  const created = ref<Instance[]>([]);

  let listening = false;

  /** Начать нельзя, пока хоть у одной цели есть ошибка. */
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
    const res = await commands.probeArchive(path);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return false;
    }
    info.value = res.data;
    return true;
  }

  async function recheck(): Promise<void> {
    if (!info.value) return;
    const res = await commands.checkTargets(info.value, targets.value);
    checks.value = res.status === 'ok' ? res.data : [];
  }

  function reset(): void {
    step.value = 'archive';
    info.value = null;
    targets.value = [];
    checks.value = [];
    progress.value = null;
    created.value = [];
  }

  async function start(): Promise<void> {
    if (!info.value) return;
    running.value = true;
    step.value = 'running';
    progress.value = null;
    await listen();

    const res = await commands.runInstall(info.value, targets.value);
    running.value = false;

    if (res.status === 'error') {
      ui.pushError(res.error);
      // Возврат к целям, а не к архиву: чаще всего чинить надо именно путь
      // назначения — он занят, короче нужного или на переполненном диске.
      step.value = 'targets';
      await recheck();
      return;
    }

    created.value = res.data;
    step.value = 'done';
    await instances.load();
    await loadHistory();
  }

  async function cancel(): Promise<void> {
    await commands.cancelInstall();
  }

  return {
    step,
    history,
    info,
    targets,
    checks,
    progress,
    running,
    created,
    blocked,
    loadHistory,
    forget,
    chooseArchive,
    recheck,
    reset,
    start,
    cancel,
  };
});
