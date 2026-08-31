<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Done

<!-- J-01 · step 7 · US-INST-06 · US-INST-07 -->

The result shows what appeared rather than what happened: the cards of the
created builds are clickable and lead to their screens. From here, too, comes a
second run straight away, if not everything that was planned got installed.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="steps">
      <span class="step done"><u>✓</u>Archive</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Destinations</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Shared resources</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Unpacking</span><span class="step-sep"></span>
      <span class="step now"><u>5</u>Done</span>
    </div>
    <div class="step-bar">
      <h3>Ready</h3>
      <span class="t-label">2 instances added</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost">Add another</span>
        <span class="btn primary lg">Go to instances</span>
      </span>
    </div>
    <div class="cards grid">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
            <div class="card-name">Flux test</div>
            <span class="pill stopped"><i></i>Stopped</span>
          </div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.31.0</span><span>:8188</span><span class="tag">shared models</span></div>
          <div class="src">D:\AI\Flux</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-moss)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-moss)">F</span>
            <div class="card-name">Flux clean</div>
            <span class="pill stopped"><i></i>Stopped</span>
          </div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.31.0</span><span>:8189</span><span class="tag">shared models</span></div>
          <div class="src">E:\AI\Flux_clean</div>
        </div>
      </div>
    </div>
  </div>
</Window>
