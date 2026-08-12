import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  type InstanceCompat,
  type LibraryItem,
  type LibraryScan,
  type WorkflowMeta,
} from '../bindings';
import { useUiStore } from './ui';

/**
 * Запись манифеста с заполненными полями.
 *
 * В `WorkflowMeta` из Rust они необязательные, и это честно: манифест лежит
 * в папке пользователя, его правят руками и пишут старые версии приложения,
 * поэтому `serde(default)` там обязателен. Но тащить `?? []` через каждый
 * фильтр и каждый шаблон — верный способ однажды забыть.
 *
 * Поэтому нормализуем один раз на границе, и дальше весь фронт работает
 * с полным значением.
 */
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

/**
 * Библиотека воркфлоу.
 *
 * Живёт в сторе, а не в экране: библиотеку спрашивают и раздел «Библиотека»,
 * и вкладка на экране инстанса, и мастер установки — все должны видеть одно.
 */
export const useWorkflowsStore = defineStore('workflows', () => {
  const ui = useUiStore();

  const path = ref('');
  const scan = ref<LibraryScan | null>(null);
  const scanning = ref(false);
  const loaded = ref(false);

  /** Выбранный воркфлоу — по нему считается совместимость. */
  const selected = ref<string | null>(null);
  const compat = ref<InstanceCompat[]>([]);

  const query = ref('');
  const favoritesOnly = ref(false);

  const configured = computed(() => path.value !== '');
  const available = computed(() => scan.value?.available === true);
  const items = computed<LibItem[]>(
    () => scan.value?.items.map((i) => ({ ...i, meta: fullMeta(i.meta) })) ?? [],
  );

  /** Поиск идёт и по имени, и по тегам: искать по одному имени бесполезно. */
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

  /** Все теги библиотеки — для подсказок при поиске. */
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
    // Настройка тоже с умолчанием на стороне Rust, то есть необязательная:
    // в файле, записанном до этой фазы, ключа нет вовсе.
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
      // Выбранный мог исчезнуть — например, его удалили через проводник.
      if (selected.value && !res.data.items.some((i) => i.path === selected.value)) {
        selected.value = null;
        compat.value = [];
      }
    } finally {
      scanning.value = false;
    }
  }

  async function setPath(next: string): Promise<void> {
    // Прежняя папка не трогается: смена библиотеки — это смена адреса,
    // а не переезд с уничтожением старого.
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

    const res = await commands.workflowCompat(item.nodes);
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

  // --- массовые операции --------------------------------------------------

  /** Отмеченные для массовой операции. */
  const marked = ref<Set<string>>(new Set());

  function toggleMark(rel: string): void {
    const next = new Set(marked.value);
    if (!next.delete(rel)) next.add(rel);
    marked.value = next;
  }

  function clearMarks(): void {
    marked.value = new Set();
  }

  /**
   * Ход массовой операции.
   *
   * Считается по парам «воркфлоу × сборка»: пользователь может выбрать
   * три воркфлоу и две сборки, и честное «сделано 4 из 6» получается
   * только так.
   */
  const bulk = ref<{
    done: number;
    total: number;
    ok: string[];
    failed: { name: string; reason: string }[];
  } | null>(null);

  /** Прерывание. Уже добавленное остаётся на месте — откатывать нечего. */
  let cancelBulk = false;
  function cancel(): void {
    cancelBulk = true;
  }

  /**
   * Кладёт отмеченные воркфлоу в выбранные сборки.
   *
   * Отказ по одной паре не отменяет остальные: инстансы независимы,
   * и «конфликт имён на втором из двадцати» не повод бросить всё.
   * Конфликты в массовой операции не спрашиваются поштучно — это двадцать
   * вопросов подряд; они собираются в отчёт как неудачи с причиной.
   */
  async function pushMany(instanceIds: string[]): Promise<void> {
    const rels = [...marked.value];
    if (rels.length === 0 || instanceIds.length === 0) return;

    cancelBulk = false;
    bulk.value = { done: 0, total: rels.length * instanceIds.length, ok: [], failed: [] };

    for (const rel of rels) {
      for (const id of instanceIds) {
        if (cancelBulk) return;
        const res = await commands.pushWorkflow(id, path.value, rel, false);
        const label = `${rel} → ${id}`;
        if (res.status === 'error') {
          bulk.value.failed.push({ name: label, reason: res.error.code });
        } else if (res.data === 'conflict') {
          bulk.value.failed.push({ name: label, reason: 'workflows.nameTaken' });
        } else {
          bulk.value.ok.push(label);
        }
        bulk.value.done += 1;
      }
    }
  }

  function clearBulk(): void {
    bulk.value = null;
  }

  /** Кладёт файл с диска в библиотеку. `false` — отказ, уже сообщённый. */
  async function addFile(source: string, overwrite = false): Promise<boolean> {
    const res = await commands.addWorkflowFile(path.value, source, null, overwrite);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return false;
    }
    await rescan();
    return true;
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
    marked,
    toggleMark,
    clearMarks,
    bulk,
    pushMany,
    cancel,
    clearBulk,
  };
});
