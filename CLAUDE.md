# ComfyUI Portable Organizer

A Windows desktop app for managing several ComfyUI portable builds: registry,
install from archive, launch, embedded tabs, shared models, workflow library.

## Where things live

| Document | Question it answers |
|---|---|
| `PLAN.md` | plan index: context, stack, repository layout, section table |
| `plan/` | **how** we build: architecture, findings from ComfyUI sources, phase checklist, write-ups |
| `specs/` | **what** we build: functional requirements, user stories, NFRs |
| `apps/design/` | **how it looks**: showcase of the style guide and screens by scenario (VitePress) |

The source of truth for tokens and component CSS is `apps/desktop/src/styles/`
itself, not a separate folder: `apps/design` reads those files directly,
without copies.

The plan is cut into sections: load only the one you need, not all of it.
The entry point is the table in `PLAN.md`. The phase checklist is
`plan/phases.md`; the "what got settled" write-ups from finished phases live
in `plan/notes/`.

Before working on a phase, check its checklist in `plan/phases.md` and the
epics in `specs/` that the phase references with `@phase-N` tags.

## Language

- **Code comments, commit messages and conversation are in English.**
- **UI strings go through `t()` only**, starting from the very first screen.
  The source of truth is `en.json`; Russian, Chinese and Spanish follow it.
- Paths, instance names and log contents are never translated.

## Rules already paid for

Each one was found the expensive way — none may be broken.

- **Tauri commands must be `async fn`.** A synchronous command runs on the
  main thread, and `add_child` inside it queues work onto that same thread and
  waits for it — a deadlock without a single error. The logic lives in plain
  synchronous functions; commands are thin wrappers over them.
- **The child webview is a native window on top of our HTML.** Dropdowns and
  modals physically cannot cover it. Settings, adding an instance, the argument
  editor — **separate routes**, not modals. On the tab screen this applies to
  toasts as well: they pop up in the corner of the content area, which is
  underneath the tab. Messages there are shown as a banner **above** its
  rectangle, and the log console takes its place instead of overlaying it.
- **The child webview requires `disable_drag_drop_handler()`** — otherwise
  dragging images and workflows onto the ComfyUI canvas breaks.
- **The child webview has no auto-layout.** The frontend computes the rectangle
  and passes it to `set_webview_bounds` on every change: window resize, rail
  collapse, navigation.
- **Settings structs carry `#[serde(default)]` on the container.** Without it
  an added field breaks parsing of the whole file, and `settings::load` treats
  the failure as "no settings" — silently resetting theme and language along
  with the new field. The side effect to live with: `specta` marks every field
  of such a struct as optional, so the frontend reads them through `??`.
- **`getBoundingClientRect()` on an element under `v-show` returns zeros.**
  The container must be shown before measuring.
- **Errors from Rust are not translated on the backend.** Commands return
  `AppError { code, params }`, and the frontend maps `errors.<code>`. An unknown
  code is shown as the code itself — the UI neither crashes nor shows blankness.
- **`opener:allow-open-path` without a scope denies every path.** `open_path`
  has a scope check in the plugin, `reveal_item_in_dir` has none at all, which
  makes moving from one command to the other look harmless. Declaring the
  identifier as a bare string opens the command with an empty list of allowed
  paths, and `is_path_allowed` denies all of them: the permission must be an
  object with `allow`. Our paths are arbitrary — builds live anywhere — so it
  is `{ "path": "**" }` there.
- **A rejected promise from a Tauri plugin is not an `AppError`.**
  `void openPath(…)` swallows the rejection whole: the button is pressed,
  nothing happens, the screen stays empty. Plugin calls are wrapped in
  `try/catch`, and the rejection is mapped to an error code and shown as
  a toast, like an error from our own commands.
- **In `apps/design/**/*.md` a blank line before markup indented by ≥4 spaces
  breaks the page silently.** markdown-it (CommonMark) treats such a line as an
  indented code block rather than a continuation of the HTML inside
  `<Window>`/`<ThemePair>` — the component falls apart with "Element is missing
  end tag" far away from the edit. There must be no blank lines inside those
  two components at all, at any nesting depth.
