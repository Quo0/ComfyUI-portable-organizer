# Reference

What the app writes, where it writes it, and what it hands to ComfyUI.
This section exists so you can check the app rather than trust it.

- [Flags we add](/reference/flags) — the three command-line flags, and why
  each one is there.
- [instances.json](/reference/instances-json) — the build registry format.
- [Launch profiles](/reference/launch-profile) — how a `.bat` file becomes
  a profile.
- [Generated YAML](/reference/extra-model-paths) — the shared-models config
  we produce.

## Where things live

| Path | What |
| --- | --- |
| `%APPDATA%\io.github.quo0.comfyui-organizer\settings.json` | Settings: theme, language, shared models, library path |
| `%APPDATA%\io.github.quo0.comfyui-organizer\instances.json` | The build registry |
| `%APPDATA%\io.github.quo0.comfyui-organizer\shared-models.yaml` | The generated config, in flag mode |
| `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer\` | Node snapshots per build, WebView2 data |
| `<library>\_library.json` | Tags, notes, favourites — inside your library, not here |

All of these are plain text. Nothing about your builds is stored anywhere
else, and nothing is stored in the Windows registry beyond the uninstall
entry the installer creates.
