<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Shared resources

<!-- J-01 · step 5 · US-SHARED-01 · US-WF-02 -->

There are two resources and they are about different things: models and
workflows. It all used to go as one list of fields, and the only way to tell
what the next path referred to was its caption — hence two separate panels.

It is set up right here, with the same components as in "Settings": there is no
point dragging someone into another section for a single folder in the middle of
an install `US-SHARED-01/AC-4`.

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
      <span class="step now"><u>3</u>Shared resources</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Unpacking</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Done</span>
    </div>
    <div class="step-bar">
      <h3>Connect the new builds to shared models</h3>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Back</span>
        <span class="btn primary lg">Unpack</span>
      </span>
    </div>
    <div class="pane">
      <div class="pane-head"><span class="title">Shared models</span></div>
      <div class="scroll-pad">
        <div class="field">
          <span class="t-label">Shared models folder</span>
          <div class="path-row">
            <div class="input mono"><span>D:\AI\_shared\models</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <div class="hint">5 categories recognised · 231 GB</div>
        </div>
        <div class="toggle-row">
          <span class="toggle"></span>
          <div>
            <div class="t-base">Connect both new builds</div>
            <div class="hint">A model downloaded in one build becomes visible to all of them.</div>
          </div>
        </div>
        <!-- How to apply is shown only when the toggle is on: with nothing
             connected there is nothing to choose. -->
        <div class="field">
          <span class="t-label">How to apply</span>
          <div class="seg">
            <span aria-pressed="true">Leave the build alone</span>
            <span>Write a file into the build</span>
          </div>
          <div class="hint">The config stays with the app and is handed to the build at startup. Nothing is written into the build folder.</div>
        </div>
      </div>
    </div>
    <div class="pane">
      <div class="pane-head"><span class="title">Workflow library</span></div>
      <div class="scroll-pad">
        <div class="field">
          <span class="t-label">Library folder</span>
          <div class="path-row">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Browse</span>
          </div>
          <div class="hint">14 workflows</div>
        </div>
      </div>
    </div>
  </div>
</Window>
