# EP-SHARED — Shared model storage

The most valuable capability for actor A3 and the main reason the project was
started at all. A single checkpoint weighs from two to twenty gigabytes; across
three to five instances the duplicates eat hundreds.

Four promises of this section, verified in the criteria below:

- **Shared models are added, they do not replace the local ones.** Connecting
  takes nothing away.
- **Model files disappear from a build only when someone asked for it.** Moving
  into the shared folder and cleaning up a confirmed duplicate are the only two
  cases, and both are started by the user, who sees the list in advance.
- **The app does not change someone else's settings silently.** If the instance
  already holds a path-configuration file of its own, the user learns about it
  before anything happens.
- **Disconnecting puts everything back as it was.** Changing one's mind is
  possible at any moment.

## Functional requirements

| ID | Requirement |
|---|---|
| `FR-SHARED-010` | The user sets the shared models root folder |
| `FR-SHARED-020` | The app recognises the model categories from the contents of the shared folder |
| `FR-SHARED-030` | An instance connects to the shared root and disconnects from it |
| `FR-SHARED-040` | Shared paths are added to the local ones rather than replacing them |
| `FR-SHARED-050` | The user decides where newly downloaded models end up |
| `FR-SHARED-060` | Two ways of applying are supported: without changing the instance's folder, and by writing a file into it |
| `FR-SHARED-070` | An existing path-configuration file in an instance is not overwritten without explicit consent |
| `FR-SHARED-080` | The custom nodes directory never enters the shared paths |
| `FR-SHARED-090` | Unavailability of the shared root is detected before the instance is launched |
| `FR-SHARED-100` | Changes take effect from the next launch, and the user is told so |
| `FR-SHARED-110` | The app shows a report on duplicated models without performing any actions on files |
| `FR-SHARED-120` | Several shared roots are supported |
| `FR-SHARED-130` | Models are moved from a build into the shared folder by the app's own means |
| `FR-SHARED-140` | A local copy is deleted only as a confirmed duplicate of something already lying in the shared folder |

---

### US-SHARED-01 — Setting the shared models root

**As** A3
**I want** to point at the folder where I will keep the models for every build
**so that** I stop keeping copies in each of them.

Tags: `@FR-SHARED-010` `@FR-SHARED-120` `@phase-2.5` `@area-shared`

**Preconditions**
- The app is running.

**Acceptance criteria**
- **AC-1.** The user picks the shared root folder through the system dialog.
- **AC-2.** It is explained how that folder should be arranged: subfolders by model category.
- **AC-3.** The shared root is set in the settings and is available for changing at any moment.
- **AC-4.** The same choice is available right in the install wizard, without forcing a trip to the settings.
- **AC-5.** The setting is kept between launches of the app.
- **AC-6.** The shared root can be set before any models appear in it.

**Negative and edge cases**
- **AC-7.** A folder that does not exist is not accepted.
- **AC-8.** A folder inside an instance is accepted as a shared root, but the user is warned about the non-obvious consequences.
- **AC-9.** Changing the shared root applies to every connected instance, and the user is warned about this in advance.

---

### US-SHARED-02 — An overview of the shared root's contents

**As** a user who has set the shared root
**I want** to see what the app found in it
**so that** I can be sure the folder is arranged correctly.

Tags: `@FR-SHARED-020` `@FR-SHARED-080` `@phase-2.5` `@area-shared`

**Preconditions**
- The shared root is set.

**Acceptance criteria**
- **AC-1.** The recognised model categories are listed.
- **AC-2.** For each category it is visible whether it holds any files.
- **AC-3.** Subfolders matching no known category are shown separately as unrecognised.
- **AC-4.** The custom nodes directory, if it is in the shared folder, never enters the shared paths, and it is explained why: the builds are kept apart precisely because of conflicts between nodes.
- **AC-5.** The app offers to create the missing standard subfolders.
- **AC-6.** The set of recognisable categories does not go stale when ComfyUI is updated: recognition rests on the folder's contents rather than on a baked-in list.

**Negative and edge cases**
- **AC-7.** An empty shared root does not count as an error: the user is told there are no models yet.
- **AC-8.** Counting the size of the contents does not block the interface.

---

### US-SHARED-03 — Connecting an instance to the shared models

**As** A3
**I want** to give an instance access to the shared models
**so that** I do not have to copy them into its folder.

Tags: `@FR-SHARED-030` `@FR-SHARED-040` `@FR-SHARED-100` `@phase-2.5` `@area-shared`

**Preconditions**
- The shared root is set.
- The instance is registered.

