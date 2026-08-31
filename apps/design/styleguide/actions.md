# Actions

<!-- US-RUN-01 · US-REG-05 · NFR-300 -->

The primary action is neutral: a brand colour would argue with the instance accents.

The primary action button is filled with neutral ink rather than a brand
colour, because colour in this app is already taken: the user picks one for
every instance. Dangerous actions are outlined rather than filled — removing an
entry from the registry must not look more inviting than starting a build.

<ThemePair>
  <div class="stack" style="gap:var(--space-4)">
    <div class="row">
      <span class="btn primary">Start</span>
      <span class="btn secondary">Open folder</span>
      <span class="btn ghost">Cancel</span>
      <span class="btn danger">Remove from the list</span>
    </div>
    <div class="row">
      <span class="btn primary lg">Install ComfyUI</span>
      <span class="btn secondary"><i class="spin"></i>Stopping</span>
      <span class="btn secondary" aria-disabled="true">Start</span>
    </div>
    <div class="row">
      <span class="split">
        <span class="btn primary">Start</span>
        <span class="btn primary">▾</span>
      </span>
      <span class="t-label">split button for choosing a profile</span>
    </div>
  </div>
</ThemePair>

The profile dropdown on the split button opens **only while the instance is not
running**. Once it starts, the content area is taken by the native ComfyUI
window, and our list physically cannot float above it — which is why it is
shown closed here.

## Long-string check · button captions

<div class="longform">
  <div class="lf-head">Long-string check · button captions</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Launch · Open folder · Remove from registry</span></div>
    <div class="lf-row"><b>RU</b><span>Запустить · Открыть папку · Убрать из реестра</span></div>
    <div class="lf-row"><b>ES</b><span>Iniciar · Abrir carpeta · Quitar del registro</span></div>
    <div class="lf-row"><b>ZH</b><span>启动 · 打开文件夹 · 从注册表移除</span></div>
  </div>
</div>
