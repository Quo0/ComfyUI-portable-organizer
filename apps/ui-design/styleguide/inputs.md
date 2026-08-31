# Input fields

<!-- US-REG-01 · US-REG-02 · US-SHARED-04 -->

Path, name, port, colour — the four things set on every instance you add.

Picking the accent colour is the only field with a contrast check: the user is
free to take any of them, but offering one that is illegible from the start is
out `US-UI-01/AC-8`. Hence a set limited to eight, each of which clears the
threshold on both backgrounds.

<ThemePair>
  <div class="stack" style="gap:var(--space-4)">
    <div class="field">
      <label>Build folder</label>
      <div class="path-row">
        <div class="input mono"><span>D:\AI\ComfyUI_SDXL</span></div>
        <span class="btn secondary">Browse</span>
      </div>
      <div class="hint">Registering changes nothing inside the folder</div>
    </div>
    <div class="field">
      <label>Name</label>
      <div class="input">SDXL stable</div>
    </div>
    <div class="field">
      <label>Name</label>
      <div class="input bad"></div>
      <div class="hint bad">The name cannot be empty</div>
    </div>
    <div class="row" style="gap:var(--space-6); align-items:flex-start">
      <div class="field">
        <label>Port</label>
        <div class="input num mono">8188</div>
      </div>
      <div class="field" style="flex:1">
        <label>Accent colour</label>
        <div class="picker">
          <i style="background:var(--accent-ember)"></i>
          <i style="background:var(--accent-amber)"></i>
          <i style="background:var(--accent-moss)"></i>
          <i class="on" style="background:var(--accent-teal)"></i>
          <i style="background:var(--accent-azure)"></i>
          <i style="background:var(--accent-indigo)"></i>
          <i style="background:var(--accent-orchid)"></i>
          <i style="background:var(--accent-rose)"></i>
        </div>
      </div>
    </div>
    <div class="toggle-row">
      <span class="toggle"></span>
      <div>
        <div class="t-base">Download new models into the shared folder</div>
        <div class="hint">A model downloaded in one instance becomes visible to all of them</div>
      </div>
    </div>
    <div class="toggle-row">
      <span class="toggle off"></span>
      <div>
        <div class="t-base">Write a config file into the build</div>
        <div class="hint">Needed if the build is sometimes started bypassing the app</div>
      </div>
    </div>
  </div>
</ThemePair>

## Long-string check · field captions

<div class="longform">
  <div class="lf-head">Long-string check · field captions</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Download new models to the shared folder</span></div>
    <div class="lf-row"><b>RU</b><span>Скачивать новые модели в общую папку</span></div>
    <div class="lf-row"><b>ES</b><span>Descargar los modelos nuevos en la carpeta compartida</span></div>
    <div class="lf-row"><b>ZH</b><span>将新模型下载到共享文件夹</span></div>
  </div>
</div>
