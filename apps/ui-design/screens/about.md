<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, FolderOpen } from '@lucide/vue';
</script>

# About the app

<!-- J-06 · step 1 · US-DATA-01 · US-DATA-03 -->

The requirement is stated plainly: uninstall the app and everything of its own
goes, but not the models and not the library. For that to be trustworthy, the
border is visible in advance rather than after the uninstall. A block of its own
names the files the app wrote inside someone else's folders on the user's
command: they will stay, and hiding that would not be honest.

There are four blocks, and each is built the same way: a caption, an outlined
list of rows, one line of explanation under it. Telling them apart by heading
size did not work — the screen read as one sheet of identical fields, and the
main thing on it, the border between "ours" and "yours", had to be deduced.
What survives is not marked by colour: green in this palette is the colour of a
live process, and on a list of folders it would promise a state a folder does
not have. The border is held by the block headings and by the line under each
list.

Updating lives here as well, above the grid and across the full width. The
version is already in this screen's header, and "what do I have installed and is
there a newer one" is one question, not two in different sections. It does not
take a fifth place in the grid: the other four answer where things are, while
this one offers an action.

A new version is visible from outside the screen too — as a dot by the section
in the rail. A toast will not do for that: the check happens at startup, and the
toast would have to be either dismissed or lost, with nowhere to learn about the
version a second time. The dot is blue, from the accent palette: green would
promise a running process, red a crash, and this is news.

The builds in the registry get their own block rather than a list inside "Stays
on disk": there can be eight of them, and mixed in with the shared folders their
names would read as two more kinds of shared property. In the app the layout is
a CSS grid with named areas: a wide panel pairs "What uninstalling removes" with
"Stays on disk" on the row above and "What the app wrote" with "Builds in the
registry" on the row below, and the threshold depends on the width of the panel
itself (`@container`), not of the window. Here, in a static showcase, the four
blocks are direct children of the `.cols` auto-grid in the same DOM order: on a
wide panel it places them two per row in the same pairs, and on a narrow one
folds them into a single column in the same order as the app's live grid. The
difference from the app is only in the mechanism (auto-grid versus named areas),
not in the result.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item on"><Info class="ico" /><span>About</span><i class="dot nav-mark"></i></div>
    </nav>
  </template>
  <div class="content">
    <!-- The version is one fact, not a block: it stands next to the
         section's name, and the room on screen goes to what is about the
         folders. The app's name is not repeated here: it is in the window
         title as it is. -->
    <div class="row"><h3>About</h3><span class="hint">Version 0.1.0</span></div>
    <!-- Updating sits above the grid and across the full width: the other
         four blocks are about where things are, while this one offers an
         action, and it pairs with none of them. The version number in the
         heading is mandatory — "an update is available" without it does not
         let anyone decide whether they need it right now. -->
    <div class="group">
      <span class="t-label">Updates</span>
      <div class="banner">
        <div><b>Version 0.2.0 is available</b><p class="t-sm">You have 0.1.0 · Released 22 Aug 2026</p></div>
        <span class="spacer"></span>
        <span class="btn primary">Download and install</span>
      </div>
      <!-- The notes are not written here: they are the new version's
           CHANGELOG section, carried inside the updater's manifest and
           rendered from markdown. Hence the shape — a lead paragraph, a
           heading and a list — and hence the absence of a scroll of its own:
           the screen already has one, and the section flows inside it. -->
      <div class="release-notes">
        <span class="t-label">What changed</span>
        <p>The workflow library moved into "Settings". The release is a maintenance one: nothing in the launch path changed.</p>
        <h4 class="t-label">Changed</h4>
        <ul>
          <li><b>The workflow library</b> moved into "Settings" and no longer takes a place in the rail.</li>
          <li><b>Cancelling an unpack</b> no longer looks like a hang: the wizard reports the rollback and clears <code class="t-mono">extra_model_paths.yaml</code>.</li>
        </ul>
      </div>
      <div class="row"><span class="btn secondary">Check now</span></div>
      <div class="toggle-row">
        <span class="toggle"></span>
        <div>
          <div class="t-base">Check for updates on startup</div>
          <p class="hint">The only thing the app sends outside your computer: the version number, to ask whether a newer one exists.</p>
        </div>
      </div>
    </div>
    <!-- Every block is a caption, an outlined list and one line of
         explanation under it. The blocks used to differ only by heading
         size, and the screen read as one sheet.
         The four blocks are direct children of `.cols`, in the same DOM
         order as in the app: folded into a single column on a narrow panel,
         the order stays the same as in the live grid, not just the layout
         on a wide one. -->
    <div class="cols">
      <div class="group">
        <span class="t-label">What uninstalling removes</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Settings and the build registry<span class="hint">C:\Users\andrew\AppData\Roaming\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Roaming\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Cache and browser data<span class="hint">C:\Users\andrew\AppData\Local\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Local\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Uninstalling removes the app and its own settings. Your models, shared templates and ComfyUI folders stay where they are.</p>
      </div>
      <div class="group">
        <span class="t-label">Stays on disk</span>
        <!-- This block is why the screen exists: hundreds of gigabytes must
             not depend on faith that the uninstaller will do the right
             thing. The block's heading and the line under the list answer
             for that, not tinted rows. -->
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Shared models<span class="hint">D:\AI\_shared\models</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Workflow library<span class="hint">D:\AI\_shared\workflows</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\workflows"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">These folders belong to you. Removing the app does not touch them.</p>
      </div>
      <!-- A block of its own rather than a row among what survives: that one
           answers "where is my stuff", this one "what did you touch in it".
           The two kinds of file are named one per row: as a paragraph they
           read like a disclaimer. -->
      <div class="group">
        <span class="t-label">What the app wrote inside your builds</span>
        <div class="paths">
          <div class="path-item"><span class="lbl">extra_model_paths.yaml</span><span class="val">Only in “file inside the build” mode</span></div>
          <div class="path-item"><span class="lbl">Workflow copies</span><span class="val">Only the ones you added to a build</span></div>
        </div>
        <p class="hint">Both stay after the app is removed — they live inside someone else's installation, and taking them out is not ours to do.</p>
      </div>
      <!-- The builds get their own block rather than a list inside "Stays on
           disk": there can be eight of them, and mixed in with the shared
           folders their names would read as two more kinds of shared
           property. A separate block also pairs with "What the app wrote" on
           a wide panel — both are about what falls to each build. -->
      <div class="group">
        <span class="t-label">Builds in the registry</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">SDXL stable<span class="hint">D:\AI\comfy-sdxl</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-sdxl"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Flux test<span class="hint">D:\AI\comfy-flux</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-flux"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Animation<span class="hint">D:\AI\comfy-anim</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-anim"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">The registry only remembers where each build lives. Removing the app clears the entries, not the builds themselves.</p>
      </div>
    </div>
  </div>
