// Хуки Claude Code для этого репозитория.
//
// Один диспетчер вместо команд в строке настроек: на Windows одна и та же
// строка по-разному ломается в PowerShell и в Git Bash, а Node здесь есть
// гарантированно.
//
// Подключается в .claude/settings.json:
//   node "$CLAUDE_PROJECT_DIR/tools/claude-hook.mjs" pre
//   node "$CLAUDE_PROJECT_DIR/tools/claude-hook.mjs" post
//
// Хук не имеет права уронить сессию: любая внутренняя ошибка гасится
// и трактуется как «замечаний нет».

import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

/** Путь относительно корня репозитория, прямыми слэшами. */
function inRepo(filePath) {
  if (!filePath) return null;
  const rel = relative(root, resolve(root, filePath)).replaceAll('\\', '/');
  return rel.startsWith('..') ? null : rel;
}

/**
 * Генерируемые файлы. Правило есть в CLAUDE.md, но правила нарушаются
 * именно тогда, когда контекст поджат, — поэтому здесь запрет, а не
 * напоминание. Правка любого из них живёт до следующей сборки.
 */
const GENERATED = [
  {
    match: (p) => p === 'apps/desktop/src/bindings.ts',
    source: 'сигнатуры команд и событий в apps/desktop/src-tauri/src/',
    rebuild: 'pnpm dev:desktop (tauri-specta генерирует файл при запуске)',
  },
  {
    match: (p) => p.startsWith('design/dist/'),
    source: 'design/tokens/tokens.css и design/styles/app.css',
    rebuild: 'pnpm design:build',
  },
  {
    match: (p) => p === 'design/preview.html' || p === 'design/screens.html',
    source: 'design/preview.src.html и design/screens.src.html',
    rebuild: 'pnpm design:build',
  },
  {
    match: (p) =>
      p === 'apps/desktop/src/styles/tokens.css' ||
      p === 'apps/desktop/src/styles/components.css',
    source: 'design/tokens/tokens.css и design/styles/app.css',
    rebuild: 'pnpm design:sync',
  },
];

const isLocale = (p) => /^apps\/desktop\/src\/i18n\/locales\/[\w-]+\.json$/.test(p);
const isDesignSource = (p) => p.startsWith('design/tokens/') || p.startsWith('design/styles/');

/** Дескриптор 0 — событие приходит на stdin одним JSON. */
function readStdin() {
  try {
    return JSON.parse(readFileSync(0, 'utf8'));
  } catch {
    return null;
  }
}

/** Пути, которые трогает вызов инструмента. */
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
        `Файл ${path} генерируется, руками не правится — правка исчезнет ` +
          `при следующей сборке. Источник: ${rule.source}. ` +
          `Пересборка: ${rule.rebuild}.`,
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
        ? output || 'Локали: паритет сохранён.'
        : `Локали разошлись, i18n:check упал:\n${output}`,
    );
  }

  if (paths.some(isDesignSource)) {
    notes.push(
      'Правка источника дизайна. В приложение она попадёт только после ' +
        '`pnpm design:check` и `pnpm design:sync` — именно в этом порядке.',
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
  // Молча: сломанный хук не должен мешать работе.
}
process.exit(0);
