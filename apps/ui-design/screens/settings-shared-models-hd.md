<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Shared models on a 1920×1080 monitor

<!-- J-03 · monitor · US-SHARED-01 · US-SHARED-02 -->

The frames above are drawn at 940 by 560 — the lower bound at which the screen
must still work. That is not where it lives: ComfyUI builds are kept on a
machine with a large monitor, and the window there is maximised. This frame is a
real 1920 by 1080, scaled down to fit the page, so the ratio of text to space
here is honest. The captions read if you zoom the page in.

What this particular size proves: the room a monitor gives goes to the list of
categories rather than to rows stretched across the whole width. The root stays
in place at the top, the downloads toggle and the config on the right, and none
of them scrolls away however many folders the shared folder holds. Exactly two
areas scroll: the list and the config itself.

**Scrolling.** Thirty-four categories; a 1920×1080 window scaled down to 55%.

<Window :fixed="true" :hd="true" scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Running · 4</div>
      <div class="nav-runs">
        <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL stable</em><i class="dot" style="background:var(--state-running)"></i></div>
        <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux test</em><i class="dot" style="background:var(--state-starting)"></i></div>
        <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Animation</em><span class="badge">!</span></div>
        <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-moss)">E</span><em>Experiments</em><i class="dot" style="background:var(--state-running)"></i></div>
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
            <label>Shared models folder</label>
            <div class="path-row">
              <div class="input mono"><span>D:\AI\_shared\models</span></div>
              <span class="btn secondary">Browse</span>
            </div>
            <div class="hint">32 categories recognised · 604 GB · 4 instances connected</div>
          </div>
        </div>
        <div class="split-master shared">
          <div class="pane">
            <div class="pane-head"><span class="title">Category folders</span></div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
              <div class="cat"><code>checkpoints</code><span class="n">21 files · 274 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>loras</code><span class="n">312 files · 96 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>vae</code><span class="n">9 files · 3.4 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>controlnet</code><span class="n">17 files · 12 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>text_encoders</code><span class="n">6 files · 18 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>clip_vision</code><span class="n">4 files · 2.6 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>diffusion_models</code><span class="n">8 files · 141 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>unet</code><span class="n">2 files · 11 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>upscale_models</code><span class="n">14 files · 1.2 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>embeddings</code><span class="n">61 files · 0.2 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>style_models</code><span class="n">3 files · 1.7 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>hypernetworks</code><span class="n">empty</span><span class="tag">recognised</span></div>
              <div class="cat"><code>photomaker</code><span class="n">1 file · 0.9 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>gligen</code><span class="n">empty</span><span class="tag">recognised</span></div>
              <div class="cat"><code>diffusers</code><span class="n">2 files · 14 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>configs</code><span class="n">7 files · 0.1 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>vae_approx</code><span class="n">4 files · 0.1 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>audio_encoders</code><span class="n">2 files · 1.4 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>model_patches</code><span class="n">empty</span><span class="tag">recognised</span></div>
              <div class="cat"><code>ipadapter</code><span class="n">11 files · 6.3 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>insightface</code><span class="n">3 files · 0.7 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>instantid</code><span class="n">2 files · 1.9 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>animatediff_models</code><span class="n">5 files · 8.1 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>animatediff_motion_lora</code><span class="n">12 files · 0.9 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>facerestore_models</code><span class="n">4 files · 1.1 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>sams</code><span class="n">3 files · 2.4 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>ultralytics</code><span class="n">9 files · 0.3 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>grounding-dino</code><span class="n">2 files · 0.8 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>onnx</code><span class="n">6 files · 0.5 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>LLM</code><span class="n">1 file · 4.1 GB</span><span class="tag">recognised</span></div>
              <div class="cat"><code>clip</code><span class="n">empty</span><span class="tag">recognised</span></div>
              <div class="cat"><code>xlabs</code><span class="n">2 files · 0.6 GB</span><span class="tag">recognised</span></div>
              <div class="cat unknown"><code>my_experiments</code><span class="n">7 files · 1.3 GB</span><span class="tag warn">not recognised</span></div>
              <div class="cat blocked"><code>custom_nodes</code><span class="n">never shared</span><span class="tag stop">excluded</span></div>
              <!-- The explanation stands under the list it concerns, and
                   inside its scroll: it is a footnote to a row, not a
                   permanent caption on the screen. -->
              <div class="hint">Custom nodes are never shared: separate builds exist precisely because nodes conflict with each other.</div>
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
  clip_vision: clip_vision/
  diffusion_models: diffusion_models/
  unet: unet/
  upscale_models: upscale_models/
  embeddings: embeddings/
  style_models: style_models/
  hypernetworks: hypernetworks/
  photomaker: photomaker/
  gligen: gligen/
  diffusers: diffusers/
  configs: configs/
  vae_approx: vae_approx/
  audio_encoders: audio_encoders/
  model_patches: model_patches/
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
