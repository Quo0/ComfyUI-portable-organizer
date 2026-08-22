# Launch profiles

A profile is one way to start a build. The `.bat` files that ship with the
portable package become profiles automatically, including the ones in
`advanced\`.

## The `.bat` files are read, not run

Those files are one-liners:

```bat
.\python_embeded\python.exe -s ComfyUI\main.py --windows-standalone-build --fast fp16_accumulation
```

The app parses the line and starts `python.exe` directly. Two things go
away as a result: the `pause` at the end of the file, which would sit there
waiting for Enter, and the extra `cmd.exe` in the process tree, which makes
stopping the server less reliable.

Relative paths resolve from the folder of the `.bat` file itself — that is
what happens when you double-click it, and `advanced\*.bat` uses `..\`
paths that depend on it.

**Your `.bat` files are never modified.** They are re-read on every launch,
which is also why edits to them show up immediately.

If a file cannot be parsed, the profile falls back to running it through
`cmd /c` and is marked as such in the interface. Stopping such a process is
less reliable, and you should know that before you rely on it.

## What the app adds to the command line

Three flags, and you can see the exact resulting command in the arguments
editor before you start:

| Flag | Why |
| --- | --- |
| `--port <n>` | The app assigns the port. Any `--port` already in the profile is removed together with its value. |
| `--disable-auto-launch` | So no browser tab opens. ComfyUI applies this flag after `--windows-standalone-build`, so it always wins. |
| `--extra-model-paths-config <path>` | Only when the build is connected to a [shared models folder](/guide/shared-models) in flag mode. |

An `--extra-model-paths-config` that was already in the `.bat` is left
alone: the flag accumulates, and ours is added next to it rather than
instead of it.

## Your own profiles

**Settings of a build → Parameters** lets you build a profile of your own
on top of a parsed one: change the arguments, keep it under a name of your
choice. The originals stay as they are, because they are files inside
someone else's installation.

The editor shows the final command line — the one that will actually be
handed to Windows, with our flags already applied. Arguing with an
invisible command line is no fun.

## Which profile starts

The **Start** button is a split button: the main half starts the last
profile you used for that build, the arrow opens the list. Profiles that do
not start a server — the update scripts that ship in the package — are
filtered out of that list, because they are not launches.

## The startup log

Everything the process writes to stdout and stderr is streamed into the
app, live, from the first line. Progress bars from `tqdm` overwrite their
line instead of flooding the buffer, the same way they do in a terminal.
About 5 000 lines are kept.

While a build is starting, the app polls `GET /system_stats` until the
server answers, then swaps the log for the ComfyUI tab. A cold start of a
big build takes minutes; the timeout is five.
