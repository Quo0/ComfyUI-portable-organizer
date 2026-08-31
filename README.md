# ComfyUI Portable Organizer

**Several portable ComfyUI builds, one window.**

A registry for the portable ComfyUI folders you already have, a launcher
that picks a free port for you, and ComfyUI itself rendered as a tab inside
the app — plus one shared models folder instead of five copies of the same
20 GB checkpoint.

[![License: GPL v3](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-0078d4.svg)](https://quo0.github.io/ComfyUI-portable-organizer/download)
[![Latest release](https://img.shields.io/github/v/release/Quo0/ComfyUI-portable-organizer?display_name=tag&sort=semver)](https://github.com/Quo0/ComfyUI-portable-organizer/releases)

> Not affiliated with, endorsed by, or sponsored by Comfy Org. This is an
> independent third-party tool. It does not bundle ComfyUI — it manages
> copies you downloaded yourself.

## Why

People end up with several portable ComfyUI installs for a good reason:
custom nodes conflict with each other, and updating one build breaks
another. What that leaves you with is a folder of `.bat` files, no list of
what is installed where, no control over ports, no visibility into the
startup log, and a browser tab per build.

This app is the missing shell around those folders.

## What it does

- **Every build in one list.** Point it at a portable folder and it reads
  the ComfyUI version, the Python version and the launch scripts. Nothing
  inside the folder is modified.
- **ComfyUI inside the window.** Each running build gets a tab. Switching
  between two builds is one click, and the browser never opens behind your
  back.
- **One models folder.** Several builds share it through ComfyUI's own
  `extra_model_paths.yaml`. `custom_nodes` is never shared — that would
  undo the whole reason the builds are separate.
- **A workflow library outside the builds**, with tags and notes, and a
  check for which nodes a build is missing before you open a graph.
- **Install from an archive**, with the contents read before unpacking and
  free space checked before the first byte lands.

## What it does not do

- Edit workflow graphs or render previews of them.
- Install or update custom nodes — that is ComfyUI-Manager's job.
- Change ComfyUI's own theme or language settings.
- Delete your models or workflows. Three operations touch your files, each
  one starts with your click, and none of them is silent.

## Install

Windows 10 or 11, 64-bit. No administrator rights — the installer puts the
app into your user profile.

**[Download the installer →](https://quo0.github.io/ComfyUI-portable-organizer/download)**

Windows will show a "Windows protected your PC" warning: the app is not
signed with a code-signing certificate. [What to do about
it](https://quo0.github.io/ComfyUI-portable-organizer/guide/smartscreen).

## Documentation

[English](https://quo0.github.io/ComfyUI-portable-organizer/) ·
[Русский](https://quo0.github.io/ComfyUI-portable-organizer/ru/) ·
[Español](https://quo0.github.io/ComfyUI-portable-organizer/es/) ·
[简体中文](https://quo0.github.io/ComfyUI-portable-organizer/zh/)

## Building from source

Requires Node 20+, pnpm, and the Rust toolchain with the
[Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) for
Windows.

```
pnpm install
pnpm desktop:dev      # dev build, hot reload
pnpm desktop:build    # release build, NSIS installer
```

Other useful scripts:

```
pnpm typecheck          # vue-tsc over the frontend
pnpm i18n:check         # locale key parity against en.json
pnpm ui-design:check    # design tokens, contrast and theme parity
pnpm docs:build         # the documentation site
```

## Repository layout

| Path | What lives there |
|---|---|
| `apps/desktop` | the app itself — Vue frontend, Tauri/Rust backend |
| `apps/docs` | the user documentation site (VitePress, four locales) |
| `apps/ui-design` | the style guide and screen showcase |
| `specs/` | what the app is meant to do: requirements, stories, acceptance criteria |

## License

**[GNU General Public License, version 3 only](LICENSE).**

In plain terms:

- **Use it for anything**, including commercially and at work. No fee, no
  key, no seat count, no asterisk.
- **Fork it and change it** freely.
- **If you pass it on**, the person you give it to gets the source under
  this same licence, with no restrictions added on top. What you received,
  they receive.

Selling a copy is permitted — but the buyer gets the source under the GPL
and may publish it for everyone, free, immediately. The licence does not
forbid the price; it removes the ability to defend one. Charging for
support, setup or hosting is a service, and that was always fine.

**The name is not part of the grant.** "ComfyUI Portable Organizer", the
short form "CPO", and the project's icons are not licensed to forks. A fork
ships under its own name, so users can tell whose build they are running.
Describing the lineage factually — "a fork of ComfyUI Portable Organizer" —
is fine and always will be. See [NOTICE](NOTICE).

**Your models and workflows are not covered by any of this.** The licence
applies to the app's code, not to what you make with it.

The full plain-language summary lives at
[the licence page](https://quo0.github.io/ComfyUI-portable-organizer/license).
