<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive, Plus, ListChecks, Check, ExternalLink } from '@lucide/vue';
</script>

# Settings: workflow library

<!-- J-05 · steps 4-5 · US-WF-02 · US-WF-04 · US-WF-05 -->

The library is a settings section, not a rail section. It is built exactly like
the shared models: one folder outside the builds and its contents. Configuring
the folder and seeing what lies in it used to sit in different places — the
path in the settings, the list in the rail — and the question "where does this
get put" had to be answered by going to another section. Now both the path and
the contents are on one screen, and the rail keeps four permanent places:
builds, where to get a new one, settings, about.

The compatibility check is what the transfer lives in the app for rather than
in Explorer. For a stopped instance there is no exact answer, and marking it
unknown is more honest than showing a green tick.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <!-- The padding comes from the content: the head and the master-detail
           each have their own. -->
      <div class="content flush">
        <!-- The path sits in the section head rather than in a field of its
             own above the list: it is one line, set once, and giving it half
             a screen would speak of it louder than of the library itself. -->
        <!-- The ways to fill the library are collected under one button with
             a menu, pushed to the right edge. There are two of them, they are
             about the same thing, and next to the path they read as three
             equal controls; the freed space went to the path — the row exists
             for it. -->
        <div class="lib-head">
          <h3>Workflow library</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Add</span>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <!-- Multiple selection is declared from here rather than
                   starting by itself on the first tick. The button stands
                   before the search field: it changes what the list is, while
                   search and favourites only narrow what is shown. -->
              <span class="btn ghost icon"><ListChecks class="ico" /></span>
              <div class="input search"><span>Search by name and tags</span></div>
              <span class="btn ghost icon" aria-pressed="true"><span class="star">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
              <!-- There are no tick boxes in the normal mode: keeping an empty
                   column for them at all times would push the names aside for
                   a control that is not on the screen. -->
              <div class="wf-row"><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span><span class="tag">upscale</span></span></div>
              <div class="wf-row on"><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
              <div class="wf-row"><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
              <div class="wf-row lost"><span class="star off">★</span><span class="nm">video / ltx-draft.json</span><span class="tags"><span class="tag stop">file gone</span></span></div>
            </div></div>
            <!-- The counter is at the bottom of the list, not in its head: the
                 head is taken by the search, and the number answers a question
                 about what is under it — and answers after the filter, not
                 before it. -->
            <div class="pane-foot"><span class="t-label">14 workflows</span></div>
          </div>
          <div class="pane">
            <div class="pane-head">
              <span class="title">portrait-v3.json</span>
              <span class="star lg">★</span>
            </div>
            <div class="tabs">
              <span aria-selected="true">Where it opens</span>
              <span>Note</span>
              <span>Tags <span class="n">1</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="compat">
                <!-- Leaving for the build's page is in the right corner, on
                     the same line as the name: this is a move to another
                     object, not an action on the workflow, and it must not be
                     confused with "put it there". -->
                <div class="compat-row ok">
                  <span class="chip" style="--instance-accent:var(--accent-teal)"></span>
                  <span class="act on"><Check class="ico" /></span>
                  <span class="nm">SDXL stable</span><span class="compat-note">all nodes present</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
                <div class="compat-row warn">
                  <span class="chip" style="--instance-accent:var(--accent-indigo)"></span>
                  <span class="act"><Plus class="ico" /></span>
                  <span class="nm">Flux test</span><span class="compat-note">2 nodes missing</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
                <div class="missing">IPAdapterUnifiedLoader · ReActorFaceSwap</div>
                <div class="compat-row">
                  <span class="chip" style="--instance-accent:var(--accent-moss)"></span>
                  <span class="act"><Plus class="ico" /></span>
                  <span class="nm">Experiments</span><span class="compat-note">per the last run</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
              </div>
            </div></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The folder has not been chosen yet

This section is the setting, so there is nowhere to send the user — the choice
stands right here.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <h3>Workflow library</h3>
        <div class="empty">
          <p>
            The library is a folder outside the builds. Workflows in it survive
            a ComfyUI reinstall and are visible to every build at once.
          </p>
          <div class="field" style="width:100%;max-width:520px">
            <label>Library folder</label>
            <div class="path-row">
              <div class="input mono"><span>not chosen</span></div>
              <span class="btn secondary">Browse</span>
            </div>
            <!-- Suggesting a spot next to the shared models is not a rule but
                 a convenience: they usually sit on a roomy drive. -->
            <div class="hint">Next to the shared models: D:\AI\_shared\workflows</div>
          </div>
          <span class="btn primary">Put it next to the shared models</span>
        </div>
      </div>
    </div>
  </div>
</Window>

## Pasting from the clipboard

A graph arrives as text — from a chat, from a forum, from the machine next
door.

