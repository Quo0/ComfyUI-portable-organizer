import { ref } from 'vue';
import { defineStore } from 'pinia';

import { commands, events, type UpdateInfo } from '../bindings';
import { useUiStore } from './ui';

export const useUpdatesStore = defineStore('updates', () => {
  const ui = useUiStore();

  const checking = ref(false);
  const installing = ref(false);

  const info = ref<UpdateInfo | null>(null);

  const checked = ref(false);

  const progress = ref<{ downloaded: number; total: number | null } | null>(null);

  const running = ref<string | null>(null);

  let listening = false;

  async function check(manual: boolean): Promise<void> {
    if (checking.value || installing.value) return;
    checking.value = true;
    const res = await commands.checkUpdate();
    checking.value = false;

    if (res.status === 'error') {
      if (manual) ui.pushError(res.error);
      return;
    }
    info.value = res.data;
    checked.value = true;
  }

  async function install(stopRunning: boolean): Promise<void> {
    if (installing.value) return;
    running.value = null;
    installing.value = true;
    progress.value = { downloaded: 0, total: null };

    if (!listening) {
      listening = true;
      await events.updateProgress.listen((event) => {
        progress.value = {
          downloaded: event.payload.downloaded ?? 0,
          total: event.payload.total,
        };
      });
    }

    const res = await commands.installUpdate(stopRunning);

    installing.value = false;
    progress.value = null;

    if (res.status === 'error') {
      if (res.error.code === 'update.instancesRunning') {
        running.value = res.error.params.names ?? '';
        return;
      }
      ui.pushError(res.error);
    }
  }

  function dismissRunning(): void {
    running.value = null;
  }

  return { checking, installing, info, checked, progress, running, check, install, dismissRunning };
});
