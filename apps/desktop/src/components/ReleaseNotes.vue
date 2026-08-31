<script setup lang="ts">

// The release notes of an update: the new version's `CHANGELOG.md` section,
// carried inside `latest.json`. It arrives as markdown, and the four
// constructs a changelog actually uses are rendered — headings, bullet lists,
// bold and inline code. Everything else is printed as written: notes must
// never come out empty or mangled because a future changelog reached for a
// table or a nested list.
//
// The text is parsed into a structure and rendered with `v-for`, never with
// `v-html`. `latest.json` comes over the network and only the installer bytes
// are signature-checked — the manifest itself is not — so interpolating it as
// HTML would be an injection path, not a shortcut.

import { computed, Fragment, h } from 'vue';

const { text } = defineProps<{ text: string }>();

type Inline = { text: string; bold?: boolean; code?: boolean };

type Block =
  | { kind: 'head'; parts: Inline[] }
  | { kind: 'para'; parts: Inline[] }
  | { kind: 'list'; items: Inline[][] };

const blocks = computed(() => parse(text));

/**
 * Splits a line into runs of plain text, `**bold**` and `` `code` ``.
 *
 * A `[text](url)` link keeps its text: the panel opens nothing, and a bare URL
 * in the middle of a sentence reads worse than the words it was written for.
 */
function inlines(line: string): Inline[] {
  const flat = line.replace(/\[([^\]]+)\]\([^)]*\)/g, '$1');
  const parts: Inline[] = [];
  const re = /\*\*([^*]+)\*\*|`([^`]+)`/g;
  let at = 0;

  for (let m = re.exec(flat); m; m = re.exec(flat)) {
    if (m.index > at) parts.push({ text: flat.slice(at, m.index) });
    if (m[1] !== undefined) parts.push({ text: m[1], bold: true });
    else parts.push({ text: m[2]!, code: true });
    at = m.index + m[0].length;
  }

  if (at < flat.length) parts.push({ text: flat.slice(at) });

  return parts;
}

/**
 * Lines into blocks.
 *
 * **Soft wraps are joined.** `CHANGELOG.md` wraps at 80 columns, and the panel
 * is of a different width: without gluing a continuation line back onto its
 * paragraph or list item, every wrap of the source would show as a line break
 * in the middle of a sentence.
 *
 * A nested list is flattened into the one above it — one level is all this
 * block has room for, and the alternative is a stray `-` in the text.
 */
function parse(source: string): Block[] {
  const blocks: Block[] = [];

  let para: string[] = [];
  let list: string[][] | null = null;

  const flushPara = (): void => {
    if (!para.length) return;
    blocks.push({ kind: 'para', parts: inlines(para.join(' ')) });
    para = [];
  };

  const flushList = (): void => {
    if (!list) return;
    blocks.push({ kind: 'list', items: list.map((item) => inlines(item.join(' '))) });
    list = null;
  };

  for (const raw of source.replace(/\r\n/g, '\n').split('\n')) {
    const line = raw.trim();

    if (!line) {
      flushPara();
      flushList();
      continue;
    }

    const head = /^#{1,6}\s+(.+)$/.exec(line);
    if (head) {
      flushPara();
      flushList();
      blocks.push({ kind: 'head', parts: inlines(head[1]!) });
      continue;
    }

    const bullet = /^[-*]\s+(.+)$/.exec(line);
    if (bullet) {
      flushPara();
      list ??= [];
      list.push([bullet[1]!]);
      continue;
    }

    // Not a bullet and not blank: a continuation of whatever is open.
    if (list) list[list.length - 1]!.push(line);
    else para.push(line);
  }

  flushPara();
  flushList();

  return blocks;
}

/**
 * The inline runs of one line. A component rather than a repeated template
 * fragment: paragraphs, headings and list items all need the same three cases.
 */
function Line(props: { parts: Inline[] }) {
  return h(
    Fragment,
    props.parts.map((part) => {
      if (part.bold) return h('b', part.text);
      if (part.code) return h('code', { class: 't-mono' }, part.text);
      return part.text;
    }),
  );
}
</script>

<template>
  <var class="ReleaseNotes">
    <template v-for="(block, i) in blocks" :key="i">

      <h4 v-if="block.kind === 'head'" class="t-label">
        <Line :parts="block.parts" />
      </h4>

      <ul v-else-if="block.kind === 'list'">
        <li v-for="(item, j) in block.items" :key="j">
          <Line :parts="item" />
        </li>
      </ul>

      <p v-else>
        <Line :parts="block.parts" />
      </p>
    </template>
  </var>
</template>
