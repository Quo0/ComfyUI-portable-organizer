import { createI18n } from 'vue-i18n';

import en from './locales/en.json';
import ru from './locales/ru.json';
import es from './locales/es.json';
import zhHans from './locales/zh-Hans.json';

/** Порядок задаёт порядок в списке выбора языка. */
export const LOCALES = ['en', 'ru', 'zh-Hans', 'es'] as const;
export type Locale = (typeof LOCALES)[number];

/** Подписи языков всегда на самом языке: искать свой в переводе неудобно. */
export const LOCALE_NAMES: Record<Locale, string> = {
  en: 'English',
  ru: 'Русский',
  'zh-Hans': '简体中文',
  es: 'Español',
};

export const FALLBACK_LOCALE: Locale = 'en';

/**
 * Русская плюрализация: три формы вместо двух, и «одиннадцать» ведёт себя
 * не как «один». Дефолтное правило vue-i18n этого не умеет.
 *
 * Возвращает индекс формы в строке `один | два | пять`.
 */
function russianPluralRule(choice: number, choicesLength: number): number {
  // Подстраховка на случай, если в переводе окажется две формы вместо трёх:
  // лучше показать не идеальную форму, чем пустую строку.
  if (choicesLength < 3) return choice === 1 ? 0 : 1;

  const n = Math.abs(choice) % 100;
  const last = n % 10;

  if (n > 10 && n < 20) return 2; // 11..19 — «инстансов»
  if (last === 1) return 0; // 1, 21, 31 — «инстанс»
  if (last >= 2 && last <= 4) return 1; // 2..4, 22..24 — «инстанса»
  return 2;
}

/**
 * Числа и даты идут только через `n()` и `d()`: разделитель разрядов
 * и порядок частей даты у четырёх языков разные, и руками это не собрать.
 */
const numberFormats = {
  /**
   * Размер на диске. Единица идёт через Intl, а не через ключ перевода:
   * «GB», «ГБ» и «GB» с китайской пунктуацией браузер знает сам,
   * и заводить под них ручные строки значит плодить работу переводчикам.
   */
  gigabytes: {
    style: 'unit',
    unit: 'gigabyte',
    unitDisplay: 'short',
    maximumFractionDigits: 1,
  },
  integer: { style: 'decimal', maximumFractionDigits: 0 },
} as const;

const datetimeFormats = {
  /** Дата и время последнего запуска — короткая форма, без секунд. */
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
  // Composition API. Legacy-режим тянет за собой глобальный `this.$t`,
  // который в `<script setup>` всё равно недоступен.
  legacy: false,
  locale: FALLBACK_LOCALE,
  fallbackLocale: FALLBACK_LOCALE,
  messages: { en, ru, 'zh-Hans': zhHans, es },
  pluralRules: { ru: russianPluralRule },
  numberFormats: perLocale(numberFormats),
  datetimeFormats: perLocale(datetimeFormats),
  // Недостающий ключ падает в fallback молча — про него скажет `pnpm i18n:check`,
  // а не поток предупреждений в консоли на каждый рендер.
  missingWarn: false,
  fallbackWarn: false,
});

/**
 * Приводит системный тег языка к одной из поддерживаемых локалей.
 *
 * Любой китайский сводится к упрощённому: традиционного письма у нас нет,
 * и показать `zh-Hans` ближе к правде, чем откатиться в английский.
 */
export function matchLocale(tag: string | null | undefined): Locale | null {
  if (!tag) return null;
  const lower = tag.toLowerCase();
  if (lower.startsWith('zh')) return 'zh-Hans';
  const base = lower.split('-')[0];
  return LOCALES.find((l) => l.toLowerCase().split('-')[0] === base) ?? null;
}

/**
 * Определение языка при первом запуске: системная локаль из Rust →
 * язык вебвью → английский.
 */
export function detectLocale(systemLocale: string | null): Locale {
  return (
    matchLocale(systemLocale) ??
    matchLocale(navigator.language) ??
    FALLBACK_LOCALE
  );
}

/** Применяет язык к i18n и к `<html lang>` — от него зависят переносы и шрифт. */
export function applyLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
  document.documentElement.lang = locale;
}
