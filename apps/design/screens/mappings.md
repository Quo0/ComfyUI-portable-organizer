<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, Pencil, X, Ban, Check } from '@lucide/vue';
</script>

# Destinations

<!-- J-01 · step 4 · US-INST-02 · US-INST-08 -->

There can be several destinations in one run — the unpacking happens once, and
the other copies are made by copying the finished tree. The warning about a long
path stands here rather than after a failure: some of the build's files are
nested deep enough to hit the Windows limit.

The buttons that move on are **in the row with the step's name**, not in a
footer: with several destinations "Next" scrolled out of sight exactly when it
became needed. "Back" is always to the left of the action button. Pinned under
them is the summary of the archive — what used to be shown as a step of its own.

The form is on the left, the list on the right: it throws destination after
destination into the list and returns to its initial state after each one. A
panel with all the fields for every target at once took a screenful of scrolling
per target, and there can be six targets.

The description is not printed in the row — it has four columns as it is — but
it pops up as a hint on hover. Rows without a description have no hint at all.

An edit opens **under its own row**, in the same form: the list stays where it
is, and it is visible what exactly is being edited. It is a copy that is edited
— "Cancel" must roll back rather than "save it back". The edit form's buttons
are icons: "Add to the list" happens once, while these are two per row, and with
captions they would push out of the row the very thing it exists for.

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
      <span class="step now"><u>2</u>Destinations</span><span class="step-sep"></span>
      <span class="step"><u>3</u>Shared resources</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Unpacking</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Done</span>
    </div>
    <div class="step-bar">
      <h3>Where to unpack</h3>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
        <span class="btn primary lg">Next</span>
      </span>
    </div>
    <div class="meta">
      <span>ComfyUI_windows_portable_nvidia_0.31.0.7z</span>
      <span>56,128 files</span>
      <span>Unpacks to 9.7 GB</span>
      <span>Needs about 19.4 GB free</span>
    </div>
    <!-- What ends up at the top level after unpacking: for a build that is
         a single root folder, but not every archive is like that, and the
         line is always there. -->
    <p class="hint">Root folder in the archive: ComfyUI_windows_portable</p>
    <div class="cols targets">
      <div class="pane">
        <div class="pane-head">
          <span class="title">New destination</span>
          <span class="btn primary">Add to the list</span>
        </div>
        <div class="scroll-pad">
          <div class="field">
            <span class="t-label">Build folder</span>
            <div class="path-row">
              <div class="input mono"><span>D:\AI\Flux</span></div>
              <span class="btn secondary">Choose…</span>
            </div>
            <!-- A warning rather than a permanent hint: it appears only
                 when the path really is long, and it arrives with the
                 check of the destinations. -->
            <div class="hint">The deepest file would land 246 characters in. Unpacking will still work, but ComfyUI and pip may not — pick a shorter path.</div>
          </div>
          <div class="field">
            <label>Name</label>
            <div class="input">Flux test</div>
          </div>
          <div class="field">
            <label>Description</label>
            <div class="input"></div>
          </div>
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
          <div class="field">
            <label>Preferred port</label>
            <div class="input num">8188</div>
            <div class="hint">Used at startup when it is free.</div>
          </div>
        </div>
      </div>
      <!-- The colour is visible right in the row: two builds with similar
           paths are told apart by it, not by the path. It sits between the
           name and the buttons — there the squares line up in a column
           instead of wandering with the length of the name. The pencil
           opens the edit under the row itself — the list stays where it
           is, and it is visible what exactly is being edited. -->
      <div class="field">
        <span class="t-label">Will be unpacked into</span>
        <div class="paths">
          <!-- The description does not fit in the row, but it must not be
               lost either: it pops up as a hint. A row without one has no
               attribute at all — an empty hint would flash a frame about
               nothing. -->
          <div class="path-item editable" title="A branch for Flux experiments, put on top of the fresh 0.31">
            <span class="lbl">D:\AI\Flux</span>
            <span class="val">Flux test</span>
            <span class="chip sm" style="--instance-accent:var(--accent-teal)"></span>
            <span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span>
          </div>
          <div class="path-item editable">
            <span class="lbl">E:\AI\Flux_clean</span>
            <span class="val">Flux clean</span>
            <span class="chip sm" style="--instance-accent:var(--accent-indigo)"></span>
            <span class="acts"><span class="act" aria-pressed="true"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span>
          </div>
          <div class="pane">
            <div class="pane-head">
              <span class="title">Editing</span>
              <span class="acts"><span class="act"><Ban class="ico" /></span><span class="act"><Check class="ico" /></span></span>
            </div>
            <div class="scroll-pad">
              <div class="field">
                <span class="t-label">Build folder</span>
                <div class="path-row">
                  <div class="input mono"><span>E:\AI\Flux_clean</span></div>
                  <span class="btn secondary">Choose…</span>
                </div>
              </div>
              <div class="field">
                <label>Name</label>
                <div class="input">Flux clean</div>
              </div>
              <div class="field">
                <label>Description</label>
                <div class="input"></div>
              </div>
              <div class="field">
                <span class="t-label">Accent colour</span>
                <div class="picker">
                  <i style="background:var(--accent-teal)"></i><i class="on" style="background:var(--accent-indigo)"></i>
                  <i style="background:var(--accent-ember)"></i><i style="background:var(--accent-moss)"></i>
                  <i style="background:var(--accent-azure)"></i><i style="background:var(--accent-orchid)"></i>
                  <i style="background:var(--accent-rose)"></i><i style="background:var(--accent-amber)"></i>
                  <span class="swatch-custom" title="Pick your own colour"></span>
                </div>
              </div>
              <div class="field">
                <label>Preferred port</label>
                <div class="input num">8189</div>
                <div class="hint">Used at startup when it is free.</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## A realistic amount of data

