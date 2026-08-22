# Flags we add

The app starts `python.exe` directly, with the arguments parsed from the
profile plus at most three of its own. You can see the resulting command
line in the arguments editor before starting anything.

## `--port <n>`

The app assigns the port: the build's preferred port if it is free, the
next free one otherwise.

Any `--port` already present in the profile is removed **together with its
value**, in both spellings (`--port 8188` and `--port=8188`). Two builds
with the same hard-coded port would mean the second one failing at startup
for a reason that looks unrelated to anything you did.

## `--disable-auto-launch`

So that starting a build does not open a browser tab.

This works without patching anything, because of the order inside
ComfyUI's own argument handling (`comfy/cli_args.py`):

```python
if args.windows_standalone_build:
    args.auto_launch = True
if args.disable_auto_launch:
    args.auto_launch = False
```

The disable flag is applied second and always wins, so appending it to the
command line is enough.

## `--extra-model-paths-config <path>`

Added only when the build is connected to a shared models folder in **flag
mode**. The path points at the config in the app's own data folder, which
is regenerated whenever the settings change.

An `--extra-model-paths-config` already present in the profile is **not**
touched. The flag accumulates (`nargs='+'`, `action='append'`), and
ComfyUI loads `ComfyUI/extra_model_paths.yaml` first and the files from the
flag afterwards, so ours adds to yours instead of replacing it.

In "file inside the build" mode no flag is added at all: the file sits in
the build folder and ComfyUI picks it up on its own.

## What we never add

- `--enable-cors-header`. It would let the app embed ComfyUI in an
  `<iframe>` — and it disables ComfyUI's cross-site protection entirely,
  which means any website open in your browser could talk to your local
  server. The app uses a native child window instead, which ComfyUI's own
  middleware accepts as an ordinary top-level navigation.
- `--listen`. The server stays on `127.0.0.1`.
- Anything that changes model paths beyond the config above.
