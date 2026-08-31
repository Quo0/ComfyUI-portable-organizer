<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, FolderOpen, RotateCw, ChevronRight } from '@lucide/vue';
</script>

# Connecting shared models

<!-- J-03 · step 4 · US-SHARED-03 · US-SHARED-05 -->

The warning about restarting is mandatory: the config is read only at startup,
and without it the user decides the feature is broken `US-SHARED-03/AC-7`. It
stands right under the toggle — where they have just clicked — rather than as a
separate strip at the top of the screen. The default way of applying writes
nothing into the build folder.

The toggle lives on the "Models" tab and in exactly one place. On "Overview"
the connection is only shown as a line: two switches for the same thing
inevitably drift apart.

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
      <span aria-selected="true">Models</span>
      <span>Workflows</span>
      <span>Settings</span>
    </div>
    <!-- The tab is two sections: the shared folder and the build's own
         models. Their headings have one shape — caption, icon of their
         folder, then the content: otherwise the top of the tab read as a
         set of scattered lines with no border between the sections. -->
    <div class="row">
      <span class="t-label">Shared models</span>
      <!-- The same folder icon as on the build's models below, but it leads
           to the shared one: sorting out by hand what has already been
           moved is done exactly there. -->
      <span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span>
    </div>
    <!-- The toggle gets a line of its own, and the caption to its right says
         what it does. The section's name went into the heading, and there is
         no point repeating it next to the toggle. -->
    <div class="toggle-row">
      <span class="toggle"></span>
      <div>
        <div class="t-base">Give this build access to the shared folder without copying anything into it.</div>
        <div class="hint">D:\AI\_shared\models · build folder untouched</div>
      </div>
    </div>
    <div class="group">
      <span class="t-label">How to apply</span>
      <div class="seg">
        <span aria-pressed="true">Leave the build alone</span>
        <span>Write a file into the build</span>
      </div>
      <!-- The hint names the app explicitly: "the app" read as both the
           organizer and the ComfyUI build itself. And it states the price of
           the choice: no file in the build, but then the shared models are
           only there when it is started from here. -->
      <p class="hint">The list of shared folders stays with ComfyUI Portable Organizer and is handed to the build as a startup argument. Nothing is written into the build folder — but the shared models are only there when this app starts the build.</p>
    </div>
    <p class="hint bad">The build reads this setting only when it starts. Restart it for the change to take effect.</p>
    <!-- Moving the build's models into the shared folder is the second half
         of the same tab: connecting and moving are different actions, and
         they have to be done one after the other. -->
    <div class="row">
      <span class="t-label">Models of this build</span>
      <!-- The same folder icon as in "Installer archives": the path to the
           models is shown nowhere else in the panel, and sorting things out
           by hand means going exactly there. -->
      <span class="btn ghost icon" title="D:\AI\ComfyUI_windows_portable\ComfyUI\models"><FolderOpen class="ico" /></span>
      <span class="spacer"></span>
      <!-- "Refresh" with the reload icon: the button re-reads the build's
           folders rather than retrying a failed action. -->
      <span class="btn ghost"><RotateCw class="ico" />Refresh</span>
    </div>
    <!-- One list for everything: a category expands and the models inside
         are shown with their verdicts. Duplicates and "name matched" used to
         be separate lists at the bottom, and the link back to the category
         had to be rebuilt from the name in the row.
         The toggle stands on every model: one LoRA out of twenty may be
         needed locally by the build. On a category it governs the whole
         content and can show a mixed state. A row with foreign content has
         no toggle at all — it is never moved and never removed.
         The mark on the category stays for the collapsed view: otherwise
         such a build looked like a dead end — rows in place, and zero files
         in the summary. -->
    <div class="cats">
      <div class="cat marked">
        <button class="disclose" aria-expanded="true"><ChevronRight class="ico" /><code>checkpoints</code></button>
        <span class="n">6 items · 38.2 GB</span><span class="tag warn">2 already shared</span><span class="toggle mixed"></span>
      </div>
      <div class="cat model"><code>model-a.safetensors</code><span class="n">7.4 GB</span><span class="toggle"></span></div>
      <!-- The verdict has a hover hint: two words do not show what it rests
           on, and the difference matters. For a file the bytes were
           compared, for a folder only the size and the number of files
           inside, and "looks the same" is exactly about that. -->
      <div class="cat model marked"><code>sd_xl_base_1.0.safetensors</code><span class="n">6.9 GB</span><span class="tag" title="The size matches, and so does a megabyte read from each end — almost certainly the same file. Removing the local copy leaves the one in the shared folder untouched.">same file</span><span class="toggle"></span></div>
      <!-- The unchecked row is also what makes the toggle above mixed: not
           everything in the category is selected. -->
      <div class="cat model marked"><code>refiner</code><span class="n">5.8 GB</span><span class="tag warn" title="This one is a folder: the total size and the number of files inside match, but the contents themselves were not compared. Removing the local copy leaves the one in the shared folder untouched.">looks the same</span><span class="toggle off"></span></div>
      <!-- This category has no "already shared" mark: one name inside it is
           taken, but it was not judged a duplicate, and the mark counts only
           what can be removed. -->
      <div class="cat">
        <button class="disclose" aria-expanded="true"><ChevronRight class="ico" /><code>loras</code></button>
        <span class="n">21 items · 4.7 GB</span><span class="toggle"></span>
      </div>
      <div class="cat model marked"><code>detail_tweaker.safetensors</code><span class="n">144 MB</span><span class="tag stop" title="The shared folder has something under this name, but the contents differ. It is never moved and never removed.">different content</span><span class="no-toggle"></span></div>
      <div class="cat model"><code>style_v2.safetensors</code><span class="n">320 MB</span><span class="toggle"></span></div>
      <div class="cat">
        <button class="disclose" aria-expanded="false"><ChevronRight class="ico" /><code>vae</code></button>
        <span class="n">3 items · 1.6 GB</span><span class="toggle off"></span>
      </div>
    </div>
    <p class="hint">1 name clash is not a duplicate: the shared folder has that name but different content, so it stays in the build and is never offered for removal.</p>
    <p class="hint bad">Stop the build first: its files cannot be taken while it is running.</p>
    <!-- Each button stands next to its own description: one deletes files,
         the other moves them, and they must not be confused. Cleanup comes
         first — it is about what has already been moved, and moving is about
         what has not.
         One grid for both rows, no per-row wrappers: that is what makes the
         button column take the width of the widest caption, so the buttons
         come out the same size in any language. -->
    <div class="act-grid">
      <span class="btn danger">Remove the local copies</span>
      <!-- "Found" covers everything judged a duplicate; "frees" counts only
           the checked rows. `refiner` is unchecked, so its 5.8 GB do not
           count. -->
      <p class="hint">2 duplicates found: the shared folder already has those files and the build takes them from there. Removing the local copies frees 6.9 GB.</p>
      <span class="btn primary" aria-disabled="true">Move to the shared folder</span>
      <p class="hint">Will move 24 files, 30.1 GB. What came with the build stays: the placeholder markers and the model configs folder.</p>
    </div>
  </div>
