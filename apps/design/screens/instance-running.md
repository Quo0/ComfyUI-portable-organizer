<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, ScrollText, FolderOpen, ExternalLink, RotateCw } from '@lucide/vue';
</script>

# Инстанс работает

<!-- J-01 · шаг 9 · US-TAB-02 · US-TAB-06 -->

Ради этого экрана всё и затевалось: ComfyUI внутри окна, а не во вкладке
браузера. Область холста занимает **нативное окно поверх нашего
интерфейса** — поэтому ни одно наше меню, всплывашка или тост не может
её перекрыть. Отсюда и вся раскладка: рейл слева, инструменты инстанса
полосой сверху, и ничего плавающего.

**Единственный экран, где прокрутка запрещена.** Позиция нативного окна
задаётся командой `set_webview_bounds`. Если бы контейнер этой области
прокручивался, прямоугольник разошёлся бы с содержимым: вебвью осталось
бы на месте, а разметка уползла. Поэтому тулбар закреплён, холст занимает
всё оставшееся место, и ни у одного родителя нет собственной прокрутки.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL</em></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux</em></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Анимация</em></div>
    </nav>
  </template>
  <div class="content flush">
    <!-- Значки, а не подписи: шесть текстовых кнопок в ряд не помещались
         на узком окне. Словами остались только остановка и перезапуск —
         у них цена ошибки выше, чем экономия места. -->
    <div class="inst-toolbar">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <span class="name">SDXL стабильная</span>
      <span class="port">127.0.0.1:8188</span>
      <span class="pill running"><i></i>Работает</span>
      <span class="spacer"></span>
      <span class="tools">
        <span class="btn ghost icon"><ScrollText class="ico" /></span>
        <span class="btn ghost icon"><FolderOpen class="ico" /></span>
        <span class="btn ghost icon"><ExternalLink class="ico" /></span>
        <span class="btn ghost icon"><RotateCw class="ico" /></span>
      </span>
      <span class="btn secondary">Перезапустить</span>
      <span class="btn danger">Остановить</span>
    </div>
    <div class="comfy">
      <div class="comfy-node" style="left:8%; top:16%">
        <b>Load Checkpoint</b><span>sdxl_base_1.0</span>
      </div>
      <div class="comfy-node" style="left:44%; top:34%">
        <b>KSampler</b><span>steps 28 · cfg 6.5</span>
      </div>
      <div class="comfy-node" style="left:74%; top:14%">
        <b>Save Image</b><span>ComfyUI_00042_</span>
      </div>
      <div class="comfy-wire" style="left:22%; top:26%; width:22%"></div>
      <div class="comfy-wire" style="left:57%; top:26%; width:17%"></div>
      <span class="comfy-label">область встроенной вкладки ComfyUI</span>
    </div>
  </div>
</Window>
