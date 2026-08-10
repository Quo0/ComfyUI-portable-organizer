import { i18n } from '../i18n';
import type { AppError } from '../bindings';

/**
 * Текст ошибки из кода, пришедшего с бэкенда.
 *
 * Rust не переводит ничего: он отдаёт `code` и `params`, а текст живёт
 * в `errors.<code>`. Неизвестный код показывается как сам код — интерфейс
 * не должен падать или показывать пустоту из-за пропущенного ключа,
 * а по коду ошибку всё равно можно найти.
 */
export function errorText(error: AppError): string {
  const key = `errors.${error.code}`;
  const { te, t } = i18n.global;
  // Ключ динамический, поэтому типизация от en.json здесь неприменима:
  // компилятор не может проверить строку, собранную во время работы.
  const has = te as unknown as (k: string) => boolean;
  const translate = t as unknown as (
    k: string,
    p: Record<string, unknown>,
  ) => string;

  return has(key) ? translate(key, error.params ?? {}) : error.code;
}

/**
 * Полный текст для копирования: перевод плюс код и подстановки.
 *
 * Пользователь копирует ошибку, чтобы её кому-то показать, — значит
 * получатель должен опознать её независимо от языка отправителя.
 */
export function errorDetails(error: AppError): string {
  const params = Object.entries(error.params ?? {})
    .map(([k, v]) => `  ${k}: ${v}`)
    .join('\n');
  return [errorText(error), `[${error.code}]`, params].filter(Boolean).join('\n');
}
