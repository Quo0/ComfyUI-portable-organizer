# Releasing

The artifacts live in GitHub Releases; the documentation site links to them
with a "Download" button. The repository is public — otherwise the assets
could not be downloaded without authorisation, and Pages would need a paid
plan.

## The version has a single source

`apps/desktop/src-tauri/tauri.conf.json`, the `version` field. The tag must
match it, and the first step of the workflow checks exactly that: `v<version>`
against the tag name, a mismatch fails the build. Without that check a release
will one day go out where the tag says one thing and the app shows another,
and there will be no way left to tell what a user has installed.

## The workflow `release-desktop.yml`

Triggered by a push of a `v*` tag, runs on `windows-latest`.

The order: check the tag against the version → Node and pnpm → Rust with the
Cargo cache → `pnpm install` → **quality gates** → `tauri-apps/tauri-action`.

The gates are `pnpm ui-design:check`, `pnpm i18n:check` and `pnpm typecheck`.
They stand before the build deliberately: rolling back a published installer
with a broken dark theme or a missing translation costs more than not
releasing.

`tauri-action` creates the release as a **draft**. The next step computes the
SHA-256 sums and appends them to the release body, and only then is the draft
published — so there is never a state where the file is already visible and
there is nothing to verify it against.

A tag with a hyphen (`v0.2.0-beta.1`) is marked a prerelease. Then
`releases/latest` skips it and the button on the site keeps pointing at the
stable version.

**The prerelease flag is not cosmetic, and a mistake in it looks like a broken
app.** Both consumers of a release depend on `releases/latest`: the updater
endpoint (`releases/latest/download/latest.json`) and the data loader behind
the "Download" button. Both skip a prerelease, so with perfectly fine assets
the updater answers "could not fetch valid JSON" and the site silently falls
back to the link to the releases page. That is exactly what happened on
`v0.1.0`. Which is why the publish step sets `--prerelease` /
`--prerelease=false --latest` explicitly, from the presence of a hyphen in the
tag, rather than trusting the state `tauri-action` left the draft in.

The release body is taken from the `CHANGELOG.md` section for this version.
`tools/release-notes.mjs` cuts the section out and checks the tag against the
version; no section means the release stops before the build.

## The release checklist

The order is mandatory: the tag goes last, because the workflow builds **the
commit the tag points at**. An edit made afterwards does not make it into the
build — it is not "one more push" but a tag that has to be moved.

1. **Raise the version** in `apps/desktop/src-tauri/tauri.conf.json` — the
   single source. And with it `apps/desktop/src-tauri/Cargo.toml`, its
   `Cargo.lock` entry, `apps/desktop/package.json` and the root
   `package.json`: the workflow does not check those, but numbers that have
   drifted apart are afterwards hunted by eye.
2. **Rename `## Unreleased`** to `## <version> — <date>`. An empty section
   fails the release just as a missing one does.
3. **Run the gates locally**: `pnpm ui-design:check`, `pnpm i18n:check`,
   `pnpm typecheck`. They also stand in the workflow, but failing them there
   costs twenty minutes of Rust build.
4. **Check the section is cut correctly**: `node tools/release-notes.mjs
   v<version>`. The script prints what will become the release body and fails
   in exactly the place CI would.
5. **Commit and push** to `master` — the whole release, including
   `bindings.ts` if it changed.
6. **Tag that commit and push the tag**:
   `git tag v<version> && git push origin v<version>`.

The conditions without which the release does not go through, each of them
either checked before the build or failing it midway:

| Condition | Where it breaks |
|---|---|
| `tauri.conf.json.version` == the tag without `v` | the check step, before installing anything |
| `CHANGELOG.md` has a non-empty `## <version>` | the same step |
| the secrets `TAURI_SIGNING_PRIVATE_KEY` and `..._PASSWORD` are set | `latest.json` is built unsigned and the updater refuses to install it |
| `pnpm-lock.yaml` is committed and current | `pnpm install --frozen-lockfile` |
| `bindings.ts` is committed and matches the Rust side | `pnpm typecheck` |
| locale and theme parity | `pnpm i18n:check`, `pnpm ui-design:check` |
| Pages enabled with the GitHub Actions source | the `docs` job at the end of the release |
| the `github-pages` environment allows deploys from `v*` tags | the same job |

