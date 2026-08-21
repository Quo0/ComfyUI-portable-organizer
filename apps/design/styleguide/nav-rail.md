<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Рейл навигации

<!-- US-TAB-01 · US-TAB-03 · US-UI-04 -->

Единственная часть интерфейса, видимая всегда — включая экран работающего ComfyUI.

Два блока, разделённых чертой: четыре раздела приложения и список
запущенных инстансов в их акцентных цветах. Второй блок решает две задачи
разом — переключение между сборками в один клик и, что важнее,
**видимость аварий**: когда на экране развёрнут ComfyUI,
рейл остаётся единственным местом, где заметно, что процесс упал
`US-UI-04/AC-2`. Свёрнутый вид оставляет от рейла 56&nbsp;px.

<ThemePair light="Светлая · развёрнутый и свёрнутый" dark="Тёмная · развёрнутый и свёрнутый">
  <div class="rails">
    <nav class="nav">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Запущены</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL стабильная</em><i class="dot" style="background:var(--state-running)"></i></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux тест</em><i class="dot" style="background:var(--state-starting)"></i></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Анимация</em><span class="badge">!</span></div>
    </nav>
    <nav class="nav collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL</em></div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux</em></div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Анимация</em></div>
    </nav>
  </div>
</ThemePair>

## Второй уровень: разделы настроек

Внутри «Настроек» свой список — без значков и без сворачивания.
Значки нужны рейлу, который виден всегда и должен читаться в 56&nbsp;px;
здесь же пять подписей рядом читаются быстрее пяти картинок.
Библиотека воркфлоу стоит именно тут: это папка снаружи сборок,
устроенная ровно как общие модели.

<ThemePair>
  <nav class="settings-sections" style="width:200px">
    <div class="nav-item"><span>Внешний вид</span></div>
    <div class="nav-item"><span>Общие модели</span></div>
    <div class="nav-item on"><span>Библиотека воркфлоу</span></div>
    <div class="nav-item"><span>Отчёт по диску</span></div>
    <div class="nav-item"><span>Архивы установщика</span></div>
  </nav>
</ThemePair>

## Проверка на длинных строках · пункты рейла в 208 px

<div class="longform">
  <div class="lf-head">Проверка на длинных строках · пункты рейла в 208 px</div>
  <div class="lf-rows">
    <div class="lf-row"><b>EN</b><span>Instances · Add build · Settings · About</span></div>
    <div class="lf-row"><b>RU</b><span>Инстансы · Добавление · Настройки · О приложении</span></div>
    <div class="lf-row"><b>ES</b><span>Instancias · Añadir · Configuración · Acerca de</span></div>
    <div class="lf-row"><b>ZH</b><span>实例 · 添加 · 设置 · 关于</span></div>
  </div>
</div>
