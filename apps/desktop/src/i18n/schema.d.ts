
//

import type en from './locales/en.json';

type Schema = typeof en;

declare module 'vue-i18n' {
  export interface DefineLocaleMessage extends Schema {}

  type SizeFormat = {
    style: 'unit';
    unit: string;
    unitDisplay: 'short';
    maximumFractionDigits: number;
  };

  export interface DefineNumberFormat {
    bytes: SizeFormat;
    kilobytes: SizeFormat;
    megabytes: SizeFormat;
    gigabytes: SizeFormat;
    terabytes: SizeFormat;
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
