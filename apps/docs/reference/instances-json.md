# `instances.json`

The build registry. Plain JSON, in
`%APPDATA%\io.github.quo0.comfyui-organizer\instances.json`, under a single
`instances` key.

It is a list of pointers. It never contains models, workflows or anything
copied out of your builds — only where each build is and how you labelled
it.

```json
{
  "instances": [
    {
      "id": "i1750000000000",
      "name": "SDXL stable",
      "description": "The one that works",
      "path": "D:\\AI\\comfy-sdxl",
      "accent": "azure",
      "preferredPort": 8188,
      "comfyVersion": "0.3.62",
      "pythonVersion": "3.12.10",
      "createdAt": 1750000000000,
      "lastStartedAt": 1750600000000,
      "shared": { "enabled": true, "applyMode": "flag" },
      "customProfiles": [
        {
          "id": "custom:1",
          "name": "SDXL, low VRAM",
          "baseId": "run_nvidia_gpu.bat",
          "args": ["-s", "ComfyUI\\main.py", "--lowvram"]
        }
      ],
      "source": {
        "archivePath": "D:\\downloads\\ComfyUI_windows_portable_nvidia.7z",
        "archiveLabel": "ComfyUI_windows_portable_nvidia.7z",
        "installedAt": 1750000000000
      }
    }
  ]
}
```

## Fields

| Field | Meaning |
| --- | --- |
| `id` | Generated once, never reused. Referenced by the workflow manifest. |
| `name`, `description` | Yours. Never translated, never guessed at. |
| `path` | Absolute path to the build root — the folder holding `python_embeded\` and `ComfyUI\`. |
| `accent` | A palette token name (`azure`, `moss`, …) or your own colour as `#rrggbb`. |
| `preferredPort` | Tried first at launch; the next free port is used if it is taken. |
| `comfyVersion`, `pythonVersion` | Read at registration, for display only. |
| `createdAt`, `lastStartedAt` | Milliseconds since the epoch. `lastStartedAt` is absent until the first launch. |
| `shared` | `enabled` plus `applyMode`: `flag` or `instanceFile`. See [Shared models](/guide/shared-models#two-ways-to-apply-it). |
| `customProfiles` | Profiles you built on top of a parsed one. `baseId` is the `.bat` they extend. |
| `source` | Present when the build came from the unpacking wizard. |

Fields added by later versions of the app carry defaults, so a registry
written by an older version is read without complaint — and, more to the
point, without resetting anything else in it.

`available` is not stored. Whether the folder still exists is recomputed
every time the registry is read.

## Editing it by hand

Nothing stops you, when the app is closed. It is a small file with absolute
paths in it, and fixing a moved build there is faster than clicking. Two
things to keep in mind: `path` needs escaped backslashes, as JSON requires,
and an `id` referenced by your workflow library should not be changed
casually.
