<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Settings: shared models

<!-- J-01 · steps 1-3 · US-SHARED-01 · US-SHARED-02 · US-SHARED-04 -->

This screen is what the project was started for: one checkpoint weighs up to
twenty gigabytes, and with five builds the duplicates eat hundreds. The
categories are recognised from the folder's contents rather than from a
hardcoded list — otherwise the config goes stale with the next ComfyUI update.

Settings are a second level of navigation: the list of sections on the left, the
chosen one on the right. A "Settings → Shared models" breadcrumb did not give
that — it said where you were but did not show what was next to it, and the only
way to a neighbouring section was to go back.

Master-detail rather than a feed of blocks: the list of categories is what
grows, while the downloads toggle and the config button belong to the setting as
a whole. The root is pinned above both: it is the address of everything below
and must not scroll away with the list.

This frame is the default window, 1100 wide: that leaves under 700 for the
section's screen, and at that width the columns collapse into rows — the
controls stand above the list. Width is worth more to the list here — the folder
names are monospaced and are not truncated. To the right of the list they fit on
a monitor; that is a separate frame below, at a real 1920 by 1080.

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
        <div class="nav-item on"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content framed no-foot">
        <div class="pinned">
          <h3>Shared models</h3>
          <div class="field">
            <label>Shared models folder</label>
            <div class="path-row">
              <div class="input mono"><span>D:\AI\_shared\models</span></div>
              <span class="btn secondary">Browse</span>
            </div>
            <div class="hint">5 categories recognised · 231 GB · 3 instances connected</div>
          </div>
        </div>
        <div class="split-master shared">
          <div class="pane">
            <div class="pane-head"><span class="title">Category folders</span></div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
              <div class="cat"><code>checkpoints</code><span class="n">14 files · 187 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>loras</code><span class="n">126 files · 41 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>vae</code><span class="n">6 files · 2.1 GB</span><span class="tag">recognised</span></div>
              <div class="cat unknown"><code>my_experiments</code><span class="n">3 files · 0.4 GB</span><span class="tag warn">not recognised</span></div>
              <div class="cat blocked"><code>custom_nodes</code><span class="n">never shared</span><span class="tag stop">excluded</span></div>
            </div></div>
          </div>
          <div class="side">
            <div class="toggle-row">
              <span class="toggle"></span>
              <div>
                <div class="t-base">Download new models into the shared folder</div>
                <div class="hint">A model downloaded in one build becomes visible to all of them.</div>
                <div class="hint">Takes effect the next time a build starts.</div>
              </div>
            </div>
            <div class="row"><span class="btn ghost" aria-pressed="true">Hide the generated config</span></div>
            <!-- The config is longer than the column: it takes everything
                 left under the button and scrolls within itself. -->
            <div class="pane">
              <div class="pane-head"><span class="title">extra_model_paths.yaml</span></div>
              <div class="scroll"><pre class="console">cpo_shared_0:
  base_path: D:/AI/_shared/models
  is_default: true
  checkpoints: checkpoints/
  loras: loras/
  vae: vae/
  text_encoders: |
    text_encoders/
    clip/</pre></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## A realistic amount of data

**Scrolling.** Twenty-five categories; the root, the toggle and the config are
pinned — only the list scrolls.

Twenty-five categories dragged the toggle and the config below the bottom edge:
they belong to the setting as a whole rather than to a row of the list, so they
are lifted out of the scroll entirely. On a narrow screen they stand above the
list, on a monitor — to the right of it.

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
        <div class="nav-item on"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content framed no-foot">
        <div class="pinned">
          <h3>Shared models</h3>
          <div class="field">
            <div class="path-row">
              <div class="input mono"><span>D:\AI\_shared\models</span></div>
              <span class="btn secondary">Browse</span>
            </div>
            <div class="hint">25 categories recognised · 231 GB · 8 instances connected</div>
          </div>
        </div>
        <div class="split-master shared">
          <div class="pane">
            <div class="pane-head"><span class="title">Category folders</span></div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
            <div class="cat"><code>checkpoints</code><span class="n">14 files · 187 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>loras</code><span class="n">126 files · 41 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>vae</code><span class="n">6 files · 2.1 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>controlnet</code><span class="n">9 files · 4.8 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>text_encoders</code><span class="n">4 files · 3.2 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>diffusion_models</code><span class="n">3 files · 22 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>clip_vision</code><span class="n">2 files · 1.1 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>upscale_models</code><span class="n">11 files · 0.9 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>embeddings</code><span class="n">38 files · 0.1 GB</span><span class="tag">recognised</span></div>
            <div class="cat"><code>style_models</code><span class="n">empty</span><span class="tag">recognised</span></div>
            <div class="cat"><code>hypernetworks</code><span class="n">empty</span><span class="tag">recognised</span></div>
            <div class="cat"><code>photomaker</code><span class="n">empty</span><span class="tag">recognised</span></div>
            <div class="cat unknown"><code>my_experiments</code><span class="n">3 files · 0.4 GB</span><span class="tag warn">not recognised</span></div>
            <div class="cat blocked"><code>custom_nodes</code><span class="n">never shared</span><span class="tag stop">excluded</span></div>
            </div></div>
          </div>
          <div class="side">
            <div class="toggle-row">
              <span class="toggle"></span>
              <div>
                <div class="t-base">Download new models into the shared folder</div>
                <div class="hint">A model downloaded in one build becomes visible to all of them.</div>
                <div class="hint">Takes effect the next time a build starts.</div>
              </div>
            </div>
            <div class="row"><span class="btn ghost" aria-pressed="true">Hide the generated config</span></div>
            <div class="pane">
              <div class="pane-head"><span class="title">extra_model_paths.yaml</span></div>
              <div class="scroll"><pre class="console">cpo_shared_0:
  base_path: D:/AI/_shared/models
  is_default: true
  checkpoints: checkpoints/
  loras: loras/
  vae: vae/
  controlnet: controlnet/
  upscale_models: upscale_models/
  embeddings: embeddings/
  style_models: style_models/
  hypernetworks: hypernetworks/
  photomaker: photomaker/
  diffusion_models: diffusion_models/
  clip_vision: clip_vision/
  text_encoders: |
    text_encoders/
    clip/</pre></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>
