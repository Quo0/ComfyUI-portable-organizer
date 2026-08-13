import { useI18n } from 'vue-i18n';

import type { Accent } from '../bindings';

/**
 * Ступени размера. Двоичные, как у проводника Windows: пользователь
 * сравнивает наши цифры с тем, что показывает система, и расхождение
 * в семь процентов выглядит ошибкой.
 */
const STEPS = [
  { key: 'terabytes', factor: 1024 ** 4 },
  { key: 'gigabytes', factor: 1024 ** 3 },
  { key: 'megabytes', factor: 1024 ** 2 },
  { key: 'kilobytes', factor: 1024 },
  { key: 'bytes', factor: 1 },
] as const;

/**
 * Числа `f64` приезжают из Rust как `number | null`: в JSON нет NaN
 * и бесконечностей, и specta честно это отражает. Поэтому обе функции
 * принимают отсутствующее значение и возвращают пустую строку —
 * «ещё не посчитано» показывается пропуском, а не нулём.
 */
type Maybe = number | null | undefined;

export function useFormat() {
  const { n, d } = useI18n();

  /** Размер на диске в подходящих единицах и по правилам локали. */
  function bytes(value: Maybe): string {
    if (value === null || value === undefined) return '';
    const step = STEPS.find((s) => value >= s.factor) ?? STEPS[STEPS.length - 1];
    return n(value / step.factor, step.key);
  }

  /** Момент времени из миллисекунд эпохи, как их отдаёт Rust. */
  function moment(ms: Maybe): string {
    if (ms === null || ms === undefined) return '';
    return d(new Date(ms), 'short');
  }

  return { bytes, moment };
}

/**
 * Акцент инстанса — либо имя токена палитры, либо свой цвет в виде
 * `#rrggbb`.
 *
 * Имя лучше и остаётся выбором по умолчанию: у токена своё значение
 * в каждой теме, и читаемость проверена сборкой дизайна. Свой цвет
 * одинаков в обеих темах — за него отвечает уже пользователь.
 */
export function accentVar(accent: Accent): string {
  return accent.startsWith('#') ? accent : `var(--accent-${accent})`;
}

/** Свой ли это цвет — то есть выбранный пипеткой, а не из палитры. */
export function isCustomAccent(accent: Accent): boolean {
  return accent.startsWith('#');
}

/** Первая буква имени для цветной метки в рейле. */
export function initial(name: string): string {
  return [...name.trim()][0]?.toUpperCase() ?? '?';
}
