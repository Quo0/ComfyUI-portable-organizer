// Типизация ключей интерфейса от en.json.
//
// Смысл: неизвестный ключ в `t()` становится ошибкой компиляции, а не пустой
// строкой в интерфейсе. `en.json` — источник правды и по смыслу, и по типам;
// остальные локали догоняют его, паритет проверяет `pnpm i18n:check`.

import type en from './locales/en.json';

type Schema = typeof en;

declare module 'vue-i18n' {
  export interface DefineLocaleMessage extends Schema {}

  export interface DefineNumberFormat {
    gigabytes: {
      style: 'unit';
      unit: 'gigabyte';
      unitDisplay: 'short';
      maximumFractionDigits: number;
    };
    integer: { style: 'decimal'; maximumFractionDigits: number };
  }

  export interface DefineDateTimeFormat {
    short: {
      year: 'numeric';
      month: 'short';
      day: 'numeric';
      hour: '2-digit';
      minute: '2-digit';
    };
  }
}
