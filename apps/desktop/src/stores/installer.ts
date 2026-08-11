import { computed, ref } from 'vue';
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
import { useInstancesStore } from './instances';
import { useSharedStore } from './shared';
import { useUiStore } from './ui';

export type WizardStep = 'archive' | 'targets' | 'shared' | 'running' | 'done';

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

  /**
   * Подключить свежие инстансы к общим моделям сразу после распаковки.
   * Шаг мастера, а не отдельный поход в настройки: US-SHARED-01/AC-4.
   */
  const connectShared = ref(false);
  const sharedMode = ref<ApplyMode>('flag');

  /**
   * Идёт разбор заголовка архива. Секунда с лишним на 56 тысяч записей —
   * достаточно, чтобы выбор файла показался проигнорированным.
   */
  const reading = ref(false);

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
    connectShared.value = false;
    sharedMode.value = 'flag';
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

    // Подключение — после регистрации и по одному: инстансы независимы,
    // и отказ на одном не должен отменять остальные. Ошибку показывает
    // стор общих моделей, мастер при этом доводит установку до конца:
    // распакованное никуда не делось, подключить можно и потом.
    if (connectShared.value) {
      const shared = useSharedStore();
      for (const instance of created.value) {
        await shared.connect(instance.id, sharedMode.value);
      }
    }

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
    connectShared,
    sharedMode,
    reading,
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
