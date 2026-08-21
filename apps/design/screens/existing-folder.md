<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Папка уже есть

<!-- J-02 · шаги 3-4 · US-REG-01 · US-REG-02 -->

Путь актора, у которого сборка уже работает. Приложение показывает, что
прочитало — версию, интерпретатор и профили запуска, — и обещает, что
ничего внутри не тронет. Это главный страх такого пользователя, поэтому
обещание стоит на экране, а не в документации.

Ряд шага такой же, как в мастере: обе дорожки «Добавления» начинаются
одинаково. «Назад» ведёт в «Добавление», а не в список сборок — сюда
приходят только оттуда, — и это единственный выход с экрана.

Главное действие стоит там же, где во всём разделе, — в ряду шага,
правее «Назад». Под формой оно уезжало из виду вместе с прочитанным
о папке. Пока папку не выбрали, добавлять нечего, и кнопка выключена,
а не спрятана: пропадающая кнопка не говорит, чем экран закончится.
Исчезает она только там, где действие другое, — когда папка уже
в списке.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="step-bar">
      <h3>Добавить инстанс</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Назад</span><span class="btn primary lg" aria-disabled="true">Добавить в список</span></span>
    </div>
    <p class="t-sm">Выберите папку с портабл-сборкой ComfyUI. Внутри неё ничего не изменится.</p>
    <div class="field">
      <span class="t-label">Папка сборки</span>
      <div class="path-row">
        <div class="input mono"><span></span></div>
        <span class="btn secondary">Выбрать папку…</span>
      </div>
    </div>
  </div>
</Window>

**Папка выбрана и прочитана** — слева прочитанное, справа заполняемое.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="step-bar">
      <h3>Добавить инстанс</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Назад</span><span class="btn primary lg">Добавить в список</span></span>
    </div>
    <p class="t-sm">Выберите папку с портабл-сборкой ComfyUI. Внутри неё ничего не изменится.</p>
    <div class="field">
      <span class="t-label">Папка сборки</span>
      <div class="path-row">
        <div class="input mono"><span>D:\program_files\comfyui\ComfyUI_windows_portable</span></div>
        <span class="btn secondary">Выбрать папку…</span>
      </div>
    </div>
    <!-- Слева прочитанное, справа заполняемое: одним свитком
         прочитанное оттесняло форму вниз, за край экрана. -->
    <div class="cols">
      <div>
        <div class="paths">
          <div class="path-item"><span class="lbl">Версия ComfyUI</span><span class="val">0.30.2</span></div>
          <div class="path-item"><span class="lbl">Версия Python</span><span class="val">3.13.12</span></div>
          <div class="path-item"><span class="lbl">Профили запуска</span><span class="val">4</span></div>
        </div>
        <div class="row">
          <span class="pill stopped">nvidia_gpu</span><span class="pill stopped">fast_fp16</span>
          <span class="pill stopped">cpu</span><span class="pill stopped">disable_api_nodes<em class="advanced">продвинутые</em></span>
        </div>
      </div>
      <div>
        <div class="field"><label>Имя</label><div class="input">SDXL стабильная</div></div>
        <div class="field"><label>Описание</label><div class="input"></div></div>
        <div class="field">
          <span class="t-label">Акцентный цвет</span>
          <div class="picker">
            <i class="on" style="background:var(--accent-teal)"></i><i style="background:var(--accent-indigo)"></i>
            <i style="background:var(--accent-ember)"></i><i style="background:var(--accent-moss)"></i>
            <i style="background:var(--accent-azure)"></i><i style="background:var(--accent-orchid)"></i>
            <i style="background:var(--accent-rose)"></i><i style="background:var(--accent-amber)"></i>
            <span class="swatch-custom" title="Выбрать свой цвет"></span>
          </div>
        </div>
        <div class="field"><label>Предпочитаемый порт</label><div class="input num">8188</div><div class="hint">Используется при запуске, если свободен.</div></div>
        <!-- Кнопок под формой нет: главное действие уехало в ряд
             шага — там же, где во всём разделе, — а внизу оно
             уезжало из виду вместе с прочитанным о папке. -->
      </div>
    </div>
  </div>
</Window>

**Папка не похожа на сборку** — ошибка на месте, а не тостом: она про эту папку.

Самый частый промах — выбрать папку уровнем выше или ниже нужной.
Приложение не просто отказывает, а называет ту, которую имело в виду:
отличить `ComfyUI_windows_portable` от вложенной
`ComfyUI` со стороны невозможно.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="step-bar">
      <h3>Добавить инстанс</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Назад</span><span class="btn primary lg" aria-disabled="true">Добавить в список</span></span>
    </div>
    <p class="t-sm">Выберите папку с портабл-сборкой ComfyUI. Внутри неё ничего не изменится.</p>
    <div class="field">
      <span class="t-label">Папка сборки</span>
      <div class="path-row">
        <div class="input mono"><span>D:\program_files\comfyui</span></div>
        <span class="btn secondary">Выбрать папку…</span>
      </div>
      <p class="hint bad">Почти: сборка уровнем ниже. Выберите D:\program_files\comfyui\ComfyUI_windows_portable.</p>
    </div>
  </div>
</Window>

**Папка уже в списке** — второй сборки из неё не появится.

Повторная регистрация той же папки не заводит вторую сборку: форма
не показывается вовсе, вместо неё — переход к уже заведённой
`US-REG-02`.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="step-bar">
      <h3>Добавить инстанс</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Назад</span></span>
    </div>
    <p class="t-sm">Выберите папку с портабл-сборкой ComfyUI. Внутри неё ничего не изменится.</p>
    <div class="field">
      <span class="t-label">Папка сборки</span>
      <div class="path-row">
        <div class="input mono"><span>D:\program_files\comfyui\ComfyUI_windows_portable</span></div>
        <span class="btn secondary">Выбрать папку…</span>
      </div>
    </div>
    <div class="group">
      <span class="btn primary">Эта папка уже в списке. Открыть её</span>
    </div>
  </div>
</Window>
