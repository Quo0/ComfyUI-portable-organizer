<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# The folder already exists

<!-- J-02 · steps 3-4 · US-REG-01 · US-REG-02 -->

The path of the actor whose build already works. The app shows what it has read
— the version, the interpreter and the launch profiles — and promises to touch
nothing inside. That is the main fear of such a user, so the promise stands on
the screen rather than in the documentation.

The step row is the same as in the wizard: both paths of "Add build" start
alike. "Back" leads to "Add build" rather than to the build list — this screen
is only reached from there — and it is the only way out of it.

The primary action stands where it does throughout the section — in the step
row, to the right of "Back". Under the form it scrolled out of sight along with
what had been read about the folder. Until a folder is chosen there is nothing
to add, so the button is disabled rather than hidden: a button that disappears
does not tell you how the screen ends. It vanishes only where the action is a
different one — when the folder is already on the list.

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
    <div class="step-bar">
      <h3>Add an instance</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Back</span><span class="btn primary lg" aria-disabled="true">Add to the list</span></span>
    </div>
    <p class="t-sm">Choose the folder of a portable ComfyUI build. Nothing inside it will be changed.</p>
    <div class="field">
      <span class="t-label">Build folder</span>
      <div class="path-row">
        <div class="input mono"><span></span></div>
        <span class="btn secondary">Choose folder…</span>
      </div>
    </div>
  </div>
</Window>

**The folder is chosen and read** — what was read on the left, what is filled in on the right.

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
    <div class="step-bar">
      <h3>Add an instance</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Back</span><span class="btn primary lg">Add to the list</span></span>
    </div>
    <p class="t-sm">Choose the folder of a portable ComfyUI build. Nothing inside it will be changed.</p>
    <div class="field">
      <span class="t-label">Build folder</span>
      <div class="path-row">
        <div class="input mono"><span>D:\program_files\comfyui\ComfyUI_windows_portable</span></div>
        <span class="btn secondary">Choose folder…</span>
      </div>
    </div>
    <!-- What was read on the left, what is filled in on the right: in a
         single scroll what was read pushed the form down, past the edge
         of the screen. -->
    <div class="cols">
      <div>
        <div class="paths">
          <div class="path-item"><span class="lbl">ComfyUI version</span><span class="val">0.30.2</span></div>
          <div class="path-item"><span class="lbl">Python version</span><span class="val">3.13.12</span></div>
          <div class="path-item"><span class="lbl">Launch profiles</span><span class="val">4</span></div>
        </div>
        <div class="row">
          <span class="pill stopped">nvidia_gpu</span><span class="pill stopped">fast_fp16</span>
          <span class="pill stopped">cpu</span><span class="pill stopped">disable_api_nodes<em class="advanced">advanced</em></span>
        </div>
      </div>
      <div>
        <div class="field"><label>Name</label><div class="input">SDXL stable</div></div>
        <div class="field"><label>Description</label><div class="input"></div></div>
        <div class="field">
          <span class="t-label">Accent colour</span>
          <div class="picker">
            <i class="on" style="background:var(--accent-teal)"></i><i style="background:var(--accent-indigo)"></i>
            <i style="background:var(--accent-ember)"></i><i style="background:var(--accent-moss)"></i>
            <i style="background:var(--accent-azure)"></i><i style="background:var(--accent-orchid)"></i>
            <i style="background:var(--accent-rose)"></i><i style="background:var(--accent-amber)"></i>
            <span class="swatch-custom" title="Pick your own colour"></span>
          </div>
        </div>
        <div class="field"><label>Preferred port</label><div class="input num">8188</div><div class="hint">Used at startup when it is free.</div></div>
        <!-- No buttons under the form: the primary action moved into the
             step row — where it is throughout the section — and at the
             bottom it scrolled out of sight along with what had been read
             about the folder. -->
      </div>
    </div>
  </div>
</Window>

**The folder does not look like a build** — the error is in place rather than as a toast: it is about this folder.

The most common slip is choosing the folder one level above or below the right
one. The app does not simply refuse but names the one it meant: telling
`ComfyUI_windows_portable` from the nested `ComfyUI` is impossible from the
outside.

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
    <div class="step-bar">
      <h3>Add an instance</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Back</span><span class="btn primary lg" aria-disabled="true">Add to the list</span></span>
    </div>
    <p class="t-sm">Choose the folder of a portable ComfyUI build. Nothing inside it will be changed.</p>
    <div class="field">
      <span class="t-label">Build folder</span>
      <div class="path-row">
        <div class="input mono"><span>D:\program_files\comfyui</span></div>
        <span class="btn secondary">Choose folder…</span>
      </div>
      <p class="hint bad">Almost — the build is one level down. Choose D:\program_files\comfyui\ComfyUI_windows_portable.</p>
    </div>
  </div>
</Window>

**The folder is already on the list** — no second build will come out of it.

Registering the same folder again does not create a second build: the form is
not shown at all, and in its place comes a way through to the one already
created `US-REG-02`.

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
    <div class="step-bar">
      <h3>Add an instance</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Back</span></span>
    </div>
    <p class="t-sm">Choose the folder of a portable ComfyUI build. Nothing inside it will be changed.</p>
    <div class="field">
      <span class="t-label">Build folder</span>
      <div class="path-row">
        <div class="input mono"><span>D:\program_files\comfyui\ComfyUI_windows_portable</span></div>
        <span class="btn secondary">Choose folder…</span>
      </div>
    </div>
    <div class="group">
      <span class="btn primary">This folder is already on the list. Open it</span>
    </div>
  </div>
</Window>
