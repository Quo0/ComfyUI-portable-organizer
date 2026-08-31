<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Empty registry

<!-- J-01 · step 1 · US-ONB-01 -->

The first thing seen by someone who has nothing yet. Navigation is available
straight away — the app does not hide its structure behind a welcome screen.
The sections that need instances explain what to do first.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="row"><h3>Instances</h3></div>
    <div class="empty">
      <h4>No instances yet</h4>
      <p>The app launches portable ComfyUI builds, shows the startup log and opens the interface right in this window.</p>
      <!-- Exactly the same two paths as on the "Add build" screen:
           one component in two places. While they were described in
           place, one of the forks drifted into three different wordings. -->
      <div class="forks">
        <div class="fork">
          <b>I already have a folder</b>
          <p>Register a build that is already unpacked. The folder is left untouched.</p>
          <span class="btn secondary">Choose a folder</span>
        </div>
        <div class="fork">
          <b>Unpack from an archive</b>
          <p>Unpack a portable build into one or several destinations and register them at once.</p>
          <span class="btn primary">Choose a .7z archive…</span>
        </div>
      </div>
    </div>
  </div>
</Window>
