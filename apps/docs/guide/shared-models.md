# Shared models

Every portable build carries its own `ComfyUI\models\`. One checkpoint is
2–20 GB, so three or four builds mean hundreds of gigabytes of the same
files. ComfyUI has a mechanism for exactly this — `extra_model_paths.yaml`
— and the app's job is to generate a correct one and hand it to the build.

## How to lay the folder out

Pick a folder outside every build, on a drive with room:

```
D:\AI\_shared_models\
├─ checkpoints\
├─ loras\
├─ vae\
├─ controlnet\
├─ upscale_models\
├─ embeddings\
├─ text_encoders\        (or clip\)
└─ diffusion_models\     (or unet\)
```

Then **Settings → Shared models**, pick the folder, and the app scans it.

**Sections are built from the subfolders that actually exist**, not from a
hard-coded list of categories. That is on purpose: ComfyUI adds model types
between versions, and a list frozen in our code would go stale. Add a
subfolder, rescan, and it appears in the config.

Two legacy names are mapped for you: `unet\` is offered under
`diffusion_models` and `clip\` under `text_encoders`, which is what ComfyUI
itself does internally. Both the legacy and the modern folder can exist,
and both end up in the same key.

Subfolders the app does not recognise are listed separately rather than
silently dropped — you will see what was left out.

`custom_nodes` is excluded unconditionally, even though the key is valid
and appears in ComfyUI's own example file. Sharing custom nodes undoes the
reason your builds are separate: they conflict with each other, and one
shared folder would break all of them at once.

## Your local models are not replaced

The paths are **additive**. ComfyUI appends the extra paths to the ones a
build already has, so models inside the build keep working exactly as
before. Nothing is moved anywhere by connecting a folder.

The **"Download new models here"** switch controls one thing: whether the
shared folder goes first in the list, which is where the ComfyUI Manager
puts newly downloaded files. It does not affect what is found.

## Connecting a build

On the build's **Models** tab there is a switch. Turn it on, and the build
uses the shared folder from the next start on. Arguments of a running
process cannot be changed, so the app says plainly that a restart is needed
rather than pretending the switch took effect.

## Two ways to apply it

**Flag mode — the default.** The generated YAML lives in the app's own data
folder, and `--extra-model-paths-config <path>` is added to the command
line at launch. Nothing at all is written inside your build. Turning it off
means unticking a box; one config serves every build and is edited in one
place.

**File inside the build.** The app writes
`<build>\ComfyUI\extra_model_paths.yaml`. Choose this if you sometimes
start the build with its `.bat` file, outside the app — the file is picked
up by ComfyUI on its own.

The collision policy for that mode is strict:

- no file there → written;
- our file there (recognised by a marker line in the header) → rewritten;
- **someone else's file there → not touched.** You get a comparison screen,
  the original is backed up as `extra_model_paths.yaml.bak-<timestamp>`,
  and the file is replaced only after you confirm. The alternative offered
  on that same screen is to leave it alone and use flag mode instead.

Disconnecting in this mode removes our file and restores the backup if
there was one.

## If the folder is not there

An external drive that is not plugged in is caught **before** the build
starts, with an offer to start without shared models. Finding out through
"model not found" in the middle of a generation is not an option.

## Moving models into the shared folder

The build's **Models** tab lists what is inside that build and what already
exists in the shared folder under the same name and size.

- **Move to shared folder** copies the file, reads the copy back, compares
  it, and only then removes the original. Until the copy is verified, the
  original stays where it is.
- **Remove duplicates** deletes files from the build that are already in
  the shared folder — after checking, again, that the twin is really there.

Both refuse to run on a build that is running: ComfyUI holds those files
open and resolved its paths at startup.

Nothing is ever deleted without you asking for it, and you are told exactly
what will go before it goes.

## Checking that it worked

The most direct check does not involve the interface at all. With the build
running, open
`http://127.0.0.1:<port>/internal/folder_paths` — it lists every category
with the paths behind it, shared folder included, and with
"download new models here" on, the shared path comes first in its array.

The startup log shows the same thing as it happens: ComfyUI logs
`Adding extra search path checkpoints D:\…` for every path it takes on.
