import { computed, ref, watch } from 'vue';
import { defineStore } from 'pinia';
import { getCurrentWindow } from '@tauri-apps/api/window';

import { commands, type AppError, type ThemeChoice } from '../bindings';
import { applyLocale, detectLocale, i18n, LOCALES, type Locale } from '../i18n';
import { errorDetails, errorText } from '../lib/errors';

export type ToastKind = 'ok' | 'err';

export interface Toast {
  id: number;
  kind: ToastKind;
  /** Уже переведённый текст: стор не знает, откуда он взялся. */
  text: string;
  /** Полный текст с кодом — только у ошибок, для копирования. */
  details?: string;
  /** Сколько раз подряд пришло одно и то же. */
  count: number;
}

/** Больше трёх одновременно — это уже не уведомление, а стена. */
const MAX_TOASTS = 3;
const OK_LIFETIME_MS = 4000;

let nextToastId = 1;

export const useUiStore = defineStore('ui', () => {
  const theme = ref<ThemeChoice>('system');
  const locale = ref<Locale>('en');
  const railCollapsed = ref(false);
  const toasts = ref<Toast[]>([]);

  /** Пути и версия для раздела «О приложении». */
  const appDataDir = ref('');
  const appLocalDataDir = ref('');
  const version = ref('');

  /** Тёмная ли тема в Windows прямо сейчас. */
  const systemDark = ref(false);
  /** Пользователь выбрал язык сам — определение по системе больше не применяется. */
  const localeChosen = ref(false);
  /** До конца загрузки настройки не сохраняем: иначе первый же watch
      перезапишет файл значениями по умолчанию. */
  const ready = ref(false);

  const effectiveTheme = computed<'light' | 'dark'>(() =>
    theme.value === 'system' ? (systemDark.value ? 'dark' : 'light') : theme.value,
  );

  const darkQuery = window.matchMedia('(prefers-color-scheme: dark)');

  function applyTheme(): void {
    const root = document.documentElement;
    if (theme.value === 'system') {
      // Атрибута нет — работает @media (prefers-color-scheme), как задумано
      // в tokens.css. Явный атрибут победил бы медиа-запрос в обе стороны.
      delete root.dataset.theme;
    } else {
      root.dataset.theme = theme.value;
    }
    // Декорации окна системные, и без этого тёмное окно получит белый
    // заголовок. null означает «следовать системе».
    void getCurrentWindow().setTheme(theme.value === 'system' ? null : theme.value);
  }

  async function init(): Promise<void> {
    systemDark.value = darkQuery.matches;
    // Смена темы Windows на лету: перезапуск ради этого не нужен.
    darkQuery.addEventListener('change', (e) => {
      systemDark.value = e.matches;
    });

    const res = await commands.loadBootstrap();
    if (res.status === 'error') {
      // Настройки не прочитались — работаем на значениях по умолчанию.
      // Останавливать запуск из-за этого нельзя.
      applyTheme();
      applyLocale(detectLocale(null));
      pushError(res.error);
      ready.value = true;
      return;
    }

    const boot = res.data;
    theme.value = boot.settings.theme;
    railCollapsed.value = boot.settings.railCollapsed;
    appDataDir.value = boot.appDataDir;
    appLocalDataDir.value = boot.appLocalDataDir;
    version.value = boot.version;

    const stored = boot.settings.locale;
    if (stored && (LOCALES as readonly string[]).includes(stored)) {
      locale.value = stored as Locale;
      localeChosen.value = true;
    } else {
      locale.value = detectLocale(boot.systemLocale);
    }

    applyTheme();
    applyLocale(locale.value);
    void syncTray();
    ready.value = true;
  }

  /**
   * Подписи меню трея.
   *
   * Меню нативное, и `t()` до него не дотягивается: строки приходится
   * отправлять в Rust. Зато перевод остаётся в локалях, а не расползается
   * по двум языкам, и меню следует за сменой языка вместе со всем остальным.
   */
  async function syncTray(): Promise<void> {
    const t = i18n.global.t as unknown as (k: string) => string;
    await commands.setTrayLabels({
      show: t('tray.show'),
      stopAll: t('tray.stopAll'),
      quit: t('tray.quit'),
    });
  }

  function setTheme(value: ThemeChoice): void {
    theme.value = value;
    applyTheme();
  }

  function setLocale(value: Locale): void {
    locale.value = value;
    localeChosen.value = true;
    applyLocale(value);
    void syncTray();
  }

  function toggleRail(): void {
    railCollapsed.value = !railCollapsed.value;
  }

  // Сохранение отдельным наблюдателем, а не внутри сеттеров: тогда ни одно
  // изменение не потеряется, даже если появится новый способ его сделать.
  watch([theme, locale, railCollapsed, localeChosen], () => {
    if (!ready.value) return;
    void persist();
  });

  async function persist(): Promise<void> {
    const res = await commands.saveSettings({
      theme: theme.value,
      locale: localeChosen.value ? locale.value : null,
      railCollapsed: railCollapsed.value,
    });
    if (res.status === 'error') pushError(res.error);
  }

  function push(kind: ToastKind, text: string, details?: string): void {
    // Повторы схлопываются: десять одинаковых ошибок подряд — это одна
    // ошибка, случившаяся десять раз, и место на экране она занимать
    // не должна десять раз тоже.
    const same = toasts.value.find((t) => t.kind === kind && t.text === text);
    if (same) {
      same.count += 1;
      return;
    }

    const toast: Toast = { id: nextToastId++, kind, text, details, count: 1 };
    toasts.value.push(toast);
    if (toasts.value.length > MAX_TOASTS) toasts.value.shift();

    // Успех уходит сам, ошибка ждёт закрытия: её надо успеть прочитать
    // и, возможно, скопировать.
    if (kind === 'ok') {
      window.setTimeout(() => dismiss(toast.id), OK_LIFETIME_MS);
    }
  }

  function pushOk(text: string): void {
    push('ok', text);
  }

  function pushError(error: AppError): void {
    push('err', errorText(error), errorDetails(error));
  }

  function dismiss(id: number): void {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  }

  return {
    theme,
    locale,
    railCollapsed,
    toasts,
    appDataDir,
    appLocalDataDir,
    version,
    effectiveTheme,
    systemDark,
    ready,
    init,
    setTheme,
    setLocale,
    toggleRail,
    pushOk,
    pushError,
    dismiss,
  };
});