</Window>

## No models of its own

*connected, with nothing to move*

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
      <span class="chip" style="--instance-accent:var(--accent-amber)">F</span>
      <h3>Flux experiments</h3>
      <span class="spacer"></span>
      <span class="pill stopped"><i></i>Stopped</span>
      <span class="btn primary lg">Start</span>
    </div>
    <div class="tabs">
      <span>Overview</span>
      <span aria-selected="true">Models</span>
      <span>Workflows</span>
      <span>Settings</span>
    </div>
    <div class="row">
      <span class="t-label">Shared models</span>
      <span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span>
    </div>
    <div class="toggle-row">
      <span class="toggle"></span>
      <div>
        <div class="t-base">Give this build access to the shared folder without copying anything into it.</div>
        <div class="hint">D:\AI\_shared\models · build folder untouched</div>
      </div>
    </div>
    <div class="group">
      <span class="t-label">How to apply</span>
      <div class="seg">
        <span aria-pressed="true">Leave the build alone</span>
        <span>Write a file into the build</span>
      </div>
      <p class="hint">The list of shared folders stays with ComfyUI Portable Organizer and is handed to the build as a startup argument. Nothing is written into the build folder — but the shared models are only there when this app starts the build.</p>
    </div>
    <div class="row">
      <span class="t-label">Models of this build</span>
      <span class="btn ghost icon" title="D:\AI\ComfyUI_flux\ComfyUI\models"><FolderOpen class="ico" /></span>
      <span class="spacer"></span>
      <span class="btn ghost"><RotateCw class="ico" />Refresh</span>
    </div>
    <!-- The section stays: the heading and the path to the folder do not go
         anywhere, only its content changes. That is why the state comes as a
         hint rather than a large empty block: that one would look like a
         separate screen dropped into the middle of the tab, and would tear
         the tab in two harder than the border between sections does.
         There is neither a list nor move buttons here — there is nothing to
         govern. -->
    <!-- `blank`, not `hint`: this is not an explanation of the control next
         to it but a statement that there is no list. Every such line looks
         the same, and that look is set in one place. -->
    <p class="blank">Nothing to move: this build has no models of its own.</p>
  </div>
</Window>
