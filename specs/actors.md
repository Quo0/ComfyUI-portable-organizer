# Actors

Three roles that differ not in permissions but in what the user already has on
disk and what they are wary of. The app is local, there is no login and no
access control — so "actors" here is about context, not about roles in a
system.

Every story in the specs begins with "As \<actor\>" and refers to one of them.

---

## A1 — A newcomer without ComfyUI

**What they have:** a graphics card, the wish to try, and a portable build
archive downloaded from somewhere. Possibly not even the archive.

**What they know:** they have heard that ComfyUI is more powerful and flexible
than the alternatives, and that "it is all complicated there". About portable
builds, `.bat` files, ports and custom nodes they know nothing.

**What they want:** for it to work, and not to have to understand how it is put
together before the first result.

**What they fear:** breaking the computer, cluttering the system, not
understanding an error message. Separately — that they will not be able to
clean all of it up afterwards.

**What our app means to them:** a way to install ComfyUI without reading
instructions and without opening a console.

**Consequences for the interface:** they must not be shown a choice between
four launch profiles before they have installed anything at all. The term
"instance" means nothing to them at first; it will have to be explained on the
spot.

---

## A2 — The owner of one build

**What they have:** one unpacked ComfyUI folder that they launch by
double-clicking a `.bat`. Models have already accumulated inside it, and
probably a dozen custom nodes.

**What they know:** what `run_nvidia_gpu.bat` and `run_cpu.bat` are for, that
the server opens in a browser, that models go into `models\checkpoints`. They
may have heard about node conflicts already, but have not been burned yet.

**What they want:** order. To launch it from somewhere other than Explorer, to
see the startup logs, not to hunt for the tab among twenty others in the
browser.

**What they fear:** that the app will go poking around in their working folder
and spoil something. This is the sharpest fear of all three actors, and it is
justified.

**What our app means to them:** a shell over something that already works.

**Consequences for the interface:** any action that writes into their folder
has to be explicit and reversible. Registering a folder must change nothing
inside it. They are the first candidate to try a second build and become A3.

---

## A3 — The owner of a zoo of builds

**What they have:** three to five ComfyUI folders: a stable one, an
experimental one, one for a particular model, and another "I do not remember
what for". Hundreds of gigabytes of models, partly duplicated between the
folders.

**What they know:** all of the above plus why builds have to be kept apart.
They can read a traceback in a console, they know about ports, and they may
have fiddled with symlinks by hand to avoid duplicating models.

**What they want:** to stop duplicating models, to switch between builds
quickly, to roll out a new version next to the old one without losing the old
one, to carry workflows between builds.

**What they fear:** losing a working configuration. An update that silently
changes the behaviour of an existing build is worse for them than no update at
all.

**What our app means to them:** the tool they came for. Their pain is exactly
the original reason for the project.

**Consequences for the interface:** they need the details that would only
frighten A1: the exact path, the ComfyUI version, the port number, which
archive the instance was unpacked from. Those have to be shown in a way that
does not get in A1's way — that is, on the instance card, not on the first
screen.

---

## How the actors map onto the scenarios

| Actor | Main end-to-end journeys |
|---|---|
| A1 | `J-01` — a clean machine |
| A2 | `J-02` — there is already one folder |
| A3 | `J-03`, `J-04`, `J-05` — shared models, a new version alongside, carrying workflows over |
| All | `J-06` — uninstalling the app |

The priority when requirements conflict: **A3 defines the functionality, A1
defines the first impression.** If a capability is needed by A3 but frightens
A1, it stays, but it is moved out of A1's way.
