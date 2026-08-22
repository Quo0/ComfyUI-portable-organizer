# Launch profiles

A profile is what one `.bat` file means, in a form the app can start
directly. Profiles are re-derived from disk on every launch — the `.bat`
files are the source of truth, and they are never modified.

## The shape

| Field | Meaning |
| --- | --- |
| `id` | Path of the `.bat` relative to the build root. Also its identity: `run_nvidia_gpu.bat`, `advanced\run_nvidia_gpu_disable_api_nodes.bat`. |
| `name` | File name without the extension. Never translated. |
| `advanced` | It came from the `advanced\` subfolder. |
| `pythonPath` | Absolute path to the interpreter the line invokes. |
| `args` | The arguments, tokenised, quotes honoured. |
| `cwd` | The folder of the `.bat` file itself. |
| `env` | Variables from `set KEY=VALUE` lines above the command. |
| `fallback` | Parsing failed; the file will be run through `cmd /c` instead. |

Custom profiles of your own reuse everything from a base profile and
replace only `args` — see
[`instances.json`](/reference/instances-json).

## How the file is read

`echo`, `pause`, `rem` and blank lines are skipped. `set KEY=VALUE` lines
become environment variables. The first real command line is tokenised
with quotes respected, the executable becomes `pythonPath` and the rest
becomes `args`.

Relative paths — including the `..\` in `advanced\*.bat` — are resolved
against the folder of the `.bat` file, because that is what happens when
you double-click it. That folder becomes the working directory too.

Profiles that do not start a server, such as the update scripts shipped
inside the package, are kept out of the launch list.

## `fallback: true`

When a line cannot be parsed, the profile is not dropped: it is marked as a
fallback and started through `cmd /c` instead. The interface says so,
because that route puts an extra `cmd.exe` between the app and Python and
makes stopping the process less reliable.

## Paths derived from the arguments

Three folders are resolved out of the arguments rather than assumed,
following the same precedence ComfyUI itself uses:

| Folder | Flags consulted, in order |
| --- | --- |
| Models | `--base-directory`, else `<build>\ComfyUI\models` |
| User data (workflows live here) | `--user-directory`, then `--base-directory`, else `<build>\ComfyUI\user` |
| Output | `--output-directory`, then `--base-directory`, else `<build>\ComfyUI\output` |

This is why "open the output folder" and "add a workflow to a stopped
build" land in the right place even when your profile moves those folders
somewhere else.
