<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Настройки: отчёт по диску

<!-- US-SHARED-09 -->

Раздел называется отчётом, а не «дубликатами»: дубликаты — то, что он
нашёл, а не то, чем он является. Он только считает. Ни одной кнопки,
которая что-нибудь удалит, здесь нет и не будет: освобождение места —
отдельное осознанное действие на своём экране, и отчёт объясняет, где
именно оно делается.

Обход десятков гигабайт по нескольким сборкам занимает минуты, поэтому
идёт полоса и названо место, которое смотрят прямо сейчас: пауза без
подписи читается как зависание. Прерванный обход честно помечает отчёт
неполным, а недоступные папки перечислены отдельно — молча пропущенная
папка превратила бы отчёт в неправду.

«Одно имя, разный размер» вынесено из дубликатов в свой перечень
и не предлагается ни к чему: совпадение имени не говорит о содержимом
ничего, и подать такие файлы как дубли значило бы подтолкнуть к удалению
разных моделей.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Внешний вид</span></div>
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item on"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <h3>Отчёт по диску</h3>
        <p class="t-sm">Отчёт по всем зарегистрированным сборкам и общей папке. Он только считает: ничего не удаляет, не переносит и не связывает ссылками.</p>
        <div class="row">
          <span class="btn primary">Построить отчёт</span>
        </div>
        <div class="group">
          <span class="t-label">Впустую на дублях: 6,9 ГБ</span>
          <div class="dup-list">
            <div class="dup-row">
              <span class="nm">sd_xl_base_1.0.safetensors</span>
              <span class="tag">checkpoints</span>
              <span class="t-mono">6,5 ГБ</span>
              <span class="where hint">SDXL стабильная · Flux тест</span>
            </div>
            <div class="dup-row">
              <span class="nm">ip-adapter-plus_sdxl.bin</span>
              <span class="tag">ipadapter</span>
              <span class="t-mono">0,8 ГБ</span>
              <span class="where hint">SDXL стабильная · Эксперименты · Видео</span>
            </div>
            <div class="dup-row">
              <span class="nm">4x-UltraSharp.pth</span>
              <span class="tag">upscale_models</span>
              <span class="t-mono">64 МБ</span>
              <span class="where hint">Апскейл · Видео</span>
            </div>
          </div>
          <p class="hint">Освобождение места — отдельное осознанное действие: откройте сборку и воспользуйтесь панелью «Модели этой сборки».</p>
        </div>
        <div class="group">
          <span class="t-label">Одно имя, разный размер</span>
          <p class="hint">Это не дубликаты: совпадение имени ничего не говорит о содержимом. Они перечислены, чтобы вы посмотрели на них сами.</p>
          <div class="dup-list">
            <div class="dup-row">
              <span class="nm">lora_style_v2.safetensors</span>
              <span class="tag">loras</span>
              <span class="t-mono"></span>
              <span class="where hint">SDXL стабильная · 144 МБ · Эксперименты · 151 МБ</span>
            </div>
          </div>
        </div>
        <p class="hint bad">Пропущено, папка недоступна: E:\AI\Flux_clean</p>
      </div>
    </div>
  </div>
</Window>

## Обход идёт

Минуты работы: молчать нельзя.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Внешний вид</span></div>
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item on"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <h3>Отчёт по диску</h3>
        <p class="t-sm">Отчёт по всем зарегистрированным сборкам и общей папке. Он только считает: ничего не удаляет, не переносит и не связывает ссылками.</p>
        <div class="row">
          <span class="btn primary" aria-disabled="true">Построить отчёт</span>
          <span class="btn ghost">Отмена</span>
        </div>
        <div class="group">
          <div class="bar"><i style="width:38%"></i></div>
          <span class="hint">Смотрим D:\AI\_shared\models\checkpoints…</span>
        </div>
      </div>
    </div>
  </div>
</Window>
