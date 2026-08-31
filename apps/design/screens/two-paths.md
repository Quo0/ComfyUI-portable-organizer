<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Two paths

<!-- revision · US-ONB-02 · US-REG-01 -->

The section is permanent rather than first-run only, and it is the only place
where builds are created: the "Add" button was taken off the instance list,
there must not be two doors into one room.

The paths run left to right rather than one under the other: this is a fork,
and both branches have to be visible at once for there to be anything to choose
between.

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
    <h3>Add a build</h3>
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
</Window>
