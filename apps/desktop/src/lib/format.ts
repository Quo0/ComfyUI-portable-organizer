import { useI18n } from 'vue-i18n';

import type { Accent } from '../bindings';

const STEPS = [
  { key: 'terabytes', factor: 1024 ** 4 },
  { key: 'gigabytes', factor: 1024 ** 3 },
  { key: 'megabytes', factor: 1024 ** 2 },
  { key: 'kilobytes', factor: 1024 },
  { key: 'bytes', factor: 1 },
] as const;

type Maybe = number | null | undefined;

export function useFormat() {
  const { n, d } = useI18n();

  function bytes(value: Maybe): string {
    if (value === null || value === undefined) return '';
    const step = STEPS.find((s) => value >= s.factor) ?? STEPS[STEPS.length - 1];
    return n(value / step.factor, step.key);
  }

  function moment(ms: Maybe): string {
    if (ms === null || ms === undefined) return '';
    return d(new Date(ms), 'short');
  }

  return { bytes, moment };
}

export function accentVar(accent: Accent): string {
  return accent.startsWith('#') ? accent : `var(--accent-${accent})`;
}

export function isCustomAccent(accent: Accent): boolean {
  return accent.startsWith('#');
}

export function initial(name: string): string {
  return [...name.trim()][0]?.toUpperCase() ?? '?';
}
