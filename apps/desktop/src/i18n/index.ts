import { createI18n } from 'vue-i18n';

import en from './locales/en.json';
import ru from './locales/ru.json';
import es from './locales/es.json';
import zhHans from './locales/zh-Hans.json';

export const LOCALES = ['en', 'ru', 'zh-Hans', 'es'] as const;
export type Locale = (typeof LOCALES)[number];

export const LOCALE_NAMES: Record<Locale, string> = {
  en: 'English',
  ru: 'Русский',
  'zh-Hans': '简体中文',
  es: 'Español',
};

export const FALLBACK_LOCALE: Locale = 'en';

function russianPluralRule(choice: number, choicesLength: number): number {
  if (choicesLength < 3) return choice === 1 ? 0 : 1;

  const n = Math.abs(choice) % 100;
  const last = n % 10;

  if (n > 10 && n < 20) return 2;
  if (last === 1) return 0;
  if (last >= 2 && last <= 4) return 1;
  return 2;
}

const sizeFormat = (unit: string) =>
  ({ style: 'unit', unit, unitDisplay: 'short', maximumFractionDigits: 1 }) as const;

const numberFormats = {
  bytes: { style: 'unit', unit: 'byte', unitDisplay: 'short', maximumFractionDigits: 0 },
  kilobytes: sizeFormat('kilobyte'),
  megabytes: sizeFormat('megabyte'),
  gigabytes: sizeFormat('gigabyte'),
  terabytes: sizeFormat('terabyte'),
  integer: { style: 'decimal', maximumFractionDigits: 0 },
} as const;

const datetimeFormats = {
  short: {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  },
} as const;

const perLocale = <T>(value: T): Record<Locale, T> =>
  Object.fromEntries(LOCALES.map((l) => [l, value])) as Record<Locale, T>;

export const i18n = createI18n({
  legacy: false,
  locale: FALLBACK_LOCALE,
  fallbackLocale: FALLBACK_LOCALE,
  messages: { en, ru, 'zh-Hans': zhHans, es },
  pluralRules: { ru: russianPluralRule },
  numberFormats: perLocale(numberFormats),
  datetimeFormats: perLocale(datetimeFormats),

  missingWarn: false,
  fallbackWarn: false,
});

export function matchLocale(tag: string | null | undefined): Locale | null {
  if (!tag) return null;
  const lower = tag.toLowerCase();
  if (lower.startsWith('zh')) return 'zh-Hans';
  const base = lower.split('-')[0];
  return LOCALES.find((l) => l.toLowerCase().split('-')[0] === base) ?? null;
}

export function detectLocale(systemLocale: string | null): Locale {
  return (
    matchLocale(systemLocale) ??
    matchLocale(navigator.language) ??
    FALLBACK_LOCALE
  );
}

export function applyLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
  document.documentElement.lang = locale;
}
