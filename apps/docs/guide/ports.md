# Ports and conflicts

## The app picks the port

Every build has a **preferred port** — 8188 by default, the same as
ComfyUI's own. When you start a build, the app takes that port if it is
free and the next free one if it is not, then passes it as `--port`.

A `--port` written into the `.bat` file is removed from the command line
together with its value. Two builds started with the same hard-coded port
would mean the second one failing at startup with a message that has
nothing to do with what you did.

## Two builds at once

Starting a second build while another is running is allowed, but the app
asks first. The reason is not the port — that one it solves — but the
video memory: the second build will load its models into the same GPU and
fail somewhere inside a generation queue, with an error that says nothing
about the real cause.

The choice is yours: **stop the other one**, or **start anyway**. On a
machine with two GPUs, or with a CPU profile, starting anyway is exactly
right.

## Stopping

**Stop** terminates the process tree and waits for the port to be released
before reporting the build as stopped. That wait matters: without it, a
restart would race the operating system for the port.

All ComfyUI processes started by the app live in a Windows Job Object with
`KILL_ON_JOB_CLOSE`. If the app itself is killed — from Task Manager, or by
an installer during an update — the servers die with it. This is
deliberate: an orphaned `python.exe` holding twenty gigabytes of video
memory is worse than a stopped server.

It is also why the app asks what to do with running builds when you close
the window or install an update.

## When ComfyUI restarts itself

ComfyUI-Manager restarts the server after installing custom nodes: it kills
the process and starts a new one. The app loses its handle on it — the PID
it knew is gone, the port is still answering.

That state is shown as **detached**: the server is alive but not ours, and
pretending otherwise would be a lie. The app polls the port for 15 seconds
after such an exit and then offers to **reconnect**: it finds the process
that owns the port, records its PID and everything works again — stopping
included.

The tab is not closed while the build is detached. The server behind it is
alive, and taking a working interface away from you would be rude.

## Crashes

If the process exits with a non-zero code, the build is marked **crashed**,
the exit code is shown, and a badge appears next to it in the rail — even
if you are on another screen when it happens. The log stays where it is, so
you can read the ending.

No toast is raised for a crash while its tab is open: the status in the
rail and the log say it better, and a toast would be swallowed by the
native ComfyUI window anyway.
