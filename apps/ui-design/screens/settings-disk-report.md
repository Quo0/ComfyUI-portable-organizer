<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Settings: disk report

<!-- US-SHARED-09 -->

The section is called a report rather than "duplicates": duplicates are what it
found, not what it is. It only counts. There is not one button here that deletes
anything, and there never will be: freeing space is a separate, deliberate
action on its own screen, and the report explains where exactly that is done.

Walking tens of gigabytes across several builds takes minutes, so there is a bar
and the place being looked at right now is named: a pause with no caption reads
as a hang. An interrupted walk honestly marks the report incomplete, and folders
that could not be reached are listed separately — a folder skipped in silence
would turn the report into a lie.

"Same name, different size" is lifted out of the duplicates into a list of its
own and is not offered for anything: a matching name says nothing about the
contents, and presenting such files as duplicates would nudge someone into
deleting two different models.

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
        <div class="nav-item on"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <h3>Disk report</h3>
        <p class="t-sm">A report across every registered build and the shared folder. It only counts: nothing is deleted, moved or linked.</p>
        <div class="row">
          <span class="btn primary">Build the report</span>
        </div>
        <div class="group">
          <span class="t-label">Wasted on duplicates: 6.9 GB</span>
          <div class="dup-list">
            <div class="dup-row">
              <span class="nm">sd_xl_base_1.0.safetensors</span>
              <span class="tag">checkpoints</span>
              <span class="t-mono">6.5 GB</span>
              <span class="where hint">SDXL stable · Flux test</span>
            </div>
            <div class="dup-row">
              <span class="nm">ip-adapter-plus_sdxl.bin</span>
              <span class="tag">ipadapter</span>
              <span class="t-mono">0.8 GB</span>
              <span class="where hint">SDXL stable · Experiments · Video</span>
            </div>
            <div class="dup-row">
              <span class="nm">4x-UltraSharp.pth</span>
              <span class="tag">upscale_models</span>
              <span class="t-mono">64 MB</span>
              <span class="where hint">Upscale · Video</span>
            </div>
          </div>
          <p class="hint">Freeing this space is a separate, deliberate step: open a build and use the “Models of this build” panel.</p>
        </div>
        <div class="group">
          <span class="t-label">Same name, different size</span>
          <p class="hint">These are not duplicates: a matching name proves nothing about the contents. They are listed so you can look at them yourself.</p>
          <div class="dup-list">
            <div class="dup-row">
              <span class="nm">lora_style_v2.safetensors</span>
              <span class="tag">loras</span>
              <span class="t-mono"></span>
              <span class="where hint">SDXL stable · 144 MB · Experiments · 151 MB</span>
            </div>
          </div>
        </div>
        <p class="hint bad">Skipped, folder unavailable: E:\AI\Flux_clean</p>
      </div>
    </div>
  </div>
</Window>

## The walk is running

Minutes of work: saying nothing is not an option.

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
        <div class="nav-item on"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <h3>Disk report</h3>
        <p class="t-sm">A report across every registered build and the shared folder. It only counts: nothing is deleted, moved or linked.</p>
        <div class="row">
          <span class="btn primary" aria-disabled="true">Build the report</span>
          <span class="btn ghost">Cancel</span>
        </div>
        <div class="group">
          <div class="bar"><i style="width:38%"></i></div>
          <span class="hint">Looking through D:\AI\_shared\models\checkpoints…</span>
        </div>
      </div>
    </div>
  </div>
</Window>
