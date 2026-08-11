import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  type ApplyMode,
  type RootScan,
  type SharedSettings,
} from '../bindings';
import { useInstancesStore } from './instances';
import { useUiStore } from './ui';

/**
 * Общее хранилище моделей.
 *
 * Настройки и результат сканирования держим в сторе, а не в экране:
 * подключение инстанса спрашивают и с экрана инстанса, и из мастера
 * установки, и оба должны видеть одно и то же состояние.
 */
export const useSharedStore = defineStore('shared', () => {
  const ui = useUiStore();
  const instances = useInstancesStore();

  const settings = ref<SharedSettings>({ roots: [], makeDefaultTarget: true });
  const scan = ref<RootScan | null>(null);
  const yaml = ref('');

  /**
   * Идёт обход дерева. На папке в сотни гигабайт это десятки тысяч
   * обращений к метаданным; без индикатора экран выглядит замершим.
   */
  const scanning = ref(false);
  const loaded = ref(false);

  /** Первый корень: интерфейс заводит один, модель допускает несколько. */
  const root = computed(() => settings.value.roots[0] ?? null);
  const configured = computed(() => root.value !== null);
  const available = computed(() => scan.value?.available === true);

  const recognized = computed(
    () => scan.value?.categories.filter((c) => c.status === 'recognized') ?? [],
  );
  const unknown = computed(
    () => scan.value?.categories.filter((c) => c.status === 'unknown') ?? [],
  );
  const blocked = computed(
    () => scan.value?.categories.filter((c) => c.status === 'blocked') ?? [],
  );

  /**
   * Сколько инстансов подключено — для сводки на экране настроек.
   *
   * `shared` необязательное: у записей реестра, сделанных до этой фазы,
   * поля нет вовсе. Так и должно быть — умолчание на стороне Rust спасает
   * старый реестр от падения разбора, а фронт обязан это учитывать.
   */
  const connected = computed(
    () => instances.items.filter((i) => i.shared?.enabled).length,
  );

  async function load(): Promise<void> {
    const res = await commands.loadSharedSettings();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    settings.value = res.data;
    loaded.value = true;
    if (root.value) await rescan();
  }

  async function rescan(): Promise<void> {
    if (!root.value) {
      scan.value = null;
      yaml.value = '';
      return;
    }
    scanning.value = true;
    try {
      const res = await commands.scanSharedRoot(root.value.path);
      if (res.status === 'error') {
        ui.pushError(res.error);
        return;
      }
      scan.value = res.data;
      await refreshPreview();
    } finally {
      scanning.value = false;
    }
  }

  async function refreshPreview(): Promise<void> {
    const res = await commands.previewSharedYaml(settings.value);
    yaml.value = res.status === 'ok' ? res.data : '';
  }

  async function persist(): Promise<void> {
    const res = await commands.saveSharedSettings(settings.value);
    if (res.status === 'error') ui.pushError(res.error);
  }

  /** Задаёт общий корень. Пустой путь означает «убрать». */
  async function setRoot(path: string): Promise<void> {
    settings.value.roots = path
      ? [{ id: 'root-0', path, label: '', enabled: true }]
      : [];
    await persist();
    await rescan();
  }

  async function setDefaultTarget(value: boolean): Promise<void> {
    settings.value.makeDefaultTarget = value;
    await persist();
    // Предпросмотр обязан отреагировать сразу: тумблер меняет ровно одну
    // строку YAML, и увидеть её — единственный способ понять, что он делает.
    await refreshPreview();
  }

  async function createMissing(): Promise<void> {
    if (!root.value || !scan.value) return;
    scanning.value = true;
    try {
      const res = await commands.createSharedFolders(
        root.value.path,
        scan.value.missing,
      );
      if (res.status === 'error') {
        ui.pushError(res.error);
        return;
      }
      scan.value = res.data;
      await refreshPreview();
    } finally {
      scanning.value = false;
    }
  }

  /**
   * Подключает инстанс. Возвращает путь резервной копии, если чужой файл
   * пришлось подвинуть, `false` — если отказано, и `null` — если копии
   * не потребовалось.
   *
   * Отказ по коду `shared.foreignConfig` — не ошибка, а развилка: экран
   * инстанса покажет сравнение и переспросит.
   */
  async function connect(
    id: string,
    mode: ApplyMode,
    confirmOverwrite = false,
  ): Promise<string | null | false> {
    const res = await commands.connectShared(id, mode, confirmOverwrite);
    if (res.status === 'error') {
      if (res.error.code !== 'shared.foreignConfig') ui.pushError(res.error);
      return false;
    }
    await instances.load();
    return res.data;
  }

  async function disconnect(id: string): Promise<boolean> {
    const res = await commands.disconnectShared(id);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return false;
    }
    await instances.load();
    return true;
  }

  return {
    settings,
    scan,
    yaml,
    scanning,
    loaded,
    root,
    configured,
    available,
    recognized,
    unknown,
    blocked,
    connected,
    load,
    rescan,
    persist,
    setRoot,
    setDefaultTarget,
    createMissing,
    connect,
    disconnect,
  };
});
