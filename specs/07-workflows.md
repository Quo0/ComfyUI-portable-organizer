# EP-WF — The workflow library

Workflows live inside each build and do not travel between them. "That one
graph that works" has to be hunted for across folders by hand. The library is a
shared place for favourite workflows, from which they can be added to any
instance.

The real value lies in our app rather than in an ordinary folder: before adding
one, it is visible whether all the required nodes are in the target instance.

The robustness principle: **the library is a folder with files.** The
information about tags and favourites merely enriches it; it does not define
it. The user can copy, rename and delete files through Explorer without
breaking anything.

## Functional requirements

| ID | Requirement | Rationale in `PLAN.md` |
|---|---|---|
| `FR-WF-010` | The user sets the workflow library folder | «Библиотека воркфлоу» |
| `FR-WF-020` | Workflows are marked with favourites, tags and notes | «Хранилище» |
| `FR-WF-030` | The information about tags and notes is stored in the library itself and survives a reinstall of the app | «Хранилище» |
| `FR-WF-040` | Workflows are added to the library from a file, from text in the clipboard, and from an existing instance | «Наполнение библиотеки» |
| `FR-WF-050` | A workflow is added to an instance regardless of whether it is running | «Добавление в инстанс» |
| `FR-WF-060` | A name conflict on adding is resolved by the user, not silently | «Добавление в инстанс» |
| `FR-WF-070` | The app shows which nodes are missing in the target instance | «Проверка совместимости» |
| `FR-WF-080` | Missing nodes warn but do not block the addition | «Проверка совместимости» |
| `FR-WF-090` | The library works when the folder is changed by hand through Explorer | «Хранилище» |
| `FR-WF-100` | Operations over several workflows and several instances at once are supported | «Добавление в инстанс» |
| `FR-WF-110` | Unavailability of the library folder does not put the app out of action | «Грабли» |

---

### US-WF-01 — Setting the library folder

**As** A3
**I want** to say where to keep my favourite workflows
**so that** they do not get lost inside individual builds.

Tags: `@FR-WF-010` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Библиотека воркфлоу»

**Preconditions**
- The app is running.

**Acceptance criteria**
- **AC-1.** The user picks the library folder through the system dialog.
- **AC-2.** The library is configured independently of the shared models and works even if those are not set.
- **AC-3.** If the shared models root is set, a sensible location for the library next to it is offered.
- **AC-4.** The setting is kept between launches of the app.
- **AC-5.** The library folder can be changed, and the contents of the previous one are not deleted in the process.

**Negative and edge cases**
- **AC-6.** An empty folder is accepted as a library without errors.
- **AC-7.** A folder containing unrelated files is accepted; the unrelated files are not shown as workflows.

---

### US-WF-02 — Organising the library

**As** a user with fifty workflows
**I want** to mark and find the ones I need
**so that** I do not have to go through the list by eye.

Tags: `@FR-WF-020` `@FR-WF-030` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Хранилище»

**Preconditions**
- The library is set and holds workflows.

**Acceptance criteria**
- **AC-1.** A workflow can be marked as a favourite.
- **AC-2.** A workflow can be given tags and a note.
- **AC-3.** Searching by name and by tag is available.
- **AC-4.** Filtering down to favourites only is available.
- **AC-5.** The marks, tags and notes are kept together with the library rather than in the app's data.
- **AC-6.** After the app is uninstalled and installed again, the marks, tags and notes are in place.
- **AC-7.** Moving the library folder to another computer keeps the marks, tags and notes.

---

### US-WF-03 — Filling the library

**As** a user
**I want** to put a workflow into the library from a file or from a build
**so that** it becomes available to every instance.

Tags: `@FR-WF-040` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Наполнение библиотеки»

**Preconditions**
- The library is set.

**Acceptance criteria**
- **AC-1.** A workflow is added by picking a file.
- **AC-2.** A workflow is added by dragging a file into the app.
- **AC-2a.** A workflow is added by pasting text: the user pastes JSON and sets a name, without saving what was sent into a file merely to pick it in a dialog.
- **AC-2b.** Before saving it is visible that what was pasted really is a workflow: the number of nodes is shown. Not a workflow means a refusal on the spot, and no file is created.
- **AC-2c.** A name from the input field cannot take the record outside the library folder and cannot take a name already taken inside it.
- **AC-3.** From a registered instance one can view the list of its workflows and take the needed ones into the library.
- **AC-4.** Taking from an instance works both when the instance is running and when it is stopped.
- **AC-5.** The original workflow in the instance stays in place when it is taken.
- **AC-6.** For a taken workflow, which instance it came from is kept.

