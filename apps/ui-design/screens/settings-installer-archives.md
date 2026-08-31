<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive, FolderOpen } from '@lucide/vue';
</script>

# Settings: installer archives

<!-- US-INST-07 -->

The commands "remember archives" and "forget an archive" existed from the very
start, and they had no screen: there was nowhere to see what the app remembers.
The section shows the list and exactly one action per entry.

"Remove from the list" removes the **entry**. The archive itself stays where it
was downloaded, and that is said right above the list — a button next to a file
name otherwise reads as deleting the file.

An archive that has gone does not vanish from the list in silence: the row stays
and says that the file is gone or has changed, and its "Open folder" button is
off. The user may have moved the archive themselves and has to see it.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item on"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <h3>Installer archives</h3>
        <p class="t-sm">Archives the install wizard has seen. Forgetting one removes the entry only — the file itself stays where you downloaded it.</p>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">
              ComfyUI_windows_portable_nvidia.7z
              <span class="hint">D:\downloads\ComfyUI_windows_portable_nvidia.7z</span>
            </span>
            <span class="val">9.7 GB · today, 12:04</span>
            <span class="acts">
              <span class="btn ghost icon"><FolderOpen class="ico" /></span>
              <span class="btn ghost">Remove from the list</span>
            </span>
          </div>
          <div class="path-item">
            <span class="lbl">
              ComfyUI_windows_portable_nvidia_v0.30.7z
              <span class="hint">D:\downloads\old\ComfyUI_windows_portable_nvidia_v0.30.7z</span>
            </span>
            <span class="val">9.1 GB · 12 March</span>
            <span class="acts">
              <span class="btn ghost icon"><FolderOpen class="ico" /></span>
              <span class="btn ghost">Remove from the list</span>
            </span>
          </div>
          <!-- The file is gone: the row stays, "Open folder" is off. -->
          <div class="path-item">
            <span class="lbl">
              ComfyUI_windows_portable_cpu.7z
              <span class="hint">E:\temp\ComfyUI_windows_portable_cpu.7z</span>
            </span>
            <span class="val">The file is gone or has changed</span>
            <span class="acts">
              <span class="btn ghost icon" aria-disabled="true"><FolderOpen class="ico" /></span>
              <span class="btn ghost">Remove from the list</span>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## The list is empty

The wizard has not remembered anything yet.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item on"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <h3>Installer archives</h3>
        <p class="t-sm">Archives the install wizard has seen. Forgetting one removes the entry only — the file itself stays where you downloaded it.</p>
        <!-- A line about the emptiness rather than an empty screen: the
             section has a heading and an explanation, and a large block
             under them would look like a separate screen dropped into the
             middle of the settings. -->
        <p class="blank">No archives yet — the wizard remembers them as you use them.</p>
      </div>
    </div>
  </div>
</Window>