**Acceptance criteria**
- **AC-1.** Connecting is done in a single action on the instance's screen.
- **AC-2.** If the shared root is not set, setting it first is offered instead of connecting.
- **AC-3.** After connecting and launching the instance, the models from the shared root are available in the model pickers inside ComfyUI.
- **AC-4.** The instance's local models stay available: the shared paths are added to its own rather than replacing them.
- **AC-5.** The connection state is visible in the list of instances.
- **AC-6.** Connecting can be done for several instances independently.

**Negative and edge cases**
- **AC-7.** Connecting or disconnecting a running instance does not take effect until it is restarted, and this is said explicitly — otherwise the user will conclude the feature is broken.
- **AC-8.** If a model with the same name exists both locally and in the shared folder, both are visible, and which one is used by default is determined by the setting from `US-SHARED-04`.

---

### US-SHARED-04 — Choosing where new downloads go

**As** A3
**I want** downloaded models to land in the shared folder straight away
**so that** they become available to every build without being moved by hand.

Tags: `@FR-SHARED-050` `@phase-2.5` `@area-shared`

**Preconditions**
- The shared root is set.

**Acceptance criteria**
- **AC-1.** The user chooses where new models go: into the shared folder or into the instance's folder.
- **AC-2.** The shared folder is chosen by default — downloaded once, available to all.
- **AC-3.** The consequences of the choice are explained on the spot rather than hidden in the documentation.
- **AC-4.** When the shared folder is chosen, models downloaded by ComfyUI's own means end up in it.
- **AC-5.** The setting applies to every connected instance alike.

**Negative and edge cases**
- **AC-6.** A change to the setting takes effect from the instance's next launch, and this is said.

---

### US-SHARED-05 — Choosing the way of applying

**As** a user who sometimes launches a build past the app
**I want** the shared models to work in that case too
**so that** I do not lose them when launching from Explorer.

Tags: `@FR-SHARED-060` `@phase-2.5` `@area-shared`

**Preconditions**
- The shared root is set, the instance is connected.

**Acceptance criteria**
- **AC-1.** Two ways of applying are available: without changing the instance's folder, and by writing a settings file inside it.
- **AC-2.** The way that does not change the instance's folder is chosen by default.
- **AC-3.** The difference is explained in understandable terms: the second way works when the build is launched past the app as well.
- **AC-4.** The current way is visible on the instance's screen and can be changed.
- **AC-5.** With the way that does not change the folder, not a single new file appears in the instance's folder.
- **AC-6.** With the way that writes a file, the shared models are available when the build is launched by its own means too.

**Negative and edge cases**
- **AC-7.** Changing the way of applying does not lose the shared root settings.

---

### US-SHARED-06 — An existing path-configuration file in the instance

**As** A2, who has already configured the model paths by hand
**I want** the app not to wipe out my configuration
**so that** I do not lose a working setup.

Tags: `@FR-SHARED-070` `@phase-2.5` `@area-shared`

**Preconditions**
- The way of applying that writes a file into the instance's folder is chosen.
- The instance's folder already holds a model-path configuration file of its
  own.

**Acceptance criteria**
- **AC-1.** The existing file is not overwritten automatically.
- **AC-2.** The user is shown what kind of file is already lying there.
- **AC-3.** A choice is offered: overwrite it while keeping a copy of the previous one, or leave the file and apply the shared models the other way.
- **AC-4.** On overwriting, a copy of the original file is kept, and where it is is stated.
- **AC-5.** A file created earlier by the app itself is recognised as ours and updated without extra questions.

**Negative and edge cases**
- **AC-6.** Declining both options leaves the instance in its previous state.
- **AC-7.** If the file cannot be read, the app does not touch it and says so.

---

### US-SHARED-07 — Disconnecting from the shared models

**As** a user who changed their mind
**I want** to return the instance to its original state
**so that** it works as it did before connecting.

Tags: `@FR-SHARED-030` `@FR-SHARED-070` `@phase-2.5` `@area-shared`

**Preconditions**
- The instance is connected to the shared root.

**Acceptance criteria**
- **AC-1.** Disconnecting is done in a single action.
- **AC-2.** After disconnecting and restarting, the instance sees only its local models.
- **AC-3.** With the way that writes a file, disconnecting removes the file the app created.
- **AC-4.** If a copy of a previous file was kept at connection time, it is restored.
- **AC-5.** The models in the shared folder are not touched on disconnecting.

**Negative and edge cases**
- **AC-6.** Disconnecting a running instance takes effect after a restart, and this is said.

---

### US-SHARED-08 — An unavailable shared root

