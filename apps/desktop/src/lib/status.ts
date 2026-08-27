import type { Instance, RunStatus } from '../bindings';

export type DisplayStatus =
  | 'stopped'
  | 'starting'
  | 'running'
  | 'stopping'
  | 'crashed'
  | 'detached'
  | 'unavailable';

export function displayStatus(instance: Instance, run?: RunStatus): DisplayStatus {
  if (!instance.available) return 'unavailable';
  return run?.state ?? 'stopped';
}

export function isLive(status: DisplayStatus): boolean {
  return status !== 'stopped';
}

export const STATE_DOT: Record<DisplayStatus, string> = {
  stopped: 'var(--state-stopped)',
  starting: 'var(--state-starting)',
  running: 'var(--state-running)',
  stopping: 'var(--state-starting)',
  crashed: 'var(--state-crashed)',
  detached: 'var(--state-crashed)',
  unavailable: 'var(--state-unavailable)',
};
