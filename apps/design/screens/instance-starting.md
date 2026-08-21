<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Инстанс стартует

<!-- J-01 · шаг 8 · US-RUN-03 -->

Холодный старт с большим набором нод занимает минуты, и это штатно.
Логи текут в реальном времени, потому что иначе первые минуты выглядят как
зависание. Строка загрузки моделей перерисовывается на месте, а не плодит
тысячи строк.

Экран тот же, что и у остановленной сборки, — со своей шапкой
и вкладками. Отличий два: лог раскрывается сам, потому что в эти минуты
он и есть содержимое экрана, и под шапкой идёт полоса без доли
выполненного.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Запущены</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux тест</em><i class="dot" style="background:var(--state-starting)"></i></div>
    </nav>
  </template>
  <div class="content">
    <!-- Хлебных крошек в приложении нет: путь ровно один уровень
         в глубину, и возврат делает кнопка «Назад». -->
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
      <h3>Flux тест</h3>
      <span class="spacer"></span>
      <span class="pill starting"><i></i>Стартует</span>
      <!-- Настоящий порт выдан при старте: предпочитаемый 8188 занят
           соседней сборкой. В обзоре под подписью «Предпочитаемый
           порт» так и останется 8188 — расходятся они только здесь. -->
      <span class="t-mono">:8189</span>
      <span class="btn primary lg">Открыть ComfyUI</span>
      <span class="btn secondary">Перезапустить</span>
      <span class="btn danger">Остановить</span>
    </div>
    <div class="track indet"><i></i></div>
    <div class="tabs">
      <span aria-selected="true">Обзор</span>
      <span>Модели</span>
      <span>Воркфлоу</span>
      <span>Параметры</span>
    </div>
    <div class="row">
      <span class="t-label">Лог запуска</span>
      <span class="hint">312 строк</span>
      <span class="spacer"></span>
      <span class="btn secondary">Свернуть лог</span>
    </div>
    <div class="console">D:\AI\Flux&gt;.\python_embeded\python.exe -s ComfyUI\main.py --port 8189 --disable-auto-launch
Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Loading custom nodes: 47 found
<span class="dim">Loading model  ████████████░░░░░░░  61%  4.2/6.9 GB</span></div>
  </div>
</Window>

## Реальный объём данных

*прокрутка · полторы тысячи строк; следование приостановлено*

<Window fixed scroll>
  <template #nav>
    <nav class="nav in-win">
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
  </template>
  <div class="content framed no-foot">
    <div class="pinned">
      <div class="row">
        <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
        <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
        <h3>Flux тест</h3>
        <span class="spacer"></span>
        <span class="pill starting"><i></i>Стартует</span>
        <span class="t-mono">:8189</span>
        <span class="btn primary lg">Открыть ComfyUI</span>
        <span class="btn danger">Остановить</span>
      </div>
      <div class="track indet"><i></i></div>
      <div class="tabs">
        <span aria-selected="true">Обзор</span>
        <span>Модели</span>
        <span>Воркфлоу</span>
        <span>Параметры</span>
      </div>
      <div class="row">
        <span class="t-label">Лог запуска</span>
        <span class="hint">1 482 строки</span>
        <span class="spacer"></span>
        <span class="btn secondary">Свернуть лог</span>
      </div>
    </div>
    <div class="data">
      <!-- Полторы тысячи строк старта: сорока хватает, чтобы
           лог не помещался в область и полоса прокрутки была
           настоящей. -->
      <div class="log">
        <div class="console">Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Using pytorch attention
Loading custom nodes: 47 found
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
  ComfyUI-VideoHelperSuite
  was-node-suite-comfyui
  ComfyUI-Impact-Pack
  comfyui_controlnet_aux
Import times for custom nodes:
   0.1 seconds: ComfyUI-Manager
   2.4 seconds: was-node-suite-comfyui</div>
        <span class="log-follow">К последним строкам <span class="n">+128</span></span>
      </div>
    </div>
  </div>
</Window>
