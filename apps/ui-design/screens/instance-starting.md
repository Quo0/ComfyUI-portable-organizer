<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Instance starting

<!-- J-01 · step 8 · US-RUN-03 -->

A cold start with a large set of nodes takes minutes, and that is normal. The
logs stream in real time, because otherwise those first minutes look like a
hang. The model-loading line is redrawn in place rather than breeding thousands
of lines.

The screen is the same as for a stopped build — with its own header and tabs.
Two things differ: the log opens by itself, because in these minutes it is the
content of the screen, and under the header runs a bar with no fraction done.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Running</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux test</em><i class="dot" style="background:var(--state-starting)"></i></div>
    </nav>
  </template>
  <div class="content">
    <!-- There are no breadcrumbs in the app: the path is exactly one level
         deep, and the "Back" button is what returns. -->
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
      <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
      <h3>Flux test</h3>
      <span class="spacer"></span>
      <span class="pill starting"><i></i>Starting</span>
      <!-- The real port was assigned at startup: the preferred 8188 is taken
           by a neighbouring build. On the overview, under the "Preferred
           port" caption, it stays 8188 — they diverge only here. -->
      <span class="t-mono">:8189</span>
      <span class="btn primary lg">Open ComfyUI</span>
      <span class="btn secondary">Restart</span>
      <span class="btn danger">Stop</span>
    </div>
    <div class="track indet"><i></i></div>
    <div class="tabs">
      <span aria-selected="true">Overview</span>
      <span>Models</span>
      <span>Workflows</span>
      <span>Settings</span>
    </div>
    <div class="row">
      <span class="t-label">Startup log</span>
      <span class="hint">312 lines</span>
      <span class="spacer"></span>
      <span class="btn secondary">Hide the log</span>
    </div>
    <div class="console">D:\AI\Flux&gt;.\python_embeded\python.exe -s ComfyUI\main.py --port 8189 --disable-auto-launch
Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Loading custom nodes: 47 found
<span class="dim">Loading model  ████████████░░░░░░░  61%  4.2/6.9 GB</span></div>
  </div>
</Window>

## A realistic amount of data

*scrolling · fifteen hundred lines; following is paused*

<Window fixed scroll>
  <template #nav>
    <nav class="nav in-win">
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
  </template>
  <div class="content framed no-foot">
    <div class="pinned">
      <div class="row">
        <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
        <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
        <h3>Flux test</h3>
        <span class="spacer"></span>
        <span class="pill starting"><i></i>Starting</span>
        <span class="t-mono">:8189</span>
        <span class="btn primary lg">Open ComfyUI</span>
        <span class="btn danger">Stop</span>
      </div>
      <div class="track indet"><i></i></div>
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
      <!-- Fifteen hundred lines of a startup: forty are enough for the log
           not to fit the area and for the scrollbar to be real. -->
      <div class="log">
        <div class="console">Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Using pytorch attention
Loading custom nodes: 47 found
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
Import times for custom nodes:
   0.1 seconds: ComfyUI-Manager
   2.4 seconds: was-node-suite-comfyui</div>
        <span class="log-follow">To the latest lines <span class="n">+128</span></span>
      </div>
    </div>
  </div>
</Window>
