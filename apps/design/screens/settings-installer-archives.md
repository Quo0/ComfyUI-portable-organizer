<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive, FolderOpen } from '@lucide/vue';
</script>

# Настройки: архивы установщика

<!-- US-INST-07 -->

Команды «помнить архивы» и «забыть архив» существовали с самого начала,
а экрана у них не было: посмотреть, что приложение помнит, было негде.
Раздел показывает список и ровно одно действие над записью.

«Убрать из списка» убирает **запись**. Сам архив остаётся там, куда его
скачали, и это сказано прямо над списком — кнопка рядом с именем файла
иначе читается как удаление файла.

Пропавший архив не исчезает из списка молча: строка остаётся и говорит,
что файла нет или он изменился, а кнопка «Открыть папку» у неё погашена.
Пользователь мог перенести архив сам и должен это увидеть.

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
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item on"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <h3>Архивы установщика</h3>
        <p class="t-sm">Архивы, которые видел мастер установки. «Забыть» убирает только запись — сам файл остаётся там, куда его скачали.</p>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">
              ComfyUI_windows_portable_nvidia.7z
              <span class="hint">D:\downloads\ComfyUI_windows_portable_nvidia.7z</span>
            </span>
            <span class="val">9,7 ГБ · сегодня, 12:04</span>
            <span class="acts">
              <span class="btn ghost icon"><FolderOpen class="ico" /></span>
              <span class="btn ghost">Убрать из списка</span>
            </span>
          </div>
          <div class="path-item">
            <span class="lbl">
              ComfyUI_windows_portable_nvidia_v0.30.7z
              <span class="hint">D:\downloads\old\ComfyUI_windows_portable_nvidia_v0.30.7z</span>
            </span>
            <span class="val">9,1 ГБ · 12 марта</span>
            <span class="acts">
              <span class="btn ghost icon"><FolderOpen class="ico" /></span>
              <span class="btn ghost">Убрать из списка</span>
            </span>
          </div>
          <!-- Файл пропал: строка остаётся, «Открыть папку» погашена. -->
          <div class="path-item">
            <span class="lbl">
              ComfyUI_windows_portable_cpu.7z
              <span class="hint">E:\temp\ComfyUI_windows_portable_cpu.7z</span>
            </span>
            <span class="val">Файла нет или он изменился</span>
            <span class="acts">
              <span class="btn ghost icon" aria-disabled="true"><FolderOpen class="ico" /></span>
              <span class="btn ghost">Убрать из списка</span>
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Список пуст

Мастер ещё ничего не запомнил.

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
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item on"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <h3>Архивы установщика</h3>
        <p class="t-sm">Архивы, которые видел мастер установки. «Забыть» убирает только запись — сам файл остаётся там, куда его скачали.</p>
        <!-- Строка о пустоте, а не пустой экран: у раздела есть
             заголовок и объяснение, и крупный блок под ними выглядел
             бы отдельным экраном, приехавшим в середину настроек. -->
        <p class="blank">Архивов пока нет — мастер запоминает их по мере использования.</p>
      </div>
    </div>
  </div>
</Window>
