<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, Pencil, X, Ban, Check } from '@lucide/vue';
</script>

# Назначения

<!-- J-01 · шаг 4 · US-INST-02 · US-INST-08 -->

Назначений может быть несколько за один прогон — распаковка выполняется
один раз, остальные копии получаются копированием готового дерева.
Предупреждение о длинном пути стоит здесь, а не после сбоя: часть файлов
сборки вложена настолько глубоко, что упирается в ограничение Windows.

Кнопки перехода — **в ряду с названием шага**, а не в подвале: при
нескольких назначениях «Дальше» уезжала из виду ровно тогда, когда
становилась нужна. «Назад» всегда левее кнопки действия. Под ними
закреплена сводка по архиву — то, что раньше показывалось отдельным
шагом.

Слева форма, справа список: она накидывает в него назначение
за назначением и после каждого встаёт в исходное. Панель со всеми
полями на каждую цель разом занимала по экрану прокрутки на цель,
а целей бывает шесть.

Описание в строку не выводится — она и без него из четырёх колонок, —
но всплывает подсказкой при наведении. У строк без описания подсказки
нет вовсе.

Правка разворачивается **под своей строкой**, той же формой: список
остаётся на месте, и видно, что именно правится. Правится копия —
«Отмена» обязана откатывать, а не «сохранять обратно». Кнопки формы
правки — значками: «Добавить в список» бывает раз, а этих две на
каждую строку, и подписями они вытеснят из строки то, ради чего она
есть.

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
    <div class="steps">
      <span class="step done"><u>✓</u>Архив</span><span class="step-sep"></span>
      <span class="step now"><u>2</u>Назначения</span><span class="step-sep"></span>
      <span class="step"><u>3</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Распаковка</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Готово</span>
    </div>
    <div class="step-bar">
      <h3>Куда распаковать</h3>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
        <span class="btn primary lg">Дальше</span>
      </span>
    </div>
    <div class="meta">
      <span>ComfyUI_windows_portable_nvidia_0.31.0.7z</span>
      <span>56 128 файлов</span>
      <span>Распакуется в 9,7 ГБ</span>
      <span>Нужно примерно 19,4 ГБ свободного места</span>
    </div>
    <!-- Что окажется на верхнем уровне после распаковки: у сборки
         это одна корневая папка, но так бывает не у всякого архива,
         и строка стоит всегда. -->
    <p class="hint">Корневая папка в архиве: ComfyUI_windows_portable</p>
    <div class="cols targets">
      <div class="pane">
        <div class="pane-head">
          <span class="title">Новое назначение</span>
          <span class="btn primary">Добавить в список</span>
        </div>
        <div class="scroll-pad">
          <div class="field">
            <span class="t-label">Папка сборки</span>
            <div class="path-row">
              <div class="input mono"><span>D:\AI\Flux</span></div>
              <span class="btn secondary">Выбрать…</span>
            </div>
            <!-- Предупреждение, а не постоянная подсказка: оно
                 появляется, только когда путь действительно
                 длинный, и приходит проверкой назначений. -->
            <div class="hint">Самый глубокий файл окажется на 246 символах. Распаковка пройдёт, но ComfyUI и pip могут не справиться — выберите путь короче.</div>
          </div>
          <div class="field">
            <label>Имя</label>
            <div class="input">Flux тест</div>
          </div>
          <div class="field">
            <label>Описание</label>
            <div class="input"></div>
          </div>
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
          <div class="field">
            <label>Предпочитаемый порт</label>
            <div class="input num">8188</div>
            <div class="hint">Используется при запуске, если свободен.</div>
          </div>
        </div>
      </div>
      <!-- Цвет виден прямо в строке: две сборки с похожими путями
           различают по нему, а не по пути. Стоит между именем
           и кнопками — там квадраты выстраиваются в колонку,
           а не гуляют вслед за длиной имени. Карандаш разворачивает
           правку под самой строкой — список остаётся на месте,
           и видно, что именно правится. -->
      <div class="field">
        <span class="t-label">Распакуется в</span>
        <div class="paths">
          <!-- Описание в строку не лезет, но и пропасть не должно:
               оно всплывает подсказкой. У строки без описания
               атрибута нет вовсе — пустая подсказка мигала бы
               рамкой ни о чём. -->
          <div class="path-item editable" title="Ветка для экспериментов с Flux, ставим поверх свежей 0.31">
            <span class="lbl">D:\AI\Flux</span>
            <span class="val">Flux тест</span>
            <span class="chip sm" style="--instance-accent:var(--accent-teal)"></span>
            <span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span>
          </div>
          <div class="path-item editable">
            <span class="lbl">E:\AI\Flux_clean</span>
            <span class="val">Flux чистый</span>
            <span class="chip sm" style="--instance-accent:var(--accent-indigo)"></span>
            <span class="acts"><span class="act" aria-pressed="true"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span>
          </div>
          <div class="pane">
            <div class="pane-head">
              <span class="title">Редактирование</span>
              <span class="acts"><span class="act"><Ban class="ico" /></span><span class="act"><Check class="ico" /></span></span>
            </div>
            <div class="scroll-pad">
              <div class="field">
                <span class="t-label">Папка сборки</span>
                <div class="path-row">
                  <div class="input mono"><span>E:\AI\Flux_clean</span></div>
                  <span class="btn secondary">Выбрать…</span>
                </div>
              </div>
              <div class="field">
                <label>Имя</label>
                <div class="input">Flux чистый</div>
              </div>
              <div class="field">
                <label>Описание</label>
                <div class="input"></div>
              </div>
              <div class="field">
                <span class="t-label">Акцентный цвет</span>
                <div class="picker">
                  <i style="background:var(--accent-teal)"></i><i class="on" style="background:var(--accent-indigo)"></i>
                  <i style="background:var(--accent-ember)"></i><i style="background:var(--accent-moss)"></i>
                  <i style="background:var(--accent-azure)"></i><i style="background:var(--accent-orchid)"></i>
                  <i style="background:var(--accent-rose)"></i><i style="background:var(--accent-amber)"></i>
                  <span class="swatch-custom" title="Выбрать свой цвет"></span>
                </div>
              </div>
              <div class="field">
                <label>Предпочитаемый порт</label>
                <div class="input num">8189</div>
                <div class="hint">Используется при запуске, если свободен.</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Реальный объём данных

