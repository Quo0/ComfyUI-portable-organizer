# EP-REG — The instance registry

The app knows about instances but does not own them. Registering remembers the
path and the metadata; the folder on disk stays untouched. That is this
section's main promise and actor A2's main fear.

## Functional requirements

| ID | Requirement | Rationale in `PLAN.md` |
|---|---|---|
| `FR-REG-010` | The app checks that the chosen folder is a portable ComfyUI build, and on refusal names the reason | «discovery/windows_portable.rs» |
| `FR-REG-020` | Registering does not change the contents of the instance's folder | «Дисциплина хранения данных» |
| `FR-REG-030` | The app determines the ComfyUI version, the Python version and the available launch profiles | «discovery/windows_portable.rs» |
| `FR-REG-040` | An instance is given a name, a description, an accent colour and a preferred port | «discovery/windows_portable.rs» |
| `FR-REG-050` | An instance's metadata can be changed after registration | «Экраны» |
| `FR-REG-060` | Removing from the registry does not delete the folder from the disk | «Чеклист, Фаза 1» |
| `FR-REG-070` | For instances unpacked by the wizard, the source archive is kept and shown | «Мастер установки» |
| `FR-REG-080` | An instance's size on disk is computed without blocking the interface | «Грабли» |
| `FR-REG-090` | Registering the same folder again does not create a duplicate | «discovery/windows_portable.rs» |

---

### US-REG-01 — Registering an existing folder

**As** A2, the owner of a working build
**I want** to point the app at my ComfyUI folder
**so that** I can manage it from here without breaking anything in it.

Tags: `@FR-REG-010` `@FR-REG-020` `@FR-REG-030` `@phase-1` `@area-reg`
Rationale: `PLAN.md` → «discovery/windows_portable.rs»

**Preconditions**
- An unpacked portable ComfyUI build is on the disk.

**Acceptance criteria**
- **AC-1.** The user picks the folder through the system folder picker.
- **AC-2.** The folder is accepted if it holds the embedded Python interpreter and ComfyUI's main file.
- **AC-3.** For an accepted folder the following are shown: the ComfyUI version, the Python version and the list of launch profiles found.
- **AC-4.** The list of profiles includes both the main launch variants and the variants from the nested folder of additional scenarios.
- **AC-5.** Profiles that relate to updating the build do not make it into the list — they do not start the server.
- **AC-6.** The user sees the recognised profiles before confirming the registration.
- **AC-7.** After the registration the folder's contents have not changed: the build still launches by its own means past the app.

**Negative and edge cases**
- **AC-8.** A folder without the embedded interpreter or without ComfyUI's main file is rejected, and the message names exactly what is missing.
- **AC-9.** When a folder one level above or below the right one is picked, the message hints at which folder to pick.
- **AC-10.** If the profiles could not be recognised, the instance is still registered, and the user is warned that the profiles have to be set by hand.
- **AC-11.** Picking an already registered folder again does not create a second instance but shows the existing one.
- **AC-12.** A path containing spaces and non-Latin characters is handled the same as any other.

---

### US-REG-02 — An instance's metadata at registration

**As** the owner of several builds
**I want** to give the instance a clear name, description and colour
**so that** I can tell them apart at a glance.

Tags: `@FR-REG-040` `@phase-1` `@area-reg`
Rationale: `PLAN.md` → «discovery/windows_portable.rs»

**Preconditions**
- The folder has passed the check and is ready for registration.

**Acceptance criteria**
- **AC-1.** The user sets the instance's name; a sensible default value is offered.
- **AC-2.** The user can set a description — free text about the build's purpose.
- **AC-3.** The user picks an accent colour that will set the instance apart in the lists.
- **AC-4.** The user can set a preferred port; a default value is offered.
- **AC-5.** The chosen accent colour stays distinguishable in both the light and the dark theme.

**Negative and edge cases**
- **AC-6.** An empty name is not accepted.
- **AC-7.** A name matching an existing one is allowed, but the user is warned.
- **AC-8.** A port value outside the permitted range is not accepted, with the range explained.

---

### US-REG-03 — Viewing the list of instances

**As** the owner of several builds
**I want** to see every instance and its state
**so that** I can pick the one I need.

Tags: `@FR-REG-030` `@FR-REG-070` `@FR-REG-080` `@phase-1` `@area-reg`
Rationale: `PLAN.md` → «Экраны»

**Preconditions**
- There is at least one instance in the registry.

**Acceptance criteria**
- **AC-1.** For every instance the name, the description, the accent colour and the current state are visible.
- **AC-2.** The state distinguishes at least: stopped, starting, running, crashed.
- **AC-3.** Opening an instance shows the path, the ComfyUI version, the Python version and the size on disk.
- **AC-4.** For instances unpacked by the wizard, the source archive and the installation date are shown.
- **AC-5.** The size on disk appears as it is computed and does not delay showing the rest of the details.
- **AC-6.** A previously computed size is shown immediately, with a note of when it was measured.

**Negative and edge cases**
- **AC-7.** If an instance's folder has disappeared from the disk, the instance is marked unavailable rather than vanishing from the list.
- **AC-8.** An unavailable instance cannot be launched, and the user is offered either to point at a new path or to remove it from the registry.
- **AC-9.** With an empty registry the list explains how to add the first instance.

---

### US-REG-04 — Changing the metadata

**As** the owner of an instance
**I want** to rename it and change its colour
**so that** the registry reflects the build's current purpose.

Tags: `@FR-REG-050` `@phase-1` `@area-reg`
Rationale: `PLAN.md` → «Чеклист, Фаза 1»

**Preconditions**
- The instance is registered.

**Acceptance criteria**
- **AC-1.** The name, the description, the accent colour and the preferred port are available for changing.
- **AC-2.** The changes are saved and survive a restart of the app.
- **AC-3.** Changing the metadata does not affect the contents of the instance's folder.
- **AC-4.** Changing the preferred port of a running instance does not affect the current run and applies from the next one.

**Negative and edge cases**
- **AC-5.** The restrictions on the name and the port are the same as at registration.

---

### US-REG-05 — Removing from the registry

**As** the owner of an instance that is no longer needed
**I want** to take it out of the app
**so that** the list does not get cluttered.

Tags: `@FR-REG-060` `@phase-1` `@area-reg`
Rationale: `PLAN.md` → «Чеклист, Фаза 1»

**Preconditions**
- The instance is registered.

**Acceptance criteria**
- **AC-1.** The user can take an instance out of the registry.
- **AC-2.** Before the removal it is stated explicitly that the folder on disk will stay untouched.
- **AC-3.** The removal requires a confirmation.
- **AC-4.** After the removal the folder can be registered again, and it will pass the check once more.
- **AC-5.** Files the app previously wrote inside the instance's folder are not taken away on removal from the registry; the user is told which files those are.

**Negative and edge cases**
- **AC-6.** A running instance cannot be taken out of the registry until it is stopped; stopping it is offered.
