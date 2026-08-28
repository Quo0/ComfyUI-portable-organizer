# Glossary

The language of the domain. The terms from here are used in the stories, in
the interface and in the documentation without synonyms: if it says "instance"
here, then the interface cannot say "build", "installation" or "copy".

## About ComfyUI

**ComfyUI** — an application for generating images with an interface in the
form of a graph of nodes. It runs as a local web server and opens in a browser.

**Portable build** — the way ComfyUI is distributed on Windows: an archive
holding both ComfyUI itself and an embedded Python with all its dependencies.
Nothing is installed into the system, everything lives in one folder.

**Instance** — one portable ComfyUI folder registered in the app. Users keep
several instances because custom nodes conflict with each other, and updating
one build breaks another.

**Custom node** — an extension of ComfyUI that adds new node types to the
graph. The main reason instances have to be kept apart: two nodes may require
incompatible versions of the same library.

**ComfyUI-Manager** — a popular custom node for installing other nodes. It
matters because it can restart the server by itself, without our app knowing.

**Launch profile** — a named set of "interpreter plus command-line arguments"
that an instance starts with. In a portable build the profiles correspond to
the `.bat` files in its root: an ordinary launch on the graphics card, a launch
with acceleration, a launch on the CPU, and the variants from the `advanced`
subfolder.

**Workflow** — a graph of nodes saved into a JSON file. The unit of the user's
work: "that one workflow that works" gets carried between instances and kept
for years.

**Model** — a file of neural network weights: a checkpoint, a LoRA, a VAE, a
ControlNet and so on. It weighs from hundreds of megabytes to twenty gigabytes.

**Model category** — the type of a model, which determines the subfolder
ComfyUI looks for it in: `checkpoints`, `loras`, `vae`, `controlnet` and so
forth. The set of categories changes from one ComfyUI version to the next.

## About our app

**Registry** — the list of instances known to the app. Registering neither
copies nor moves the folder: the app only remembers the path and the metadata.

**Install wizard** — the scenario for unpacking a new instance from a portable
build archive, into one or several folders at once.

**Source archive** — the `.7z` file with a portable build that the user
downloaded themselves. The app remembers a history of such archives so that
different versions can be rolled out at different times.

**Shared models root** — a folder outside the instances holding models that
are available to every connected instance at once. Inside it are subfolders by
category.

**Connecting to shared models** — the action after which an instance sees the
models from the shared root in addition to its own. The local models do not
disappear in the process.

**Workflow library** — a folder outside the instances where the user keeps
their favourite workflows, so as to add them to any instance.

**Library manifest** — a file inside the library holding tags, notes and
"favourite" marks. It lies next to the workflows rather than in the app's data,
so that the library stays self-sufficient across a reinstall of the app and a
move to another machine.

**Lost record** — a record in the manifest that no longer has a workflow file.
Not an error: the user may have deleted the file through Explorer.

**Workflow compatibility** — the correspondence between the node types a
workflow requires and the node types available in a particular instance. A
mismatch is grounds for a warning, but not grounds for forbidding the addition.

## About the interface

**Shell** — the parts of the interface belonging to our app rather than to
ComfyUI: the navigation rail, the toolbars, the settings screens. Everything
that is not ComfyUI itself inside the embedded tab.

**Rail** — the vertical menu on the left, always visible. It holds the app's
sections and the list of running instances.

**Content area** — the whole window minus the rail. This is where the app's
screens are shown, and for a running instance, ComfyUI itself.

**Embedded tab** — the ComfyUI interface shown inside our app's window instead
of a browser.

**Instance state** — one of: stopped, starting, running, stopping, crashed.

**Readiness** — the moment when the ComfyUI server has answered a request and
its interface can be shown. Several minutes may pass between starting the
process and readiness.

## About data

**App data** — the settings, the instance registry, the paths to shared
resources. They belong to the app and are deleted along with it.

**User content** — the models and the workflow library. They lie in folders the
user chose, and they are never deleted.