- **Layout/UI components in `apps/desktop/src/components/ui/` have `<style>`
  without `scoped`.** These components are thin wrappers around a class and
  a slot (`Group`, `ScreenHeader`, `KeyValueRow`…), and their CSS styles what
  came in through `<slot />`, not just their own markup. With a scoped style
  the data-v attribute goes to elements literally written IN THAT FILE; slot
  content written in the calling screen carries the caller's attribute, not the
  component's — so a flat selector like `.screen-head .lead` in scoped CSS
  silently matches nothing. `:slotted()`/`:deep()` fix this for `apps/desktop`
  but break `apps/design`: the showcase imports the same `.css` directly
  (`import '...css'` in `.vitepress/theme/index.ts`), bypassing the Vue SFC
  compiler, and those pseudo-classes are constructs of that compiler
  specifically — outside an SFC they mean nothing and the whole selector is
  silently dropped. Without `scoped`, one and the same flat CSS works correctly
  in both places.

## Scrolling

The window never scrolls. A screen has one scroll area; the exception is
master-detail, which has two. The ComfyUI area neither scrolls nor drifts.
Panels with a fixed header and footer keep the scroll inside, between them.

## Generated files are not edited by hand

| File | Produced by |
|---|---|
| `apps/desktop/src/bindings.ts` | `tauri-specta` when the dev build runs |
| `apps/design/.vitepress/theme/preview-tokens.css` | `pnpm design:tokens` from `apps/desktop/src/styles/tokens.css` |

Edits go into the source, then it is regenerated. Both are guarded by a hook —
the refusal names the source and the regeneration command.

The logo is an exception: the design file and the slicer live outside the
repository, only the result is here. `apps/docs/public/logo.svg`,
`apps/docs/public/favicon.svg` and `apps/desktop/src-tauri/icons/` are ordinary
assets and are edited as they are. Changing the mark means replacing files from
the external set.

**`bindings.ts` is nevertheless version-controlled**, unlike the second file.
Only a dev build can generate it — `cargo test` does not work here and never
will — and the frontend build does not run without it at all. Without it in the
repository, CI can build neither the release nor the quality gates. So the file
is committed: any drift from the Rust signatures is caught by the first
`pnpm typecheck` after a dev build.

## Commands

```
pnpm dev:desktop      dev build of the app (Vite + cargo, hot reload)
pnpm dev:build-desktop  the same, but without the devtools feature — no inspector
pnpm build:desktop    release build, NSIS installer
pnpm design:check     rebuild showcase tokens and run contrast and theme-parity checks
pnpm design:tokens    rebuild apps/design/.vitepress/theme/preview-tokens.css from the app
pnpm i18n:check       locale key parity against en.json
pnpm typecheck        vue-tsc over the frontend
pnpm kill             kill the process holding the dev server port
```

## What lives in `.claude/`

Short absolute rules — they are loaded into every session in full. Procedures
are not added here; they belong in skills: only a skill's name and description
stay in context permanently, the body is pulled in on demand.

| Skill | When it fires |
|---|---|
| `phase` | opening and closing a phase: what to read, what to update |
| `verify` | what to run before a commit, and in what order |
| `i18n` | any edit to UI strings |
| `design-ui` | laying out a screen: what already exists in `apps/desktop/src/styles/` |
| `vue` | Vue 3 Composition API, by antfu |

| Agent | What for |
|---|---|
| `rust-check` | build Rust and return a diagnosis instead of a wall of `cargo` output |
| `spec-audit` | check what was built against the acceptance criteria before closing a phase |

Hooks: a ban on editing generated files, `i18n:check` after locale edits,
a reminder about `design:tokens` after editing the app's tokens. The dispatcher
is `tools/claude-hook.mjs`.

Settings and skills are read at session start: a new skill or an edit to
`settings.json` takes effect only after a restart.

## Commits

Conventional Commits, subject and body in English. The body explains **why**,
it does not retell the diff: what problem was being solved, what surfaced along
the way, what decision got settled. Empty bodies will not do.

## Boundaries

- The user's models and shared templates are **never** deleted, including on
  app uninstall. There are exactly three exceptions, and the user starts each
  one with an explicit action, seeing in advance what exactly will go away:

  1. moving models into the shared folder — the source disappears only after
     verifying the copy is in place;
  2. cleaning up a duplicate whose identical file already lies in the shared
     folder;
  3. taking a workflow into the library — same order as moving models: the copy
     is written, read back and compared, and only then the source is removed
     from the build. There is no overwriting at all: a taken name is resolved by
     comparing contents, and a diverged version is taken under a free name.
     Replacing would mean erasing one piece of work with another, leaving
     neither.

  **Nothing is ever deleted silently.**
- The app does not touch ComfyUI's own theme and language settings inside the
  tab.
- `custom_nodes` in `extra_model_paths.yaml` is blacklisted.
