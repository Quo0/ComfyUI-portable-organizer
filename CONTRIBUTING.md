# Contributing

Thank you for looking. This is a small project with one maintainer, so the
most useful thing you can do is make your contribution cheap to accept: say
what problem it solves, run the checks yourself, and keep one topic per pull
request.

Everything here is written in English — code, comments, commits and
discussion. The exceptions, places where Russian, Spanish or Chinese is the
data rather than a note, are listed in [CLAUDE.md](CLAUDE.md#language).

## Start with an issue

For anything larger than a typo, open an issue before you write the code.

The app has a deliberate shape: it is a shell around portable ComfyUI folders
it does not own. What it is meant to do is written down in [`specs/`](specs/),
and what it will not do is in the README under
[What it does not do](README.md#what-it-does-not-do). A pull request that
solves a problem the app has decided not to solve costs you an evening and the
maintainer an awkward reply. An issue costs a paragraph.

The [issue forms](https://github.com/Quo0/ComfyUI-portable-organizer/issues/new/choose)
sort this out: bug, feature request, a build the registry refuses, a
translation or wording problem. If you have already written the patch, still
open the issue and link it — the work is not wasted, the discussion just
happens in the right order.

**Problems inside ComfyUI itself belong
[upstream](https://github.com/comfyanonymous/ComfyUI/issues).** This app starts
builds and shows them; it does not run the graph, install custom nodes or
update them. The quickest way to tell the two apart is to start the same build
by double-clicking its own `.bat` with the app closed.

## What is easy to accept

- **A bug fix with a reproduction.** The narrower the diff, the faster it
  merges.
- **Translation and wording fixes.** See [Translations](#translations) below —
  these are a minute of work and they never get done otherwise.
- **Documentation.** English is the source; a page that is only English is
  fine, say so in the pull request and the other locales can follow.
- **A feature that already carries acceptance criteria in `specs/`.** If it
  does not, that discussion happens in the issue first — the spec is written
  before the screen, not after it.

## Running it

The app itself is Windows-only, and not incidentally: it parses `.bat`
launchers, embeds ComfyUI through WebView2 and ships as an NSIS installer.
The documentation site and the design showcase build anywhere.

You need:

- **Windows 10 or 11, 64-bit** for the app.
- **Node 22.13 or newer.** pnpm requires it for `node:sqlite`.
- **pnpm** — the version is pinned in `package.json` under `packageManager`;
  `corepack enable` picks it up.
- **The Rust toolchain** plus the
  [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for
  Windows: MSVC build tools and the WebView2 runtime.

```
pnpm install
pnpm desktop:dev        # the app, hot reload
pnpm ui-design:dev      # the style guide and screen showcase
pnpm docs:dev           # the documentation site
pnpm kill               # frees the dev server port when a run is left behind
```

**You do not need a real ComfyUI build for most work.** Launch scenarios run
against `tools/fixtures/fake-instance/`, which starts in a second instead of
five minutes; if its `python_embeded` junction is missing, create it with
`node tools/fixtures/make-fixture.mjs`. It is a stub — it does not parse YAML,
so anything about the shared models folder still needs a real build.

## Rules that a pull request must not break

Each of these was found the expensive way. The full list with the reasoning is
in [CLAUDE.md](CLAUDE.md#rules-already-paid-for); these are the ones a patch
runs into most often.

1. **UI strings go through `t()` only.** `en.json` is the source of truth;
   Russian, Spanish and Chinese follow it. Paths, instance names and log
   contents are never translated.
2. **Generated files are not edited by hand.** `apps/desktop/src/bindings.ts`
   comes from `tauri-specta` on a dev build;
   `apps/ui-design/.vitepress/theme/preview-tokens.css` comes from
   `pnpm ui-design:tokens`. Edit the source and regenerate.
3. **Tauri commands are `async fn`.** A synchronous command runs on the main
   thread and `add_child` inside it deadlocks silently. Keep the logic in plain
   synchronous functions and make the command a thin wrapper.
4. **The ComfyUI tab is a native window on top of our HTML.** Dropdowns, modals
   and toasts physically cannot cover it. Settings, adding an instance and the
   argument editor are separate routes; messages on the tab screen are a banner
   above the webview's rectangle, and the log console takes its place rather
   than overlaying it.
5. **Errors from Rust are not translated on the backend.** Commands return
   `AppError { code, params }` and the frontend maps `errors.<code>`.
6. **Settings structs carry `#[serde(default)]` on the container.** Without it
   one added field breaks parsing of the whole file, and the failure reads as
   "no settings" — silently resetting the user's theme and language.
7. **Colours, spacing and radii come from the tokens** in
   `apps/desktop/src/styles/`. Nothing is hardcoded: the showcase reads those
   same files, and `pnpm ui-design:check` fails on contrast and theme parity.
8. **The user's models and workflows are never deleted.** There are exactly
   three operations that remove a file, each one starts with an explicit click
   that shows what will go away, and each verifies the copy before removing the
   source. Nothing is ever deleted silently.

## Checks before you open a pull request

**There is no CI on pull requests.** The quality gates run on a release tag,
which is far too late to be your feedback loop — whatever you did not run
locally, nobody ran.

| What you edited | What to run |
|---|---|
| `apps/desktop/src/i18n/locales/*.json` | `pnpm i18n:check` |
| `apps/desktop/src/styles/tokens.css` or `components.css` | `pnpm ui-design:check` |
| Vue, TypeScript, app styles | `pnpm typecheck` |
| Rust in `src-tauri/src/**` | `cargo check`, plus the relevant `examples/` check |
| A Tauri command or event signature | restart `pnpm desktop:dev` **first**, then `pnpm typecheck` |

Two traps in that table:

- **`bindings.ts` is generated only by `pnpm desktop:dev`.** Running
  `pnpm typecheck` before restarting the dev build checks the old types, so it
  passes while the frontend and the backend disagree.
- **`cargo test` does not work in this repository and will not.** The test
  binary fails while loading the image, before `main`, as soon as it pulls in
  the `tauri_specta` machinery. The Rust checks live in
  `apps/desktop/src-tauri/examples/` instead and run as
  `cargo run --example check_profiles`. A new check goes in the same place with
  the same shape: print the steps, exit non-zero on a mismatch.

Some things do not automate — no 403 in the child webview, dragging an image
onto the ComfyUI canvas, behaviour on a Windows theme change. If your change
touches one of them, say in the pull request what you did by hand.

Say what you ran and what came out. "Everything checked" without a list of
commands means nothing; a check you skipped is fine as long as you name the
reason.

## Commits

[Conventional Commits](https://www.conventionalcommits.org/), subject and body
in English. The types in use here are `feat`, `fix`, `docs`, `refactor`,
`style`, `ci` and `chore`; the scopes are real areas of the repository — `ui`,
`i18n`, `specs`, `site`, `docs`, `install`, `update`, `release`, `workflows`.

**The body explains why; it does not retell the diff.** What problem was being
solved, what surfaced along the way, what decision got settled. `git log` is
the reference for the tone, and empty bodies will not do — the diff already
says what changed, and in a year that is never the question anyone has.

One logical change per commit. There is no need to polish history into a
single commit; there is a need for each commit to make sense on its own.

## The pull request

- **Branch off `master`**, one topic per branch.
- **Do not raise the version and do not edit `CHANGELOG.md`.** Releases are cut
  by the maintainer — the version lives in five files and the changelog section
  becomes the release body and the app's update panel, so a contributor's edit
  there is a merge conflict at best. Instead, put the one user-facing sentence
  describing your change in the pull request body; the template asks for it and
  it is what ends up in the changelog.
- **Screenshots for anything visual**, in both themes. Light and dark are not a
  filter over one another here, and a change that looks right in one can be
  unreadable in the other.
- **Draft pull requests are welcome** if you want a direction checked before
  you finish.
- **Leave `pnpm-lock.yaml` alone** unless you deliberately changed a
  dependency.

Review is by one person and is not instant. A pull request that names its
issue, lists the checks it ran and holds one change is the one that gets merged
in an evening.

## Translations

The app's strings live in `apps/desktop/src/i18n/locales/`: `en.json` is the
source, `ru.json`, `es.json` and `zh-Hans.json` follow it.

- **Add a key to all four locales at once**:
  `node tools/i18n-add.mjs some.key --en "…" --ru "…" --es "…" --zh "…"`, or
  `--file keys.json` for a batch. Four hand edits is how a locale silently
  falls behind.
- **`pnpm i18n:check` verifies key parity** against `en.json`. It is a release
  gate, so a missing key blocks a release rather than reaching a user.
- **Length is part of the translation.** Russian and Spanish run visibly longer
  than English, and a string that fits in `en.json` can overflow its button
  elsewhere. If the meaning survives, shorter wins; the language-length demos
  in the showcase exist for exactly this.
- **Do not translate paths, instance names or log contents**, and do not touch
  ComfyUI's own terminology where the user already knows the English word.

The documentation site is separate: English pages sit at the root of
`apps/docs/`, and `ru/`, `es/` and `zh/` mirror them. A new page needs a nav
entry per locale in `apps/docs/.vitepress/config.ts`.

**Adding a fifth language is a bigger commitment** than it looks — the app, the
docs site, and every string added afterwards. Open an issue first so it does
not become a locale that is half-translated forever.

## Licence

The project is [GPL-3.0-only](LICENSE). By opening a pull request you agree
your contribution is licensed under the same terms. There is no CLA and no
copyright assignment.

If you adapted code from somewhere, say where in the pull request body, and
check that its licence is compatible — GPL-incompatible code cannot be
accepted no matter how good it is. The project's name and icons are not part
of the licence grant; see [NOTICE](NOTICE).

## Security

If you believe you have found a security problem, do not open a public issue.
Use GitHub's
[private vulnerability reporting](https://github.com/Quo0/ComfyUI-portable-organizer/security/advisories/new)
instead, and give it the same detail you would give a bug report.
