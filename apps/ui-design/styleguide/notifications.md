# Notifications

<!-- US-UI-03 · US-UI-05 -->

Success disappears on its own. An error waits until it has been read.

The difference in behaviour is not cosmetic but a requirement: a successful
operation is acknowledged and forgotten, while an error message has to be read
to the end, expanded and copied together with its code
`US-UI-03/AC-3…AC-5`. The error code is always shown — it is what the user
searches the documentation with, even when the message itself is untranslated.

<ThemePair>
  <div class="toasts">
    <div class="toast ok">
      <i></i>
      <div class="toast-in">
        <div class="toast-head"><b>Instance added</b><span class="close">✕</span></div>
        <p>SDXL stable — launch profiles recognised</p>
        <div class="toast-life"><i></i></div>
      </div>
    </div>
    <div class="toast err">
      <i></i>
      <div class="toast-in">
        <div class="toast-head"><b>Could not write the settings</b><span class="close">✕</span></div>
        <p>The build folder is read-only. Shared models can be connected the other way — without writing into the folder.</p>
        <div class="disclosure"><span>Details</span><code>ERR_INSTANCE_READONLY</code><span style="margin-left:auto">Copy</span></div>
      </div>
    </div>
  </div>
</ThemePair>

### Progress — only where the operation really is long

There is no permanent indicator on every action: nearly everything in the app is
instant, and a bar that flickers past is just noise `US-UI-05/AC-6`. The
determinate one shows fractions when the amount of work is known; the
indeterminate one only when it is not, and without a false scale.

<ThemePair>
  <div class="stack" style="gap:var(--space-5)">
    <div class="prog">
      <div class="prog-head"><span>Unpacking · D:\AI\Flux</span><span class="count">1 / 2</span></div>
      <div class="track"><i style="width:64%"></i></div>
      <div class="prog-file">python_embeded\Lib\site-packages\torch\_inductor\kernel\mm.py</div>
    </div>
    <div class="prog">
      <div class="prog-head"><span>Walking the shared models folder</span><span class="count">412 GB</span></div>
      <div class="track indet"><i></i></div>
      <div class="prog-file">the duration is not known in advance</div>
    </div>
  </div>
</ThemePair>

## Long-string check · notification texts

<div class="longform">
  <div class="lf-head">Long-string check · notification texts</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Could not write settings — the instance folder is read-only</span></div>
    <div class="lf-row"><b>RU</b><span>Не удалось записать настройки — папка инстанса доступна только для чтения</span></div>
    <div class="lf-row"><b>ES</b><span>No se pudo escribir la configuración: la carpeta de la instancia es de solo lectura</span></div>
    <div class="lf-row"><b>ZH</b><span>无法写入设置——实例文件夹为只读</span></div>
  </div>
</div>
