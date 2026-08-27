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

  text: string;

  details?: string;

  count: number;
}

const MAX_TOASTS = 3;
const OK_LIFETIME_MS = 4000;

let nextToastId = 1;

export const useUiStore = defineStore('ui', () => {
  const theme = ref<ThemeChoice>('dark');
  const locale = ref<Locale>('en');
  const railCollapsed = ref(false);

  const checkUpdates = ref(true);
  const toasts = ref<Toast[]>([]);

  const appDataDir = ref('');
  const appLocalDataDir = ref('');
  const version = ref('');

  const systemDark = ref(false);

  const localeChosen = ref(false);

  const ready = ref(false);

  const effectiveTheme = computed<'light' | 'dark'>(() =>
    theme.value === 'system' ? (systemDark.value ? 'dark' : 'light') : theme.value,
  );

  const darkQuery = window.matchMedia('(prefers-color-scheme: dark)');

  function applyTheme(): void {
    const root = document.documentElement;
    if (theme.value === 'system') {
      delete root.dataset.theme;
    } else {
      root.dataset.theme = theme.value;
    }

    void getCurrentWindow().setTheme(theme.value === 'system' ? null : theme.value);
  }

  async function init(): Promise<void> {
    systemDark.value = darkQuery.matches;

    darkQuery.addEventListener('change', (e) => {
      systemDark.value = e.matches;
    });

    const res = await commands.loadBootstrap();
    if (res.status === 'error') {
      applyTheme();
      applyLocale(detectLocale(null));
      pushError(res.error);
      ready.value = true;
      return;
    }

    const boot = res.data;

    theme.value = boot.settings.theme ?? 'dark';
    railCollapsed.value = boot.settings.railCollapsed ?? false;
    checkUpdates.value = boot.settings.checkUpdates ?? true;
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

  function setCheckUpdates(value: boolean): void {
    checkUpdates.value = value;
  }

  watch([theme, locale, railCollapsed, localeChosen, checkUpdates], () => {
    if (!ready.value) return;
    void persist();
  });

  async function persist(): Promise<void> {
    const res = await commands.saveSettings({
      theme: theme.value,
      locale: localeChosen.value ? locale.value : null,
      railCollapsed: railCollapsed.value,
      checkUpdates: checkUpdates.value,
    });
    if (res.status === 'error') pushError(res.error);
  }

  function push(kind: ToastKind, text: string, details?: string): void {
    const same = toasts.value.find((t) => t.kind === kind && t.text === text);
    if (same) {
      same.count += 1;
      return;
    }

    const toast: Toast = { id: nextToastId++, kind, text, details, count: 1 };
    toasts.value.push(toast);
    if (toasts.value.length > MAX_TOASTS) toasts.value.shift();

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
    checkUpdates,
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
    setCheckUpdates,
    pushOk,
    pushError,
    dismiss,
  };
});
