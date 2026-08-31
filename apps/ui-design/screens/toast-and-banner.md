<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, ScrollText, FolderOpen, ExternalLink, RotateCw } from '@lucide/vue';
</script>

# Messages: toast and banner

<!-- US-RUN-06 -->

Two surfaces for the same thing — saying what happened — and the choice between
them is not a matter of taste. A toast pops up in the corner of the content
area and leaves on its own. It works where our own markup is underneath it.

An error arrives from the backend as a code, not as text: `AppError` carries a
`code` and its substitutions, while the text lives in the locales. Hence the
toast's "Details" and "Copy" — whoever receives what was copied recognises the
error regardless of the sender's language. Repeats do not stack to the ceiling:
identical ones fold into a single toast with a counter.

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
    <div class="row">
      <h3>Instances</h3>
      <span class="t-sm">2</span>
      <span class="spacer"></span>
      <div class="input" style="width:220px">Search by name</div>
    </div>
    <div class="cards grid">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
            <div class="card-name">SDXL stable</div>
            <span class="pill running"><i></i>Running</span>
          </div>
          <div class="card-desc">The working build, I do not touch its nodes</div>
          <div class="meta"><span>0.30.2</span><span>:8188</span><span>52.4 GB</span></div>
          <div class="src">Last run: today, 14:20</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
            <div class="card-name">Flux test</div>
            <span class="pill stopped"><i></i>Stopped</span>
          </div>
          <div class="card-desc">This is where I try new nodes out</div>
          <div class="meta"><span>0.31.0</span><span>38.1 GB</span></div>
          <div class="src">Last run: yesterday, 21:03</div>
        </div>
      </div>
    </div>
    <!-- The stack grows upwards, the freshest toast at the bottom — by the
         edge where the cursor is, and closest to what was just clicked. -->
    <div class="toasts win-toasts">
      <div class="toast ok">
        <i></i>
        <div class="toast-in">
          <div class="toast-head">
            <b>The workflow is in the build</b>
            <span class="close">✕</span>
          </div>
          <p>The build shows it after you refresh the ComfyUI page.</p>
        </div>
      </div>
      <!-- An error with details and a repeat counter. The details are folded
           by default: the code and the substitutions are for someone about
           to show them to another person, not for everyone. -->
      <div class="toast err">
        <i></i>
        <div class="toast-in">
          <div class="toast-head">
            <b>Could not read the folder</b>
            <span class="badge">×2</span>
            <span class="close">✕</span>
          </div>
          <div class="row">
            <span class="btn ghost">Details</span>
            <span class="btn ghost">Copy</span>
          </div>
          <pre class="toast-details">Could not read the folder D:\AI\_shared\models
[shared.readFailed]
  path: D:\AI\_shared\models</pre>
        </div>
      </div>
    </div>
  </div>
</Window>

## The banner

*where a toast is physically impossible*

On the embedded tab screen the native ComfyUI window lies on top of our HTML. A
toast would pop up in the corner of the content area — that is, underneath it,
and the user would never see it. So the message here goes as a banner **above**
the tab's rectangle, in the flow, taking its height from the canvas. The
rectangle is recomputed: the frontend already computes it on every layout
change anyway.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="inst-toolbar">
      <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
      <span class="chip" style="--instance-accent:var(--accent-ember)">A</span>
      <span class="name">Animation</span>
      <span class="port">127.0.0.1:8190</span>
      <span class="pill crashed"><i></i>Crashed</span>
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
    <div class="banner bad">
      <b>The server stopped on its own, exit code 3221225477.</b>
      <p>The last lines of the log are in the console; the tab stayed where it was.</p>
      <span class="spacer"></span>
      <span class="btn secondary">Show the log</span>
    </div>
    <div class="comfy">
      <div class="comfy-node" style="left:10%; top:20%">
        <b>Load Checkpoint</b><span>animatediff_v3</span>
      </div>
      <div class="comfy-node" style="left:52%; top:40%">
        <b>KSampler</b><span>steps 20 · cfg 7</span>
      </div>
      <div class="comfy-wire" style="left:24%; top:30%; width:28%"></div>
      <span class="comfy-label">the embedded ComfyUI tab area</span>
    </div>
  </div>
</Window>
