# End-to-end journeys

The user's path in full, from launching the app to the goal being reached.
Unlike the stories, which describe a single action, a journey shows the
**order** and the **transitions** — the thing the structure of the screens and
the table of contents of the manual are derived from.

Every step refers to the story where it is written out in detail. Here there is
only the sequence and the decisions the user makes along the way.

---

## J-01 — A clean machine: there is nothing

**Actor:** A1, a newcomer without ComfyUI.
**Precondition:** the app is installed and launched for the first time. The
registry is empty. The user downloaded a portable build archive in advance.
**Goal:** to see a working ComfyUI.

| Step | What happens | Story |
|---|---|---|
| 1 | The app opens on an empty registry and explains the two possibilities: point at a folder they already have, or install ComfyUI | `US-ONB-01` |
| 2 | The user chooses the installation | `US-ONB-02` |
| 3 | Points at the downloaded archive; the app shows what is inside and how much space will be needed | `US-INST-01` |
| 4 | Points at one destination folder and gives the instance a name | `US-INST-02` |
| 5 | Skips setting up shared resources — they have nothing to share yet | `US-INST-04` |
| 6 | Watches the extraction with its progress | `US-INST-05` |
| 7 | The instance appears in the registry | `US-INST-06` |
| 8 | Launches it; sees the startup logs and waits for readiness | `US-RUN-02`, `US-RUN-03` |
| 9 | ComfyUI opens inside the app's window | `US-TAB-02` |

**Forks:** there is no archive at all — the app explains where to get one and
the journey pauses. There is not enough space on the disk — a refusal before
the extraction begins, `US-INST-03`.

**For design:** this is the only journey where the user has never seen the app
before. Every screen has to explain itself without leaning on previous
experience.

---

## J-02 — There is already one ComfyUI folder

**Actor:** A2, the owner of one build.
**Precondition:** an unpacked and working ComfyUI folder is on the disk, the
app is launched for the first time.
**Goal:** to launch the existing build from the app without breaking anything
in it.

| Step | What happens | Story |
|---|---|---|
| 1 | The app opens on an empty registry and offers the fork | `US-ONB-01` |
| 2 | The user chooses "I already have one" | `US-ONB-03` |
| 3 | Points at the folder; the app checks it and shows what it found: the ComfyUI version, the Python version, the launch profiles | `US-REG-01` |
| 4 | Gives the instance a name and a colour, confirms the registration | `US-REG-02` |
| 5 | Launches the familiar profile | `US-RUN-01`, `US-RUN-02` |
| 6 | ComfyUI opens inside the window | `US-TAB-02` |

**The journey's key promise:** after the registration the folder's contents
have not changed. The user can still launch it by double-clicking the `.bat`
past the app, and everything will work as before. This is checked directly in
`US-REG-01/AC-7`.

**Forks:** the folder does not pass the check — the app says exactly which file
is missing, `US-REG-01`. The port from the familiar profile is taken — the app
hands out another and says so, `US-RUN-04`.

---

## J-03 — Stop duplicating models

**Actor:** A3, the owner of the zoo.
**Precondition:** there are three instances in the registry, each with its own
models, some of them duplicated.
**Goal:** to keep the models in one place and hand them out to every instance.

| Step | What happens | Story |
|---|---|---|
| 1 | The user creates a shared models root by pointing at a folder | `US-SHARED-01` |
| 2 | The app shows which categories it recognised in it and how many files are there | `US-SHARED-02` |
| 3 | Decides whether new downloads should go into the shared folder | `US-SHARED-04` |
| 4 | Connects the first instance | `US-SHARED-03` |
| 5 | Launches it and makes sure the shared models are visible and the local ones have not disappeared | `US-SHARED-03` |
| 6 | Connects the remaining instances | `US-SHARED-03` |
| 7 | Later looks at the duplicate report and sorts out the excess by hand | `US-SHARED-09` |

