# Карточка инстанса

<!-- US-REG-03 · US-INST-07 · US-RUN-06 -->

Самый частый и самый насыщенный данными элемент приложения.

Состояний пять, а не два. Недоступный инстанс — папка исчезла или диск
отключён — **не пропадает из списка**, а помечается пунктиром
`US-REG-03/AC-7`: молча удалять запись нельзя, пользователь
должен понять, что произошло. Строка источника показывает, из какого архива
развёрнут инстанс, — по ней различаются версии, стоящие рядом
`US-INST-07/AC-5`.

<ThemePair light="Светлая · пять состояний" dark="Тёмная · пять состояний">
  <div class="cards">
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">SDXL стабильная</div><span class="pill running"><i></i>Работает</span></div>
        <div class="card-desc">Рабочая сборка, ноды не трогаю</div>
        <div class="meta"><span>ComfyUI 0.30.2</span><span>Python 3.13.12</span><span>:8188</span><span>52.3 GB</span></div>
        <div class="src">Из архива <code>portable_nvidia_0.30.2.7z</code></div>
      </div>
    </div>
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Flux тест</div><span class="pill starting"><i></i>Стартует</span></div>
        <div class="card-desc">Проверяю новые ноды перед переносом</div>
        <div class="bar"><i></i></div>
        <div class="meta"><span>ComfyUI 0.31.0</span><span>Python 3.13.12</span><span>:8189</span><span>18.7 GB</span></div>
      </div>
    </div>
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-ember)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Анимация</div><span class="pill crashed"><i></i>Аварийно завершён</span></div>
        <div class="card-desc">Не хватило видеопамяти при втором запуске</div>
        <div class="meta"><span>ComfyUI 0.30.2</span><span>Python 3.13.12</span><span>:8190</span><span>41.0 GB</span></div>
      </div>
    </div>
    <div class="card">
      <div class="card-accent" style="--instance-accent:var(--accent-moss)"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Эксперименты</div><span class="pill stopped"><i></i>Остановлен</span></div>
        <div class="card-desc">Песочница под чужие воркфлоу</div>
        <div class="meta"><span>ComfyUI 0.29.4</span><span>Python 3.12.8</span><span>:8191</span><span>9.4 GB</span></div>
      </div>
    </div>
    <div class="card gone">
      <div class="card-accent"></div>
      <div class="card-in">
        <div class="card-top"><div class="card-name">Архив на внешнем диске</div><span class="pill gone">Недоступен</span></div>
        <div class="card-desc">Папка не найдена — диск отключён</div>
        <div class="meta"><span>E:\ComfyUI\Legacy</span></div>
      </div>
    </div>
  </div>
</ThemePair>

## Проверка на длинных строках · подписи состояний

<div class="longform">
  <div class="lf-head">Проверка на длинных строках · подписи состояний</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Stopped · Starting · Running · Crashed · Unavailable</span></div>
    <div class="lf-row"><b>RU</b><span>Остановлен · Стартует · Работает · Аварийно завершён · Недоступен</span></div>
    <div class="lf-row"><b>ES</b><span>Detenido · Iniciando · En ejecución · Finalizado con error · No disponible</span></div>
    <div class="lf-row"><b>ZH</b><span>已停止 · 启动中 · 运行中 · 异常终止 · 不可用</span></div>
  </div>
</div>
