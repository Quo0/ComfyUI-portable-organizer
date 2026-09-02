// Shared bits of the pre-commit and pre-push hooks: running a command with its
// output visible, and asking git things.

import { spawnSync } from 'node:child_process'

// `shell: true` is not optional on Windows: pnpm is a .cmd shim there, and
// spawn without a shell cannot execute it. The command is handed over as one
// string rather than as a command plus an args array — with a shell those two
// are concatenated anyway, and Node 24 deprecates the pairing (DEP0190).
// Everything here is a literal from this file, so there is nothing to escape.
export function run(label, command, args) {
  const started = Date.now()
  process.stderr.write(`  ${label}\n`)
  const result = spawnSync([command, ...args].join(' '), { stdio: 'inherit', shell: true })
  const seconds = ((Date.now() - started) / 1000).toFixed(1)

  if (result.status === 0) {
    process.stderr.write(`  ok — ${seconds}s\n\n`)
    return true
  }

  process.stderr.write(`\n  FAILED after ${seconds}s: ${command} ${args.join(' ')}\n`)
  return false
}

export function git(args) {
  const result = spawnSync('git', args, { encoding: 'utf8' })
  if (result.status !== 0) return null
  return result.stdout
}

export function gitLines(args) {
  const out = git(args)
  if (out === null) return null
  return out.split(/\r?\n/).map(line => line.trim()).filter(Boolean)
}

export function has(command) {
  const result = spawnSync(command, ['--version'], { stdio: 'ignore', shell: true })
  return result.status === 0
}

export function skipped(name) {
  if (!process.env.CPO_SKIP) return false
  process.stderr.write(`${name}: skipped (CPO_SKIP is set)\n`)
  return true
}