**Forks:** the instance already holds a path-configuration file of its own —
the app does not touch it silently but shows a choice, `US-SHARED-06`. The
shared root is on a removable drive and is unavailable at launch — a warning
before the launch, `US-SHARED-08`. The user changed their mind —
disconnecting returns the instance to its original state, `US-SHARED-07`.

**For documentation:** this is the most valuable journey for A3 and the most
likely source of questions. It deserves a manual page of its own.

---

## J-04 — A new version alongside the old one

**Actor:** A3.
**Precondition:** there is a working instance, a new version of ComfyUI is out,
the archive is downloaded.
**Goal:** to try the new version without losing anything.

| Step | What happens | Story |
|---|---|---|
| 1 | The user opens the installation and picks the new archive instead of the remembered one | `US-INST-07` |
| 2 | Points at a new destination folder and a name that reflects the version | `US-INST-02` |
| 3 | Connects the new instance to the shared models and the library right away | `US-INST-04` |
| 4 | Waits out the extraction | `US-INST-05` |
| 5 | Sees both instances in the registry, and for each of them which archive it was unpacked from | `US-REG-05` |
| 6 | Launches the new one; the old one stays untouched | `US-RUN-02` |

**The key promise:** the app never updates an instance in place. The new one is
unpacked alongside, the old one does not change.

**Forks:** the user wants to unpack into two folders in a single run —
`US-INST-08`. A previously used archive has been deleted from the disk — the
app says so on entering the installation, `US-INST-07`.

---

## J-05 — Carrying a workflow between instances

**Actor:** A3.
**Precondition:** one instance has a debugged workflow, it is needed in
another.
**Goal:** to carry it over while understanding whether it will work in the new
place.

| Step | What happens | Story |
|---|---|---|
| 1 | The user creates the library by pointing at a folder | `US-WF-01` |
| 2 | Takes the workflow from the instance into the library | `US-WF-03` |
| 3 | Marks it a favourite, adds tags and a note | `US-WF-02` |
| 4 | Picks the target instance and sees whether all the required nodes are in it | `US-WF-05` |
| 5 | Adds the workflow to the target instance | `US-WF-04` |
| 6 | Opens the instance and works with the workflow | `US-TAB-02` |

**Forks:** the target instance is missing nodes — a warning with the list, but
it can still be added, `US-WF-05`. The instance already has a workflow with
that name — a choice between overwriting and another name, `US-WF-04`. The
library folder is unavailable — the app says so and keeps working, `US-WF-06`.

---

## J-06 — Uninstalling the app

**Actor:** any.
**Precondition:** the app has been used, there are instances, shared models and
a library.
**Goal:** to remove the app without losing the models and the workflows.

| Step | What happens | Story |
|---|---|---|
| 1 | The user looks in advance at where everything is stored and what will disappear on removal | `US-DATA-01` |
| 2 | Uninstalls the app through the standard Windows facility | `US-DATA-02` |
| 3 | Confirms the deletion of the app's data | `US-DATA-02` |
| 4 | Makes sure the models, the library and the instance folders themselves are in place | `US-DATA-03` |

**The key promise:** user content is never deleted. It is separately stipulated
that the files the app wrote inside the instance folders at the user's request
also stay — they lie in someone else's installation, and the app has no right
to take them away, `US-DATA-03`.

**Forks:** the user clears the "delete data" checkbox — the settings and the
registry are kept, and a reinstall picks them up, `US-DATA-02`.

---

## Actor coverage

| Journey | A1 | A2 | A3 |
|---|:---:|:---:|:---:|
| `J-01` A clean machine | ● | | |
| `J-02` There is already a folder | | ● | ● |
| `J-03` Shared models | | ○ | ● |
| `J-04` A new version alongside | | | ● |
| `J-05` Carrying a workflow over | | ○ | ● |
| `J-06` Uninstalling | ● | ● | ● |

● primary, ○ possible.
