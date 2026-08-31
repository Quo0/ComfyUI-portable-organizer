<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Scrolling

<!-- NFR-420 · NFR-430 · NFR-440 -->

The window has a fixed height, the data does not. What is pinned and what moves away is the architecture of a screen.

There is one rule: **the window never scrolls as a whole**. The rail is pinned,
and inside the content area there is exactly one vertical scroll — the data
area, with everything that controls it lifted out. A wizard footer that scrolls
away makes the wizard impossible to finish; a details panel that scrolls away
makes the selection invisible. Nested scrolling is allowed only in the
master-detail layout: two independent regions side by side.

### The frame: pinned header → data area → pinned footer

<ThemePair>
  <div class="pane" style="height:220px">
    <div class="pane-head"><span class="title">Destinations</span><span class="t-label">6</span></div>
    <div class="scroll"><div class="scroll-pad">
      <div class="path-item"><span class="lbl">D:\AI\Flux</span><span class="val">Flux test</span></div>
      <div class="path-item"><span class="lbl">E:\AI\Flux_clean</span><span class="val">Flux clean</span></div>
      <div class="path-item"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span></div>
      <div class="path-item"><span class="lbl">E:\AI\Sandbox</span><span class="val">Sandbox</span></div>
      <div class="path-item"><span class="lbl">D:\AI\Video</span><span class="val">Video</span></div>
      <div class="path-item"><span class="lbl">E:\AI\Archive_030</span><span class="val">Archive 0.30</span></div>
    </div></div>
    <div class="pane-foot"><span class="btn ghost">Back</span><span class="btn primary">Next</span></div>
  </div>
</ThemePair>

### List and details — two independent scrolls

<ThemePair>
  <div class="split-master" style="height:220px">
    <div class="pane">
      <div class="pane-head"><span class="title">Workflows</span><span class="t-label">214</span></div>
      <div class="scroll"><div class="scroll-pad" style="gap:1px">
        <div class="wf-row"><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span><span class="star">★</span></div>
        <div class="wf-row"><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span><span class="star">★</span></div>
        <div class="wf-row"><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span><span></span></div>
        <div class="wf-row"><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span><span></span></div>
        <div class="wf-row"><span class="nm">utils / batch-rename.json</span><span class="tags"></span><span></span></div>
        <div class="wf-row"><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span><span></span></div>
      </div></div>
    </div>
    <div class="pane">
      <div class="pane-head"><span class="title">Selected</span></div>
      <div class="scroll"><div class="scroll-pad">
        <div class="t-sm">flux / portrait-v3.json</div>
        <div class="t-label">Note</div>
        <p class="t-sm" style="margin:0">A portrait with two LoRAs and an upscale. Only works where IPAdapter is installed.</p>
        <div class="t-label">Compatibility</div>
        <div class="compat-note" style="color:var(--state-starting)">2 nodes missing in “Flux test”</div>
      </div></div>
      <div class="pane-foot"><span class="btn primary">Add</span></div>
    </div>
  </div>
</ThemePair>

Scrolling the list does not move the details panel, and the other way round. In
a narrow window the layout collapses into a single column and the details become
a screen of their own — otherwise there is no room left for a 300&nbsp;px panel.

### The console: following the tail is paused

While the user is at the bottom, the console follows the new lines. The moment
they scroll up, following is **paused** — otherwise the log yanks the text out
from under the cursor on every new line. The return button shows how many lines
have piled up.

<ThemePair>
  <div class="log" style="height:180px">
    <div class="console">Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Loading custom nodes: 47 found
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
<span class="dim">  ... 43 more</span>
Import times for custom nodes:
   0.1 seconds: ComfyUI-Manager
   2.4 seconds: was-node-suite-comfyui
Starting server on 127.0.0.1:8188</div>
    <span class="log-follow">To the latest lines <span class="n">+128</span></span>
  </div>
</ThemePair>

### The rail: sections pinned, running instances scroll

<ThemePair>
  <nav class="nav" style="height:240px">
    <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
    <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
    <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
    <div class="nav-item"><Info class="ico" /><span>About</span></div>
    <div class="nav-sep"></div>
    <div class="nav-note">Running · 8</div>
    <div class="nav-runs">
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL stable</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux test</em><i class="dot" style="background:var(--state-starting)"></i></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Animation</em><span class="badge">!</span></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-moss)">E</span><em>Experiments</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-azure)">V</span><em>Video</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-orchid)">I</span><em>Inpaint</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-rose)">U</span><em>Upscale</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-amber)">N</span><em>Node tests</em><i class="dot" style="background:var(--state-running)"></i></div>
    </div>
  </nav>
</ThemePair>

<div class="longform">
  <div class="lf-head">Mechanics that are easy to get wrong</div>
  <div class="lf-rows">
    <div class="lf-row"><b>1</b><span><code>min-height: 0</code> on every link of the chain — otherwise a grid item will not shrink below its content and no scroll appears at all</span></div>
    <div class="lf-row"><b>2</b><span><code>overscroll-behavior: contain</code> — otherwise scrolling the list to its end starts scrolling the page behind it</span></div>
    <div class="lf-row"><b>3</b><span>Pinning through <code>position: sticky</code> inside the panel, not <code>fixed</code> — that one would attach to the window</span></div>
    <div class="lf-row"><b>4</b><span>The scrollbar comes from the tokens: a light system one looks alien in a dark interface</span></div>
  </div>
</div>
