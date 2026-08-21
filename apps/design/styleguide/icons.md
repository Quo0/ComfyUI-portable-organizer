<script setup lang="ts">
import {
  Layers, FolderPlus, Workflow, SlidersHorizontal, Info,
  Plus, Check, ListChecks, Pencil, Ban, X,
  ArrowLeft, ScrollText, FolderOpen, ExternalLink, RotateCw,
  ChevronLeft, ChevronRight, Palette, Database, HardDrive, Archive,
} from '@lucide/vue';
</script>

# Значки

<!-- NFR-300 -->

Свой набор, не библиотека: шестнадцать штук дешевле нарисовать, чем тащить зависимость.

Значки — компоненты `@lucide/vue`, тот же пакет и в приложении, и здесь:
подменять реальный компонент нарисованным вручную квадратом незачем,
а разойтись с приложением спрайт-копия уже не может — копии нет.
Размер и цвет задаёт класс `.ico` (16×16, цвет наследуется от текста
через `currentColor`), а не сам значок — поэтому они одинаково читаются
в обеих темах.

`pnpm design:check` проверяет, что каждый импортированный здесь значок
реально существует в `@lucide/vue` — опечатка в имени иначе тихо дала бы
пустой квадрат вместо значка.

<ThemePair>
  <div class="icons">
    <div class="icon-cell"><Layers class="ico" /><code>i-inst</code></div>
    <div class="icon-cell"><FolderPlus class="ico" /><code>i-install</code></div>
    <div class="icon-cell"><Workflow class="ico" /><code>i-wf</code></div>
    <div class="icon-cell"><SlidersHorizontal class="ico" /><code>i-set</code></div>
    <div class="icon-cell"><Info class="ico" /><code>i-about</code></div>
    <div class="icon-cell"><Plus class="ico" /><code>i-plus</code></div>
    <div class="icon-cell"><Check class="ico" /><code>i-check</code></div>
    <div class="icon-cell"><ListChecks class="ico" /><code>i-checklist</code></div>
    <div class="icon-cell"><Pencil class="ico" /><code>i-edit</code></div>
    <div class="icon-cell"><Ban class="ico" /><code>i-ban</code></div>
    <div class="icon-cell"><X class="ico" /><code>i-close</code></div>
    <div class="icon-cell"><ArrowLeft class="ico" /><code>i-back</code></div>
    <div class="icon-cell"><ScrollText class="ico" /><code>i-log</code></div>
    <div class="icon-cell"><FolderOpen class="ico" /><code>i-folder</code></div>
    <div class="icon-cell"><ExternalLink class="ico" /><code>i-external</code></div>
    <div class="icon-cell"><RotateCw class="ico" /><code>i-reload</code></div>
    <div class="icon-cell"><ChevronLeft class="ico" /><code>i-collapse</code></div>
    <div class="icon-cell"><ChevronRight class="ico" /><code>i-expand</code></div>
    <div class="icon-cell"><Palette class="ico" /><code>i-palette</code></div>
    <div class="icon-cell"><Database class="ico" /><code>i-db</code></div>
    <div class="icon-cell"><HardDrive class="ico" /><code>i-disk</code></div>
    <div class="icon-cell"><Archive class="ico" /><code>i-archive</code></div>
  </div>
</ThemePair>