Workflows are sent as text more often than as a file, and saving what was sent
into a file only to pick it in a dialog straight away is a detour. The form
takes the place of the list rather than popping up over it: the same component
will stand on a build's "Workflows" tab, where a native ComfyUI window lies on
top of our HTML and there is physically nothing to pop up above. The name is
asked for at once: text from the clipboard has none, and inventing
`workflow (3)` for the user guarantees a dump.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <div class="row">
          <h3>Workflow library</h3>
          <span class="spacer"></span>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
        </div>
        <div class="paste">
          <div class="field">
            <label>Name</label>
            <div class="input"><span>portrait-v3</span></div>
            <!-- The extension is appended by itself: a hand-typed `.jsn` or
                 `.json.json` is not the user's choice but a typo, and there is
                 no point catching it with a question. -->
            <div class="hint">Will be saved as portrait-v3.json</div>
          </div>
          <div class="field">
            <label>Workflow JSON</label>
            <div class="input area mono">{"last_node_id": 42, "last_link_id": 61, "nodes": [{"id": 1, "type": "CheckpointLoaderSimple", "pos": [80, 120], "widgets_values": ["sd_xl_base_1.0.safetensors"]}, {"id": 2, "type": "CLIPTextEncode", …</div>
            <!-- Parsing runs as the text is typed rather than on the press:
                 "Save" on unparsed text is a question whose answer is already
                 known. -->
            <div class="hint">Parsed: 27 nodes · IPAdapterUnifiedLoader and 12 more types</div>
          </div>
          <div class="row">
            <span class="btn primary">Save to the library</span>
            <span class="btn ghost">Cancel</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The clipboard holds something else

The refusal happens on the spot, before a file is written.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <div class="row">
          <h3>Workflow library</h3>
          <span class="spacer"></span>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
        </div>
        <div class="paste">
          <div class="field">
            <label>Name</label>
            <!-- A taken name is not an input error: a file in the library is
                 never overwritten, and this has to be said before the press,
                 not after it. -->
            <div class="input bad"><span>portrait-v3</span></div>
            <div class="hint bad">portrait-v3.json is already in the library. There is nothing to replace it with: choose another name.</div>
          </div>
          <div class="field">
            <label>Workflow JSON</label>
            <div class="input area mono bad">{"prompt": "sunset over the sea", "steps": 30}</div>
            <div class="hint bad">This is JSON, but not a workflow: it has no nodes. Nothing has been saved.</div>
          </div>
          <div class="row">
            <span class="btn primary" aria-disabled="true">Save to the library</span>
            <span class="btn ghost">Cancel</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The real volume of data

**Scrolling.** Two hundred workflows; the section list and the detail pane do
not ride away with the list.

<Window :fixed="true" scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
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
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Workflow library</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Add</span>
          </div>
        </div>
        <!-- Multiple selection. The pane on the right is replaced whole:
             it used to live in two modes at once — a head with a star about
             the selected workflow, and a body under it about everything
             ticked. -->
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon" aria-pressed="true"><ListChecks class="ico" /></span>
              <div class="input search"><span>Search by name and tags</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list picking">
                <!-- data-repeat="3": the source repeats one and the same block
                     of eight rows three times in a row, depicting two hundred
                     workflows without two hundred unique entries. Below are
                     those same three repeats, written out by hand. -->
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / img2img-refine.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">flux / lora-stack.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / img2img-refine.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">flux / lora-stack.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / img2img-refine.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">flux / lora-stack.json</span><span class="tags"><span class="tag">flux</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">214 workflows</span></div>
          </div>
          <div class="pane">
            <!-- The button in the head, the counter of what is ticked in the
                 foot — the same as the list counter on the left: both panes
                 answer "how many are here" at the bottom and keep the action
                 at the top. -->
            <div class="pane-head"><span class="title">Multiple selection</span><span class="btn primary">Add</span></div>
            <div class="scroll"><div class="scroll-pad">
              <!-- Builds are ticked rather than starting the write on a click:
                   files go to disk irreversibly, and that must not begin with
                   the mouse landing on a row. The button at the bottom does
                   the putting, and it says so. -->
              <div class="pick-list">
                <div class="pick-head"><span class="check mixed"></span><span>All builds</span></div>
                <div class="pick-row"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="chip" style="--instance-accent:var(--accent-teal)"></span><span class="nm">SDXL stable</span></div>
                <div class="pick-row"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="chip" style="--instance-accent:var(--accent-indigo)"></span><span class="nm">Flux test</span></div>
                <div class="pick-row"><span class="check"></span><span class="chip" style="--instance-accent:var(--accent-moss)"></span><span class="nm">Experiments</span></div>
                <div class="pick-row"><span class="check"></span><span class="chip" style="--instance-accent:var(--accent-azure)"></span><span class="nm">Video</span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">6 workflows ticked</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The bulk write is running

The pane is taken by the progress of the operation: there is nothing to pick
builds in.

The count goes by operations, not by files: two workflows into two builds are
two files but four independent writes, and each can fail for its own reason.
The counter holds the ones that succeeded, while the bar goes by the ones that
are done: its question is "how much longer", and on failures it is obliged to
move, otherwise the operation looks stuck.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Workflow library</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Add</span>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon" aria-pressed="true"><ListChecks class="ico" /></span>
              <div class="input search"><span>Search by name and tags</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list picking">
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">5 workflows</span></div>
          </div>
          <div class="pane">
            <!-- There is no "Add" button in the head: while the write is
                 running, there is no point offering to start another one on
                 top of it. -->
            <div class="pane-head"><span class="title">Multiple selection</span></div>
            <div class="scroll"><div class="scroll-pad">
              <div class="group">
                <p class="t-sm">1 of 4 operations done</p>
                <div class="bar"><i style="width:50%"></i></div>
                <div class="row"><span class="btn danger">Cancel</span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">2 workflows ticked</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The report: nothing went through

Both workflows are already in both builds.

A taken name is not an error but a refusal by rule: there is no overwriting in
a bulk operation at all, and it asks no questions — twenty in a row must not be
asked. So every pair has its own line and its own reason: on top the pair
"workflow → build" in the colour of the main text, below it the explanation in
an undertone. The red is left to the single line with the count: alarm smeared
over the whole block stops being alarm, and four identical red lines read as
a broken app. The advice stands once for the whole list rather than repeating
in every line.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Workflow library</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Add</span>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon" aria-pressed="true"><ListChecks class="ico" /></span>
              <div class="input search"><span>Search by name and tags</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list picking">
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">5 workflows</span></div>
          </div>
          <div class="pane">
            <div class="pane-head"><span class="title">Multiple selection</span></div>
            <div class="scroll"><div class="scroll-pad">
              <div class="group">
                <p class="t-sm">0 of 4 operations done</p>
                <div class="bar"><i style="width:100%"></i></div>
                <p class="hint bad">4 did not go through</p>
                <div class="fails">
                  <div class="fail">
                    <span class="fail-pair">sdxl / base-upscale.json → SDXL stable</span>
                    <span class="fail-why">the build already has a workflow with this name, it is untouched</span>
                  </div>
                  <div class="fail">
                    <span class="fail-pair">sdxl / base-upscale.json → Flux test</span>
                    <span class="fail-why">the build already has a workflow with this name, it is untouched</span>
                  </div>
                  <div class="fail">
                    <span class="fail-pair">flux / portrait-v3.json → SDXL stable</span>
                    <span class="fail-why">the build already has a workflow with this name, it is untouched</span>
                  </div>
                  <div class="fail">
                    <span class="fail-pair">flux / portrait-v3.json → Flux test</span>
                    <span class="fail-why">the build already has a workflow with this name, it is untouched</span>
                  </div>
                </div>
                <p class="hint">Replacing is only possible one at a time: the app will ask before overwriting.</p>
                <div class="row"><span class="btn ghost">Close</span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">2 workflows ticked</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The "Add" menu open

Two ways to fill the library under one button.

There are two ways and they are about the same thing; next to the path they
read as three equal controls, and the space they took went to the path — the
row exists for it. This is the only pop-up in the app apart from the toasts,
and it must not be placed on the embedded tab screen: there a native window
lies on top of our HTML and the menu simply would not be visible. That is why
the paste form unfolds in the place of the list rather than popping up over it.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Workflow library</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary" aria-expanded="true"><Plus class="ico" />Add</span>
            <div class="menu-pop">
              <span>From a file…</span>
              <span>Paste as text</span>
            </div>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon"><ListChecks class="ico" /></span>
              <div class="input search"><span>Search by name and tags</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list">
                <div class="wf-row"><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">14 workflows</span></div>
          </div>
          <div class="pane">
            <div class="pane-head">
              <span class="title">portrait-v3.json</span>
              <span class="star lg">★</span>
            </div>
            <div class="tabs">
              <span aria-selected="true">Where it opens</span>
              <span>Note</span>
              <span>Tags <span class="n">1</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="compat">
                <div class="compat-row ok">
                  <span class="chip" style="--instance-accent:var(--accent-teal)"></span>
                  <span class="act on"><Check class="ico" /></span>
                  <span class="nm">SDXL stable</span><span class="compat-note">all nodes present</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
                <div class="compat-row">
                  <span class="chip" style="--instance-accent:var(--accent-moss)"></span>
                  <span class="act"><Plus class="ico" /></span>
                  <span class="nm">Experiments</span><span class="compat-note">per the last run</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
              </div>
            </div></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>
