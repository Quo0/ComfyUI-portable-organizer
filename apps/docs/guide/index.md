# What this is

ComfyUI Portable Organizer is a Windows desktop app that manages several
portable ComfyUI installations: a registry of your builds, a launcher, and
ComfyUI itself rendered as a tab inside the app window.

It is a shell around folders you already have. It never patches ComfyUI,
never edits your `.bat` files, and never touches `custom_nodes`.

## The vocabulary

**Build** — one portable ComfyUI folder, the kind you get by unpacking
`ComfyUI_windows_portable_nvidia.7z`. It contains `python_embeded\` and
`ComfyUI\`, and it is self-contained. The app calls these builds; the
registry is the list of them.

**Launch profile** — one way to start a build. The `.bat` files that ship
with the portable package (`run_nvidia_gpu`, `run_cpu`,
`advanced\run_nvidia_gpu_disable_api_nodes`…) each become a profile. You
can add your own on top; the `.bat` files themselves stay untouched.

**Shared models folder** — one folder outside the builds where the heavy
files live. Builds are pointed at it through ComfyUI's own
`extra_model_paths.yaml` mechanism.

**Workflow library** — one folder outside the builds where the graphs you
care about live, with tags, notes and favourites.

## The short version

1. [Install the app](/guide/install-app), click through the SmartScreen
   warning.
2. [Add a build](/guide/add-build) — point at an existing folder, or
   [unpack one from an archive](/guide/install-from-archive).
3. Press **Start**. The app picks a free port, adds
   `--disable-auto-launch` so no browser opens, and streams the startup log
   until the server answers. Then ComfyUI appears in a tab.
4. Optionally: point the build at a
   [shared models folder](/guide/shared-models) and a
   [workflow library](/guide/workflows).

## What it will not do

- Edit graphs, render canvas previews, or version your workflows.
- Install or update custom nodes — that is ComfyUI-Manager's job. The app
  only tells you which nodes a workflow needs and a build does not have.
- Change ComfyUI's own theme or language.
- Share `custom_nodes` between builds, ever. Conflicting custom nodes are
  the reason separate builds exist in the first place.
- Delete your models or workflows. See [Uninstalling](/guide/uninstall)
  for the exact list of what is and is not removed.