**Moving a tag** (the first build failed and the commit has to be fixed):

```
git push --delete origin v<version>
git tag -d v<version>
# fix, commit, push
git tag v<version> && git push origin v<version>
```

If the build failed after `tauri-action`, a draft release with the same tag is
left behind. It has to be deleted by hand before the next attempt: otherwise
the previous pass's assets mix with the new ones in one release.

## What a release contains

| File | What for |
|---|---|
| `...-setup.exe` | the NSIS installer, installs into the user profile |
| `...-setup.exe.sig` | the update signature |
| `latest.json` | the updater manifest |
| `SHA256SUMS.txt` | manual integrity verification |

The checksums matter more than usual here: the app is unsigned, and comparing
the sum is the only way available to a user to be sure the file was not
swapped.

## The "Download" button on the site

The link is neither written by hand nor fetched from the browser. VitePress
runs data loaders at build time: `download.data.ts` asks the GitHub API for
the latest non-prerelease release and returns the version, link, size, date
and checksum. All of it ends up in the static output — the page makes no
network request of its own.

**The site build must never fail because of GitHub.** With the API
unreachable or the rate limit exhausted, the loader returns a fallback — the
link to the releases page without the details. The site is always published.

**The site is rebuilt when a release is published**: the release workflow
calls the docs deploy at the end. Otherwise the button would lag behind a
fresh release until the next documentation edit.

**A prerelease does not rebuild the site** — the job carries
`if: !contains(github.ref_name, '-')`. The button's data loader asks for the
latest non-prerelease release, so after `v1.0.0-rc.1` it would build exactly
the same site as before it; there is no point publishing Pages for an
unchanged result.

A deploy called that way runs **with the tag's ref**, not with a branch: a
reusable workflow inherits the caller's ref. GitHub creates the `github-pages`
environment with the rule "default branch only", and the call from the release
is rejected on the `deploy` job after the artifact has already been built —
"Tag is not allowed to deploy to github-pages due to environment protection
rules". So the environment must carry a rule for ref type Tag with the pattern
`v*`. A manual docs run from `master` always passes that rule, which is why
the problem is invisible until a release.

Next to the button: the version, date, size, checksum and a link to all
versions — the user must see what they are downloading before the click.

## There is no signature, and that has to be explained

Azure Trusted Signing — the cheapest route — is available only to
organisations and sole proprietors in the US and Canada. An ordinary OV
certificate costs hundreds of dollars a year and still needs to accumulate
reputation before SmartScreen calms down. The decision: distribute unsigned.

Hence the mandatory `guide/smartscreen` page: what the "Windows protected your
PC" warning looks like, why it appears, what to click, how to compare the
checksum, and a direct admission that there is no signature. The link to it
sits next to the download button rather than hiding deep in the documentation:
without the explanation a noticeable share of people will decide they
downloaded a virus.

## Auto-update

Three requirements of Tauri v2, each easy to miss:
`bundle.createUpdaterArtifacts: true`, a key pair from `tauri signer generate`
(the public one in `plugins.updater.pubkey`, the private one and its password
in the CI secrets), and the `signature` field in `latest.json`, which holds
the **contents** of the `.sig` file, not a path to it.

**The private key is stored outside the repository and backed up.** If it is
lost, releasing an update for copies already installed becomes impossible
forever.

**An update signature is not a code signature.** The minisign keys prove the
update was released by you; they have no effect on SmartScreen. The two
mechanisms are unrelated.

**An update is never installed silently — and here is why that is critical.**
On Windows the app is force-closed during installation: that is a limitation
of the installer. And our child processes live in a Job Object with
`KILL_ON_JOB_CLOSE`. Put the two together: the user is generating an image,
agrees to the update — and ComfyUI dies along with the queue, while the next
cold start takes minutes.

So before installing, the app looks at whether any instances are running, and
if there are, offers a choice: stop them and update, or postpone the
installation until the next start.

**The update check can be turned off.** It is the only thing the app sends
outwards, and it is covered by a separate exception in `NFR-350`.
