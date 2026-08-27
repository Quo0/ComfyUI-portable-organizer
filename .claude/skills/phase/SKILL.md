---
name: phase
description: The ritual for opening and closing a ComfyUI Portable Organizer development phase — what to read before the work and what to update after. Use when asked to "continue with the plan", "next phase", "let's close phase N", or when the work starts from a phase checklist.
---

# Phase: open and close

Phases are numbered in `plan/phases.md`. Work on a phase starts not with code
but with three documents, and ends not with a commit but with a write-up of what
was learned.

## Read before the work

**1. The phase checklist** — `plan/phases.md`, the relevant subsection. It gives
the scope: what is in the phase and what was deferred on purpose. Read only your
phase, not the whole file.

**2. Stories with acceptance criteria** — `specs/`, by phase tag:

```
grep -rn '@phase-2.5' specs/
```

The tag sits on the `Теги:` line of each story (the specs are in Russian, that
label is literal). The acceptance criteria are the source of the user-facing
checks; there is no need — and no permission — to restate them. Negative cases
are a separate block in the stories, and that is exactly where they get lost
most often.

**3. Mockups** — `apps/design/screens/*.md`, the screen for the matching
scenario. The components drawn there already exist in
`apps/desktop/src/styles/components.css`; there is no need to build them again.
See the `design-ui` skill for details.

**4. The technical plan section** — via the table in `PLAN.md`. For Phase 2.5
that is `plan/shared-models.md`, for 2.6 — `plan/workflows.md`, and so on. That
is where the findings from the ComfyUI sources live — the reason the phase is
shaped the way it is.

If the phase builds on a previous one, its write-up is in `plan/notes/`.

## Update after the work

**The checklist** — tick the finished items in `plan/phases.md`. An unfinished
item stays unfinished: leaving the box unticked on what was deferred is what
makes the report honest.

**The write-up** — a new file in `plan/notes/`, modelled on its neighbours. What
goes there is not a retelling of what was done but what was learned along the
way: hypotheses disproved, numbers measured, rules found the expensive way. If
a rule is absolute and may never be broken, its short form is duplicated into
`CLAUDE.md` while the reasoning stays in the write-up.

**Statuses** — `specs/traceability.md`, the «Статус» column for the affected
requirements.

**Commits** — plan edits go in a separate commit from code. That is written into
the plan itself and exists so that `git log plan/` reads as the evolution of the
intent, not mixed with the implementation.

## What not to do

Do not grow the phase as you go. Neighbouring checklist items look cheap right
up until you take them on; the scope of a phase is a decision made in advance
and with reasons.

Do not rewrite acceptance criteria to match what came out. A criterion diverging
from the implementation is either a defect or an unnoticed design decision; both
are fixed explicitly, and `specs/README.md` describes how.
