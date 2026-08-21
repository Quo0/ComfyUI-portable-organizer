<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, FolderOpen, RotateCw } from '@lucide/vue';
</script>

# Забор воркфлоу в библиотеку

<!-- J-05 · шаг 2 · US-WF-03 -->

Забор **переносит**: файл уходит из сборки и остаётся только
в библиотеке. Сказано это строкой под списком, а не узнаётся
по исчезнувшей строке. Порядок тот же, что у переноса моделей —
копия пишется, читается обратно и сверяется, и лишь потом исходник
убирается из сборки.

Меток две, и это главное здесь. По одному имени метка врала бы:
правленная в сборке версия под занятым именем выглядела бы
сохранённой. Поэтому содержимое сверяется целиком — сначала байты,
потом разобранный JSON, иначе пересохранённый в ComfyUI воркфлоу
объявлялся бы разошедшимся из-за одних отступов.

Погашена кнопка только у того, что уже лежит в библиотеке тем же
файлом. У разошедшегося она работает и забирает под свободным именем:
замены нет вовсе, потому что заменить значило бы затереть одну работу
другой, не оставив ни той, ни другой.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Запущены</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL стабильная</em><i class="dot" style="background:var(--state-running)"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <h3>SDXL стабильная</h3>
      <span class="spacer"></span>
      <span class="pill running"><i></i>Работает</span>
      <span class="t-mono">:8188</span>
      <span class="btn primary lg">Открыть ComfyUI</span>
      <span class="btn secondary">Перезапустить</span>
      <span class="btn danger">Остановить</span>
    </div>
    <div class="tabs">
      <span>Обзор</span>
      <span>Модели</span>
      <span aria-selected="true">Воркфлоу</span>
      <span>Параметры</span>
    </div>
    <div class="row">
      <span class="t-label">Воркфлоу этой сборки</span>
      <!-- Тот же значок и то же место, что у моделей сборки: путь
           к папке воркфлоу в панели больше нигде не показан,
           а идти разбираться руками приходится именно туда. -->
      <span class="btn ghost icon" title="D:\AI\ComfyUI_windows_portable\ComfyUI\user\default\workflows"><FolderOpen class="ico" /></span>
      <span class="spacer"></span>
      <span class="btn ghost"><RotateCw class="ico" />Обновить</span>
    </div>
    <!-- Ни отметки для массовой операции, ни звёздочки избранного:
         отметка нужна тому, над чем бывают действия скопом,
         а избранное живёт в манифесте библиотеки — эти файлы туда
         ещё не попали. Отсюда своя сетка строки. -->
    <div class="wf-list of-instance">
      <!-- Имени в библиотеке нет — забирается без разговоров. -->
      <div class="wf-row">
        <span class="nm">sdxl / img2img-refine.json</span>
        <span class="tags"></span>
        <span class="btn ghost">Забрать в библиотеку</span>
      </div>
      <!-- Тот же файл: забирать нечего, кнопка справедливо погашена. -->
      <div class="wf-row">
        <span class="nm">flux / portrait-v3.json</span>
        <span class="tags"><span class="tag">в библиотеке</span></span>
        <span class="btn ghost" aria-disabled="true">Забрать в библиотеку</span>
      </div>
      <!-- Имя занято чужой работой. Кнопка работает и предложит
           «base-upscale (2).json» — но спросит до того, как двинет. -->
      <div class="wf-row">
        <span class="nm">sdxl / base-upscale.json</span>
        <span class="tags"><span class="tag warn">имя занято, файл другой</span></span>
        <span class="btn ghost">Забрать в библиотеку</span>
      </div>
    </div>
    <p class="hint">Забор переносит: файл уходит из сборки и остаётся только в библиотеке.</p>
    <!-- Легенда меток: слева та же метка, что в строке, справа — что
         она значит. Абзацем это уже было, и метку в нём приходилось
         искать глазами. -->
    <dl class="tag-legend">
      <dt><span class="tag">в библиотеке</span></dt>
      <dd>— там лежит тот же самый воркфлоу: забирать нечего, кнопка погашена.</dd>
      <dt><span class="tag warn">имя занято, файл другой</span></dt>
      <dd>— содержимое разошлось: это две разные работы, и та, что в сборке, заберётся под свободным именем, а не поверх чужой.</dd>
    </dl>
    <p class="hint">Список берётся у запущенной сборки, поэтому в нём есть и то, что она сохранила минуту назад.</p>
  </div>
</Window>
