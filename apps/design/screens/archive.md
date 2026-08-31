<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Archive

<!-- J-01 · step 3 · US-INST-01 · US-INST-03 -->

Choosing an archive leads straight to the second step: what is inside the
archive and how much room it needs are visible there, in the pinned header —
that is, before anything begins. Not enough space stops the install before the
unpacking rather than halfway through it `US-INST-03/AC-2`.

Parsing an index of fifty-six thousand entries takes more than a second, and
the caption about it stands **next to the button** that was just clicked rather
than as a separate block below.

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
      <span class="step now"><u>1</u>Archive</span><span class="step-sep"></span>
      <span class="step"><u>2</u>Destinations</span><span class="step-sep"></span>
      <span class="step"><u>3</u>Shared resources</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Unpacking</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Done</span>
    </div>
    <!-- The step row is there on the first step too, for the sake of the
         single way out of the wizard. The wizard has no screen header of
         its own: its "Back" and heading would repeat this row. -->
    <div class="step-bar">
      <h3>Archive</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Back</span></span>
    </div>
    <p>Download the portable build archive yourself — the app does not fetch it.</p>
    <!-- The caption about reading the index is in the same row as the
         button: as a separate block below it looked like an answer to
         something else. -->
    <div class="row">
      <span class="btn primary">Choose a .7z archive…</span>
      <span class="hint">Reading the archive index…</span>
      <span class="bar indet grow"><i></i></span>
    </div>
    <!-- Recent ones as cards rather than rows: the path to an archive is
         long and does not fit on a line, and it is exactly what tells apart
         two builds of one version sitting in different folders. -->
    <div class="group">
      <span class="t-label">Recently used</span>
      <div class="cards grid">
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## A realistic amount of data

*scrolling · the archive history grows with every new version*

<!-- Neither a footer with "Cancel/Next" nor the chosen archive in the
     header: the buttons that move on to 4b went into the step row, and
     choosing an archive leads straight to the second step — lingering on
     the first one with an archive already chosen is impossible. -->
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
        <span class="step now"><u>1</u>Archive</span><span class="step-sep"></span>
        <span class="step"><u>2</u>Destinations</span><span class="step-sep"></span>
        <span class="step"><u>3</u>Shared resources</span><span class="step-sep"></span>
        <span class="step"><u>4</u>Unpacking</span><span class="step-sep"></span>
        <span class="step"><u>5</u>Done</span>
      </div>
      <div class="step-bar">
        <h3>Archive</h3>
        <span class="spacer"></span>
        <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Back</span></span>
      </div>
      <p>Download the portable build archive yourself — the app does not fetch it.</p>
      <div class="row">
        <span class="btn primary">Choose a .7z archive…</span>
      </div>
      <span class="t-label">Recently used</span>
    </div>
    <div class="scroll"><div class="scroll-pad">
      <div class="cards grid">
        <!-- The history grows with every new build version: over a year
             two dozen of them pile up. -->
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2.2 GB</span><span>today</span></div>
            <div class="row"><span class="btn secondary">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">The file is gone or has changed</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1.9 GB</span><span>30 May</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Next</span><span class="btn ghost">Remove from the list</span></div>
          </div>
        </div>
      </div>
    </div></div>
  </div>
</Window>
