# Workflow library

Workflows live inside each build, under `user\default\workflows\`, and they
do not travel between builds. The library is one folder outside every
build where the graphs you actually use are kept, with tags, notes and
favourites.

Set the folder in **Settings → Workflow library**. The app suggests one
next to your shared models folder, but there is no hard link between them:
the library works without shared models and the other way round.

## The library is a folder, not a database

```
<library>\
├─ _library.json      manifest: tags, notes, favourites
├─ sdxl\
│  └─ base.json
└─ upscale.json
```

The manifest lives **inside the library**, not in the app's data folder, so
the library is self-contained: it survives reinstalling the app and moves
to another machine as one folder.

Editing that folder in Explorer breaks nothing. A file with no manifest
entry is valid and shown as it is; an entry whose file is gone is marked
lost, with an offer to drop the record. The files are the truth; the
manifest only enriches them.

## Filling it

- **Add file** and drag-and-drop of `.json` onto the library screen.
- **Drop a PNG** produced by ComfyUI: the graph travels inside the image,
  in its `workflow` text chunk, and the app extracts it. Going back to a
  good generation usually starts from the picture.
- **Paste as text.** Graphs are shared in chats and forums as text more
  often than as files. The paste form reads the clipboard by itself, parses
  as you type and shows the node count before you save — the only way to
  see that you pasted the right thing without reading two thousand lines.
  The name is asked for right away, and it is validated: it becomes a path.
- **Collect from a build** — from the build's **Workflows** tab.

## Collecting moves, it does not copy

Taking a workflow from a build into the library removes it from the build.
The order is the same as for models, and for the same reason: the copy is
written, read back and compared, and only then is the original removed —
through ComfyUI's own API if the build is running, so it knows its folder
changed.

**There is no overwrite.** A name that is already taken is resolved by
comparing contents, not by asking you to replace something:

- identical → there is nothing to collect, the button is off;
- different → these are two pieces of work, and the one from the build is
  collected under a free name (`base.json` → `base (2).json`) after asking.

Replacing would grind one piece of work under another and leave you with
neither.

Comparison looks at the bytes first and at the parsed JSON second. The
second step is not optional: ComfyUI rewrites the file on every save, so
without it almost every already-collected workflow would be declared
different.

## Adding a workflow to a build

- **Running build** → uploaded through its API with `overwrite=false`. A
  409 from ComfyUI turns into a question — overwrite, or save under another
  name — never a silent replacement.
- **Stopped build** → copied into `user\default\workflows\`. If the profile
  has `--user-directory`, that path is resolved from it rather than
  assumed.

The list inside a running ComfyUI refreshes when its page does.

Bulk operations work in both directions: several workflows into one build,
one workflow into several builds.

## The missing-nodes warning

This is why the feature lives here rather than in Explorer. The app takes
the node types out of the workflow JSON and compares them with what a build
actually has:

- **Running build** — asked directly, right now.
- **Stopped build** — compared against a snapshot taken at its last
  successful start, and labelled as such.
- **Never started while the app was watching** — *unknown*, and shown as
  unknown. A green tick with nothing behind it is worse than no tick.

It is a warning, not a block. Installing the missing nodes is
ComfyUI-Manager's job; the app only tells you what to look for.

## What it deliberately does not do

No graph editing, no canvas previews, no versioning, and no installing of
missing nodes.

Two rejected shortcuts, in case you were about to build them yourself:
sharing the whole `user\` directory through `--user-directory` also shares
frontend settings and extension state, which conflict between builds with
different node sets — the very problem separate builds exist to avoid. A
junction on the `workflows` folder means editing one workflow silently
changes it everywhere, and deleting the folder once deletes it for
everyone.
