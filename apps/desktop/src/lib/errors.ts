import { i18n } from '../i18n';
import type { AppError } from '../bindings';

export function errorText(error: AppError): string {
  const key = `errors.${error.code}`;
  const { te, t } = i18n.global;

  const has = te as unknown as (k: string) => boolean;
  const translate = t as unknown as (
    k: string,
    p: Record<string, unknown>,
  ) => string;

  return has(key) ? translate(key, error.params ?? {}) : error.code;
}

export function errorDetails(error: AppError): string {
  const params = Object.entries(error.params ?? {})
    .map(([k, v]) => `  ${k}: ${v}`)
    .join('\n');
  return [errorText(error), `[${error.code}]`, params].filter(Boolean).join('\n');
}
