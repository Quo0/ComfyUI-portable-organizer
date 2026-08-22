# Add a build

Go to **Add**. There are two routes: point at a folder you already have,
or [unpack a new build from an archive](/guide/install-from-archive).

## Pointing at an existing folder

Pick the **root** of the portable build — the folder that contains
`python_embeded\` and `ComfyUI\`, not one of them:

```
ComfyUI_windows_portable\      ← this one
├─ python_embeded\
│  └─ python.exe
├─ ComfyUI\
│  └─ main.py
├─ run_nvidia_gpu.bat
├─ run_cpu.bat
└─ advanced\
   └─ run_nvidia_gpu_disable_api_nodes.bat
```

The app checks that `python_embeded\python.exe` and `ComfyUI\main.py` both
exist, reads the ComfyUI version from `ComfyUI\comfyui_version.py`, asks
`python.exe` for its version, and collects the `.bat` files as
[launch profiles](/guide/profiles).

Then you give the build a name, an optional description, an accent colour
and a preferred port. All four are yours to change later.

## What the app writes into the build folder

By default: **nothing**. The registry lives in the app's own data folder
and only remembers where each build is.

Exactly two things can ever be written inside a build folder, and each one
starts with a click of yours:

- `ComfyUI\extra_model_paths.yaml`, and only if you choose the
  ["file inside the build" mode](/guide/shared-models#two-ways-to-apply-it)
  for shared models;
- copies of workflows you add to that build from
  [the library](/guide/workflows).

Both are listed on the **About** screen, and both stay when the app is
uninstalled — they sit inside someone else's installation, and taking them
out is not ours to do.

## A missing folder is not a deleted build

If you move or rename a build folder, the entry stays in the registry and
is marked unavailable. It does not silently disappear — that would look
like the app lost your build. Point it at the new location or remove the
entry.

**Removing an entry removes the entry.** The folder on disk is not touched.

## Two builds of the same version, side by side

Nothing stops you from registering the same archive unpacked twice, or two
builds that differ only in custom nodes. Give them different names and
accent colours; the colour follows the build through the rail, the cards
and the tab, and it is the fastest way to tell two similar builds apart.
