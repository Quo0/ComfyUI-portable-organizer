<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Экран сборки: вкладки

<!-- ревизия · US-RUN-01 · US-REG-04 -->

До ревизии здесь была одна лента: запуск, общие модели, модели сборки,
воркфлоу, путь, версии, размер, профили, источник, форма
редактирования и удаление — подряд, примерно на тысячу строк вёрстки.
Найти нужное можно было только прокруткой.

Вкладки разводят четыре независимых набора данных. Шапка со сборкой
и кнопкой запуска закреплена: она нужна на любой вкладке. На широком
окне содержимое раскладывается в две колонки — приложение
разворачивают на 2K, и лента в 720 пикселей оставляет половину окна
пустой.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <h3>SDXL стабильная</h3>
      <span class="spacer"></span>
      <span class="pill stopped"><i></i>Остановлен</span>
      <span class="btn primary lg">Запустить</span>
      <!-- Выбор профиля — системный список, а не кнопка с галкой:
           имена профилей это имена .bat-файлов, и их бывает восемь. -->
      <span class="input" style="width:170px">run_nvidia_gpu</span>
    </div>
    <!-- Чисел на вкладках нет намеренно: и число моделей, и число
         воркфлоу стоят обхода папок сборки, а у запущенной — ещё
         и запроса к её серверу. Платить этим за украшение шапки
         при каждом открытии экрана нельзя. -->
    <div class="tabs">
      <span aria-selected="true">Обзор</span>
      <span>Модели</span>
      <span>Воркфлоу</span>
      <span>Параметры</span>
    </div>
    <div class="cols">
      <div>
        <div class="field">
          <label>Последний запуск</label>
          <div class="paths">
            <div class="path-item"><span class="lbl">Последний запуск: сегодня, 14:20</span><span class="val">готов за 54 с</span></div>
            <div class="path-item"><span class="lbl">Предпочитаемый порт</span><span class="val">8188</span></div>
            <div class="path-item"><span class="lbl">Профили запуска</span><span class="val">run_nvidia_gpu</span></div>
          </div>
        </div>
        <div class="field">
          <label>Лог запуска</label>
          <div class="row">
            <span class="btn secondary">Показать лог</span>
            <span class="hint">1 482 строки · раскроется на всю область</span>
          </div>
        </div>
        <div class="field">
          <label>Описание</label>
          <p class="t-sm">Рабочая сборка, ноды не трогаю</p>
        </div>
      </div>
      <div>
        <div class="field">
          <label>Папка сборки</label>
          <div class="path-row">
            <div class="input mono"><span>D:\program_files\comfyui\SDXL</span></div>
            <span class="btn secondary">Открыть папку</span>
          </div>
        </div>
        <!-- Общие модели здесь только показаны. Тумблер живёт
             на вкладке «Модели» и ровно в одном месте: два
             переключателя одного и того же расходятся. -->
        <div class="paths">
          <div class="path-item"><span class="lbl">Версия ComfyUI</span><span class="val">0.30.2</span></div>
          <div class="path-item"><span class="lbl">Версия Python</span><span class="val">3.13.12</span></div>
          <div class="path-item"><span class="lbl">Размер на диске</span><span class="val">52,4 ГБ</span></div>
          <div class="path-item"><span class="lbl">Профили запуска</span><span class="val">4 + 1 свой</span></div>
          <div class="path-item"><span class="lbl">Общие модели</span><span class="val">D:\AI\_shared\models · папка сборки не тронута</span></div>
        </div>
        <div class="row">
          <span class="hint">измерено 12 минут назад</span>
          <span class="btn ghost">Пересчитать</span>
          <span class="btn ghost">Править аргументы</span>
        </div>
        <div class="src">Распакован из ComfyUI_windows_portable_nvidia.7z, 12 марта</div>
      </div>
    </div>
  </div>
</Window>

## Лог раскрыт

*та же вкладка, кнопка нажата*

Лог не висит куском в 260 пикселей посреди свитка и не показывается
всегда: он раскрывается кнопкой на всю область данных и сворачивается
обратно. При старте раскрывается сам — там он и есть содержимое
экрана.

<Window fixed>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL</em><i class="dot" style="background:var(--state-starting)"></i></div>
    </nav>
  </template>
  <div class="content framed no-foot">
    <div class="pinned">
      <div class="row">
        <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
        <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
        <h3>SDXL стабильная</h3>
        <span class="spacer"></span>
        <span class="pill starting"><i></i>Стартует</span>
        <span class="btn primary lg">Открыть ComfyUI</span>
        <span class="btn secondary">Перезапустить</span>
        <span class="btn danger">Остановить</span>
      </div>
      <!-- Доля выполненного неизвестна: старт с большим набором нод
           занимает минуты, и полоса говорит только, что процесс жив. -->
      <div class="track indet"><i></i></div>
      <!-- Вкладки остаются на месте: лог занимает область данных,
           а не подменяет собой весь экран. -->
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
      <div class="log">
        <div class="console">Total VRAM 24564 MB, total RAM 65451 MB
pytorch version: 2.13.0+cu130
Set vram state to: NORMAL_VRAM
Device: cuda:0 NVIDIA GeForce RTX 4090
Loading custom nodes: 47 found
  ComfyUI-Manager 3.19.4
  ComfyUI_IPAdapter_plus
<span class="dim">Loading model  ████████████░░░░░░░  61%  4.2/6.9 GB</span></div>
        <span class="log-follow">К последним строкам <span class="n">+128</span></span>
      </div>
    </div>
  </div>
</Window>