**As** a user keeping models on an external drive
**I want** to learn that the folder is unavailable in advance
**so that** I do not find out in the middle of my work through a "model not
found" error.

Tags: `@FR-SHARED-090` `@phase-2.5` `@area-shared`

**Preconditions**
- The instance is connected to the shared root, and the folder is currently
  unavailable.

**Acceptance criteria**
- **AC-1.** The unavailability is detected before the instance is launched.
- **AC-2.** The user is warned and can launch the instance without the shared models or cancel the launch.
- **AC-3.** The app neither crashes nor hangs because of an unavailable folder.
- **AC-4.** The unavailability of the shared root is visible in the settings too.

**Negative and edge cases**
- **AC-5.** If the folder became unavailable after the launch, the app does not try to fix it, and the user can work out the reason from the instance's log.

---

### US-SHARED-09 — The duplicate models report

**As** A3
**I want** to see how much space the duplicates eat
**so that** I can decide what to move into the shared folder.

Tags: `@FR-SHARED-110` `@phase-4` `@area-shared`

**Preconditions**
- There is more than one instance in the registry.

**Acceptance criteria**
- **AC-1.** The app shows the models occurring in more than one place.
- **AC-2.** For each of them it states which instances it lies in and how much space the copies take in total.
- **AC-3.** The overall size lost to duplicates is visible.
- **AC-4.** The report performs no actions on files: it does not delete, does not move, does not create links.
- **AC-5.** Models with the same name but a different size are marked separately — a matching name does not mean matching contents.
- **AC-6.** Building the report does not block the interface and can be interrupted.

**Negative and edge cases**
- **AC-7.** Unavailable instance folders are skipped, and the report says so.

---

### US-SHARED-10 — Moving a build's models into the shared folder

**As** A3, whose models are already laid out inside a build
**I want** to move them into the shared folder in one action
**so that** I do not have to shift tens of gigabytes through Explorer.

Tags: `@FR-SHARED-130` `@FR-SHARED-140` `@phase-2.5` `@area-shared`

**Preconditions**
- The shared root is set.
- The build is registered and stopped.

**Acceptance criteria**
- **AC-1.** Before the move it is visible what exactly will be moved: the categories, the number of entries and the size.
- **AC-2.** Every model category of the build is moved, including those created by custom nodes and those unfamiliar to the app.
- **AC-3.** A model lying in the build as a directory rather than a file is moved whole.
- **AC-4.** Files shipped together with the build are not moved: the empty "put things here" marker files and the directory of model configurations.
- **AC-5.** The custom nodes directory is never moved.
- **AC-6.** The progress is visible, and the operation can be interrupted.
- **AC-7.** After the move a connected build sees these models — from its next launch, and this is said.

**Negative and edge cases**
- **AC-8.** The move is unavailable for a running build, and the reason is stated.
- **AC-9.** A shortage of space on the target disk is detected before the move begins, not in the middle of it.
- **AC-10.** An entry whose name is already taken in the shared folder is not moved and does not overwrite someone else's; it is listed in the report.
- **AC-11.** An interruption leaves neither damaged files in the shared folder nor losses in the build: what has been moved is moved, the rest is in place.
- **AC-12.** A failure on one entry does not cancel the rest, and the report shows what failed and why.

---

### US-SHARED-11 — Cleaning up duplicates after the move

**As** A3
**I want** to remove from the build what already lies in the shared folder
**so that** I free up space, given that these models are taken from the shared
one anyway.

Tags: `@FR-SHARED-140` `@phase-2.5` `@area-shared`

**Preconditions**
- The build is connected to the shared models and stopped.
- After the move, entries whose name is taken in the shared folder are left.

**Acceptance criteria**
- **AC-1.** What has been recognised as a duplicate is listed, with the size that will be freed.
- **AC-2.** A duplicate means more than a matching name: the contents are compared too, as far as that is possible without reading the whole file.
- **AC-3.** Entries with a matching name but non-matching contents are shown as a **separate group**, and deleting them is not offered.
- **AC-4.** The deletion begins only with an explicit confirmation of the list; it is no part of the move itself in any form.
- **AC-5.** After the cleanup the models stay available to the build from the shared folder.

**Negative and edge cases**
- **AC-6.** For a build not connected to the shared models the cleanup is not offered: the deletion would leave it with no models at all.
- **AC-7.** For a running build the cleanup is unavailable.
- **AC-8.** Nothing from the "different files with one name" group is deleted under any conditions.
- **AC-9.** A failure deleting one entry does not cancel the rest and is explained in the report.