*прокрутка · восемнадцать папок назначения; ряд шага с «Дальше» закреплён*

<!-- Кнопки перехода стоят здесь и никуда не уезжают: шесть назначений
     в области ниже прокручиваются, ряд шага остаётся на месте. -->
<Window fixed scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Добавление</span></div>
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
  <div class="content">
    <div class="pinned">
      <div class="steps">
        <span class="step done"><u>✓</u>Архив</span><span class="step-sep"></span>
        <span class="step now"><u>2</u>Назначения</span><span class="step-sep"></span>
        <span class="step"><u>3</u>Общие ресурсы</span><span class="step-sep"></span>
        <span class="step"><u>4</u>Распаковка</span><span class="step-sep"></span>
        <span class="step"><u>5</u>Готово</span>
      </div>
      <div class="step-bar">
        <h3>Куда распаковать</h3>
        <span class="spacer"></span>
        <span class="acts">
          <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
          <span class="btn primary lg">Дальше</span>
        </span>
      </div>
      <div class="meta">
        <span>ComfyUI_windows_portable_nvidia_0.31.0.7z</span>
        <span>56 128 файлов</span>
        <span>Нужно примерно 58,2 ГБ свободного места</span>
      </div>
    </div>
    <!-- Форма прибита к верху колонки: список растёт вниз, а она
         остаётся на месте — ровно по той же причине, по которой
         из подвала наверх переехали «Назад» и «Дальше». -->
    <div class="scroll"><div class="scroll-pad">
      <div class="cols targets">
        <div class="pane">
          <div class="pane-head">
            <span class="title">Новое назначение</span>
            <span class="btn primary">Добавить в список</span>
          </div>
          <div class="scroll-pad">
            <div class="field">
              <span class="t-label">Папка сборки</span>
              <div class="path-row">
                <div class="input mono"><span>D:\AI\Flux</span></div>
                <span class="btn secondary">Выбрать…</span>
              </div>
            </div>
            <div class="field">
              <label>Имя</label>
              <div class="input">Flux тест</div>
            </div>
            <div class="field">
              <span class="t-label">Акцентный цвет</span>
              <div class="picker">
                <i class="on" style="background:var(--accent-teal)"></i><i style="background:var(--accent-indigo)"></i>
                <i style="background:var(--accent-ember)"></i><i style="background:var(--accent-moss)"></i>
                <span class="swatch-custom" title="Выбрать свой цвет"></span>
              </div>
            </div>
          </div>
        </div>
        <div class="field">
          <span class="t-label">Распакуется в</span>
          <div class="paths">
            <div class="path-item editable" title="Ветка для экспериментов с Flux, ставим поверх свежей 0.31"><span class="lbl">D:\AI\Flux</span><span class="val">Flux тест</span><span class="chip sm" style="--instance-accent:var(--accent-teal)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Flux_clean</span><span class="val">Flux чистый</span><span class="chip sm" style="--instance-accent:var(--accent-indigo)"></span><span class="acts"><span class="act" aria-pressed="true"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <!-- Правка в середине списка: панель разворачивается
                 на своём месте, всё ниже уезжает вниз, порядок
                 строк не меняется. -->
            <div class="pane">
              <div class="pane-head">
                <span class="title">Редактирование</span>
                <span class="acts"><span class="act"><Ban class="ico" /></span><span class="act"><Check class="ico" /></span></span>
              </div>
              <div class="scroll-pad">
                <div class="field">
                  <span class="t-label">Папка сборки</span>
                  <div class="path-row">
                    <div class="input mono"><span>E:\AI\Flux_clean</span></div>
                    <span class="btn secondary">Выбрать…</span>
                  </div>
                </div>
                <div class="field">
                  <label>Имя</label>
                  <div class="input">Flux чистый</div>
                </div>
                <div class="field">
                  <label>Предпочитаемый порт</label>
                  <div class="input num">8189</div>
                  <div class="hint">Используется при запуске, если свободен.</div>
                </div>
              </div>
            </div>
            <div class="path-item editable"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span><span class="chip sm" style="--instance-accent:var(--accent-ember)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Sandbox</span><span class="val">Песочница</span><span class="chip sm" style="--instance-accent:var(--accent-moss)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\Video</span><span class="val">Видео</span><span class="chip sm" style="--instance-accent:var(--accent-azure)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Archive_030</span><span class="val">Архив 0.30</span><span class="chip sm" style="--instance-accent:var(--accent-orchid)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span><span class="chip sm" style="--instance-accent:var(--accent-ember)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Sandbox</span><span class="val">Песочница</span><span class="chip sm" style="--instance-accent:var(--accent-moss)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\Video</span><span class="val">Видео</span><span class="chip sm" style="--instance-accent:var(--accent-azure)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Archive_030</span><span class="val">Архив 0.30</span><span class="chip sm" style="--instance-accent:var(--accent-orchid)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\SDXL_new</span><span class="val">SDXL 0.31</span><span class="chip sm" style="--instance-accent:var(--accent-ember)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Sandbox</span><span class="val">Песочница</span><span class="chip sm" style="--instance-accent:var(--accent-moss)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">D:\AI\Video</span><span class="val">Видео</span><span class="chip sm" style="--instance-accent:var(--accent-azure)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
            <div class="path-item editable"><span class="lbl">E:\AI\Archive_030</span><span class="val">Архив 0.30</span><span class="chip sm" style="--instance-accent:var(--accent-orchid)"></span><span class="acts"><span class="act"><Pencil class="ico" /></span><span class="act"><X class="ico" /></span></span></div>
          </div>
          <!-- Проверка идёт по всему списку и после каждого
               добавления: место на диске могло кончиться уже
               после того, как назначение добавили. -->
          <p class="hint bad">Не хватает места: нужно примерно 58,2 ГБ.</p>
        </div>
      </div>
    </div></div>
  </div>
