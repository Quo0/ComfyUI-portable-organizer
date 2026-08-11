import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  events,
  type LaunchProfile,
  type LogLine,
  type RunStatus,
} from '../bindings';
import { useUiStore } from './ui';

/** Столько строк держим на экране. В Rust буфер тот же, менять их порознь нельзя. */
const LOG_LIMIT = 5000;

/**
 * Состояние запусков. Живёт в сторе, а не в экране инстанса, по той же
 * причине, что и мастер установки: уход в другой раздел не должен ни ронять
 * подписку на лог, ни терять уже пришедшие строки.
 */
export const useRunStore = defineStore('run', () => {
  const ui = useUiStore();

  const statuses = ref<Record<string, RunStatus>>({});
  const logs = ref<Record<string, LogLine[]>>({});
  const profiles = ref<Record<string, LaunchProfile[]>>({});
  const busy = ref<Record<string, boolean>>({});

  /**
   * Инстансы, у которых запуск упёрся в недоступный общий корень.
   * Держим и профиль: повторный запуск обязан пойти тем же профилем,
   * который выбрал пользователь, а не первым попавшимся.
   */
  const sharedWarning = ref<
    Record<string, { path: string; profileId: string | null }>
  >({});

  function dismissSharedWarning(id: string): void {
    const next = { ...sharedWarning.value };
    delete next[id];
    sharedWarning.value = next;
  }

  let listening = false;

  /** Инстансы, о которых рейл обязан сообщать независимо от раздела. */
  const active = computed(() =>
    Object.values(statuses.value).filter((s) => s.state !== 'stopped'),
  );

  function statusOf(id: string): RunStatus | undefined {
    return statuses.value[id];
  }

  async function listen(): Promise<void> {
    if (listening) return;
    listening = true;

    await events.runChanged.listen((e) => {
      // Кортежная структура из Rust экспортируется как сам внутренний тип,
      // а не как массив из одного элемента.
      const status = e.payload;
      statuses.value = { ...statuses.value, [status.instanceId]: status };
    });

    await events.runLog.listen((e) => {
      const { instanceId, line } = e.payload;
      const list = logs.value[instanceId] ?? [];
      // Прогресс от tqdm заменяет предыдущую строку, а не добавляется.
      // Флаг приходит из Rust: делить поток на строки должен тот, кто его читает.
      const next = line.replacesLast ? list.slice(0, -1) : list;
      next.push(line);
      logs.value = {
        ...logs.value,
        [instanceId]: next.length > LOG_LIMIT ? next.slice(-LOG_LIMIT) : next,
      };
    });
  }

  async function load(): Promise<void> {
    await listen();
    const res = await commands.runStatuses();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    const next: Record<string, RunStatus> = {};
    for (const status of res.data) next[status.instanceId] = status;
    statuses.value = next;
  }

  async function loadProfiles(id: string): Promise<void> {
    const res = await commands.instanceProfiles(id);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    profiles.value = { ...profiles.value, [id]: res.data };
  }

  /** Догоняет лог, накопленный пока экран был закрыт. */
  async function loadLog(id: string): Promise<void> {
    const res = await commands.runLog(id);
    if (res.status === 'error') return;
    logs.value = { ...logs.value, [id]: res.data };
  }

  /**
   * `withoutShared` — согласие стартовать без общих моделей.
   *
   * Недоступный общий корень не ошибка запуска, а развилка: бэкенд
   * отказывает кодом `shared.rootUnavailable`, экран инстанса показывает
   * выбор, и повторный вызов приходит с согласием. Тостом это сделать
   * нельзя — у тоста нет кнопок, а модалку над областью контента положить
   * невозможно (дисциплина z-order).
   */
  async function start(
    id: string,
    profileId: string | null,
    withoutShared = false,
  ): Promise<void> {
    await listen();
    busy.value = { ...busy.value, [id]: true };
    logs.value = { ...logs.value, [id]: [] };
    try {
      const res = await commands.startInstance(id, profileId, withoutShared);
      if (res.status === 'error') {
        if (res.error.code === 'shared.rootUnavailable') {
          sharedWarning.value = {
            ...sharedWarning.value,
            [id]: { path: res.error.params.path ?? '', profileId },
          };
          return;
        }
        ui.pushError(res.error);
        return;
      }
      dismissSharedWarning(id);
      statuses.value = { ...statuses.value, [id]: res.data };
    } finally {
      busy.value = { ...busy.value, [id]: false };
    }
  }

  async function stop(id: string): Promise<void> {
    busy.value = { ...busy.value, [id]: true };
    try {
      const res = await commands.stopInstance(id);
      if (res.status === 'error') ui.pushError(res.error);
    } finally {
      busy.value = { ...busy.value, [id]: false };
    }
  }

  return {
    statuses,
    logs,
    profiles,
    busy,
    sharedWarning,
    dismissSharedWarning,
    active,
    statusOf,
    load,
    loadProfiles,
    loadLog,
    start,
    stop,
  };
});
