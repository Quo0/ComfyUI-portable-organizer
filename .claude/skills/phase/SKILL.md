---
name: phase
description: The ritual for opening and closing a ComfyUI Portable Organizer development phase — what to read before the work and what to update after. Use when asked to "continue with the plan", "next phase", "let's close phase N", or when the work starts from a phase checklist.
---

# Phase: open and close

Work on a phase starts not with code but with reading, and ends not with a
commit but with a write-up of what was learned.

**The phase checklist is no longer in the repository.** `PLAN.md` and `plan/`
were removed; the scope of a phase now comes from the task itself, and the
requirements it has to satisfy come from `specs/`. If the scope is not stated,
ask for it rather than inferring it from neighbouring code.

## Read before the work

**1. Stories with acceptance criteria** — `specs/`, by phase tag:

```
grep -rn '@phase-2.5' specs/
```

The tag sits on the `Tags:` line of each story. The acceptance criteria are the
source of the user-facing checks; there is no need — and no permission — to
restate them. Negative cases are a separate block in the stories, and that is
exactly where they get lost most often.

**2. Mockups** — `apps/ui-design/screens/*.md`, the screen for the matching
scenario. The components drawn there already exist in
`apps/desktop/src/styles/components.css`; there is no need to build them again.
See the `design-ui` skill for details.

**3. The rules already paid for** — `CLAUDE.md`. Every absolute rule that
earlier phases bought the expensive way is there in short form. The reasoning
behind them is in `git log`, in the commits of the phase that found them.

## Update after the work

**Statuses** — `specs/traceability.md`, the "Status" column for the affected
requirements. It is filled in row by row, by looking: a status set without
looking is worse than no status at all.

**What was learned** — into the commit body, and it is not a retelling of the
diff: hypotheses disproved, numbers measured, rules found the expensive way. If
a rule is absolute and may never be broken, its short form goes into
`CLAUDE.md` as well — that is the only document the next session reads in full.

**Commits** — edits to `specs/` go in a separate commit from the code, so that
`git log specs/` reads as the evolution of the intent rather than being mixed
in with the implementation.

## What not to do

Do not grow the phase as you go. Adjacent work looks cheap right up until you
take it on; the scope of a phase is a decision made in advance and with
reasons.

Do not rewrite acceptance criteria to match what came out. A criterion
diverging from the implementation is either a defect or an unnoticed design
decision; both are fixed explicitly, and `specs/README.md` describes how.
