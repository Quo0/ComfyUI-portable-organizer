<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Build screen: tabs

<!-- revision · US-RUN-01 · US-REG-04 -->

Before the revision this was a single feed: launch, shared models, the build's
own models, workflows, path, versions, size, profiles, source, the edit form
and removal — one after another, about a thousand lines of markup. Finding
anything took scrolling.

The tabs separate four independent sets of data. The header with the build and
the start button is pinned: it is needed on every tab. On a wide window the
content lays out in two columns — people maximise the app on 2K, and a
720-pixel feed leaves half the window empty.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <h3>SDXL stable</h3>
      <span class="spacer"></span>
      <span class="pill stopped"><i></i>Stopped</span>
      <span class="btn primary lg">Start</span>
      <!-- The profile picker is a system list rather than a button with a
           tick: profile names are the names of .bat files, and there can
           be eight of them. -->
      <span class="input" style="width:170px">run_nvidia_gpu</span>
    </div>
    <!-- There are deliberately no counts on the tabs: both the number of
         models and the number of workflows cost a walk of the build's
         folders, and for a running one a request to its server as well.
         Paying that to decorate the header on every screen open is out. -->
    <div class="tabs">
      <span aria-selected="true">Overview</span>
      <span>Models</span>
      <span>Workflows</span>
      <span>Settings</span>
    </div>
    <div class="cols">
      <div>
        <div class="field">
          <label>Last run</label>
          <div class="paths">
            <div class="path-item"><span class="lbl">Last run: today, 14:20</span><span class="val">ready in 54 s</span></div>
            <div class="path-item"><span class="lbl">Preferred port</span><span class="val">8188</span></div>
            <div class="path-item"><span class="lbl">Launch profiles</span><span class="val">run_nvidia_gpu</span></div>
          </div>
        </div>
        <div class="field">
          <label>Startup log</label>
          <div class="row">
            <span class="btn secondary">Show the log</span>
            <span class="hint">1,482 lines · opens across the whole area</span>
          </div>
        </div>
        <div class="field">
          <label>Description</label>
          <p class="t-sm">The working build, I do not touch its nodes</p>
        </div>
      </div>
      <div>
        <div class="field">
          <label>Build folder</label>
          <div class="path-row">
            <div class="input mono"><span>D:\program_files\comfyui\SDXL</span></div>
            <span class="btn secondary">Open folder</span>
          </div>
        </div>
        <!-- Shared models are only displayed here. The toggle lives on the
             "Models" tab and in exactly one place: two switches for the
             same thing drift apart. -->
        <div class="paths">
          <div class="path-item"><span class="lbl">ComfyUI version</span><span class="val">0.30.2</span></div>
          <div class="path-item"><span class="lbl">Python version</span><span class="val">3.13.12</span></div>
          <div class="path-item"><span class="lbl">Size on disk</span><span class="val">52.4 GB</span></div>
          <div class="path-item"><span class="lbl">Launch profiles</span><span class="val">4 + 1 custom</span></div>
          <div class="path-item"><span class="lbl">Shared models</span><span class="val">D:\AI\_shared\models · build folder untouched</span></div>
        </div>
        <div class="row">
          <span class="hint">measured 12 minutes ago</span>
          <span class="btn ghost">Recount</span>
          <span class="btn ghost">Edit arguments</span>
        </div>
        <div class="src">Unpacked from ComfyUI_windows_portable_nvidia.7z, 12 March</div>
      </div>
    </div>
  </div>
</Window>

## The log opened

*the same tab, with the button pressed*

The log does not hang as a 260-pixel block in the middle of the scroll, and it
is not always shown: a button opens it across the whole data area and folds it
back. At startup it opens by itself — there it is the content of the screen.

<Window fixed>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL</em><i class="dot" style="background:var(--state-starting)"></i></div>
    </nav>
  </template>
  <div class="content framed no-foot">
    <div class="pinned">
      <div class="row">
        <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
        <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
        <h3>SDXL stable</h3>
        <span class="spacer"></span>
        <span class="pill starting"><i></i>Starting</span>
        <span class="btn primary lg">Open ComfyUI</span>
        <span class="btn secondary">Restart</span>
        <span class="btn danger">Stop</span>
      </div>
      <!-- The fraction done is unknown: a start with a large set of nodes
           takes minutes, and the bar only says the process is alive. -->
      <div class="track indet"><i></i></div>
      <!-- The tabs stay where they are: the log takes over the data area
           rather than replacing the whole screen. -->
      <div class="tabs">
        <span aria-selected="true">Overview</span>
        <span>Models</span>
        <span>Workflows</span>
        <span>Settings</span>
      </div>
      <div class="row">
        <span class="t-label">Startup log</span>
        <span class="hint">1,482 lines</span>
        <span class="spacer"></span>
        <span class="btn secondary">Hide the log</span>
      </div>
    </div>
    <div class="data">
      <div class="log">
        <div class="console">Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Loading custom nodes: 47 found
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
<span class="dim">Loading model  ████████████░░░░░░░  61%  4.2/6.9 GB</span></div>
        <span class="log-follow">To the latest lines <span class="n">+128</span></span>
      </div>
    </div>
  </div>
</Window>
