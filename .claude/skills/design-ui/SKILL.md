---
name: design-ui
description: Laying out ComfyUI Portable Organizer screens — what the design system already has, where to make style edits and how to get them into the app. Use when laying out a new screen or component, editing styles, and adding indicators, states and colours.
---

# Laying out a screen

The design system here is not decoration but a catalogue of what already exists.
Before drawing a component — look for it.

## Search first, draw second

This rule was established the expensive way. The indeterminate `.track.indet`
bar, with its animation and correct behaviour under `prefers-reduced-motion`,
had been in the system from the start and had reached the app — and the install
wizard did not use it, because nobody looked.

Where to look:

```
apps/desktop/src/styles/components.css   components, ~104 classes
apps/desktop/src/styles/tokens.css       colours, metrics, shadows, radii
apps/design/screens/                     screens by scenario, one file per screen
apps/design/styleguide/                  component library, one file per section
```

To browse — `pnpm dev:design`, which starts `apps/design` (VitePress):
"Стайлгайд" and "Экраны" are separate menu items.

## What already exists

| Area | Classes |
|---|---|
| Shell | `.app` `.rail` `.nav` `.nav-item` `.nav-run` `.masthead` `.aside` |
| Panels | `.panel` `.pane` `.pane-head` `.pane-foot` `.card` `.cards` `.split-master` |
| Controls | `.btn` (`.primary` `.secondary` `.ghost` `.danger` `.lg`) `.split` `.seg` `.toggle` `.picker` |
| Fields | `.field` `.input` (`.mono` `.num` `.bad`) `.chip` `.tag` |
| States | `.pill` (`.stopped` `.starting` `.running` `.crashed` `.gone`) `.badge` `.dot` |
| Progress | `.bar` and `.bar.indet` · `.track` and `.track.indet` · `.prog` `.prog-head` `.prog-file` · `.spin` |
| Logs | `.log` `.console` `.log-follow` |
| Notifications | `.toast` `.toasts` `.toast-head` `.toast-life` |
| Wizard | `.steps` `.step` `.step-sep` `.wizard-foot` |
| Other | `.scroll` `.empty` `.hint` `.meta` `.eyebrow` `.longform` |

## Component state: read the contract, do not guess

Modifiers in this system come in two kinds, and the component name does not tell
you which one it uses. **Read `apps/desktop/src/styles/components.css` before
wiring up a state.**

| Component | How it shows state |
|---|---|
| `.toggle` | class `.off` — bare means **on** |
| `.seg > *` | attribute `[aria-pressed="true"]` |
| `.bar`, `.track` | class `.indet` |
| `.pill` | state class: `.running`, `.crashed`, `.gone` … |
| `.cat` | class `.unknown`, `.blocked` |
| `.tag` | class `.warn`, `.stop` |
| `.card` | class `.gone` |
| `.input` | class `.bad` |

**An ARIA attribute draws nothing by itself.** It is mandatory for screen
readers and is always set, but if a component is styled by class, `aria-checked`
alone is not enough — the toggle will stay in one position forever. That is
exactly what happened with "Download new models into the shared folder": the
state changed and persisted while the picture did not move.

## Edits go into the source

The source of truth is the app itself; the showcase only reads it:

```
apps/desktop/src/styles/tokens.css        ←  the only file to edit, colours and metrics
apps/desktop/src/styles/components.css    ←  the only file to edit, components
        │  pnpm design:tokens (node tools/build-preview-tokens.mjs)
        ↓
apps/design/.vitepress/theme/preview-tokens.css   ←  NEVER EDITED BY HAND
```

The showcase reads `components.css` as is, without a copy — an edit shows up
immediately under `pnpm dev:design`. The `.t-light`/`.t-dark` rules for the
`ThemePair.vue` panels, however, are computed in a separate step:
`:root[data-theme="dark"]` will not fire inside a page, since `:root` matches
only `<html>`. The build target is in `.gitignore`; an edit to the copy survives
exactly until the next `pnpm design:tokens`.

Check before committing: `pnpm design:check` (it rebuilds `preview-tokens.css`
itself and runs the contrast and theme-parity checks).

`<style scoped>` in Vue components stays only for the layout of a specific
screen. Components are shared, globally, from `components.css`.

## Palette rules

- **Not a single colour as a literal.** Only `var(--token)`. Checked
  automatically: a token that is declared but unused is a signal that the
  palette has drifted from the mockup.
- **Every light-theme token must exist in the dark theme.** A miss leaves the
  colour undefined in exactly one of the three theme states, and on the light
  one it is invisible. Checked automatically.
- An instance's accent arrives as `--instance-accent` on the root of its card.

## Scrolling

The window never scrolls. A screen has one scroll area; the exception is
master-detail, which has two. The ComfyUI area neither scrolls nor drifts.
Panels with a fixed header and footer keep the scroll inside, between them —
that is `.pane` + `.pane-head` + `.pane-foot`.

## There are no modals

The child webview with ComfyUI is a native window on top of our HTML, and
covering it with a dropdown or a modal is physically impossible. That is why
settings, adding an instance and the argument editor are **separate routes**.
Any new component that wants to pop up over the content area is designed wrong.
The full reasoning is in `plan/ui.md`, section «Дисциплина z-order».

## Animation

Everything that moves must have a variant under `prefers-reduced-motion:
reduce` — not "switch it off and leave a hole", but a meaningful static state.
The model to follow: with motion off, `.bar.indet` becomes a muted full-width
bar instead of disappearing.
