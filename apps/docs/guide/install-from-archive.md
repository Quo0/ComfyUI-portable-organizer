# Install from an archive

You download `ComfyUI_windows_portable_nvidia.7z` yourself, from the
ComfyUI project. The app unpacks it — into one folder or several at once —
registers the results, and can connect them to your shared folders in the
same pass.

This lives in **Add → Install from an archive**, and it is a permanent
section, not a first-run step: it is also how you roll out a new ComfyUI
version next to the old ones.

## The steps

1. **Archive.** Pick the `.7z`, or reuse one from the history. Before it is
   used, the app checks that the file still exists and that its size and
   modification time are unchanged — archives get deleted and replaced.
2. **Reading.** The header is parsed for the number of entries, the total
   unpacked size and the single root folder inside. On a 2 GB archive with
   56 000 entries this takes about a second, and the screen says so.
3. **Destinations.** One or more. Each gets a path, a name, a description
   and an accent colour. The app checks that the folder is empty or absent,
   that there is enough free space, and that the path is not dangerously
   long.
4. **Shared resources.** Optional (but highly recommended) toggles for the
   [shared models folder](/guide/shared-models) and the
   [workflow library](/guide/workflows).
5. **Unpacking**, then **Done**, with the new builds ready to start.

## Choose a short destination path

The deepest file inside the archive sits about 206 characters below the
root. Windows' classic path limit is 260, so a destination like
`D:\AI\comfy-sdxl` is comfortable and
`C:\Users\you\Documents\AI stuff\ComfyUI builds\experiments\sdxl` is not.

The app warns you when the path is long. It unpacks through verbatim
`\\?\` paths, so it survives further than Explorer does, but the tools you
will use afterwards — including ComfyUI's own installers for custom
nodes — may not.

## Several destinations at once

Unpacking happens **once**. The second and further destinations are copied
from the first one, because decompression is CPU-bound and copying a ready
tree is disk-bound. With two or three destinations this saves minutes.

## Interrupting is safe

Files go into a temporary `<destination>.cpo-partial` folder and are moved
into place only when unpacking finishes. That is not decoration: a
half-unpacked tree already contains `python_embeded\python.exe` and
`ComfyUI\main.py`, so it would pass the app's own validity check and look
like a working build.

Cancel — or a crash — removes the temporary folder. Cleaning up tens of
thousands of files takes a while and is reported as its own phase, so a
cancelled install does not look frozen.

## Free space

Checked before anything starts, from the archive header, with a margin,
multiplied by the number of destinations landing on the same volume.
Running out of space nineteen gigabytes into a twenty-gigabyte unpack is
the worst possible moment to find out.
