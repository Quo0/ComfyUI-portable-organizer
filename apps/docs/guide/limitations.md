# Known limitations

Things that will surprise you if nobody says them out loud.

## Two builds share one GPU

Nothing stops you from running two builds at once, and the app asks before
it lets you. But the video memory is not divided: the second build loads
its models into the same GPU and fails inside a generation queue with an
error that says nothing about the real cause. Two GPUs, or a CPU profile,
are the cases where starting anyway makes sense.

## ComfyUI-Manager restarts the server behind our back

After installing custom nodes, ComfyUI-Manager kills the server and starts
a new process. Our handle on it is gone, so the build is shown as
**detached**: alive, but no longer ours to stop. The app offers to
reconnect — it finds the process that owns the port and takes it back. See
[Ports and conflicts](/guide/ports#when-comfyui-restarts-itself).

## Closing the app stops every server

The ComfyUI processes are children of the app, held in a Job Object that
takes them down with it. That is deliberate — an orphaned `python.exe`
holding your video memory is worse — but it means the window close button
asks a question when anything is running, and so does installing an update.

Minimise to the tray instead if you want the servers to keep working.

## The ComfyUI tab is a native window

It is not an iframe. That has consequences you can see: nothing of ours can
be drawn on top of that rectangle, which is why settings, the arguments
editor and every confirmation are separate screens rather than dialogs, and
why messages appear as a banner above the tab rather than as a toast over
it.

Downloads and "Save image" inside the tab are handled by WebView2, not by
us — they behave like they would in Edge.

## Long paths

The deepest file in a portable build sits about 206 characters below its
root, against the classic Windows limit of 260. The app unpacks through
verbatim paths and survives further than Explorer does, but tools you run
afterwards — including custom-node installers — may not. Keep destinations
short: `D:\AI\comfy-sdxl`, not a folder five levels inside your profile.

## Model size checks compare name and size, not content

The duplicate report and the "already in the shared folder" marks compare
file names and sizes. Hashing a 20 GB checkpoint to be certain would take
minutes per file, and the report is meant to be looked at, not waited for.

The duplicate report never touches a file — it only reports. Deleting is a
separate, explicit operation on the build's Models tab.

## Windows only

macOS and Linux are not built and not tested. Portable ComfyUI builds are a
Windows arrangement to begin with; on other systems people use a virtual
environment, and the problem this app solves does not exist in the same
shape.

## No code signature

The installer is unsigned, so SmartScreen warns about it. See
[the SmartScreen page](/guide/smartscreen) — including how to verify the
file instead of trusting it.
