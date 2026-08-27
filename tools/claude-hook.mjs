// Claude Code hooks for this repository.
//
// One dispatcher instead of commands in a settings line: on Windows the same
// line breaks differently in PowerShell and in Git Bash, while Node is
// guaranteed to be here.
//
// Wired up in .claude/settings.json:
//   node "$CLAUDE_PROJECT_DIR/tools/claude-hook.mjs" pre
//   node "$CLAUDE_PROJECT_DIR/tools/claude-hook.mjs" post
//
// A hook has no right to bring down the session: any internal error is
// swallowed and treated as "nothing to report".

import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

/** Path relative to the repository root, with forward slashes. */
function inRepo(filePath) {
  if (!filePath) return null;
  const rel = relative(root, resolve(root, filePath)).replaceAll('\\', '/');
  return rel.startsWith('..') ? null : rel;
}

/**
 * Generated files. The rule is in CLAUDE.md, but rules get broken exactly when
 * context is tight — so this is a ban, not a reminder. An edit to any of them
 * lives until the next build.
 */
const GENERATED = [
  {
    match: (p) => p === 'apps/desktop/src/bindings.ts',
    source: 'command and event signatures in apps/desktop/src-tauri/src/',
    rebuild: 'pnpm dev:desktop (tauri-specta generates the file on startup)',
  },
  {
    match: (p) => p === 'apps/design/.vitepress/theme/preview-tokens.css',
    source: 'apps/desktop/src/styles/tokens.css',
    rebuild: 'pnpm design:tokens',
  },
];

const isLocale = (p) => /^apps\/desktop\/src\/i18n\/locales\/[\w-]+\.json$/.test(p);
// The source of truth for the showcase: an edit to the app's tokens does not
// reach apps/design by itself — .t-light/.t-dark are computed from this file in
// a separate step (see GENERATED above).
const isAppTokensSource = (p) => p === 'apps/desktop/src/styles/tokens.css';

/** Descriptor 0 — the event arrives on stdin as a single JSON document. */
function readStdin() {
  try {
    return JSON.parse(readFileSync(0, 'utf8'));
  } catch {
    return null;
  }
}

/** The paths a tool call touches. */
function targets(event) {
  const input = event?.tool_input ?? {};
  const raw = [input.file_path, input.notebook_path, input.path].filter(Boolean);
  return raw.map(inRepo).filter(Boolean);
}

function deny(reason) {
  process.stdout.write(
    JSON.stringify({
      hookSpecificOutput: {
        hookEventName: 'PreToolUse',
        permissionDecision: 'deny',
        permissionDecisionReason: reason,
      },
    }),
  );
  process.exit(0);
}

function context(text) {
  process.stdout.write(
    JSON.stringify({
      hookSpecificOutput: { hookEventName: 'PostToolUse', additionalContext: text },
    }),
  );
  process.exit(0);
}

function pre(event) {
  for (const path of targets(event)) {
    const rule = GENERATED.find((r) => r.match(path));
    if (rule) {
      deny(
        `The file ${path} is generated and is not edited by hand — the edit ` +
          `will disappear on the next build. Source: ${rule.source}. ` +
          `Rebuild: ${rule.rebuild}.`,
      );
    }
  }
}

function post(event) {
  const paths = targets(event);
  const notes = [];

  if (paths.some(isLocale)) {
    const run = spawnSync(process.execPath, [join(root, 'tools', 'i18n-check.mjs')], {
      cwd: root,
      encoding: 'utf8',
    });
    const output = `${run.stdout ?? ''}${run.stderr ?? ''}`.trim();
    notes.push(
      run.status === 0
        ? output || 'Locales: parity holds.'
        : `The locales have diverged, i18n:check failed:\n${output}`,
    );
  }

  if (paths.some(isAppTokensSource)) {
    notes.push(
      'App tokens edited. It will not reach the showcase (apps/design) ' +
        'until `pnpm design:tokens` is run.',
    );
  }

  if (notes.length > 0) context(notes.join('\n\n'));
}

try {
  const mode = process.argv[2];
  const event = readStdin();
  if (event) {
    if (mode === 'pre') pre(event);
    else if (mode === 'post') post(event);
  }
} catch {
  // Silently: a broken hook must not get in the way of the work.
}
process.exit(0);
