<!--
  Thanks for the patch. CONTRIBUTING.md has the detail; this form is the
  short version of it.

  Delete any section that does not apply — an empty heading helps nobody.
  There is no CI on pull requests here: the checks below are the only ones
  that will have run.
-->

## What this changes

<!-- One paragraph: what was wrong or missing, and what it does now. -->

Closes #

## Why this way

<!--
  What you tried or ruled out, and anything a reviewer would otherwise have to
  reconstruct from the diff. Skip for a one-line fix; write it for anything
  that made a decision.
-->

## For the changelog

<!--
  One sentence for the person deciding whether to install the update — not a
  summary of the diff. Leave blank if nothing user-visible changed.

  Do not edit CHANGELOG.md or the version yourself: releases are cut by the
  maintainer, and this line is what gets folded in.
-->

## How it was checked

<!-- Tick what you ran. Name anything that failed, together with its output. -->

- [ ] `pnpm typecheck` — Vue, TypeScript, app styles
- [ ] `pnpm i18n:check` — locale key parity, after any string change
- [ ] `pnpm ui-design:check` — after editing `tokens.css` or `components.css`
- [ ] `cargo check` and the relevant `cargo run --example check_*`
- [ ] Restarted `pnpm desktop:dev` before typechecking, if a Tauri command or
      event signature changed (`bindings.ts` is generated only by the dev build)

By hand:

<!--
  What you clicked through, and on what. Things that do not automate: the
  ComfyUI tab itself, dragging an image onto the canvas, a Windows theme
  change, install and uninstall. Say if you tested against a real build or
  against tools/fixtures/fake-instance.
-->

## Screenshots

<!--
  For anything visual: both themes, light and dark. They are not a filter over
  one another, and a change that reads well in one can be unreadable in the
  other. If the layout moved, a narrow window helps too.
-->

## Checklist

- [ ] Branched off `master`, one topic in this pull request
- [ ] New UI strings go through `t()` and exist in all four locales
      (`node tools/i18n-add.mjs` adds them at once)
- [ ] No hand edits to generated files — `bindings.ts`, `preview-tokens.css`
- [ ] Version and `CHANGELOG.md` untouched
- [ ] Commits follow Conventional Commits, in English, with a body that
      explains **why**
