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

    // Путь идёт вместе с нодами: бэкенд по нему же отвечает, лежит ли
    // этот воркфлоу в каждой из сборок.
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

  // --- массовые операции --------------------------------------------------

  /**
   * Режим множественного выбора.
   *
   * Объявляется явно, а не выводится из наличия отметок. Пока признаком
   * режима было «отмечено хоть что-то», попасть в него можно было только
   * случайно — ткнув в маленький квадратик, который висел в каждой строке
   * всегда, — и правая панель при этом оказывалась в двух режимах разом:
   * заголовок про один воркфлоу, тело под ним про все отмеченные.
   */
  const multi = ref(false);

  /** Отмеченные воркфлоу. */
  const marked = ref<Set<string>>(new Set());

  /** Отмеченные сборки: куда класть отмеченное. */
  const markedTargets = ref<Set<string>>(new Set());

  /**
   * Отметить воркфлоу или снять отметку.
   *
   * Заодно убирает отчёт о прошлой записи — ровно так же, как убрала бы
   * его кнопка «Закрыть». Отчёт рассказывает про тот набор, который был
   * отмечен в момент запуска; стоит тронуть отбор, и он говорит о том,
   * чего на экране уже нет, — а на его месте не видно списка сборок,
   * то есть следующий шаг закрыт устаревшим ответом на прошлый вопрос.
   *
   * Идущей операции это не касается: её отчёт живой, и убирать его
   * с экрана значило бы прятать запись, которая прямо сейчас идёт.
   * На сам ход операции отметки не влияют — набор снят в начале.
   */
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

  /** Отметить все сборки разом или снять со всех. */
  function setTargets(ids: string[]): void {
    markedTargets.value = new Set(ids);
  }

  function clearMarks(): void {
    marked.value = new Set();
    markedTargets.value = new Set();
  }

  /**
   * Вход и выход из режима.
   *
   * Выход чистит обе отметки: невидимый выбор, доживший до следующего
   * включения, — это чужое решение, принятое неизвестно когда.
   *
   * И прерывает операцию, если она идёт. Без этого выход из режима убирал
   * бы с экрана отчёт, а запись файлов продолжалась бы дальше — молча
   * и без единого способа её остановить.
   */
  function setMulti(on: boolean): void {
    multi.value = on;
    if (!on) {
      cancel();
      clearMarks();
      clearBulk();
    }
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
    /**
     * Что не прошло и почему.
     *
     * Пара хранится разобранной, а не склеенной в строку: сборка нужна
     * отчёту по своему опознавателю, а показывать её надо по имени —
     * опознаватель вида `i1786962802438` не говорит читающему ничего.
     *
     * `error` — ошибка бэкенда как есть, вместе с подстановками: без них
     * от «нет доступа к {path}» остаётся полсообщения. `null` означает
     * занятое имя: это не ошибка, а развилка, на которую в массовой
     * операции никто не отвечает — двадцать вопросов подряд задавать
     * нельзя, поэтому пара откладывается в отчёт нетронутой.
     */
    failed: { workflow: string; instanceId: string; error: AppError | null }[];
    /**
     * Операция ещё идёт.
     *
     * Отдельный признак, а не `done < total`. По этому сравнению прерванная
     * операция навсегда оставалась «идущей»: отчёт после отмены нечем было
     * закрыть, кнопка предлагала отменить уже отменённое, и выйти из этого
     * состояния можно было только сняв отметки по одной.
     */
    running: boolean;
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
    bulk.value = {
      done: 0,
      // Пар, а не воркфлоу: два воркфлоу в две сборки — это четыре файла,
      // и считать их надо поштучно, иначе полоса врёт вдвое.
      total: rels.length * instanceIds.length,
      ok: [],
      failed: [],
      running: true,
    };

    // Работаем через `bulk.value`, а не через ссылку на объект, которым
    // его наполнили. `ref` заворачивает присвоенный объект в reactive-прокси,
    // и запись мимо прокси — прямо в исходный объект — экран не обновляет:
    // счётчик и полоса замирают на нуле, а кнопка навсегда остаётся
    // «Отмена», хотя запись давно прошла. Отчёт при этом врал в худшую
    // сторону — показывал незавершённой законченную операцию.
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
      // Снимается и при отмене, и при отказе: отчёт остаётся на экране,
      // но операции за ним больше нет.
      state.running = false;
    }

    // Совместимость считается по файловой системе и после записи устарела:
    // без пересчёта сборки, куда только что положили, выглядели бы пустыми.
    if (state.ok.length > 0 && selected.value) await select(selected.value);

    // Отметки снимаются только когда всё прошло. Неудачи остаются
    // выбранными: разбираться с ними — следующее действие пользователя,
    // и отбирать их заново после отчёта незачем.
    if (state.failed.length === 0 && !cancelBulk) clearMarks();
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

  /**
   * Кладёт в библиотеку граф, вставленный текстом.
   *
   * Ошибку не глотает и не показывает сама: форма вставки держит её рядом
   * с полем, которого она касается, — в тосте ответ «имя занято» уезжает
   * от поля с именем в противоположный угол экрана.
   */
  async function addText(name: string, content: string): Promise<AppError | null> {
    const res = await commands.addWorkflowText(path.value, name, content);
    if (res.status === 'error') return res.error;
    await rescan();
    // Вставленное сразу и выбрано: пользователь только что его назвал,
    // и разбираться, куда оно попало, ему не должно быть нужно.
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
