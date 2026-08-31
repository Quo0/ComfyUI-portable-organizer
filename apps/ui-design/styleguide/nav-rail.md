<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Navigation rail

<!-- US-TAB-01 · US-TAB-03 · US-UI-04 -->

The only part of the interface that is always visible — including on the screen of a running ComfyUI.

Two blocks separated by a rule: the app's four sections and the list of running
instances in their accent colours. The second block solves two problems at
once — switching between builds in one click and, more importantly,
**visibility of failures**: when ComfyUI is expanded on screen, the rail is the
only place left where a crashed process is noticeable `US-UI-04/AC-2`. The
collapsed view leaves 56&nbsp;px of rail.

<ThemePair light="Light · expanded and collapsed" dark="Dark · expanded and collapsed">
  <div class="rails">
    <nav class="nav">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Running</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL stable</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux test</em><i class="dot" style="background:var(--state-starting)"></i></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Animation</em><span class="badge">!</span></div>
    </nav>
    <nav class="nav collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Instances</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Add build</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Settings</span></div>
      <div class="nav-item"><Info class="ico" /><span>About</span></div>
      <div class="nav-sep"></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL</em></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux</em></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Animation</em></div>
    </nav>
  </div>
</ThemePair>

## Second level: the settings sections

Inside "Settings" there is a list of its own — no icons and no collapsing.
Icons are what the rail needs, since it is always visible and has to read at
56&nbsp;px; here five captions side by side read faster than five pictures.
The workflow library sits exactly there: it is a folder outside the builds,
arranged just like the shared models.

<ThemePair>
  <nav class="settings-sections" style="width:200px">
    <div class="nav-item"><span>Appearance</span></div>
    <div class="nav-item"><span>Shared models</span></div>
    <div class="nav-item on"><span>Workflow library</span></div>
    <div class="nav-item"><span>Disk report</span></div>
    <div class="nav-item"><span>Installer archives</span></div>
  </nav>
</ThemePair>

## Long-string check · rail items at 208 px

<div class="longform">
  <div class="lf-head">Long-string check · rail items at 208 px</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Instances · Add build · Settings · About</span></div>
    <div class="lf-row"><b>RU</b><span>Инстансы · Добавление · Настройки · О приложении</span></div>
    <div class="lf-row"><b>ES</b><span>Instancias · Añadir · Configuración · Acerca de</span></div>
    <div class="lf-row"><b>ZH</b><span>实例 · 添加 · 设置 · 关于</span></div>
  </div>
</div>
