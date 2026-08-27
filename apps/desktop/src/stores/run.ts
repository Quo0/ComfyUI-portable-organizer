import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  events,
  type LaunchProfile,
  type LogLine,
  type RunStatus,
  type StartOptions,
} from '../bindings';
import { i18n } from '../i18n';
import { useUiStore } from './ui';

const LOG_LIMIT = 5000;

export const useRunStore = defineStore('run', () => {
  const ui = useUiStore();

  const statuses = ref<Record<string, RunStatus>>({});
  const logs = ref<Record<string, LogLine[]>>({});
  const profiles = ref<Record<string, LaunchProfile[]>>({});
  const busy = ref<Record<string, boolean>>({});

  const sharedWarning = ref<
    Record<string, { path: string; profileId: string | null }>
  >({});

  const busyWarning = ref<
    Record<string, { other: string; profileId: string | null }>
  >({});

  function dismissSharedWarning(id: string): void {
    const next = { ...sharedWarning.value };
    delete next[id];
    sharedWarning.value = next;
  }

  function dismissBusyWarning(id: string): void {
    const next = { ...busyWarning.value };
    delete next[id];
    busyWarning.value = next;
  }

  let listening = false;

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
      const status = e.payload;
      statuses.value = { ...statuses.value, [status.instanceId]: status };
    });

    await events.runLog.listen((e) => {
      const { instanceId, line } = e.payload;
      const list = logs.value[instanceId] ?? [];

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

  async function loadLog(id: string): Promise<void> {
    const res = await commands.runLog(id);
    if (res.status === 'error') return;
    logs.value = { ...logs.value, [id]: res.data };
  }

  async function start(
    id: string,
    profileId: string | null,
    options: StartOptions = { withoutShared: false, allowMultiple: false },
  ): Promise<void> {
    await listen();
    busy.value = { ...busy.value, [id]: true };
    logs.value = { ...logs.value, [id]: [] };
    try {
      const res = await commands.startInstance(id, profileId, options);
      if (res.status === 'error') {
        if (res.error.code === 'shared.rootUnavailable') {
          sharedWarning.value = {
            ...sharedWarning.value,
            [id]: { path: res.error.params.path ?? '', profileId },
          };
          return;
        }
        if (res.error.code === 'run.otherRunning') {
          busyWarning.value = {
            ...busyWarning.value,
            [id]: { other: res.error.params.name ?? '', profileId },
          };
          return;
        }
        ui.pushError(res.error);
        return;
      }
      dismissSharedWarning(id);
      dismissBusyWarning(id);
      statuses.value = { ...statuses.value, [id]: res.data };
    } finally {
      busy.value = { ...busy.value, [id]: false };
    }
  }

  async function stopOthersAndStart(
    id: string,
    profileId: string | null,
  ): Promise<void> {
    for (const status of active.value) {
      if (status.instanceId !== id) await stop(status.instanceId);
    }
    await start(id, profileId, { withoutShared: false, allowMultiple: true });
  }

  async function adopt(id: string): Promise<void> {
    busy.value = { ...busy.value, [id]: true };
    try {
      const res = await commands.adoptInstance(id);
      if (res.status === 'error') {
        ui.pushError(res.error);
        return;
      }
      statuses.value = { ...statuses.value, [id]: res.data };
      ui.pushOk(i18n.global.t('run.adopted', { port: res.data.port ?? 0 }));
    } finally {
      busy.value = { ...busy.value, [id]: false };
    }
  }

  async function restart(id: string): Promise<void> {
    busy.value = { ...busy.value, [id]: true };
    try {
      const res = await commands.restartInstance(id);
      if (res.status === 'error') {
        ui.pushError(res.error);
        return;
      }
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
    busyWarning,
    dismissSharedWarning,
    dismissBusyWarning,
    active,
    statusOf,
    load,
    loadProfiles,
    loadLog,
    start,
    stopOthersAndStart,
    adopt,
    restart,
    stop,
  };
});
