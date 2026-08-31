<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, ScrollText, FolderOpen, ExternalLink, RotateCw } from '@lucide/vue';
</script>

# Instance running

<!-- J-01 · step 9 · US-TAB-02 · US-TAB-06 -->

This screen is what the whole thing was for: ComfyUI inside the window rather
than in a browser tab. The canvas area is taken by a **native window on top of
our interface** — which is why no menu, popover or toast of ours can cover it.
That is where the whole layout comes from: the rail on the left, the instance's
tools in a strip along the top, and nothing floating.

**The only screen where scrolling is forbidden.** The position of the native
window is set by the `set_webview_bounds` command. If the container of this
area scrolled, the rectangle would drift away from the content: the webview
would stay put while the markup crawled off. So the toolbar is pinned, the
canvas takes all the room that is left, and not one parent has a scroll of its
own.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL</em></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux</em></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Animation</em></div>
    </nav>
  </template>
  <div class="content flush">
    <!-- Icons rather than captions: six text buttons in a row did not fit on
         a narrow window. Only stopping and restarting kept their words —
         the cost of a mistake there beats the space saved. -->
    <div class="inst-toolbar">
      <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <span class="name">SDXL stable</span>
      <span class="port">127.0.0.1:8188</span>
      <span class="pill running"><i></i>Running</span>
      <span class="spacer"></span>
      <span class="tools">
        <span class="btn ghost icon"><ScrollText class="ico" /></span>
        <span class="btn ghost icon"><FolderOpen class="ico" /></span>
        <span class="btn ghost icon"><ExternalLink class="ico" /></span>
        <span class="btn ghost icon"><RotateCw class="ico" /></span>
      </span>
      <span class="btn secondary">Restart</span>
      <span class="btn danger">Stop</span>
    </div>
    <div class="comfy">
      <div class="comfy-node" style="left:8%; top:16%">
        <b>Load Checkpoint</b><span>sdxl_base_1.0</span>
      </div>
      <div class="comfy-node" style="left:44%; top:34%">
        <b>KSampler</b><span>steps 28 · cfg 6.5</span>
      </div>
      <div class="comfy-node" style="left:74%; top:14%">
        <b>Save Image</b><span>ComfyUI_00042_</span>
      </div>
      <div class="comfy-wire" style="left:22%; top:26%; width:22%"></div>
      <div class="comfy-wire" style="left:57%; top:26%; width:17%"></div>
      <span class="comfy-label">the embedded ComfyUI tab area</span>
    </div>
  </div>
</Window>
