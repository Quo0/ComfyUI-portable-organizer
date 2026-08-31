# EP-INST — The install wizard

Unpacking a new instance from a portable build archive that the user
downloaded themselves. The wizard is permanently available, not only on the
first launch: it is also what a new version is rolled out with, alongside the
old one.

The app never updates an instance in place. Everything new is installed
alongside and what exists does not change — that is the key promise for actor
A3, who is afraid of losing a working configuration.

## Functional requirements

| ID | Requirement |
|---|---|
| `FR-INST-010` | The user points at a portable build archive; the app shows its contents and the space required before the work begins |
| `FR-INST-020` | The app remembers a history of the archives used, across sessions, and checks that they are present |
| `FR-INST-030` | In a single run the archive is unpacked into one or several destination folders |
| `FR-INST-040` | Each destination is given a name, a description and an accent colour |
| `FR-INST-050` | The app refuses the installation before it begins if there is not enough space on the disk |
| `FR-INST-060` | The app warns about a destination path that is too long |
| `FR-INST-070` | An interrupted installation leaves behind no folder that the app would take for a working instance |
| `FR-INST-080` | The progress of the installation is visible: the current file, the fraction done, the current destination |
| `FR-INST-090` | Unpacked instances are registered automatically and remember the source archive |
| `FR-INST-100` | The installation does not change previously created instances |
| `FR-INST-110` | The extra nesting level from the archive is not carried into the destination folder |

---

### US-INST-01 — Choosing the source archive

**As** a user installing ComfyUI
**I want** to point at the downloaded archive and see what is in it
**so that** I can be sure I took the right file.

Tags: `@FR-INST-010` `@FR-INST-020` `@phase-1.5` `@area-inst`

**Preconditions**
- The user has opened the install wizard.

**Acceptance criteria**
- **AC-1.** The archive is chosen through the system file picker.
- **AC-2.** Previously used archives are offered as a list, so the file does not have to be hunted for again.
- **AC-3.** For the chosen archive its size and date are shown.
- **AC-4.** The size the unpacked build will take up is shown.
- **AC-5.** The user can replace the chosen archive with another without leaving the wizard.
- **AC-6.** The chosen archive is remembered and offered the next time the wizard is opened.

**Negative and edge cases**
- **AC-7.** If a remembered archive has disappeared from the disk, this is stated on entering the wizard, and choosing another file is offered.
- **AC-8.** If a remembered archive has changed in size or date, the user is warned — the file may have been replaced.
- **AC-9.** A file that is not a portable build archive is rejected with an explanation of which file is expected.
- **AC-10.** A corrupt archive is rejected before the extraction begins.

---

### US-INST-02 — Setting the destination folder and the metadata

**As** a user installing ComfyUI
**I want** to choose where to unpack it and name the instance right away
**so that** I do not have to come back to the setup later.

Tags: `@FR-INST-030` `@FR-INST-040` `@FR-INST-060` `@FR-INST-110` `@phase-1.5` `@area-inst`

**Preconditions**
- The archive has been chosen and parsed.

**Acceptance criteria**
- **AC-1.** The user picks the destination folder through the system dialog.
- **AC-2.** The instance's name, description and accent colour are set for the destination.
- **AC-3.** The final path where the build will end up is shown.
- **AC-4.** The build's contents are placed directly into the chosen folder, without the extra nesting level from the archive.
- **AC-5.** The user is offered a sensible default name, which they can change.

**Negative and edge cases**
- **AC-6.** A non-empty destination folder is rejected with an explanation; choosing another is offered.
- **AC-7.** A destination path that is too long raises a warning recommending a shorter one, because some of the build's files are nested very deeply.
- **AC-8.** A path that cannot be written to is rejected before the installation begins.
- **AC-9.** Two destination folders in one run cannot be the same.

---

### US-INST-03 — Checking the free space

**As** a user installing ComfyUI
**I want** to learn about a shortage of space in advance
**so that** I do not lose half an hour on an extraction that will break off
anyway.

Tags: `@FR-INST-050` `@phase-1.5` `@area-inst`

**Preconditions**
- The archive has been chosen, the destinations have been set.

**Acceptance criteria**
- **AC-1.** The required size is determined from the archive itself rather than estimated approximately.
- **AC-2.** The free space is checked before the extraction begins.
- **AC-3.** With several destinations on one disk the required size is counted as a total.
- **AC-4.** If there is not enough space, the installation does not begin, and how much is needed and how much is available are both stated.

**Negative and edge cases**
- **AC-5.** A margin above the bare minimum is taken into account — a disk filled to the brim does not count as suitable.

---

### US-INST-04 — Connecting to shared resources during installation

