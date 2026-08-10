import type { Instance, RunStatus } from '../bindings';

/**
 * Что показывать про инстанс. Состояния процесса приходят из Rust,
 * недоступность папки добавляется здесь: для пользователя это одна и та же
 * колонка «что сейчас», хотя источники у них разные.
 */
export type DisplayStatus =
  | 'stopped'
  | 'starting'
  | 'running'
  | 'stopping'
  | 'crashed'
  | 'detached'
  | 'unavailable';

export function displayStatus(instance: Instance, run?: RunStatus): DisplayStatus {
  // Пропавшая папка важнее любого состояния процесса: запустить нечего.
  if (!instance.available) return 'unavailable';
  return run?.state ?? 'stopped';
}

/** Состояния, о которых рейл сообщает независимо от открытого раздела. */
export function needsAttention(status: DisplayStatus): boolean {
  return status !== 'stopped';
}

/** Цвет точки в рейле. Значения — токены состояний из дизайн-системы. */
export const STATE_DOT: Record<DisplayStatus, string> = {
  stopped: 'var(--state-stopped)',
  starting: 'var(--state-starting)',
  running: 'var(--state-running)',
  stopping: 'var(--state-starting)',
  crashed: 'var(--state-crashed)',
  detached: 'var(--state-crashed)',
  unavailable: 'var(--state-unavailable)',
};
