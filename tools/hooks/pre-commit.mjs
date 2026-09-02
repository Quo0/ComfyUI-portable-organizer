#!/usr/bin/env node
// Runs the checks that the staged files actually call for, and nothing else.
//
// The full circle takes minutes; the point here is that a commit stays a
// commit and not a coffee break. Locales cost half a second to check, tokens
// three quarters, types three seconds — so each one is paid for only when
// something it covers was touched. In the worst case, where a single commit
// touches all of it, this is around five seconds.
//
// What it deliberately does not do is stash the working tree to check exactly
// what is staged. `typecheck` therefore sees the files as they are on disk,
// not as they are in the index — a partially staged file can pass here and
// fail in CI. That is the accepted trade: a hook that moves your uncommitted
// work around is a hook that will eventually lose some of it.

import { parse } from 'yaml'
import { git, gitLines, run, skipped } from './lib.mjs'

if (skipped('pre-commit')) process.exit(0)

const staged = gitLines(['diff', '--cached', '--name-only', '--diff-filter=ACMR']) ?? []
if (staged.length === 0) process.exit(0)

const matches = pattern => staged.some(file => pattern.test(file))

// YAML first: it is the cheapest, and the failure it catches is the quietest.
// A broken workflow simply never runs; a broken issue form simply never
// appears in the picker. Neither says anything on the way past.
const yamlFiles = staged.filter(file => /\.ya?ml$/.test(file))
const brokenYaml = []
for (const file of yamlFiles) {
  // The staged content, not the file on disk — that is what is being committed.
  const content = git(['show', `:${file}`])
  if (content === null) continue
  try {
    parse(content)
  }
  catch (error) {
    brokenYaml.push([file, error.message])
  }
}

if (brokenYaml.length > 0) {
  process.stderr.write('\npre-commit: YAML that will not parse\n\n')
  for (const [file, message] of brokenYaml) {
    process.stderr.write(`  ✗ ${file}\n`)
    for (const line of message.split('\n')) process.stderr.write(`      ${line}\n`)
    process.stderr.write('\n')
  }
  process.stderr.write('  Skip the check on purpose: git commit --no-verify\n\n')
  process.exit(1)
}

const tasks = []

if (matches(/^apps\/desktop\/src\/i18n\/locales\/.+\.json$/)) {
  tasks.push(['locales changed → pnpm i18n:check', 'pnpm', ['i18n:check']])
}

if (matches(/^apps\/desktop\/src\/styles\/(tokens|components)\.css$/)) {
  tasks.push(['design tokens changed → pnpm ui-design:check', 'pnpm', ['ui-design:check']])
}

if (matches(/^apps\/desktop\/src\/.+\.(vue|ts)$/)) {
  tasks.push(['frontend changed → pnpm typecheck', 'pnpm', ['typecheck']])
}

if (tasks.length === 0) process.exit(0)

process.stderr.write('\npre-commit\n\n')
for (const [label, command, args] of tasks) {
  if (!run(label, command, args)) {
    process.stderr.write('\n  Skip the check on purpose: git commit --no-verify\n\n')
    process.exit(1)
  }
}
