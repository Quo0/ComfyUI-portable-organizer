# Uninstalling

Uninstall from **Settings → Apps → Installed apps**, the ordinary Windows
way. No administrator rights are needed, because the app was installed into
your user profile in the first place.

The uninstaller asks whether to delete the application data too.

## What disappears

| Ticked "delete application data" | Removed |
| --- | --- |
| The app itself | always |
| `%APPDATA%\io.github.quo0.comfyui-organizer` — settings, build registry, the path to your library | only when ticked |
| `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer` — cache and WebView2 data | only when ticked |

Leaving the box unticked keeps your registry and settings, and reinstalling
later picks them up as they were.

## What stays, always

- **Your models.** Both the shared folder and the ones inside each build.
- **Your workflow library**, including its `_library.json`.
- **The ComfyUI builds themselves.** The registry only ever remembered
  where they are.
- **The two kinds of file the app wrote inside your builds**, if you asked
  it to: `ComfyUI\extra_model_paths.yaml` in "file inside the build" mode,
  and the workflow copies you added to a build. They sit inside someone
  else's installation, and taking them out is not ours to do.

All of this is listed in the app itself, on the **About** screen, before
you uninstall anything — with a button next to each path that opens it in
Explorer.

## Nothing is ever deleted silently

The app deletes user content in exactly three situations, and each one
starts with your click, after you have been told what will go:

1. **Moving models to the shared folder** — the original is removed only
   after the copy has been written, read back and compared.
2. **Removing duplicate models** — only files that are verifiably present
   in the shared folder already.
3. **Collecting a workflow into the library** — same order as for models:
   copy, verify, then remove.

Outside those three, nothing of yours is touched.

## Cleaning up by hand afterwards

If you unticked the box and change your mind later, delete these two
folders:

```
%APPDATA%\io.github.quo0.comfyui-organizer
%LOCALAPPDATA%\io.github.quo0.comfyui-organizer
```

There is nothing else. The app writes nothing to `Documents`, nothing to
`%PROGRAMDATA%`, and nothing to the registry beyond what the installer puts
there for the uninstall entry.
