---
name: release
description: Cutting a release of ComfyUI Portable Organizer — choosing the number, writing the CHANGELOG section, raising the version in the five places, the gates and the tag. Use when asked to "release", "cut version X", "raise the version", "prepare a release" or "publish an update".
---

# Cutting a release

The whole release is one commit plus one tag. The commit carries the raised
version and the CHANGELOG section; the tag points at that commit and starts the
workflow. **The order is not negotiable: the tag goes last**, because the build
takes the commit the tag points at — an edit made afterwards is not "one more
push" but a tag that has to be moved.

The machinery behind all of this — the workflow, the updater, the "Download"
button, why there is no code signature — is in `RELEASING.md`. This skill is the
procedure; that file is the reasoning.

## The order

**1. See where the repository stands.**

```
pnpm release:check
git log --oneline v<previous>..HEAD
```

`release:check` reads the version out of all five places at once and refuses if
they disagree. The log is what the section is written from — but it is the
source, not the text.

**2. Write the CHANGELOG section**, under a `## Unreleased` heading at the top,
above the previous version. This is the part that cannot be generated; see
"What the section says" below.

**3. Raise the version:**

```
pnpm release:version patch     # or minor, major, or an exact 0.2.0
```

The script moves all five files together and renames `## Unreleased` to
`## <version> — <date>`. It refuses if the five have drifted apart, if the
number does not go up, or if the section is missing or empty. A refusal is a
real problem, not an obstacle to work around by editing the files by hand.

**4. Check what will become the release body:**

```
pnpm release:notes v<version>
```

It prints exactly what CI will cut out and fails in exactly the place CI would.
What it prints is also what the app shows under "What changed" — read it as a
user would.

**5. Run the gates locally**: `pnpm ui-design:check`, `pnpm i18n:check`,
`pnpm typecheck`. They stand in the workflow too, but failing them there costs
twenty minutes of Rust build. If the release carries Rust changes, the `verify`
skill says what else is worth running.

**6. Commit** everything, including `bindings.ts` if it changed:
`chore(release): <version>`. The body says what the release carries and **why
this number** — the numbering is declared semantic, so a patch that added a
feature or a minor for a one-line CSS fix is a promise broken to whoever reads
the version.

**7. Tag — the user's step.** `git push` is denied in this repository's
permissions, and pushing a tag publishes a build. Print the two commands and
stop:

```
git push origin master
git tag v<version> && git push origin v<version>
```

Ask before doing anything on step 7 even if the tooling would allow it.

## What the section says

Two audiences read it and neither of them reads the diff: whoever is choosing
whether to install the update sees these exact lines rendered in a panel inside
the app, and whoever opens the release page sees them there. So it is written
for people.

- **The first words of a bullet are what changed for the user**, in bold, then
  the explanation. `**Buttons in a banner sat too high.** They were aligned on
  the first line of the text…` — not `fix(ui): centre banner buttons`.
- **Keep a Changelog groupings**: `### Added`, `### Fixed`, `### Changed`,
  `### Removed`. A release with a single theme can do without them and open with
  a paragraph instead.
- **Markdown works**: headings, bullets, bold, `code`. Long prose does not — the
  panel is narrow.
- **Nothing internal.** Refactors, moved files and dependency bumps go in the
  commit body, not here. A release that genuinely changed nothing for users says
  so in one sentence.
- **The user's language is not the CHANGELOG's.** The file is English; the app
  shows it as it is, untranslated, and that is deliberate — see `RELEASING.md`.

## Choosing the number

Semantic, and before 1.0 the meanings still hold:

| Step | When |
|---|---|
| `patch` | a fix, a text change, an empty test release — no new functionality |
| `minor` | a new screen, a new command, a new capability the user can see |
| `major` | not before 1.0; a break in the user's data or settings |

A prerelease (`0.2.0-beta.1`) is marked as such by CI from the hyphen alone, and
`releases/latest` skips it: the "Download" button and the updater keep pointing
at the last stable version. That is what a prerelease is for — and also why an
accidental hyphen in a stable tag looks exactly like a broken app.

When in doubt between patch and minor, take the smaller: a number that promises
less than the release delivers disappoints nobody.

## After the tag

The workflow is watched, not fired and forgotten. Two failure shapes are worth
knowing in advance:

- **It failed before the build** — the tag does not match the version, or the
  section is missing. Cheap: delete the tag, fix, tag again.
  `git push --delete origin v<version> && git tag -d v<version>`.
- **It failed after `tauri-action`** — a draft release with that tag is left
  behind and **must be deleted by hand** before the next attempt, or the
  previous pass's assets mix with the new ones in one release.

A release already published cannot be fixed by editing its body: `latest.json`
carries the notes as they were **at build time**. Only a new release fixes it.

## What not to do

Do not edit the five version files by hand — that drift is what the script
exists to catch, and it is only ever noticed at the next release.

Do not write the CHANGELOG section from `git log` verbatim. Commit subjects are
addressed to whoever will maintain the code; the section is addressed to whoever
is deciding whether to close the app and install something.

Do not bundle unrelated work into the release commit. `chore(release):` is the
version, the CHANGELOG and nothing else — so that a bad release can be read, and
if need be reverted, in one piece.
