import { ref } from 'vue';
import { defineStore } from 'pinia';

import { commands, events, type UpdateInfo } from '../bindings';
import { useUiStore } from './ui';

/**
 * Обновление приложения.
 *
 * Живёт в сторе, а не на экране «О приложении»: проверка идёт при запуске,
 * задолго до того, как пользователь туда зайдёт, и её результат обязан
 * пережить переходы между разделами.
 */
export const useUpdatesStore = defineStore('updates', () => {
  const ui = useUiStore();

  const checking = ref(false);
  const installing = ref(false);
  /** Найденная версия. `null` — либо не проверяли, либо установлена последняя. */
  const info = ref<UpdateInfo | null>(null);
  /** Проверка уже была: без этого «последняя версия» не отличить от «не спрашивали». */
  const checked = ref(false);

  /** Байты загрузки. `total` бывает `null` — сервер не всегда шлёт длину. */
  const progress = ref<{ downloaded: number; total: number | null } | null>(null);

  /**
   * Работающие сборки, из-за которых установка остановилась. Развилка,
   * а не ошибка: раскрывается на месте, как гард мульти-запуска.
   */
  const running = ref<string | null>(null);

  let listening = false;

  /**
   * Спрашивает манифест.
   *
   * `manual` решает судьбу ошибки: нажатую кнопку без ответа оставлять
   * нельзя, а автоматическая проверка при старте обязана молчать —
   * без сети приложение работает обычным образом, и ошибка проверки,
   * которую никто не просил, тут выглядит поломкой приложения.
   */
  async function check(manual: boolean): Promise<void> {
    if (checking.value || installing.value) return;
    checking.value = true;
    const res = await commands.checkUpdate();
    checking.value = false;

    if (res.status === 'error') {
      if (manual) ui.pushError(res.error);
      return;
    }
    info.value = res.data;
    checked.value = true;
  }

  /**
   * Ставит обновление. Первый вызов идёт без согласия останавливать сборки:
   * решение о судьбе чужой очереди генерации принимает пользователь.
   */
  async function install(stopRunning: boolean): Promise<void> {
    if (installing.value) return;
    running.value = null;
    installing.value = true;
    progress.value = { downloaded: 0, total: null };

    if (!listening) {
      listening = true;
      await events.updateProgress.listen((event) => {
        // `f64` приезжает как `number | null`: в JSON нет ни NaN,
        // ни бесконечностей, и specta это честно отражает.
        progress.value = {
          downloaded: event.payload.downloaded ?? 0,
          total: event.payload.total,
        };
      });
    }

    const res = await commands.installUpdate(stopRunning);
    // Сюда доходим только если установка не началась: удачная забирает
    // управление себе и приложение больше не возвращается.
    installing.value = false;
    progress.value = null;

    if (res.status === 'error') {
      if (res.error.code === 'update.instancesRunning') {
        running.value = res.error.params.names ?? '';
        return;
      }
      ui.pushError(res.error);
    }
  }

  /** Пользователь отложил установку. Обновление никуда не делось. */
  function dismissRunning(): void {
    running.value = null;
  }

  return { checking, installing, info, checked, progress, running, check, install, dismissRunning };
});
