# Changelog

The format is [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), the
numbering is [semantic](https://semver.org/).

**The GitHub release body is assembled from here.** The release workflow takes
the `## <version>` section whole, up to the next heading of the same level. If
the section is missing, the build fails before anything is built: a release with
no description of the changes is worse than a postponed release.

Hence the release order: the "Unreleased" section is renamed to
`## <version> — <date>`, the version in
`apps/desktop/src-tauri/tauri.conf.json` is raised to match, and only then is
the `v<version>` tag created.

## 0.1.1 — 2026-09-01

A maintenance release. The app itself has not changed: this version exists to
run the release pipeline through once more.

### Changed

- **The repository speaks English.** Rust and frontend comments, `specs/`, the
  design showcase and the infrastructure scripts have been translated. The UI
  strings, the four documentation locales and the deliberately non-ASCII test
  data are untouched.
- **The design showcase moved** from `apps/design` to `apps/ui-design`, and the
  root scripts are named target-first: `desktop:*`, `docs:*`, `ui-design:*`.

### Added

- `RELEASING.md` — the release checklist, back in the repository.

### Removed

- **The technical plan** (`PLAN.md` and `plan/`). What was absolute in it is
  repeated in `CLAUDE.md`; the reasoning stays in `git log`.

## 0.1.0 — 2026-08-27

### Added

- **Build registry.** A ComfyUI portable folder is added as is: the app checks
  `python_embeded\python.exe` and `ComfyUI\main.py`, and reads the build and
  Python versions. Name, description, colour and preferred port are per build.
- **Install wizard** from an archive: reading the contents without extracting,
  several destinations in one pass (the second and later ones are produced by
  copying rather than extracting again), a free space check before the work
  starts, cancellation without a trace.
- **Launching.** Profiles are parsed from the build's own `.bat` files, which
  are left unchanged. Custom profiles and the argument editor sit on top of the
  parsed ones. A free port is handed out, no browser is opened, and the startup
  log is visible live.
- **ComfyUI inside the window** as a separate tab per build: switching between
  running ones, a toolbar with logs, the output folder, restart and stop.
  External links go to the system browser.
- **Shared model storage** through `extra_model_paths.yaml`: the shared folder
  is connected to any build with a toggle, with two modes to choose from.
  `custom_nodes` is never shared. Models are moved into the shared folder with
  the copy verified before the source is deleted.
- **Workflow library** outside the builds: filled from a file, by drag and drop,
  by pasting text, and from a PNG image with a graph inside; adding to a build
  reports the missing nodes.
- **Duplicate model report** across all builds at once. A report only: not
  a single automatic action on files.
- **Tray icon and a single app instance.** Closing the window while servers are
  running asks what to do with them.
- **Four UI languages** (English, Russian, Chinese, Spanish), light and dark
  themes, following the Windows theme on the fly.
- **App updates**: a check at startup, which can be turned off in the "About"
  section. Installation does not start while any build is running — first comes
  the choice of stopping them or postponing the update.
