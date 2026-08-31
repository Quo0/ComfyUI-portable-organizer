<script setup lang="ts">
withDefaults(
  defineProps<{
    /** A fixed-height window — the only way real scrolling appears. */
    fixed?: boolean;
    /** A frame the size of a real monitor, scaled down to the page width. */
    hd?: boolean;
    /**
     * The frame deliberately proves real scrolling: there is more data inside
     * than fits. Not the same as `fixed`, which can be there just for a fixed
     * height (to compute the scale of an `hd` frame, say) with no claim that
     * anything scrolls. `tools/check-styles.mjs` counts frames carrying this
     * prop and demands proven data density from them.
     */
    scroll?: boolean;
    theme?: 'light' | 'dark';
    title?: string;
  }>(),
  { fixed: false, hd: false, scroll: false, theme: 'dark', title: 'ComfyUI Portable Organizer' },
);
</script>

<template>
  <var class="Window">
    <div class="frame" :class="{ hd }" :data-scroll="scroll ? '' : null">
      <div class="win" :class="[`t-${theme}`, { fixed, hd }]">
        <div class="titlebar">
          <span>{{ title }}</span>
          <span class="tb-btns"><span>—</span><span>▢</span><span>✕</span></span>
        </div>
        <div class="win-body">
          <slot name="nav" />
          <slot />
        </div>
      </div>
    </div>
  </var>
</template>

<style scoped>
/* The window chrome is not a component of the app — it is the idea "a window
   is drawn here" — so it lives with the component that embodies it rather
   than in the shared style file. */
.frame { overflow-x: auto; padding-bottom: 4px; }
.frame.hd {
  --hd: .55;
  overflow: hidden;
  height: calc(1080px * var(--hd));
}
.frame.hd > .win { transform: scale(var(--hd)); transform-origin: top left; }
@media (max-width: 1180px) {
  .frame.hd { --hd: .45; }
}

/* The width needs a ceiling as well as a floor. `min-width` says "there is
   nothing left to show"; without a maximum the frame stretched across the
   whole screen, and the toolbar with a `.spacer` in the middle empties out on
   a wide monitor until it looks broken. 1440px is the same number as
   `.screen-pad.wide` in the app itself. */
.win {
  min-width: 940px;
  max-width: 1440px;
  border: 1px solid var(--page-line);
  border-radius: 8px;
  overflow: hidden;
  background: var(--ground);
  color: var(--ink);
  font-family: var(--font-ui);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
}
.win.fixed { height: 560px; display: grid; grid-template-rows: auto minmax(0, 1fr); }
/* `hd` is a frame the size of a real 1920 monitor, scaled down to the page
   width: the ceiling above would contradict the point of the prop, so it is
   lifted here. */
.win.fixed.hd { width: 1920px; max-width: none; height: 1080px; }

.titlebar {
  height: 30px; display: flex; align-items: center; gap: var(--space-2);
  padding: 0 var(--space-3);
  background: var(--surface-sunken); border-bottom: 1px solid var(--line);
  font-size: var(--text-xs); color: var(--ink-muted);
}
.tb-btns { margin-left: auto; display: flex; gap: 16px; color: var(--ink-muted); font-size: 11px; }

.win-body { display: grid; grid-template-columns: auto minmax(0, 1fr); min-height: 430px; }
.win.fixed :deep(.win-body) { min-height: 0; overflow: hidden; }
.win.fixed :deep(.content) { min-height: 0; }

/* Slot content carries the parent's markup — the screen's .md page — so the
   selectors reach it through :deep. */
:deep(.nav.in-win) { border: 0; border-right: 1px solid var(--line); border-radius: 0; height: 100%; min-height: 0; }
:deep(.content) { padding: var(--space-4); min-width: 0; display: flex; flex-direction: column; gap: var(--space-4); position: relative; }
:deep(.content.flush) { padding: 0; }
/* The screen shell for frames with real scrolling: a pinned head, the data
   area, a pinned foot. Same nature as .win/.frame above. */
:deep(.content.framed) { display: grid; grid-template-rows: auto minmax(0, 1fr) auto; gap: var(--space-3); container-type: inline-size; }
:deep(.content.framed.no-foot) { grid-template-rows: auto minmax(0, 1fr); }
:deep(.screen-foot) { display: flex; gap: var(--space-2); justify-content: flex-end; border-top: 1px solid var(--line); padding-top: var(--space-3); }
:deep(.data) { min-height: 0; display: flex; flex-direction: column; }
:deep(.data > .scroll) { border: 1px solid var(--line); border-radius: var(--radius-md); }

/* Where the toast stack sits in the mock window: the corner of the content
   area, where the user will see it. The toast itself is a design-system
   component; only the app shell knows where to put it, and that placement is
   reproduced here with positioning alone. */
:deep(.win-toasts) {
  position: absolute; z-index: 2;
  right: var(--space-4); bottom: var(--space-4); width: 320px;
}

:deep(.comfy) {
  min-height: 340px;
  background:
    radial-gradient(circle at 1px 1px, var(--line-strong) 1px, transparent 0) 0 0/22px 22px,
    var(--surface-sunken);
  position: relative;
  display: grid; place-items: center;
}
:deep(.comfy-node) {
  position: absolute; width: 132px;
  border: 1px solid var(--line-strong); border-radius: var(--radius-sm);
  background: var(--surface);
  font-size: var(--text-xs);
}
:deep(.comfy-node > b) { display: block; padding: 3px 7px; border-bottom: 1px solid var(--line); font-weight: var(--weight-medium); }
:deep(.comfy-node > span) { display: block; padding: 5px 7px; color: var(--ink-muted); font-family: var(--font-mono); font-size: 10px; }
:deep(.comfy-wire) { position: absolute; height: 1.5px; background: var(--accent-teal); opacity: .75; }
:deep(.comfy-label) {
  font-size: var(--text-xs); color: var(--ink-muted);
  background: var(--ground); border: 1px dashed var(--line-strong);
  padding: 4px 10px; border-radius: var(--radius-pill);
}
</style>
