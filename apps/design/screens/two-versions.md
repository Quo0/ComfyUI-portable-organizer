<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Two versions side by side

<!-- J-04 · step 5 · US-REG-03 · US-INST-07 -->

The app never updates a build in place: the new version is unpacked beside it
and the old one stays untouched. So that the versions can be told apart without
opening folders, the card shows which archive the instance was unpacked from
`US-INST-07/AC-5`.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Running</div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Animation</em><span class="badge">!</span></div>
    </nav>
  </template>
  <div class="content">
    <!-- There is no "Add" button here: builds are created by the "Add build"
         section, and there must not be two doors into one place. -->
    <div class="row">
      <h3>Instances</h3>
      <span class="t-sm">3</span>
      <span class="spacer"></span>
      <div class="input" style="width:220px">Search by name</div>
      <div class="seg">
        <span aria-pressed="true">by name</span>
        <span>by last run</span>
        <span>by size</span>
      </div>
    </div>
    <div class="cards grid">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
            <div class="card-name">SDXL stable</div>
            <span class="pill stopped"><i></i>Stopped</span>
          </div>
          <div class="card-desc">The working build, I do not touch its nodes</div>
          <div class="meta"><span>0.30.2</span><span>:8188</span><span>52.3 GB</span></div>
          <div class="src">
            <div>Unpacked from ComfyUI_windows_portable_nvidia_0.30.2.7z, 4 August</div>
            <div>Last run: yesterday, 19:40</div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">S</span>
            <div class="card-name">SDXL new version</div>
            <span class="pill stopped"><i></i>Stopped</span>
          </div>
          <div class="card-desc">Checking the update before moving over</div>
          <div class="meta"><span>0.31.0</span><span>:8189</span><span>9.6 GB</span></div>
          <div class="src">
            <div>Unpacked from ComfyUI_windows_portable_nvidia_0.31.0.7z, today</div>
            <div>Not started from here yet</div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-ember)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-ember)">A</span>
            <div class="card-name">Animation</div>
            <span class="pill crashed"><i></i>Crashed</span>
          </div>
          <div class="card-desc">Ran out of video memory on the second run</div>
          <div class="meta"><span>0.30.2</span><span>:8190</span><span>41.0 GB</span></div>
          <div class="src">
            <div>Unpacked from ComfyUI_windows_portable_nvidia_0.30.2.7z, 12 March</div>
            <div>Last run: today, 11:05</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## A realistic amount of data

*scrolling · thirteen instances; the rail and the heading are pinned*

<Window fixed scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
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
  <div class="content framed no-foot">
    <div class="pinned">
      <div class="row">
        <h3>Instances</h3>
        <span class="t-sm">13</span>
        <span class="spacer"></span>
        <div class="input" style="width:220px">Search by name</div>
        <div class="seg">
          <span aria-pressed="true">by name</span>
          <span>by last run</span>
          <span>by size</span>
        </div>
      </div>
    </div>
    <div class="data">
      <div class="scroll"><div class="scroll-pad">
        <div class="cards grid">
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-teal)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><div class="card-name">SDXL stable</div><span class="pill running"><i></i>Running</span></div>
          <div class="card-desc">The working build, I do not touch its nodes</div>
          <div class="meta"><span>0.30.2</span><span>:8188</span><span>52.3 GB</span><span class="tag">shared models</span></div>
          <div class="src"><div>Last run: today, 14:20</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><div class="card-name">Flux test</div><span class="pill starting"><i></i>Starting</span></div>
          <div class="card-desc">This is where I try new nodes out</div>
          <div class="meta"><span>0.31.0</span><span>:8189</span><span>18.7 GB</span><span class="tag">shared models</span></div>
          <div class="src"><div>Last run: just now</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-ember)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><div class="card-name">Animation</div><span class="pill crashed"><i></i>Crashed</span></div>
          <div class="card-desc">Ran out of video memory on the second run</div>
          <div class="meta"><span>0.30.2</span><span>:8190</span><span>41.0 GB</span></div>
          <div class="src"><div>Last run: today, 11:05</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-moss)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-moss)">E</span><div class="card-name">Experiments</div><span class="pill stopped"><i></i>Stopped</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.29.4</span><span>:8191</span><span>9.4 GB</span></div>
          <div class="src"><div>Last run: 3 days ago</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-azure)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-azure)">V</span><div class="card-name">Video</div><span class="pill running"><i></i>Running</span></div>
          <div class="card-desc">Long clips, a separate set of nodes</div>
          <div class="meta"><span>0.30.2</span><span>:8192</span><span>77.1 GB</span></div>
          <div class="src"><div>Last run: today, 09:12</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-orchid)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-orchid)">I</span><div class="card-name">Inpaint</div><span class="pill stopped"><i></i>Stopped</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.30.0</span><span>:8193</span><span>12.8 GB</span></div>
          <div class="src"><div>Last run: 12 March</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-rose)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-rose)">U</span><div class="card-name">Upscale</div><span class="pill stopped"><i></i>Stopped</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.29.1</span><span>:8194</span><span>6.2 GB</span></div>
          <div class="src"><div>Not started from here yet</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-amber)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-amber)">N</span><div class="card-name">Node tests</div><span class="pill running"><i></i>Running</span></div>
          <div class="card-desc">I install anything here, no regrets</div>
          <div class="meta"><span>0.31.0</span><span>:8195</span><span>15.0 GB</span></div>
          <div class="src"><div>Last run: yesterday, 22:41</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-teal)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><div class="card-name">SDXL new version</div><span class="pill stopped"><i></i>Stopped</span></div>
          <div class="card-desc">Checking the update before moving over</div>
          <div class="meta"><span>0.31.0</span><span>:8196</span><span>9.6 GB</span></div>
          <div class="src"><div>Not started from here yet</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-moss)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-moss)">L</span><div class="card-name">LoRAs</div><span class="pill stopped"><i></i>Stopped</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.30.2</span><span>:8197</span><span>21.4 GB</span><span class="tag">shared models</span></div>
          <div class="src"><div>Last run: 4 August</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-azure)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-azure)">C</span><div class="card-name">ControlNets</div><span class="pill stopped"><i></i>Stopped</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.29.4</span><span>:8198</span><span>7.9 GB</span></div>
          <div class="src"><div>Last run: 28 July</div></div>
        </div></div>
        <div class="card gone"><div class="card-accent"></div><div class="card-in">
          <div class="card-top"><span class="chip">V</span><div class="card-name">Video (external drive)</div><span class="pill gone"><i></i>Folder missing</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>E:\comfy\video</span></div>
          <div class="src"><div>Last run: 12 March</div></div>
        </div></div>
        </div>
      </div></div>
    </div>
  </div>
</Window>
