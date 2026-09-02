#!/usr/bin/env node
// Checks the commit message against the three rules this repository actually
// holds: Conventional Commits, a non-empty body, and English.
//
// The last two are the reason this is a script rather than commitlint. A body
// that explains *why* is a rule of CLAUDE.md and nothing enforced it until
// now — an empty one passes review because there is nothing to look at. And
// Cyrillic is banned outside backticks rather than banned outright: the
// commit message is English, but `стенд с пробелом` is the name of a test
// fixture, and a message quoting it is correct English prose about a Russian
// string. Inside backticks the text is data, not language.
//
// All violations are reported at once. Being told about the subject, fixing
// it, and only then being told about the body is how a hook gets disabled.

import { readFileSync } from 'node:fs'

if (process.env.CPO_SKIP) process.exit(0)

const TYPES = [
  'feat', 'fix', 'docs', 'style', 'refactor',
  'perf', 'test', 'build', 'ci', 'chore', 'revert',
]

const SUBJECT_LIMIT = 72
const SCISSORS = '# ------------------------ >8 ------------------------'
const CYRILLIC = /[Ѐ-ӿ]/

const path = process.argv[2]
if (!path) {
  console.error('commit-msg: no message file given')
  process.exit(1)
}

const raw = readFileSync(path, 'utf8')

// Everything below the scissors line is the diff `git commit --verbose` pastes
// in for reading; it is not part of the message.
const beforeScissors = raw.split(SCISSORS)[0]
const lines = beforeScissors
  .split(/\r?\n/)
  .filter(line => !line.startsWith('#'))

while (lines.length && lines.at(-1).trim() === '') lines.pop()

const subject = (lines[0] ?? '').trim()

// Merges, reverts and the autosquash pair are written by git itself or consumed
// by a rebase later. Their shape is not ours to dictate.
if (
  subject === ''
  || /^Merge /.test(subject)
  || /^Revert "/.test(subject)
  || /^(fixup|squash)!/.test(subject)
) process.exit(0)

const problems = []

const headline = new RegExp(`^(${TYPES.join('|')})(\\([a-z0-9./\\- ,]+\\))?!?: .+`)
if (!headline.test(subject)) {
  problems.push([
    'The subject is not a Conventional Commit.',
    `  got:      ${subject}`,
    '  expected: type(scope): what changed',
    `  types:    ${TYPES.join(', ')}`,
  ])
}
else {
  if (subject.length > SUBJECT_LIMIT) {
    problems.push([
      `The subject is ${subject.length} characters, the limit is ${SUBJECT_LIMIT}.`,
      '  GitHub cuts it off at that width in the commit list, and the part it',
      '  cuts off is the part that says what changed.',
    ])
  }
  if (subject.endsWith('.')) {
    problems.push(['The subject ends with a period. It is a title, not a sentence.'])
  }
}

if (lines.length > 1 && lines[1].trim() !== '') {
  problems.push([
    'There is no blank line between the subject and the body.',
    '  Without it git treats the whole thing as one long subject.',
  ])
}

// A trailer is not a body. `Co-Authored-By:` alone leaves the message with
// nothing that explains the change.
const isTrailer = line => /^[A-Za-z][A-Za-z-]*: .+/.test(line) || /^Signed-off-by:/i.test(line)
const body = lines.slice(1).filter(line => line.trim() !== '')
const bodyProse = body.filter(line => !isTrailer(line))

if (bodyProse.length === 0) {
  problems.push([
    'The body is empty.',
    '  CLAUDE.md: the body explains why — what problem was being solved, what',
    '  surfaced along the way, what decision got settled. Not a retelling of',
    '  the diff, and not nothing.',
  ])
}

// Cyrillic, outside code spans. Fenced blocks are skipped whole; inline spans
// are stripped per line before the test.
let inFence = false
for (const [index, line] of lines.entries()) {
  if (/^\s*```/.test(line)) {
    inFence = !inFence
    continue
  }
  if (inFence) continue

  const withoutCode = line.replace(/`[^`]*`/g, '')
  const hit = withoutCode.match(CYRILLIC)
  if (hit) {
    problems.push([
      `Cyrillic outside backticks, line ${index + 1}.`,
      `  ${line.trim()}`,
      '  The repository writes commits in English. A Russian string that is',
      '  data — a fixture name, a path, a quoted UI string — belongs in',
      '  `backticks`, and there it is allowed.',
    ])
    break
  }
}

if (problems.length === 0) process.exit(0)

console.error('')
console.error('commit-msg: the message was not accepted')
console.error('')
for (const problem of problems) {
  console.error(`  ✗ ${problem[0]}`)
  for (const line of problem.slice(1)) console.error(`  ${line}`)
  console.error('')
}
console.error('  Your text is still there:  git commit -e -F .git/COMMIT_EDITMSG')
console.error('  Skip the check on purpose: git commit --no-verify')
console.error('')

process.exit(1)
