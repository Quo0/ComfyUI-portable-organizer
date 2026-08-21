<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Прокрутка

<!-- NFR-420 · NFR-430 · NFR-440 -->

Окно фиксированной высоты, данные не ограничены. Что закреплено, а что уезжает — это архитектура экрана.

Правило одно: **окно не прокручивается целиком**. Рейл
закреплён, внутри контентной области ровно один вертикальный скролл —
область данных, а всё управляющее из неё вынесено. Уехавший подвал
мастера делает мастер непроходимым; уехавшая панель деталей делает
выбор невидимым. Вложенная прокрутка допускается только в раскладке
«список — детали»: два независимых региона рядом.

### Каркас: закреплённый верх → область данных → закреплённый подвал

<ThemePair>
  <div class="pane" style="height:220px">
    <div class="pane-head"><span class="title">Назначения</span><span class="t-label">6</span></div>
    <div class="scroll"><div class="scroll-pad">
      <div class="path-item"><span class="lbl">D:\AI\Flux</span><span class="val">Flux тест</span></div>
      <div class="path-item"><span class="lbl">E:\AI\Flux_clean</span><span class="val">Flux чистый</span></div>
      <div class="path-item"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span></div>
      <div class="path-item"><span class="lbl">E:\AI\Sandbox</span><span class="val">Песочница</span></div>
      <div class="path-item"><span class="lbl">D:\AI\Video</span><span class="val">Видео</span></div>
      <div class="path-item"><span class="lbl">E:\AI\Archive_030</span><span class="val">Архив 0.30</span></div>
    </div></div>
    <div class="pane-foot"><span class="btn ghost">Назад</span><span class="btn primary">Далее</span></div>
  </div>
</ThemePair>

### Список и детали — две независимые прокрутки

<ThemePair>
  <div class="split-master" style="height:220px">
    <div class="pane">
      <div class="pane-head"><span class="title">Воркфлоу</span><span class="t-label">214</span></div>
      <div class="scroll"><div class="scroll-pad" style="gap:1px">
        <div class="wf-row"><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span><span class="star">★</span></div>
        <div class="wf-row"><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span><span class="star">★</span></div>
        <div class="wf-row"><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span><span></span></div>
        <div class="wf-row"><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span><span></span></div>
        <div class="wf-row"><span class="nm">utils / batch-rename.json</span><span class="tags"></span><span></span></div>
        <div class="wf-row"><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span><span></span></div>
      </div></div>
    </div>
    <div class="pane">
      <div class="pane-head"><span class="title">Выбран</span></div>
      <div class="scroll"><div class="scroll-pad">
        <div class="t-sm">flux / portrait-v3.json</div>
        <div class="t-label">Заметка</div>
        <p class="t-sm" style="margin:0">Портрет с двумя LoRA и апскейлом. Работает только там, где стоит IPAdapter.</p>
        <div class="t-label">Совместимость</div>
        <div class="compat-note" style="color:var(--state-starting)">нет 2 нод в «Flux тест»</div>
      </div></div>
      <div class="pane-foot"><span class="btn primary">Добавить</span></div>
    </div>
  </div>
</ThemePair>

Прокрутка списка не двигает панель деталей, и наоборот. При узком окне
раскладка схлопывается в одну колонку, и детали становятся отдельным
экраном — иначе на панель в 300&nbsp;px не останется места.

### Консоль: следование за лентой приостановлено

Пока пользователь внизу, консоль следует за новыми строками. Как только
он прокрутил вверх, следование **приостанавливается** —
иначе лог выдёргивает текст из-под курсора при каждой новой строке.
Кнопка возврата показывает, сколько строк накопилось.

<ThemePair>
  <div class="log" style="height:180px">
    <div class="console">Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Loading custom nodes: 47 found
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
<span class="dim">  ... ещё 43</span>
Import times for custom nodes:
   0.1 seconds: ComfyUI-Manager
   2.4 seconds: was-node-suite-comfyui
Starting server on 127.0.0.1:8188</div>
    <span class="log-follow">К последним строкам <span class="n">+128</span></span>
  </div>
</ThemePair>

### Рейл: разделы закреплены, запущенные прокручиваются

<ThemePair>
  <nav class="nav" style="height:240px">
    <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
    <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
    <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
    <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    <div class="nav-sep"></div>
    <div class="nav-note">Запущены · 8</div>
    <div class="nav-runs">
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL стабильная</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux тест</em><i class="dot" style="background:var(--state-starting)"></i></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Анимация</em><span class="badge">!</span></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-moss)">Э</span><em>Эксперименты</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-azure)">В</span><em>Видео</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-orchid)">И</span><em>Инпейнт</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-rose)">У</span><em>Апскейл</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-amber)">Т</span><em>Тесты нод</em><i class="dot" style="background:var(--state-running)"></i></div>
    </div>
  </nav>
</ThemePair>

<div class="longform">
  <div class="lf-head">Механика, которую легко сделать неправильно</div>
  <div class="lf-rows">
    <div class="lf-row"><b>1</b><span><code>min-height: 0</code> на каждом звене цепочки — иначе grid-элемент не сожмётся ниже содержимого и скролл не появится вовсе</span></div>
    <div class="lf-row"><b>2</b><span><code>overscroll-behavior: contain</code> — иначе докрутка списка до конца начинает прокручивать страницу за ним</span></div>
    <div class="lf-row"><b>3</b><span>Закрепление через <code>position: sticky</code> внутри панели, а не <code>fixed</code> — тот привязался бы к окну</span></div>
    <div class="lf-row"><b>4</b><span>Полоса прокрутки из токенов: системная светлая в тёмном интерфейсе выглядит чужеродно</span></div>
  </div>
</div>
