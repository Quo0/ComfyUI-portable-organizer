<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Build list

<!-- revision · US-REG-04 · US-RUN-05 -->

The cards go in a grid: on a maximised window a full-width column left empty
space on the right while things that would fit beside each other went down
instead. The header carries search and ordering — at five to eight builds the
list stops being readable by eye.

There is no "Add" button here: the "Add build" section is where builds are
created, and there must not be two doors into one place. The card shows what
tells builds apart: state, port, version, size and when it was last started.

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
      <span class="t-sm">5</span>
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
            <span class="pill running"><i></i>Running</span>
          </div>
          <div class="card-desc">The working build, I do not touch its nodes</div>
          <div class="meta">
            <span>0.30.2</span><span>:8188</span><span>52.4 GB</span>
            <span class="tag">shared models</span>
          </div>
          <div class="src">Last run: today, 14:20</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
            <div class="card-name">Flux test</div>
            <span class="pill starting"><i></i>Starting</span>
          </div>
          <div class="card-desc">This is where I try new nodes out</div>
          <div class="meta">
            <span>0.31.0</span><span>:8189</span><span>38.1 GB</span>
            <span class="tag">shared models</span>
          </div>
          <div class="src">Last run: just now</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-moss)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-moss)">E</span>
            <div class="card-name">Experiments</div>
            <span class="pill stopped"><i></i>Stopped</span>
          </div>
          <!-- There is no description, but the line is here: without it the
               versions in neighbouring cards would sit at different heights. -->
          <div class="card-desc"></div>
          <div class="meta">
            <span>0.29.4</span><span>:8190</span><span>12.8 GB</span>
          </div>
          <div class="src">Last run: 3 days ago</div>
        </div>
      </div>
      <div class="card gone">
        <div class="card-accent"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip">V</span>
            <div class="card-name">Video</div>
            <span class="pill gone"><i></i>Folder missing</span>
          </div>
          <div class="card-desc"></div>
          <div class="meta"><span>E:\comfy\video</span></div>
          <div class="src">Last run: 12 March</div>
        </div>
      </div>
    </div>
  </div>
</Window>
