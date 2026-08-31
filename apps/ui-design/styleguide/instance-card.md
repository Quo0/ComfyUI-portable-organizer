# Instance card

<!-- US-REG-03 · US-INST-07 · US-RUN-06 -->

The most frequent and the most data-dense element in the app.

There are five states, not two. An unavailable instance — the folder is gone or
the drive is disconnected — **does not vanish from the list**, it is marked with
a dashed outline `US-REG-03/AC-7`: silently dropping the entry is not allowed,
the user has to understand what happened. The source line shows which archive
the instance was unpacked from — that is what tells apart two versions standing
side by side `US-INST-07/AC-5`.

<ThemePair light="Light · five states" dark="Dark · five states">
  <div class="cards">
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">SDXL stable</div><span class="pill running"><i></i>Running</span></div>
        <div class="card-desc">The working build, I do not touch its nodes</div>
        <div class="meta"><span>ComfyUI 0.30.2</span><span>Python 3.13.12</span><span>:8188</span><span>52.3 GB</span></div>
        <div class="src">From archive <code>portable_nvidia_0.30.2.7z</code></div>
      </div>
    </div>
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Flux test</div><span class="pill starting"><i></i>Starting</span></div>
        <div class="card-desc">Trying new nodes out before moving them over</div>
        <div class="bar"><i></i></div>
        <div class="meta"><span>ComfyUI 0.31.0</span><span>Python 3.13.12</span><span>:8189</span><span>18.7 GB</span></div>
      </div>
    </div>
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-ember)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Animation</div><span class="pill crashed"><i></i>Crashed</span></div>
        <div class="card-desc">Ran out of video memory on the second run</div>
        <div class="meta"><span>ComfyUI 0.30.2</span><span>Python 3.13.12</span><span>:8190</span><span>41.0 GB</span></div>
      </div>
    </div>
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-moss)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Experiments</div><span class="pill stopped"><i></i>Stopped</span></div>
        <div class="card-desc">A sandbox for other people's workflows</div>
        <div class="meta"><span>ComfyUI 0.29.4</span><span>Python 3.12.8</span><span>:8191</span><span>9.4 GB</span></div>
      </div>
    </div>
    <div class="card gone">
      <div class="card-accent"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Archive on the external drive</div><span class="pill gone">Folder missing</span></div>
        <div class="card-desc">Folder not found — the drive is disconnected</div>
        <div class="meta"><span>E:\ComfyUI\Legacy</span></div>
      </div>
    </div>
  </div>
</ThemePair>

## Long-string check · state captions

<div class="longform">
  <div class="lf-head">Long-string check · state captions</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Stopped · Starting · Running · Crashed · Unavailable</span></div>
    <div class="lf-row"><b>RU</b><span>Остановлен · Стартует · Работает · Аварийно завершён · Недоступен</span></div>
    <div class="lf-row"><b>ES</b><span>Detenido · Iniciando · En ejecución · Finalizado con error · No disponible</span></div>
    <div class="lf-row"><b>ZH</b><span>已停止 · 启动中 · 运行中 · 异常终止 · 不可用</span></div>
  </div>
</div>