</Window>

## On a monitor

A 1920×1080 window scaled down to 55%: the panel is wider than the threshold,
and in the app that would switch on the second column of the named grid. The
showcase's auto-grid has a threshold of its own (`minmax(480px, 1fr)`), but at
this width both fire the same way: the same four blocks in the same DOM order
line up two per row — "what uninstalling removes" with "stays on disk" on the
row above, "what was written into the builds" with "builds in the registry" on
the row below.

<Window :fixed="true" :hd="true">
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item on"><Info class="ico" /><span>About</span><i class="dot nav-mark"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row"><h3>About</h3><span class="hint">Version 0.1.0</span></div>
    <!-- Updating sits above the grid and across the full width: the other
         four blocks are about where things are, while this one offers an
         action, and it pairs with none of them. The version number in the
         heading is mandatory — "an update is available" without it does not
         let anyone decide whether they need it right now. -->
    <div class="group">
      <span class="t-label">Updates</span>
      <div class="banner">
        <div><b>Version 0.2.0 is available</b><p class="t-sm">You have 0.1.0 · Released 22 Aug 2026</p></div>
        <span class="spacer"></span>
        <span class="btn primary">Download and install</span>
      </div>
      <!-- The notes are not written here: they are the new version's
           CHANGELOG section, carried inside the updater's manifest and
           rendered from markdown. Hence the shape — a lead paragraph, a
           heading and a list — and hence the absence of a scroll of its own:
           the screen already has one, and the section flows inside it. -->
      <div class="release-notes">
        <span class="t-label">What changed</span>
        <p>The workflow library moved into "Settings". The release is a maintenance one: nothing in the launch path changed.</p>
        <h4 class="t-label">Changed</h4>
        <ul>
          <li><b>The workflow library</b> moved into "Settings" and no longer takes a place in the rail.</li>
          <li><b>Cancelling an unpack</b> no longer looks like a hang: the wizard reports the rollback and clears <code class="t-mono">extra_model_paths.yaml</code>.</li>
        </ul>
      </div>
      <div class="row"><span class="btn secondary">Check now</span></div>
      <div class="toggle-row">
        <span class="toggle"></span>
        <div>
          <div class="t-base">Check for updates on startup</div>
          <p class="hint">The only thing the app sends outside your computer: the version number, to ask whether a newer one exists.</p>
        </div>
      </div>
    </div>
    <div class="cols">
      <div class="group">
        <span class="t-label">What uninstalling removes</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Settings and the build registry<span class="hint">C:\Users\andrew\AppData\Roaming\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Roaming\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Cache and browser data<span class="hint">C:\Users\andrew\AppData\Local\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Local\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Uninstalling removes the app and its own settings. Your models, shared templates and ComfyUI folders stay where they are.</p>
      </div>
      <div class="group">
        <span class="t-label">Stays on disk</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Shared models<span class="hint">D:\AI\_shared\models</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Workflow library<span class="hint">D:\AI\_shared\workflows</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\workflows"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">These folders belong to you. Removing the app does not touch them.</p>
      </div>
      <div class="group">
        <span class="t-label">What the app wrote inside your builds</span>
        <div class="paths">
          <div class="path-item"><span class="lbl">extra_model_paths.yaml</span><span class="val">Only in “file inside the build” mode</span></div>
          <div class="path-item"><span class="lbl">Workflow copies</span><span class="val">Only the ones you added to a build</span></div>
        </div>
        <p class="hint">Both stay after the app is removed — they live inside someone else's installation, and taking them out is not ours to do.</p>
      </div>
      <div class="group">
        <span class="t-label">Builds in the registry</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">SDXL stable<span class="hint">D:\AI\comfy-sdxl</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-sdxl"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Flux test<span class="hint">D:\AI\comfy-flux</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-flux"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Animation<span class="hint">D:\AI\comfy-anim</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-anim"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Experiments<span class="hint">D:\AI\comfy-lab</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-lab"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">The registry only remembers where each build lives. Removing the app clears the entries, not the builds themselves.</p>
      </div>
    </div>
  </div>
