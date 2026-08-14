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

/** Палитра акцентов по умолчанию — по кругу, чтобы соседние цели различались. */
const ACCENTS = ['teal', 'indigo', 'ember', 'moss', 'azure', 'orchid', 'rose', 'amber'];

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

  /**
   * Черновик — то, что заполнено в форме и ещё не добавлено в список.
   *
   * Шаг назначений устроен как «форма накидывает в список»: панель
   * со всеми полями на каждую цель разом занимала по экрану прокрутки
   * на цель, а целей бывает шесть.
   */
  const draft = ref<InstallTarget>(blankDraft());
  const draftCheck = ref<TargetCheck | null>(null);

  function blankDraft(): InstallTarget {
    return {
      path: '',
      name: '',
      description: '',
      accent: ACCENTS[targets.value.length % ACCENTS.length],
      // Порты раздаются подряд: две сборки на одном порту одновременно
      // не поднимутся, и узнать об этом лучше не на запуске.
      preferredPort: 8188 + targets.value.length,
    };
  }

  /** Добавить в список можно то, у чего есть путь, имя и нет ошибок. */
  const draftReady = computed(
    () =>
      draft.value.path.trim() !== ''
      && draft.value.name.trim() !== ''
      && (draftCheck.value?.errors.length ?? 0) === 0,
  );
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

  /**
   * Проверка идёт по списку вместе с черновиком, одним запросом.
   *
   * Так `installer.duplicateTarget` бесплатно ловит совпадение черновика
   * с уже добавленным — эта проверка смотрит на весь переданный список, —
   * а требуемое место пересчитывается по мере его роста. Последний элемент
   * ответа относится к черновику, остальные — к добавленным.
   */
  async function recheck(): Promise<void> {
    if (!info.value) return;
    const withDraft = [...targets.value, draft.value];
    const res = await commands.checkTargets(info.value, withDraft);
    if (res.status !== 'ok') {
      checks.value = [];
      draftCheck.value = null;
      return;
    }
    checks.value = res.data.slice(0, targets.value.length);
    draftCheck.value = res.data[targets.value.length] ?? null;
  }

  /** Сабмит формы: черновик уходит в список, форма встаёт в исходное. */
  async function addDraft(): Promise<void> {
    if (!draftReady.value) return;
    targets.value.push({ ...draft.value });
    draft.value = blankDraft();
    draftCheck.value = null;
    await recheck();
  }

  async function removeTarget(index: number): Promise<void> {
    targets.value.splice(index, 1);
    await recheck();
  }

  function reset(): void {
    step.value = 'archive';
    info.value = null;
    targets.value = [];
    checks.value = [];
    draft.value = blankDraft();
    draftCheck.value = null;
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
    draft,
    draftCheck,
    draftReady,
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
    reset,
    start,
    cancel,
  };
});
