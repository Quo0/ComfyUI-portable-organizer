# ComfyUI Portable Organizer specifications

This directory answers the question of **what** the app does from the user's
point of view: actors, requirements with stable IDs, stories with acceptance
criteria, end-to-end journeys.

The question of **how** it is built is deliberately not answered here. The
technical findings — which flags to append to the launch command, why a child
webview does not get a 403, where Windows breaks on long paths — are of no use
to a designer, a tester or a technical writer: there is no user behaviour in
them, only its causes. The absolute rules that came out of those findings are
in `CLAUDE.md`; the rest lives in the code.

## For whom and what for

| Role | What they take from here |
|---|---|
| Designer | `journeys.md` — which screens are needed at all and in what order the user walks through them. The stories — what has to be on each screen |
| Tester | The acceptance criteria. Each criterion is one verification step, nothing needs to be inferred |
| Technical writer | `journeys.md` gives the structure of the manual, the stories give the content of the pages |
| Developer | `traceability.md` — which requirements a phase covers and what of it is already implemented |

## The files

| File | Contents |
|---|---|
| `glossary.md` | The language of the domain. One term, one definition |
| `actors.md` | Who uses the app, what they know, what they are afraid of |
| `journeys.md` | The end-to-end journeys `J-01…J-06` — the user's path in full |
| `01-onboarding.md` | The first launch, the empty state, the fork in the road |
| `02-registry.md` | The registry: adding an existing folder, metadata, removal |
| `03-installer.md` | The install wizard, from an archive |
| `04-run.md` | Launching, logs, readiness, stopping, failures |
| `05-tabs-nav.md` | Navigation and the embedded ComfyUI tab |
| `06-shared-models.md` | Shared model storage |
| `07-workflows.md` | The workflow library |
| `08-appearance.md` | Theme, language, notifications |
| `09-data-uninstall.md` | Data storage and uninstalling the app |
| `nfr.md` | Non-functional requirements |
| `traceability.md` | The summary table of links |

## The identifier scheme

Stable IDs are the whole point of the exercise. They tie a requirement to a
mockup, a test case, a documentation page and an e2e test.

| Entity | Format | Example |
|---|---|---|
| Epic | `EP-<AREA>` | `EP-SHARED` |
| Functional requirement | `FR-<AREA>-<NNN>` | `FR-SHARED-020` |
| User story | `US-<AREA>-<NN>` | `US-SHARED-03` |
| Acceptance criterion | `AC-<N>` inside a story | `US-SHARED-03/AC-4` |
| Non-functional requirement | `NFR-<NNN>` | `NFR-050` |
| End-to-end journey | `J-<NN>` | `J-02` |

The areas:

| Code | Area |
|---|---|
| `ONB` | Onboarding, the first launch |
| `REG` | The instance registry |
| `INST` | Installing from an archive |
| `RUN` | Launching and the lifecycle |
| `TAB` | Navigation and embedded tabs |
| `SHARED` | Shared model storage |
| `WF` | The workflow library |
| `UI` | Theme, language, notifications |
| `DATA` | Data storage and removal |

**Numbering goes in steps of 10** — so that something new can be inserted
between what exists without renumbering. A deleted requirement is marked
obsolete and its number is never taken again: otherwise the links in old test
cases start pointing at the wrong thing.

## Tags

Every story has a line of tags:

```
Tags: `@FR-SHARED-020` `@FR-SHARED-030` `@phase-2.5` `@area-shared`
```

- `@FR-…` — which requirements the story implements;
- `@phase-N` — the implementation phase, so it is clear when this will appear;
- `@area-…` — the area, for filtering.

## How to write acceptance criteria

Five rules. The first matters more than the other four put together.

**1. The level of intent, not of interface mechanics.** We write "the user
connects the instance to the shared models folder", not "the user presses the
button in the top right corner". The reason is practical: these specs are the
input for the UI design, and they cannot presuppose a UI that does not exist
yet. On top of that, a criterion describing where a button sits will go stale
with the first edit to the mockup and drag the test case and the documentation
page down with it.

**2. One verifiable statement per criterion.** If a criterion contains an
"and", check whether it is not two criteria.

**3. An observable result instead of internal mechanics.** "Models from the
shared folder are visible in the node lists" — yes. "The app appends
`--extra-model-paths-config` to the arguments" — no, that is the mechanics of
the implementation and it belongs in the code.

**4. No "correctly", "conveniently", "quickly".** If quickly — then how
quickly, and in `nfr.md`.

**5. Negative cases are mandatory and go in a block of their own.** They are
exactly what gets lost in the move to test cases, because reading the happy
path does not bring them to mind.

## The story template

```markdown
### US-AREA-NN — A short title

**As** <actor>
**I want** <to do what>
**so that** <what benefit>.

Tags: `@FR-…` `@phase-N` `@area-…`

**Preconditions**
- …

**Acceptance criteria**
- **AC-1.** …

**Negative and edge cases**
- **AC-N.** …
```

## What to do when things disagree

If an acceptance criterion contradicts what the app actually does, that is not
a typo but a design decision nobody noticed. It is settled here first — the
criterion is either confirmed or rewritten — and only then does the code move.
Silently bending the criterion to fit the behaviour that has already been
written turns the specification into a description of the code, which is
exactly what it must not become.
