# EP-ONB — Onboarding and the first launch

The first thirty seconds of acquaintance with the app. This is where it is
decided whether A1 will understand what to do next, and whether A2 will believe
that the app will not touch their working folder.

There is no separate welcome screen: its role is played by the empty states of
the sections. That way, from the very first moment the user sees the app's real
structure rather than a temporary placeholder that will later disappear.

## Functional requirements

| ID | Requirement |
|---|---|
| `FR-ONB-010` | With an empty registry the app explains what it does and offers two paths: point at an existing ComfyUI folder, or install ComfyUI from an archive |
| `FR-ONB-020` | Both paths are available at any moment, not only on the first launch |
| `FR-ONB-030` | The app requires neither an account nor a network connection in order to work |
| `FR-ONB-040` | Navigation between the sections is available from the first launch, including with an empty registry |
| `FR-ONB-050` | A user who has no archive is given an explanation of exactly what to download |

---

### US-ONB-01 — The first launch with an empty registry

**As** a user opening the app for the first time
**I want** to understand at once what it does and where to start
**so that** I do not have to guess where to click.

Tags: `@FR-ONB-010` `@FR-ONB-040` `@phase-1` `@area-onb`

**Preconditions**
- The app is running, the instance registry is empty.

**Acceptance criteria**
- **AC-1.** The user is shown a brief explanation of the app's purpose.
- **AC-2.** Exactly two paths forward are offered: point at an existing ComfyUI folder, and install ComfyUI from an archive.
- **AC-3.** Each path comes with an explanation of who it suits — so the user chooses without guessing.
- **AC-4.** Navigation between the app's sections is available with an empty registry too.
- **AC-5.** The sections that require instances to exist explain this on an empty registry and say what to do first.

**Negative and edge cases**
- **AC-6.** The app works without a network connection: neither of the two paths requires the internet.
- **AC-7.** An account, a registration and a login are not asked for at any step.

---

### US-ONB-02 — Choosing the "install ComfyUI" path

**As** A1, who does not have ComfyUI yet
**I want** to install it through the app
**so that** I do not have to work out archives and `.bat` files on my own.

Tags: `@FR-ONB-010` `@FR-ONB-050` `@phase-1.5` `@area-onb`

**Preconditions**
- The user has chosen the installation path.

**Acceptance criteria**
- **AC-1.** The user arrives at the install wizard described in `US-INST-01`.
- **AC-2.** Before the archive is chosen it is explained that the archive has to be downloaded by the user, and which file exactly to look for.
- **AC-3.** It is explained roughly how much space will be needed, before the user picks a folder.

**Negative and edge cases**
- **AC-4.** If the user has no archive, they can leave the wizard without leaving behind either changed settings or created folders.

---

### US-ONB-03 — Choosing the "I already have ComfyUI" path

**As** A2, who has a working folder
**I want** to register it in the app
**so that** I can launch my familiar build from here.

Tags: `@FR-ONB-010` `@phase-1` `@area-onb`

**Preconditions**
- The user has chosen the path of registering an existing folder.

**Acceptance criteria**
- **AC-1.** The user arrives at the folder picker described in `US-REG-01`.
- **AC-2.** Before the folder is chosen it is stated that the registration will change nothing inside it.
- **AC-3.** It is stated what a suitable folder looks like — so the user understands which nesting level to pick.

**Negative and edge cases**
- **AC-4.** Declining to pick a folder returns the user to the fork without changing the state of the registry.

---

### US-ONB-04 — Returning to both paths after the first launch

**As** a user who already has instances
**I want** to add new ones the same way as the first time
**so that** I do not have to hunt for a different command for the same action.

Tags: `@FR-ONB-020` `@phase-1` `@area-onb`

**Preconditions**
- There is at least one instance in the registry.

**Acceptance criteria**
- **AC-1.** Both paths — registering an existing folder and installing from an archive — are permanently available, without emptying the registry.
- **AC-2.** Both paths lead to the same steps as on the first launch.
- **AC-3.** Adding a new instance does not affect the existing ones: neither their state nor their settings change.
