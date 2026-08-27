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

export const useSharedStore = defineStore('shared', () => {
  const ui = useUiStore();
  const instances = useInstancesStore();

  const settings = ref<SharedSettings>({ roots: [], makeDefaultTarget: true });
  const scan = ref<RootScan | null>(null);
  const yaml = ref('');

  const scanning = ref(false);
  const loaded = ref(false);

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
