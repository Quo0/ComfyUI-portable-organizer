#!/usr/bin/env node
// Points git at `.githooks/`. Runs from the root `prepare` script, so a fresh
// clone gets the hooks on its first `pnpm install` — nobody has to remember a
// separate setup command, and hooks that live in the repository are reviewable
// the way `.git/hooks` never was.
//
// This never fails the install. A missing git, a tarball with no repository, a
// CI checkout — all of them exit quietly. A hook that cannot be installed is
// an inconvenience; an install that dies because of one is a broken clone.

import { spawnSync } from 'node:child_process'

const HOOKS_PATH = '.githooks'

if (process.env.CI) process.exit(0)

const inRepo = spawnSync('git', ['rev-parse', '--git-dir'], { encoding: 'utf8' })
if (inRepo.status !== 0) process.exit(0)

const current = spawnSync('git', ['config', '--get', 'core.hooksPath'], { encoding: 'utf8' })
if (current.status === 0 && current.stdout.trim() === HOOKS_PATH) process.exit(0)

const set = spawnSync('git', ['config', 'core.hooksPath', HOOKS_PATH], { encoding: 'utf8' })
if (set.status !== 0) {
  console.warn(`[hooks] could not set core.hooksPath: ${(set.stderr || '').trim()}`)
  console.warn(`[hooks] run it by hand: git config core.hooksPath ${HOOKS_PATH}`)
  process.exit(0)
}

console.log(`[hooks] core.hooksPath = ${HOOKS_PATH}`)
