// VitePress data loader: читает tools/lib/style-tokens.mjs на этапе
// сборки/дев-сервера и отдаёт готовые данные компонентам раздела «Палитра».
// Источник — apps/desktop/src/styles/tokens.css, то же приложение.
import { loadTokens, ACCENTS, ROLES, ratio } from '../../tools/lib/style-tokens.mjs';

export interface RoleToken {
  name: string;
  what: string;
  light: string;
  dark: string;
}

export interface AccentSwatch {
  key: string;
  label: string;
  light: string;
  lightRatio: number;
  dark: string;
  darkRatio: number;
}

export interface TokensData {
  roles: RoleToken[];
  accents: AccentSwatch[];
}

declare const data: TokensData;
export { data };

export default {
  // Пересчитать данные, если правится источник токенов.
  watch: ['../desktop/src/styles/tokens.css'],
  load(): TokensData {
    const { light, dark } = loadTokens();

    const roles: RoleToken[] = ROLES.map(([name, what]) => ({
      name: name.slice(2),
      what,
      light: light.get(name)!,
      dark: dark.get(name)!,
    }));

    const accents: AccentSwatch[] = ACCENTS.map(([key, label]) => {
      const l = light.get(`--accent-${key}`)!;
      const d = dark.get(`--accent-${key}`)!;
      return {
        key,
        label,
        light: l,
        lightRatio: ratio(l, light.get('--ground')!),
        dark: d,
        darkRatio: ratio(d, dark.get('--ground')!),
      };
    });

    return { roles, accents };
  },
};
