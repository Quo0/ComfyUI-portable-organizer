#!/usr/bin/env node
// The gate before anything leaves the machine: the same three checks the
// release workflow runs, plus `cargo check` when the push carries Rust.
//
// Not the release build. That one takes twenty minutes and produces an
// installer nobody here wants — it belongs to the tag. What this catches is
// the class of breakage that would otherwise sit in master until release day
// and surface at the worst possible moment.
//
// `cargo check` is conditional for one reason: warm, against the existing
// target directory, it is under two seconds; cold — after `cargo clean` or a
// dependency bump — it is minutes. Running it on a push that touched no Rust
// would eventually teach everyone to reach for --no-verify, and a hook that is
// always skipped protects nothing.

import { readFileSync } from 'node:fs'
import { git, has, run, skipped } from './lib.mjs'

if (skipped('pre-push')) process.exit(0)

const ZERO = /^0+$/

function pushedRefs() {
  if (process.stdin.isTTY) return null
  try {
    const input = readFileSync(0, 'utf8')
    return input.split(/\r?\n/).filter(Boolean).map(line => line.split(/\s+/))
  }
  catch {
    return null
  }
}

// Which files this push carries. `null` means we could not work it out — in
// that case everything is checked, because guessing "nothing changed" is the
// only wrong answer here.
function changedFiles() {
  const refs = pushedRefs()
  if (refs === null) return null

  const alive = refs.filter(([, localSha]) => !ZERO.test(localSha))
  if (alive.length === 0) return []

  const files = new Set()
  for (const [, localSha, , remoteSha] of alive) {
    let diff
    if (ZERO.test(remoteSha)) {
      // A new branch or a tag: compare against what the remote already has.
      const base = git(['merge-base', 'origin/master', localSha])
      if (base === null) return null
      diff = git(['diff', '--name-only', base.trim(), localSha])
    }
    else {
      diff = git(['diff', '--name-only', remoteSha, localSha])
    }
    if (diff === null) return null
    for (const file of diff.split(/\r?\n/)) if (file.trim()) files.add(file.trim())
  }
  return [...files]
}

const files = changedFiles()
if (files !== null && files.length === 0) process.exit(0)

const rustTouched = files === null
  || files.some(file => file.startsWith('apps/desktop/src-tauri/'))

const tasks = [
  ['pnpm ui-design:check', 'pnpm', ['ui-design:check']],
  ['pnpm i18n:check', 'pnpm', ['i18n:check']],
  ['pnpm typecheck', 'pnpm', ['typecheck']],
]

if (rustTouched) {
  if (has('cargo')) {
    tasks.push([
      'cargo check --all-targets',
      'cargo',
      ['check', '--manifest-path', 'apps/desktop/src-tauri/Cargo.toml', '--all-targets'],
    ])
  }
  else {
    process.stderr.write('\n  cargo is not on PATH — the Rust check is skipped\n')
  }
}

process.stderr.write('\npre-push\n\n')
for (const [label, command, args] of tasks) {
  if (!run(label, command, args)) {
    process.stderr.write('\n  Push anyway: git push --no-verify\n\n')
    process.exit(1)
  }
}