*scrolling · eighteen destination folders; the step row with "Next" is pinned*

<!-- The buttons that move on stand here and never scroll away: the six
     destinations in the area below scroll, the step row stays put. -->
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
        <span class="step now"><u>2</u>Destinations</span><span class="step-sep"></span>
        <span class="step"><u>3</u>Shared resources</span><span class="step-sep"></span>
        <span class="step"><u>4</u>Unpacking</span><span class="step-sep"></span>
        <span class="step"><u>5</u>Done</span>
      </div>
      <div class="step-bar">
        <h3>Where to unpack</h3>
        <span class="spacer"></span>
        <span class="acts">
          <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
          <span class="btn primary lg">Next</span>
        </span>
      </div>
      <div class="meta">
        <span>ComfyUI_windows_portable_nvidia_0.31.0.7z</span>
        <span>56,128 files</span>
        <span>Needs about 58.2 GB free</span>
      </div>
    </div>
    <!-- The form is nailed to the top of its column: the list grows
         downwards while it stays put — for exactly the reason "Back" and
         "Next" moved up out of the footer. -->
    <div class="scroll"><div class="scroll-pad">
      <div class="cols targets">
        <div class="pane">
          <div class="pane-head">
            <span class="title">New destination</span>
            <span class="btn primary">Add to the list</span>
          </div>
          <div class="scroll-pad">
            <div class="field">
              <span class="t-label">Build folder</span>
              <div class="path-row">
                <div class="input mono"><span>D:\AI\Flux</span></div>
                <span class="btn secondary">Choose…</span>
              </div>
            </div>
            <div class="field">
              <label>Name</label>
              <div class="input">Flux test</div>
            </div>
            <div class="field">
              <span class="t-label">Accent colour</span>
              <div class="picker">
                <i class="on" style="background:var(--accent-teal)"></i><i style="background:var(--accent-indigo)"></i>
                <i style="background:var(--accent-ember)"></i><i style="background:var(--accent-moss)"></i>
                <span class="swatch-custom" title="Pick your own colour"></span>
              </div>
            </div>
          </div>
        </div>
        <div class="field">
          <span class="t-label">Will be unpacked into</span>
          <div class="paths">
            <div class="path-item editable" title="A branch for Flux experiments, put on top of the fresh 0.31"><span class="lbl">D:\AI\Flux</span><span class="val">Flux test</span><span class="chip sm" style="--instance-accent:var(--accent-teal)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Flux_clean</span><span class="val">Flux clean</span><span class="chip sm" style="--instance-accent:var(--accent-indigo)"></span><span class="acts"><span class="act" aria-pressed="true"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <!-- An edit in the middle of the list: the panel opens in its
                 own place, everything below moves down, and the order of
                 the rows does not change. -->
            <div class="pane">
              <div class="pane-head">
                <span class="title">Editing</span>
                <span class="acts"><span class="act"><Ban class="ico" /></span><span class="act"><Check class="ico" /></span></span>
              </div>
              <div class="scroll-pad">
                <div class="field">
                  <span class="t-label">Build folder</span>
                  <div class="path-row">
                    <div class="input mono"><span>E:\AI\Flux_clean</span></div>
                    <span class="btn secondary">Choose…</span>
                  </div>
                </div>
                <div class="field">
                  <label>Name</label>
                  <div class="input">Flux clean</div>
                </div>
                <div class="field">
                  <label>Preferred port</label>
                  <div class="input num">8189</div>
                  <div class="hint">Used at startup when it is free.</div>
                </div>
              </div>
            </div>
            <div class="path-item editable"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span><span class="chip sm" style="--instance-accent:var(--accent-ember)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Sandbox</span><span class="val">Sandbox</span><span class="chip sm" style="--instance-accent:var(--accent-moss)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\Video</span><span class="val">Video</span><span class="chip sm" style="--instance-accent:var(--accent-azure)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Archive_030</span><span class="val">Archive 0.30</span><span class="chip sm" style="--instance-accent:var(--accent-orchid)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span><span class="chip sm" style="--instance-accent:var(--accent-ember)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Sandbox</span><span class="val">Sandbox</span><span class="chip sm" style="--instance-accent:var(--accent-moss)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\Video</span><span class="val">Video</span><span class="chip sm" style="--instance-accent:var(--accent-azure)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Archive_030</span><span class="val">Archive 0.30</span><span class="chip sm" style="--instance-accent:var(--accent-orchid)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span><span class="chip sm" style="--instance-accent:var(--accent-ember)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Sandbox</span><span class="val">Sandbox</span><span class="chip sm" style="--instance-accent:var(--accent-moss)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\Video</span><span class="val">Video</span><span class="chip sm" style="--instance-accent:var(--accent-azure)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Archive_030</span><span class="val">Archive 0.30</span><span class="chip sm" style="--instance-accent:var(--accent-orchid)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
          </div>
          <!-- The check runs over the whole list and after every addition:
               the disk could have filled up after the destination was
               added. -->
          <p class="hint bad">Not enough free space: about 58.2 GB is required.</p>
        </div>
      </div>
    </div></div>
  </div>