**As** a user who already has shared models set up
**I want** to connect the new instance to them right away
**so that** I do not have to do it as a separate step after the installation.

Tags: `@FR-INST-090` `@phase-2.5` `@phase-2.6` `@area-inst`

**Preconditions**
- The destinations have been set.

**Acceptance criteria**
- **AC-1.** If a shared models root is set up, connecting the new instances to it is offered, and the connection is on by default.
- **AC-2.** If a workflow library is set up, connecting the new instances to it is offered.
- **AC-3.** Setting up the shared resources is available right here if they are not set yet — there is no need to leave for the settings section.
- **AC-4.** The connection applies to every destination in the current run.
- **AC-5.** The user can decline the connection and set it up later.

**Negative and edge cases**
- **AC-6.** If the shared resources are not set up and the user did not set them, the installation continues without them.

---

### US-INST-05 — The progress of the installation

**As** a user who started the extraction
**I want** to see what is happening and how much is left
**so that** I can tell whether the app has frozen or is working.

Tags: `@FR-INST-080` `@FR-INST-070` `@phase-1.5` `@area-inst`

**Preconditions**
- Every check has passed, the installation is running.

**Acceptance criteria**
- **AC-1.** Progress is visible as a fraction done, not merely as the fact that work is happening.
- **AC-2.** The file currently being processed is visible.
- **AC-3.** With several destinations it is visible which one is being processed now and how many there are in total.
- **AC-4.** The interface stays responsive: the user can leave for another section and come back without interrupting the installation.
- **AC-5.** The installation can be cancelled at any moment.

**Negative and edge cases**
- **AC-6.** After a cancellation nothing is left in the destination folder that the app would take for a working instance.
- **AC-7.** After a cancellation no temporary installation files are left.
- **AC-8.** If the app crashed during the installation, the unfinished folder is not counted as an instance on the next launch.
- **AC-9.** An extraction error interrupts the installation, reports the reason and leaves the system in the state it was in before the start.

---

### US-INST-06 — Finishing the installation

**As** a user who waited out the extraction
**I want** to move on to work straight away
**so that** I do not have to hunt for what to do next.

Tags: `@FR-INST-090` `@FR-INST-100` `@phase-1.5` `@area-inst`

**Preconditions**
- The installation finished successfully.

**Acceptance criteria**
- **AC-1.** Every instance created is listed with its name and path.
- **AC-2.** The instances are already registered — no separate registration step is required.
- **AC-3.** Their launch profiles have been recognised.
- **AC-4.** Each of them has the source archive and the installation date saved.
- **AC-5.** Launching one of the created instances right away is offered.
- **AC-6.** The instances that existed before have not changed.

---

### US-INST-07 — Installing a new version alongside the old one

**As** A3, who wants to try a fresh version
**I want** to unpack it separately
**so that** my working build stays untouched.

Tags: `@FR-INST-020` `@FR-INST-100` `@FR-INST-090` `@phase-1.5` `@area-inst`

**Preconditions**
- There is an instance in the registry unpacked from a previous archive.
- The archive of the new version has been downloaded.

**Acceptance criteria**
- **AC-1.** The user picks the new archive instead of the remembered one without losing the record of the old one.
- **AC-2.** The archive history holds both, and for each of them it is visible when it was used.
- **AC-3.** The new instance is unpacked into a separate folder and registered in addition to the existing ones.
- **AC-4.** The existing instance changes neither on disk nor in the registry.
- **AC-5.** In the registry it is visible for every instance which archive it was unpacked from — the versions can be told apart without opening the folders.

**Negative and edge cases**
- **AC-6.** An attempt to pick an existing instance's folder as the destination is rejected as non-empty.
- **AC-7.** An archive the user deleted from the disk can be taken out of the history.

---

### US-INST-08 — Several destinations in a single run

**As** A3, who needs two identical builds for different experiments
**I want** to unpack them in a single operation
**so that** I do not have to wait out the extraction twice.

Tags: `@FR-INST-030` `@FR-INST-040` `@phase-1.5` `@area-inst`

**Preconditions**
- The archive has been chosen.

**Acceptance criteria**
- **AC-1.** The user can add several destination folders in one run.
- **AC-2.** Each destination is given its own name, description and colour.
- **AC-3.** A destination can be taken out of the list before the installation begins.
- **AC-4.** On completion every instance is registered, and each of them works independently.
- **AC-5.** The total time of installing into several folders is noticeably less than the time of the same number of separate installations.

**Negative and edge cases**
- **AC-6.** An error on one of the destinations does not cancel the ones already created successfully; the user is told what succeeded and what did not.
- **AC-7.** A cancellation in the middle leaves the successfully finished destinations and removes the unfinished one.
