<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, X } from '@lucide/vue';
</script>

# Launch arguments

<!-- US-RUN-01 -->

A separate route, not a modal. Z-order discipline demands that everywhere, but
there is a reason of its own here: this screen is where the final command is
read, and a command is not something you study in a popover.

The build's own `.bat` files are never modified `US-RUN-01/AC-8`. An edit is
saved as a profile of your own on top of one of them: the `.bat` is re-parsed
on every launch, and there would be nowhere inside it to hold the edit.

The screen is linear: you came from the build screen, saved, and went back. So
"Back" here is not navigation at the left edge but a step back next to the
action — as in the wizard and when adding an instance. At the bottom, "Save"
scrolled out of sight the more surely the more arguments were edited: their
list scrolls beneath it.

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
    <div class="step-bar">
      <h3 class="title">Launch arguments</h3>
      <span class="hint">SDXL stable</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
        <span class="btn primary lg">Save</span>
      </span>
    </div>
    <p class="t-sm">The build's own .bat files are never modified. An edit is saved as a profile of your own on top of one of them.</p>
    <div class="group">
      <span class="t-label">Your profiles</span>
      <div class="wf-list">
        <div class="wf-row">
          <span class="nm">run_nvidia_gpu +</span>
          <span class="hint">run_nvidia_gpu.bat</span>
          <span class="btn ghost">Edit</span>
          <span class="btn ghost">Remove from the list</span>
        </div>
      </div>
    </div>
    <div class="two">
      <div class="field">
        <label>Based on</label>
        <span class="input">advanced\run_nvidia_gpu_disable_api_nodes.bat</span>
      </div>
      <div class="field">
        <label>Profile name</label>
        <span class="input">run_nvidia_gpu_disable_api_nodes +</span>
      </div>
    </div>
    <div class="group">
      <span class="t-label">Arguments</span>
      <!-- The same cross that removes a row in the wizard's list of
           destinations: one operation — one icon. -->
      <div class="path-row">
        <span class="input mono">-s</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="path-row">
        <span class="input mono">..\ComfyUI\main.py</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="path-row">
        <span class="input mono">--windows-standalone-build</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="path-row">
        <span class="input mono">--disable-api-nodes</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="row">
        <span class="btn secondary">Add argument</span>
        <span class="btn ghost">Reset to the .bat</span>
      </div>
    </div>
    <div class="group">
      <span class="t-label">The command that will actually run</span>
      <!-- The preview has no scroll of its own: one scroll area per screen,
           and people come here to read the command in full. -->
      <div class="console preview">D:\program_files\comfyui\SDXL\python_embeded\python.exe -s ..\ComfyUI\main.py --windows-standalone-build --disable-api-nodes --port 8188 --disable-auto-launch</div>
      <p class="hint">The port and --disable-auto-launch are added by the app. The real port is assigned at startup, so the one shown is the preferred one.</p>
    </div>
  </div>
</Window>
