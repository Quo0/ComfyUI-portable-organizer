<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Unpacking

<!-- J-01 · step 6 · US-INST-05 -->

The operation is long, so the progress is determinate and shows the current
file. You can go to another section — the unpacking will not stop. Cancelling
leaves behind no folder the app would later mistake for a working instance
`US-INST-05/AC-6`.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="steps">
      <span class="step done"><u>✓</u>Archive</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Destinations</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Shared resources</span><span class="step-sep"></span>
      <span class="step now"><u>4</u>Unpacking</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Done</span>
    </div>
    <div class="step-bar">
      <h3>Unpacking</h3>
      <span class="t-label">1 of 2</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn danger">Cancel</span>
      </span>
    </div>
    <!-- One bar per destination rather than a single combined one: with
         several targets a combined bar runs from zero to a hundred three
         times, and there is no telling from it how much work is left. -->
    <div class="prog">
      <div class="prog-head"><span>D:\AI\Flux</span><span class="count">64%</span></div>
      <div class="track"><i style="width:64%"></i></div>
      <div class="prog-file">python_embeded\Lib\site-packages\torch\_inductor\kernel\mm.py</div>
      <div class="hint">27,906 of 61,895 files · 4 GB of 9.7 GB</div>
    </div>
    <div class="prog">
      <div class="prog-head"><span>E:\AI\Flux_clean</span><span class="count">queued</span></div>
      <div class="track"><i style="width:0"></i></div>
    </div>
  </div>
</Window>

## A realistic amount of data

*scrolling · six targets; "Cancel" is always reachable*

<Window fixed scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Add build</span></div>
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
  </template>
  <div class="content">
    <div class="pinned">
      <div class="steps">
        <span class="step done"><u>✓</u>Archive</span><span class="step-sep"></span>
        <span class="step done"><u>✓</u>Destinations</span><span class="step-sep"></span>
        <span class="step done"><u>✓</u>Shared resources</span><span class="step-sep"></span>
        <span class="step now"><u>4</u>Unpacking</span><span class="step-sep"></span>
        <span class="step"><u>5</u>Done</span>
      </div>
      <div class="step-bar">
        <h3>Unpacking</h3>
        <span class="t-label">2 of 6</span>
        <span class="spacer"></span>
        <span class="acts"><span class="btn danger">Cancel</span></span>
      </div>
    </div>
    <div class="scroll"><div class="scroll-pad" style="gap:var(--space-4)">
      <div class="prog"><div class="prog-head"><span>D:\AI\Flux</span><span class="count">done</span></div><div class="track"><i style="width:100%"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Flux_clean</span><span class="count">64%</span></div><div class="track"><i style="width:64%"></i></div><div class="prog-file">python_embeded\Lib\site-packages\torch\_inductor\kernel\mm.py</div><div class="hint">27,906 of 61,895 files · 4 GB of 9.7 GB</div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\SDXL_new</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Sandbox</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\Video</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Archive_030</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\SDXL_new</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Sandbox</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\Video</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Archive_030</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\SDXL_new</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Sandbox</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\Video</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Archive_030</span><span class="count">queued</span></div><div class="track"><i style="width:0"></i></div></div>
    </div></div>
  </div>
</Window>

**A phase with no fraction done** — there is nothing to count, but saying nothing is not an option.

Preparation, clearing what an interrupted attempt left behind and registering
have no fraction done. The bar runs instead of standing at zero, a dash stands
in place of the percentage, and the name of the phase in place of the file path:
a pause with no caption and no movement reads as a hang.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="steps">
      <span class="step done"><u>✓</u>Archive</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Destinations</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Shared resources</span><span class="step-sep"></span>
      <span class="step now"><u>4</u>Unpacking</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Done</span>
    </div>
    <div class="step-bar">
      <h3>Unpacking</h3>
      <span class="t-label">1 of 2</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn danger">Cancel</span>
      </span>
    </div>
    <div class="prog">
      <div class="prog-head"><span>D:\AI\Flux</span><span class="count">—</span></div>
      <div class="track indet"><i></i></div>
      <div class="prog-file">Checking the folders and opening the archive…</div>
    </div>
    <div class="prog">
      <div class="prog-head"><span>E:\AI\Flux_clean</span><span class="count">queued</span></div>
      <div class="track"><i style="width:0"></i></div>
    </div>
  </div>
</Window>