**Negative and edge cases**
- **AC-7.** A file that is not a workflow is rejected with an explanation.
- **AC-8.** When the name matches one already in the library, the user chooses between replacing it and another name.

---

### US-WF-04 — Adding a workflow to an instance

**As** a user
**I want** to put a workflow from the library into the build I need
**so that** I can open it there.

Tags: `@FR-WF-050` `@FR-WF-060` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Добавление в инстанс»

**Preconditions**
- There is a workflow in the library and an instance in the registry.

**Acceptance criteria**
- **AC-1.** The user picks the workflow and the target instance.
- **AC-2.** Adding works when the instance is running.
- **AC-3.** Adding works when the instance is stopped.
- **AC-4.** The workflow lands exactly where this build keeps its workflows, even if its settings are non-standard.
- **AC-5.** After being added to a running instance the workflow appears in its list — the user is told that the list refreshes after the ComfyUI page is reloaded.
- **AC-6.** The workflow stays in the library after being added.

**Negative and edge cases**
- **AC-7.** If the instance already has a workflow with that name, the user chooses between overwriting and saving under another name; no silent overwrite happens.
- **AC-8.** An unavailable instance is not offered as a target.
- **AC-9.** A write error is explained, and the library is not changed in the process.

---

### US-WF-05 — Checking compatibility with an instance

**As** A3
**I want** to know in advance whether the workflow will open in the chosen
build
**so that** I do not find out through errors on the canvas.

Tags: `@FR-WF-070` `@FR-WF-080` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Проверка совместимости»

**Preconditions**
- There are workflows in the library and instances in the registry.

**Acceptance criteria**
- **AC-1.** Before adding, it is visible whether all the nodes the workflow requires are available in the target instance.
- **AC-2.** When some are missing, the specific absent nodes and their number are listed.
- **AC-3.** The compatibility is visible for several instances at once, so a suitable one can be chosen.
- **AC-4.** The warning does not block the addition: the user is entitled to add the workflow and install the nodes later.
- **AC-5.** For a running instance the answer corresponds to its current state.
- **AC-6.** For a stopped instance the answer is given from the last known data, and it is marked as such.

**Negative and edge cases**
- **AC-7.** If there is no data about the instance yet, the compatibility is marked as unknown rather than as "all good".
- **AC-8.** The app does not try to install the missing nodes — it only reports them.

---

### US-WF-06 — The library's robustness to changes made by hand

**As** a user used to working with files through Explorer
**I want** to copy and delete workflows directly
**so that** I do not depend on the app.

Tags: `@FR-WF-090` `@FR-WF-110` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Хранилище»

**Preconditions**
- The library is set and holds workflows.

**Acceptance criteria**
- **AC-1.** A file put into the library folder past the app appears in the list.
- **AC-2.** Such a file has no tags and no notes, and that does not count as an error.
- **AC-3.** If a file was deleted past the app, the corresponding record is marked lost rather than vanishing silently.
- **AC-4.** A lost record can be removed in a single action.
- **AC-5.** Nested folders inside the library are supported and reflected in the list.

**Negative and edge cases**
- **AC-6.** If the library folder became unavailable, the app says so and keeps working; the other sections are unaffected.
- **AC-7.** Damaged tag information does not lead to losing the workflows themselves: the files stay available.

---

### US-WF-07 — Operations over several workflows

**As** a user unpacking a new build
**I want** to carry a whole set of workflows into it
**so that** I do not repeat one action twenty times.

Tags: `@FR-WF-100` `@phase-2.6` `@area-wf`
Rationale: `PLAN.md` → «Добавление в инстанс»

**Preconditions**
- There are several workflows in the library and several instances in the
  registry.

**Acceptance criteria**
- **AC-1.** Several workflows can be chosen and added to one instance in a single operation.
- **AC-2.** One workflow can be added to several instances at once.
- **AC-3.** The progress is visible: how many have been processed out of the total.
- **AC-4.** On completion it is shown what succeeded and what did not, with the reasons.

**Negative and edge cases**
- **AC-5.** A name conflict on one of the workflows does not cancel the rest.
- **AC-6.** The operation can be interrupted; the workflows already added stay in place.
