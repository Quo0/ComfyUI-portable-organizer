---
name: i18n
description: UI strings of ComfyUI Portable Organizer — adding and editing keys across four locales, pluralisation, interpolations, error codes from Rust. Use for any change to on-screen text, when adding a new screen, and when pnpm i18n:check fails.
---

# UI strings

Not a single string in the markup. Everything through `t()`, starting from the
very first screen — this is written into `CLAUDE.md` and is not open to
violation. The reason is simple: localisation added later never gets added.

Four locales: `en` (source of truth), `ru`, `es`, `zh-Hans`. The files are
`apps/desktop/src/i18n/locales/*.json`.

## Adding a key

One command writes into all four files:

```
node tools/i18n-add.mjs install.run.preparing \
  --en "Checking the folders and opening the archive…" \
  --ru "Проверяем папки и открываем архив…" \
  --es "Comprobando las carpetas y abriendo el archivo…" \
  --zh "正在检查文件夹并打开压缩包…"
```

In bulk — `--file keys.json` with an object of
`{ "key": { "en": …, "ru": … } }`.

The script refuses to overwrite an existing key without `--force`, and edits
either all four files or none. A key is appended at the end of its own group so
the order of the rest does not shift — otherwise the diff would turn into
a reshuffle of the whole file.

Always finish with `pnpm i18n:check`.

## What `i18n:check` checks

Missing keys, extra keys, empty values and **the set of interpolations**. The
last one is the most valuable: a translation that lost `{reason}` looks intact
and loses half its meaning. The check catches that; the eye does not.

## Pluralisation

`vue-i18n` separates forms with a vertical bar. Russian requires three forms,
English two:

```
"instances": "1 instance | {n} instances"
"instances": "{n} инстанс | {n} инстанса | {n} инстансов"
```

The form-selection rules are configured in `apps/desktop/src/i18n/index.ts`.
Check against 1, 2 and 5 — three different forms, and the second is the one lost
most often.

## Errors from Rust

The backend **translates nothing**. Commands return `AppError { code, params }`,
and the frontend maps them onto `errors.<code>` through
`apps/desktop/src/lib/errors.ts`. An unknown code is shown as the code itself:
the UI neither crashes nor shows blankness. Added an error variant in Rust — add
`errors.<code>` to all four locales, otherwise the user sees
`installer.extractFailed` instead of text.

## What is never translated

Paths, instance names, log contents, launch profile names (`run_nvidia_gpu`),
configuration key names. They are identical across locales, and "translating"
them here is a breakage: the user is looking for that same string in Explorer or
in the ComfyUI documentation.

## String length

There is no German, but Spanish runs about twenty percent longer than English
and Chinese is half the length. Buttons and tables are laid out to survive both.
`plan/verification.md` has separate check items for this.
