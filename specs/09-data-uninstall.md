# EP-DATA — Data storage and uninstalling the app

A requirement the user stated outright: install the `.exe`, uninstall it
through the standard Windows facility — and everything superfluous disappears
from the disk, but **not** the models and **not** the workflow library.

Hence the hard boundary between two classes of data. The app owns its own
settings and is entitled to delete them. The user's content — the models, the
workflows, the ComfyUI builds themselves — does not belong to the app, and it
will not touch it on removal.

## Functional requirements

| ID | Requirement | Rationale in `PLAN.md` |
|---|---|---|
| `FR-DATA-010` | The app's data is stored in the standard places for Windows applications | «Дисциплина хранения данных» |
| `FR-DATA-020` | The user's content is stored only in folders they chose themselves | «Дисциплина хранения данных» |
| `FR-DATA-030` | The app does not write into system or user folders beyond its own | «Дисциплина хранения данных» |
| `FR-DATA-040` | Installation does not require administrator rights | «Дисциплина хранения данных» |
| `FR-DATA-050` | On removal the user chooses whether to delete the app's data | «Дисциплина хранения данных» |
| `FR-DATA-060` | Uninstalling the app does not affect the user's content | «Дисциплина хранения данных» |
| `FR-DATA-070` | The app shows where everything is stored and what will disappear on removal | «Дисциплина хранения данных» |
| `FR-DATA-080` | Updating the app keeps the data | «Дисциплина хранения данных» |
| `FR-DATA-090` | The only writes outside its own folders are made on the user's explicit command and are named to them | «Дисциплина хранения данных» |
| `FR-DATA-100` | The app reports an available new version and installs it only with the user's consent | «Выпуск» |
| `FR-DATA-110` | Before an update is installed the user decides the fate of the running instances | «Выпуск» |
| `FR-DATA-120` | Checking for updates can be switched off | «Выпуск» |

---

### US-DATA-01 — Transparency of storage

**As** a user who keeps an eye on the cleanliness of their system
**I want** to know what the app stores and where
**so that** I do not have to guess what will be left after removal.

Tags: `@FR-DATA-010` `@FR-DATA-070` `@FR-DATA-090` `@phase-4` `@area-data`
Rationale: `PLAN.md` → «Дисциплина хранения данных»

**Preconditions**
- The app is running.

**Acceptance criteria**
- **AC-1.** The locations of the app's data are shown, with the option to open them in Explorer.
- **AC-2.** The locations of the user's content are shown: the shared models root, the workflow library, the instance folders.
- **AC-3.** It is stated explicitly what will disappear when the app is removed and what will stay.
- **AC-4.** The files the app wrote inside the instance folders on the user's command are named, and it is stated that they will stay on removal.
- **AC-5.** The app's version is shown.

---

### US-DATA-02 — Uninstalling the app

**As** a user who decided to remove the app
**I want** to uninstall it through the standard Windows facility
**so that** I do not have to hunt for an uninstaller or clean the system by
hand.

Tags: `@FR-DATA-040` `@FR-DATA-050` `@FR-DATA-080` `@phase-4` `@area-data`
Rationale: `PLAN.md` → «Дисциплина хранения данных»

**Preconditions**
- The app is installed and has been used.

**Acceptance criteria**
- **AC-1.** The app is uninstalled through the standard Windows program removal mechanism.
- **AC-2.** Installation and removal do not require administrator rights.
- **AC-3.** On removal, deleting the app's data as well is offered.
- **AC-4.** On consent the app's data — the settings, the instance registry, the housekeeping files — is deleted in full.
- **AC-5.** On refusal the data is kept, and a reinstall picks up the previous registry and settings.
- **AC-6.** Updating the app over a previous version does not delete the data.

**Negative and edge cases**
- **AC-7.** After a removal with consent to the cleanup, no files of the app are left in the standard storage places.
- **AC-8.** Removal while ComfyUI servers are running leaves no processes running.

---

### US-DATA-03 — The safety of the user's content

**As** a user with hundreds of gigabytes of models
**I want** to be sure that uninstalling the app will not touch them
**so that** I am not afraid to uninstall it.

Tags: `@FR-DATA-020` `@FR-DATA-030` `@FR-DATA-060` `@phase-4` `@area-data`
Rationale: `PLAN.md` → «Дисциплина хранения данных»

**Preconditions**
- The shared models root and the workflow library are set up, and there are
  registered instances.
- The app has been uninstalled with consent to the data cleanup.

**Acceptance criteria**
- **AC-1.** The shared models root and everything in it are in place.
- **AC-2.** The workflow library, together with its tags and notes, is in place.
- **AC-3.** Every instance's folder is in place and works when launched by its own means.
- **AC-4.** The files written by the app inside the instance folders on the user's command stay — they lie in someone else's installation, and the app has no right to take them away.
- **AC-5.** The user knew about this in advance from the information in `US-DATA-01`.

**Negative and edge cases**
- **AC-6.** Under no conditions does the app delete the folders the user chose as content storage.
- **AC-7.** The app does not create files in the user's personal folders such as Documents, and leaves none behind.

---

### US-DATA-04 — Updating the app

**As** a user of the installed app
**I want** to receive new versions without losing my work and settings
**so that** an update is a benefit rather than a risk.

Tags: `@FR-DATA-100` `@FR-DATA-110` `@FR-DATA-120` `@phase-4` `@area-data`
Rationale: `PLAN.md` → «Выпуск»

**Preconditions**
- The app is installed and a newer version is out.

**Acceptance criteria**
- **AC-1.** The app reports an available new version and shows its number.
- **AC-2.** The installation begins only after explicit consent; it never installs silently.
- **AC-3.** The integrity of the update received is verified before the installation.
- **AC-4.** After the update the settings, the instance registry and the connections to shared resources are in place.
- **AC-5.** The user can see what changed in the new version.
- **AC-6.** Checking for updates can be switched off in the settings; when off, it is not performed at all.

**Negative and edge cases**
- **AC-7.** If at least one instance is running, the installation does not begin: the user is told that the app will be closed together with the running servers, and is offered a choice — stop them and update, or postpone the installation until the next launch.
- **AC-8.** With no network connection the app works as usual and shows no update-check errors.
- **AC-9.** An update that failed the integrity check is not installed, and the user is told why.
- **AC-10.** An interrupted download of an update does not leave the app in an unusable state.
