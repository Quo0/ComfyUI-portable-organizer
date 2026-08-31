<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Settings: appearance

<!-- US-UI-01 · US-UI-02 -->

The first settings section and the shortest: theme, language and a sample of
what they change. The sample is not there for decoration — the language changes
not only the captions but numbers, dates and word forms, and none of that is
visible in a list of languages.

The theme is a segmented control rather than a list: there are three options,
they are visible at once, and the chosen one is visible without opening
anything. Dark comes first and is the default. "Follow system" is a state
rather than an absence of choice, so what the system has chosen right now is
said next to the control rather than on a line below it: the fact belongs to
that option, not to all three. The language is an ordinary list: there will be
more than three languages, and its labels are always in the language itself,
because hunting for yours in someone else's translation is awkward.

The last line answers a question that otherwise gets asked as a bug report:
ComfyUI inside the embedded tab has its own theme and its own language, and the
app does not touch them.

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
        <div class="nav-item on"><Palette class="ico" /><span>Appearance</span></div>
        <div class="nav-item"><Database class="ico" /><span>Shared models</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Workflow library</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Disk report</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Installer archives</span></div>
      </nav>
      <div class="content">
        <h3>Appearance</h3>
        <div class="group">
          <span class="t-label">Theme</span>
          <div class="row">
            <div class="seg">
              <span>Dark</span>
              <span>Light</span>
              <span aria-pressed="true">Follow system</span>
            </div>
            <span class="hint">Windows is currently set to dark.</span>
          </div>
        </div>
        <div class="group">
          <span class="t-label">Language</span>
          <div class="input" style="width:240px"><span>English</span></div>
        </div>
        <div class="group">
          <span class="t-label">Preview</span>
          <div class="pane">
            <div class="pane-head">
              <span class="title">Numbers, dates and word forms follow the selected language.</span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="row">
                <span class="pill running"><i></i>Running</span>
                <span class="pill stopped"><i></i>Stopped</span>
              </div>
              <div class="meta">
                <span>3 instances</span>
                <span>Size on disk: 52.4 GB</span>
                <span>Last run: today, 14:20</span>
              </div>
            </div></div>
          </div>
        </div>
        <p class="hint">ComfyUI keeps its own theme and language inside the embedded tab, and the app does not change them.</p>
      </div>
    </div>
  </div>
</Window>