</Window>

## Installing with builds running

The Windows installer closes the app by force, and the child processes live in a
Job Object and leave with it. Updating in the middle of a generation means
losing the queue and the minutes of a cold start, so the decision is the user's,
not ours.

The fork opens as a banner in place: a modal cannot be put over the content area
(z-order discipline), and a toast has no buttons. The install button in the
banner above goes away meanwhile — otherwise there would be two ways on screen
to start the same thing, one of them bypassing the question just asked.

The download bar is indeterminate until the server sends a length: a fraction
downloaded with no known total is a fabrication.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item on"><Info class="ico" /><span>About</span><i class="dot nav-mark"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row"><h3>About</h3><span class="hint">Version 0.1.0</span></div>
    <div class="group">
      <span class="t-label">Updates</span>
      <div class="banner">
        <div><b>Version 0.2.0 is available</b><p class="t-sm">You have 0.1.0 · Released 22 Aug 2026</p></div>
      </div>
      <!-- The build names in the question are not decoration: the user has
           to recognise among them the one that is generating right now. -->
      <div class="banner">
        <div>
          <b>Builds are still running</b>
          <p class="t-sm">Installing closes the app, and every running server goes with it: SDXL stable, Flux test. Unsaved graphs and the generation queue go too.</p>
        </div>
        <span class="spacer"></span>
        <span class="btn secondary">Stop them and install</span>
        <span class="btn ghost">Postpone until next launch</span>
      </div>
      <div class="track"><i style="width: 42%"></i></div>
      <p class="hint">18.4 MB of 43.7 MB</p>
      <div class="row"><span class="btn secondary">Check now</span></div>
      <div class="toggle-row">
        <span class="toggle"></span>
        <div>
          <div class="t-base">Check for updates on startup</div>
          <p class="hint">The only thing the app sends outside your computer: the version number, to ask whether a newer one exists.</p>
        </div>
      </div>
    </div>
  </div>
</Window>