</Window>

## Список пуст

Первое, что видно на шаге.

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
    <div class="steps">
      <span class="step done"><u>✓</u>Архив</span><span class="step-sep"></span>
      <span class="step now"><u>2</u>Назначения</span><span class="step-sep"></span>
      <span class="step"><u>3</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Распаковка</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Готово</span>
    </div>
    <div class="step-bar">
      <h3>Куда распаковать</h3>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
        <span class="btn primary lg" aria-disabled="true">Дальше</span>
      </span>
    </div>
    <div class="meta">
      <span>ComfyUI_windows_portable_nvidia_0.31.0.7z</span>
      <span>56 128 файлов</span>
      <span>Распакуется в 9,7 ГБ</span>
      <span>Нужно примерно 0 байт свободного места</span>
    </div>
    <p class="hint">Корневая папка в архиве: ComfyUI_windows_portable</p>
    <!-- Форма чиста и молчит: ругаться на поле, до которого
         не дошли, — врать. Ошибки появятся, когда «Добавить
         в список» нажмут на незаполненной форме. «Дальше»
         выключена: пока список пуст, распаковывать нечего. -->
    <div class="cols targets">
      <div class="pane">
        <div class="pane-head">
          <span class="title">Новое назначение</span>
          <span class="btn primary">Добавить в список</span>
        </div>
        <div class="scroll-pad">
          <div class="field">
            <span class="t-label">Папка сборки</span>
            <div class="path-row">
              <div class="input mono"><span></span></div>
              <span class="btn secondary">Выбрать…</span>
            </div>
          </div>
          <div class="field">
            <label>Имя</label>
            <div class="input"></div>
          </div>
          <div class="field">
            <label>Описание</label>
            <div class="input"></div>
          </div>
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
          <div class="field">
            <label>Предпочитаемый порт</label>
            <div class="input num">8188</div>
            <div class="hint">Используется при запуске, если свободен.</div>
          </div>
        </div>
      </div>
      <div class="field">
        <span class="t-label">Распакуется в</span>
        <p class="blank">Заполните форму слева и добавьте первое назначение.</p>
      </div>
    </div>
  </div>
</Window>
