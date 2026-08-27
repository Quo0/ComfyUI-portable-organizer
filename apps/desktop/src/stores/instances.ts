import { ref } from 'vue';
import { defineStore } from 'pinia';

import {
  commands,
  type Accent,
  type Instance,
  type InstanceEdit,
  type ProbeResult,
} from '../bindings';
import { useUiStore } from './ui';

export const useInstancesStore = defineStore('instances', () => {
  const ui = useUiStore();

  const items = ref<Instance[]>([]);
  const loaded = ref(false);

  function byId(id: string): Instance | undefined {
    return items.value.find((i) => i.id === id);
  }

  async function load(): Promise<void> {
    const res = await commands.listInstances();
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    items.value = res.data;
    loaded.value = true;
  }

  async function probe(path: string): Promise<ProbeResult | null> {
    const res = await commands.probeFolder(path);
    if (res.status === 'error') return null;
    return res.data;
  }

  async function suggestAccent(): Promise<Accent> {
    const res = await commands.suggestAccent();
    return res.status === 'ok' ? res.data : 'teal';
  }

  async function add(path: string, edit: InstanceEdit): Promise<Instance | null> {
    const res = await commands.addInstance(path, edit);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return null;
    }
    items.value.push(res.data);
    return res.data;
  }

  async function update(id: string, edit: InstanceEdit): Promise<Instance | null> {
    const res = await commands.updateInstance(id, edit);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return null;
    }
    const at = items.value.findIndex((i) => i.id === id);
    if (at >= 0) items.value[at] = res.data;
    return res.data;
  }

  function replace(instance: Instance): void {
    const at = items.value.findIndex((i) => i.id === instance.id);
    if (at >= 0) items.value[at] = instance;
  }

  async function remove(id: string): Promise<boolean> {
    const res = await commands.removeInstance(id);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return false;
    }
    items.value = items.value.filter((i) => i.id !== id);
    return true;
  }

  async function measureSize(id: string): Promise<void> {
    const res = await commands.measureInstanceSize(id);
    if (res.status === 'error') {
      ui.pushError(res.error);
      return;
    }
    if (!res.data) return;
    const at = items.value.findIndex((i) => i.id === res.data!.id);
    if (at < 0) return;
    items.value[at] = {
      ...items.value[at],
      sizeBytes: res.data.bytes,
      sizeMeasuredAt: res.data.measuredAt,
    };
  }

  return {
    items,
    loaded,
    byId,
    load,
    probe,
    suggestAccent,
    add,
    update,
    replace,
    remove,
    measureSize,
  };
});