</Window>

## The list is empty

The first thing seen on the step.

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
      <span class="step now"><u>2</u>Destinations</span><span class="step-sep"></span>
      <span class="step"><u>3</u>Shared resources</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Unpacking</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Done</span>
    </div>
    <div class="step-bar">
      <h3>Where to unpack</h3>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
        <span class="btn primary lg" aria-disabled="true">Next</span>
      </span>
    </div>
    <div class="meta">
      <span>ComfyUI_windows_portable_nvidia_0.31.0.7z</span>
      <span>56,128 files</span>
      <span>Unpacks to 9.7 GB</span>
      <span>Needs about 0 bytes free</span>
    </div>
    <p class="hint">Root folder in the archive: ComfyUI_windows_portable</p>
    <!-- The form is clean and silent: complaining about a field nobody has
         reached yet is a lie. The errors appear when "Add to the list" is
         pressed on an empty form. "Next" is disabled: while the list is
         empty there is nothing to unpack. -->
    <div class="cols targets">
      <div class="pane">
        <div class="pane-head">
          <span class="title">New destination</span>
          <span class="btn primary">Add to the list</span>
        </div>
        <div class="scroll-pad">
          <div class="field">
            <span class="t-label">Build folder</span>
            <div class="path-row">
              <div class="input mono"><span></span></div>
              <span class="btn secondary">Choose…</span>
            </div>
          </div>
          <div class="field">
            <label>Name</label>
            <div class="input"></div>
          </div>
          <div class="field">
            <label>Description</label>
            <div class="input"></div>
          </div>
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
          <div class="field">
            <label>Preferred port</label>
            <div class="input num">8188</div>
            <div class="hint">Used at startup when it is free.</div>
          </div>
        </div>
      </div>
      <div class="field">
        <span class="t-label">Will be unpacked into</span>
        <p class="blank">Fill in the form on the left and add the first destination.</p>
      </div>
    </div>
  </div>
</Window>
