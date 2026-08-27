import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  type AppError,
  type InstanceCompat,
  type LibraryItem,
  type LibraryScan,
  type WorkflowMeta,
} from '../bindings';
import { useUiStore } from './ui';

export type FullMeta = {
  favorite: boolean;
  tags: string[];
  note: string;
  addedAt: number | null;
  sourceInstanceId: string | null;
};

export type LibItem = Omit<LibraryItem, 'meta'> & { meta: FullMeta };

function fullMeta(meta: WorkflowMeta): FullMeta {
  return {
    favorite: meta.favorite ?? false,
    tags: meta.tags ?? [],
    note: meta.note ?? '',
    addedAt: meta.addedAt ?? null,
    sourceInstanceId: meta.sourceInstanceId ?? null,
  };
}

export const useWorkflowsStore = defineStore('workflows', () => {
  const ui = useUiStore();

  const path = ref('');
  const scan = ref<LibraryScan | null>(null);
  const scanning = ref(false);
  const loaded = ref(false);

  const selected = ref<string | null>(null);
  const compat = ref<InstanceCompat[]>([]);

  const query = ref('');
  const favoritesOnly = ref(false);

  const configured = computed(() => path.value !== '');
  const available = computed(() => scan.value?.available === true);
  const items = computed<LibItem[]>(
    () => scan.value?.items.map((i) => ({ ...i, meta: fullMeta(i.meta) })) ?? [],
  );

  const visible = computed(() => {
    const needle = query.value.trim().toLowerCase();
    return items.value.filter((item) => {
      if (favoritesOnly.value && !item.meta.favorite) return false;
      if (!needle) return true;
      return (
        item.path.toLowerCase().includes(needle) ||
        item.meta.tags.some((tag) => tag.toLowerCase().includes(needle))
      );
    });
  });

  const current = computed(
    () => items.value.find((i) => i.path === selected.value) ?? null,
  );

  const allTags = computed(() => {
    const tags = new Set<string>();
    for (const item of items.value) for (const tag of item.meta.tags) tags.add(tag);
    return [...tags].sort();
  });

  async function load(): Promise<void> {
    const res = await commands.loadLibrarySettings();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }

    path.value = res.data.path ?? '';
    loaded.value = true;
    if (path.value) await rescan();
  }

  async function rescan(): Promise<void> {
    if (!path.value) {
      scan.value = null;
      return;
    }
    scanning.value = true;
    try {
      const res = await commands.scanLibrary(path.value);
      if (res.status === 'error') {
        ui.pushError(res.error);
        return;
      }
      scan.value = res.data;

      if (selected.value && !res.data.items.some((i) => i.path === selected.value)) {
        selected.value = null;
        compat.value = [];
      }
    } finally {
      scanning.value = false;
    }
  }

  async function setPath(next: string): Promise<void> {
    path.value = next;
    const res = await commands.saveLibrarySettings({ path: next });
    if (res.status === 'error') ui.pushError(res.error);
    await rescan();
  }

  async function select(rel: string | null): Promise<void> {
    selected.value = rel;
    compat.value = [];
    const item = items.value.find((i) => i.path === rel);
    if (!item || item.lost || item.broken) return;

    const res = await commands.workflowCompat(item.path, item.nodes);
    if (res.status === 'ok') compat.value = res.data;
  }

  function compatOf(instanceId: string): InstanceCompat | undefined {
    return compat.value.find((c) => c.instanceId === instanceId);
  }

  async function setMeta(rel: string, meta: WorkflowMeta): Promise<void> {
    const res = await commands.setWorkflowMeta(path.value, rel, meta);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    await rescan();
  }

  async function toggleFavorite(item: LibItem): Promise<void> {
    await setMeta(item.path, { ...item.meta, favorite: !item.meta.favorite });
  }

  async function forget(rel: string): Promise<void> {
    const res = await commands.forgetWorkflow(path.value, rel);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    if (selected.value === rel) selected.value = null;
    await rescan();
  }

  const multi = ref(false);

  const marked = ref<Set<string>>(new Set());

  const markedTargets = ref<Set<string>>(new Set());

  function toggleMark(rel: string): void {
    if (bulk.value && !bulk.value.running) clearBulk();
    const next = new Set(marked.value);
    if (!next.delete(rel)) next.add(rel);
    marked.value = next;
  }

  function toggleTarget(id: string): void {
    const next = new Set(markedTargets.value);
    if (!next.delete(id)) next.add(id);
    markedTargets.value = next;
  }

  function setTargets(ids: string[]): void {
    markedTargets.value = new Set(ids);
  }

  function clearMarks(): void {
    marked.value = new Set();
    markedTargets.value = new Set();
  }

  function setMulti(on: boolean): void {
    multi.value = on;
    if (!on) {
      cancel();
      clearMarks();
      clearBulk();
    }
  }

  const bulk = ref<{
    done: number;
    total: number;
    ok: string[];

    failed: { workflow: string; instanceId: string; error: AppError | null }[];

    running: boolean;
  } | null>(null);

  let cancelBulk = false;
  function cancel(): void {
    cancelBulk = true;
  }

  async function pushMany(instanceIds: string[]): Promise<void> {
    const rels = [...marked.value];
    if (rels.length === 0 || instanceIds.length === 0) return;

    cancelBulk = false;
    bulk.value = {
      done: 0,

      total: rels.length * instanceIds.length,
      ok: [],
      failed: [],
      running: true,
    };

    const state = bulk.value;

    try {
      for (const rel of rels) {
        for (const id of instanceIds) {
          if (cancelBulk) return;
          const res = await commands.pushWorkflow(id, path.value, rel, false);
          if (res.status === 'error') {
            state.failed.push({ workflow: rel, instanceId: id, error: res.error });
          } else if (res.data === 'conflict') {
            state.failed.push({ workflow: rel, instanceId: id, error: null });
          } else {
            state.ok.push(`${rel} → ${id}`);
          }
          state.done += 1;
        }
      }
    } finally {
      state.running = false;
    }

    if (state.ok.length > 0 && selected.value) await select(selected.value);

    if (state.failed.length === 0 && !cancelBulk) clearMarks();
  }

  function clearBulk(): void {
    bulk.value = null;
  }

  async function addFile(source: string, overwrite = false): Promise<boolean> {
    const res = await commands.addWorkflowFile(path.value, source, null, overwrite);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return false;
    }
    await rescan();
    return true;
  }

  async function addText(name: string, content: string): Promise<AppError | null> {
    const res = await commands.addWorkflowText(path.value, name, content);
    if (res.status === 'error') return res.error;
    await rescan();

    await select(res.data);
    return null;
  }

  return {
    path,
    scan,
    scanning,
    loaded,
    selected,
    compat,
    query,
    favoritesOnly,
    configured,
    available,
    items,
    visible,
    current,
    allTags,
    load,
    rescan,
    setPath,
    select,
    compatOf,
    setMeta,
    toggleFavorite,
    forget,
    addFile,
    addText,
    multi,
    setMulti,
    marked,
    toggleMark,
    markedTargets,
    toggleTarget,
    setTargets,
    clearMarks,
    bulk,
    pushMany,
    cancel,
    clearBulk,
  };
});
