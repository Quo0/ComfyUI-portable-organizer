<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Exiting with builds running

<!-- US-RUN-08 -->

The servers are child processes of the app and leave with it. Closing silently
would mean cutting off someone's generation and losing unsaved graphs, so this
screen is shown instead of closing: what exactly is running, and two honest ways
out.

The fork is a real one, not a "yes / cancel": minimising to the tray is a full
answer, not a refusal to act. The red button names the consequence in words
rather than saying "OK". The list of what is running is not decorative: it
answers "what exactly am I about to stop" before the click rather than after.

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
    <h3>Servers are still running</h3>
    <p class="t-sm">Closing the app stops every running server: they are its child processes and leave with it. Unsaved graphs and the generation queue go with them.</p>
    <div class="cards">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="card-name">SDXL stable</span>
            <span class="pill running"><i></i>Running</span>
            <span class="t-mono">:8188</span>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="card-name">Flux test</span>
            <span class="pill starting"><i></i>Starting</span>
            <span class="t-mono">:8189</span>
          </div>
        </div>
      </div>
    </div>
    <div class="row">
      <span class="btn danger lg">Stop everything and exit</span>
      <span class="btn secondary lg">Minimize to tray</span>
    </div>
    <p class="hint">The app keeps running in the notification area — click its icon to bring it back.</p>
  </div>
</Window>
