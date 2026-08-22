# Install the app

## Requirements

- Windows 10 or 11, 64-bit.
- WebView2 runtime. It ships with Windows 11 and with any recent Windows 10;
  the installer pulls it in if it is missing.
- At least one portable ComfyUI build — or an archive to unpack, which the
  app can do for you.

No administrator rights are needed.

## Installing

1. [Download the installer](/download).
2. Windows shows **"Windows protected your PC"**. This is expected and it
   is not a virus warning — see
   [the SmartScreen page](/guide/smartscreen) for what to click and how to
   verify the file.
3. The app installs into your user profile
   (`%LOCALAPPDATA%\Programs\ComfyUI Portable Organizer`) and starts.

## Where the app keeps its own data

| What | Where |
| --- | --- |
| Settings, build registry, path to your library | `%APPDATA%\io.github.quo0.comfyui-organizer` |
| Cache and WebView2 browser data | `%LOCALAPPDATA%\io.github.quo0.comfyui-organizer` |

Both folders are listed in the app under **About**, each with a button that
opens it in Explorer. Nothing is written to `Documents`, to
`%PROGRAMDATA%`, or next to the executable.

Your models, workflows and ComfyUI builds live wherever you put them, and
the app does not move them there or away. See
[Uninstalling](/guide/uninstall).

## First run

The app starts on an empty registry with one thing to do: add a build.
Language follows Windows on first run — English, Russian, Simplified
Chinese and Spanish are built in — and the theme follows the system until
you pick one in **Settings → Appearance**.
