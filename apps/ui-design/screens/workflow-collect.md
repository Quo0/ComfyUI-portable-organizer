<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, FolderOpen, RotateCw } from '@lucide/vue';
</script>

# Taking a workflow into the library

<!-- J-05 · step 2 · US-WF-03 -->

Taking one **moves** it: the file leaves the build and stays only in the
library. That is said in a line under the list rather than learned from a row
that vanished. The order is the same as for moving models — the copy is
written, read back and compared, and only then is the source removed from the
build.

There are two marks, and that is the main thing here. Going by name alone the
mark would lie: a version edited inside the build, under a taken name, would
look as if it were saved. So the contents are compared in full — bytes first,
then the parsed JSON, otherwise a workflow re-saved in ComfyUI would be
declared diverged over indentation alone.

The button is off only for what already lies in the library as the very same
file. For a diverged one it works and takes it under a free name: there is no
replacing at all, because replacing would mean erasing one piece of work with
another, leaving neither.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Running</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL stable</em><i class="dot" style="background:var(--state-running)"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <h3>SDXL stable</h3>
      <span class="spacer"></span>
      <span class="pill running"><i></i>Running</span>
      <span class="t-mono">:8188</span>
      <span class="btn primary lg">Open ComfyUI</span>
      <span class="btn secondary">Restart</span>
      <span class="btn danger">Stop</span>
    </div>
    <div class="tabs">
      <span>Overview</span>
      <span>Models</span>
      <span aria-selected="true">Workflows</span>
      <span>Settings</span>
    </div>
    <div class="row">
      <span class="t-label">Workflows in this build</span>
      <!-- The same icon in the same place as on the build's models: the path
           to the workflows folder is shown nowhere else in the panel, and
           sorting things out by hand means going exactly there. -->
      <span class="btn ghost icon" title="D:\AI\ComfyUI_windows_portable\ComfyUI\user\default\workflows"><FolderOpen class="ico" /></span>
      <span class="spacer"></span>
      <span class="btn ghost"><RotateCw class="ico" />Refresh</span>
    </div>
    <!-- Neither a checkbox for bulk operations nor a favourite star: the
         checkbox belongs to things that get acted on in bulk, and favourites
         live in the library manifest — these files have not got there yet.
         Hence a row grid of its own. -->
    <div class="wf-list of-instance">
      <!-- The name is not in the library — taken without further ado. -->
      <div class="wf-row">
        <span class="nm">sdxl / img2img-refine.json</span>
        <span class="tags"></span>
        <span class="btn ghost">Take into the library</span>
      </div>
      <!-- The same file: nothing to take, the button is rightly off. -->
      <div class="wf-row">
        <span class="nm">flux / portrait-v3.json</span>
        <span class="tags"><span class="tag">in the library</span></span>
        <span class="btn ghost" aria-disabled="true">Take into the library</span>
      </div>
      <!-- The name is taken by someone else's work. The button works and will
           offer "base-upscale (2).json" — but it asks before it moves. -->
      <div class="wf-row">
        <span class="nm">sdxl / base-upscale.json</span>
        <span class="tags"><span class="tag warn">name taken, different file</span></span>
        <span class="btn ghost">Take into the library</span>
      </div>
    </div>
    <p class="hint">Taking a workflow moves it: the file leaves the build and stays only in the library.</p>
    <!-- A legend of the marks: on the left the same mark as in the row, on
         the right what it means. It used to be a paragraph, and the mark had
         to be hunted for by eye inside it. -->
    <dl class="tag-legend">
      <dt><span class="tag">in the library</span></dt>
      <dd>— the library already has this very workflow: there is nothing to take, the button is off.</dd>
      <dt><span class="tag warn">name taken, different file</span></dt>
      <dd>— the contents have diverged: these are two different pieces of work, and the one from the build is taken under a free name, not over someone else's.</dd>
    </dl>
    <p class="hint">The list comes from the running build, so it includes what it saved a minute ago.</p>
  </div>
</Window>
