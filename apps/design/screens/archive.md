<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Архив

<!-- J-01 · шаг 3 · US-INST-01 · US-INST-03 -->

Выбор архива сразу уводит на второй шаг: состав архива и требуемое
место видны там, в закреплённой шапке, то есть до того, как что-то
начнётся. Нехватка места останавливает установку до распаковки,
а не на её середине `US-INST-03/AC-2`.

Разбор оглавления на пятьдесят шесть тысяч записей занимает больше
секунды, и подпись о нём стоит **рядом с кнопкой**, которую только что
нажали, а не отдельным блоком ниже.

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
      <span class="step now"><u>1</u>Архив</span><span class="step-sep"></span>
      <span class="step"><u>2</u>Назначения</span><span class="step-sep"></span>
      <span class="step"><u>3</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Распаковка</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Готово</span>
    </div>
    <!-- Ряд шага есть и на первом шаге, ради единственного выхода
         из мастера. Отдельной шапки экрана у мастера нет: её
         «Назад» и заголовок повторяли бы этот ряд. -->
    <div class="step-bar">
      <h3>Архив</h3>
      <span class="spacer"></span>
      <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Назад</span></span>
    </div>
    <p>Архив портабл-сборки нужно скачать самостоятельно — приложение его не загружает.</p>
    <!-- Подпись о чтении оглавления — в том же ряду, что и кнопка:
         отдельным блоком ниже она выглядела ответом на что-то
         другое. -->
    <div class="row">
      <span class="btn primary">Выбрать архив .7z…</span>
      <span class="hint">Читаем оглавление архива…</span>
      <span class="bar indet grow"><i></i></span>
    </div>
    <!-- Недавние — карточками, а не строками: путь к архиву длинный
         и в строку не помещается, а именно по нему отличают две
         сборки одной версии, лежащие в разных папках. -->
    <div class="group">
      <span class="t-label">Недавние</span>
      <div class="cards grid">
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Реальный объём данных

*прокрутка · история архивов растёт с каждой новой версией*

<!-- Ни подвала с «Отмена/Далее», ни выбранного архива в шапке:
     кнопки перехода в 4б переехали в ряд шага, а выбор архива
     сразу уводит на второй шаг — задержаться на первом
     с уже выбранным архивом невозможно. -->
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
        <span class="step now"><u>1</u>Архив</span><span class="step-sep"></span>
        <span class="step"><u>2</u>Назначения</span><span class="step-sep"></span>
        <span class="step"><u>3</u>Общие ресурсы</span><span class="step-sep"></span>
        <span class="step"><u>4</u>Распаковка</span><span class="step-sep"></span>
        <span class="step"><u>5</u>Готово</span>
      </div>
      <div class="step-bar">
        <h3>Архив</h3>
        <span class="spacer"></span>
        <span class="acts"><span class="btn ghost"><ArrowLeft class="ico" />Назад</span></span>
      </div>
      <p>Архив портабл-сборки нужно скачать самостоятельно — приложение его не загружает.</p>
      <div class="row">
        <span class="btn primary">Выбрать архив .7z…</span>
      </div>
      <span class="t-label">Недавние</span>
    </div>
    <div class="scroll"><div class="scroll-pad">
      <div class="cards grid">
        <!-- История растёт с каждой новой версией сборки: за год
             их набирается два десятка. -->
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.31.0</div></div>
            <div class="src"><code>D:\program_files\comfyui\downloads\ComfyUI_windows_portable_nvidia_0.31.0.7z</code></div>
            <div class="meta"><span>2,2 ГБ</span><span>сегодня</span></div>
            <div class="row"><span class="btn secondary">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
        <div class="card gone">
          <div class="card-accent"></div>
          <div class="card-in">
            <div class="card-top"><div class="card-name">0.28.7</div><span class="pill gone">Файла нет или он изменился</span></div>
            <div class="src"><code>E:\Archive\ComfyUI_windows_portable_nvidia_0.28.7.7z</code></div>
            <div class="meta"><span>1,9 ГБ</span><span>30 мая</span></div>
            <div class="row"><span class="btn secondary" aria-disabled="true">Дальше</span><span class="btn ghost">Убрать из списка</span></div>
          </div>
        </div>
      </div>
    </div></div>
  </div>
</Window>
