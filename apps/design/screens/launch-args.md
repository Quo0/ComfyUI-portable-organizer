<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, X } from '@lucide/vue';
</script>

# Аргументы запуска

<!-- US-RUN-01 -->

Отдельный роут, а не модалка. Дисциплина z-order этого требует всюду,
но здесь есть и своя причина: на экране читают итоговую команду,
а команду не разглядывают во всплывашке.

Файлы `.bat` самой сборки не меняются никогда
`US-RUN-01/AC-8`. Правка ложится своим профилем поверх
одного из них: разбор `.bat` перечитывается при каждом
запуске, и удержать правку внутри него было бы негде.

Экран линейный: пришли с экрана сборки, сохранили, вернулись. Поэтому
«Назад» здесь не навигация у левого края, а шаг назад рядом
с действием — как в мастере и на добавлении инстанса. Внизу
«Сохранить» уезжала из виду тем вернее, чем больше аргументов правят:
под ней скроллится их список.

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
    <div class="step-bar">
      <h3 class="title">Аргументы запуска</h3>
      <span class="hint">SDXL стабильная</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
        <span class="btn primary lg">Сохранить</span>
      </span>
    </div>
    <p class="t-sm">Файлы .bat самой сборки не меняются никогда. Правка сохраняется своим профилем поверх одного из них.</p>
    <div class="group">
      <span class="t-label">Свои профили</span>
      <div class="wf-list">
        <div class="wf-row">
          <span class="nm">run_nvidia_gpu +</span>
          <span class="hint">run_nvidia_gpu.bat</span>
          <span class="btn ghost">Изменить</span>
          <span class="btn ghost">Убрать из списка</span>
        </div>
      </div>
    </div>
    <div class="two">
      <div class="field">
        <label>На основе</label>
        <span class="input">advanced\run_nvidia_gpu_disable_api_nodes.bat</span>
      </div>
      <div class="field">
        <label>Имя профиля</label>
        <span class="input">run_nvidia_gpu_disable_api_nodes +</span>
      </div>
    </div>
    <div class="group">
      <span class="t-label">Аргументы</span>
      <!-- Крестик тот же, что убирает строку в списке назначений
           мастера: одна операция — один значок. -->
      <div class="path-row">
        <span class="input mono">-s</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="path-row">
        <span class="input mono">..\ComfyUI\main.py</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="path-row">
        <span class="input mono">--windows-standalone-build</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="path-row">
        <span class="input mono">--disable-api-nodes</span>
        <span class="acts"><span class="act"><X class="ico" /></span></span>
      </div>
      <div class="row">
        <span class="btn secondary">Добавить аргумент</span>
        <span class="btn ghost">Вернуть как в .bat</span>
      </div>
    </div>
    <div class="group">
      <span class="t-label">Команда, которая уйдёт системе</span>
      <!-- Своей прокрутки у предпросмотра нет: на экране одна область
           прокрутки, а команду сюда приходят прочитать целиком. -->
      <div class="console preview">D:\program_files\comfyui\SDXL\python_embeded\python.exe -s ..\ComfyUI\main.py --windows-standalone-build --disable-api-nodes --port 8188 --disable-auto-launch</div>
      <p class="hint">Порт и --disable-auto-launch дописывает приложение. Настоящий порт выдаётся при старте, поэтому показан предпочитаемый.</p>
    </div>
  </div>
</Window>
