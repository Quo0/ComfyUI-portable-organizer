<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Настройки: общие модели

<!-- J-01 · шаги 1-3 · US-SHARED-01 · US-SHARED-02 · US-SHARED-04 -->

Ради этого экрана проект и затевался: один чекпоинт весит до двадцати
гигабайт, и при пяти сборках дубли съедают сотни. Категории распознаются
по содержимому папки, а не по зашитому списку, — иначе конфиг устареет с
ближайшим обновлением ComfyUI.

Настройки — второй уровень навигации: слева список разделов, справа
выбранный. Хлебная крошка «Настройки → Общие модели» этого не давала —
она говорила, где ты, но не показывала, что рядом, и перейти в соседний
раздел можно было только вернувшись.

Мастер-детейл, а не лента блоков: список категорий — то, что растёт,
а тумблер загрузок и кнопка конфига относятся к настройке целиком. Корень
закреплён над обоими: он адрес всего, что ниже, и уезжать вместе со
списком не должен.

Этот кадр — окно по умолчанию, 1100 в ширину: экрану раздела остаётся
под 700, и на такой ширине колонки схлопнуты в строки — органы стоят над
списком. Списку тут дороже ширина — имена папок моноширинные и не
обрезаются. Справа от списка они встают на мониторе; это отдельный кадр
ниже, в настоящих 1920 на 1080.

<Window>
  <template #nav>
    <nav class="nav in-win">
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
        <div class="nav-item on"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content framed no-foot">
        <div class="pinned">
          <h3>Общие модели</h3>
          <div class="field">
            <label>Общий корень моделей</label>
            <div class="path-row">
              <div class="input mono"><span>D:\AI\_shared\models</span></div>
              <span class="btn secondary">Обзор</span>
            </div>
            <div class="hint">Распознано 5 категорий · 231 ГБ · подключено 3 инстанса</div>
          </div>
        </div>
        <div class="split-master shared">
          <div class="pane">
            <div class="pane-head"><span class="title">Папки категорий</span></div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
              <div class="cat"><code>checkpoints</code><span class="n">14 файлов · 187 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>loras</code><span class="n">126 файлов · 41 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>vae</code><span class="n">6 файлов · 2.1 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat unknown"><code>my_experiments</code><span class="n">3 файла · 0.4 ГБ</span><span class="tag warn">не распознано</span></div>
              <div class="cat blocked"><code>custom_nodes</code><span class="n">не шарится</span><span class="tag stop">исключено</span></div>
            </div></div>
          </div>
          <div class="side">
            <div class="toggle-row">
              <span class="toggle"></span>
              <div>
                <div class="t-base">Скачивать новые модели в общую папку</div>
                <div class="hint">Модель, загруженная в одной сборке, станет видна всем.</div>
                <div class="hint">Вступает в силу при следующем запуске сборки.</div>
              </div>
            </div>
            <div class="row"><span class="btn ghost" aria-pressed="true">Скрыть сгенерированный конфиг</span></div>
            <!-- Конфиг длиннее колонки: он забирает всё, что осталось
                 под кнопкой, и прокручивается внутри себя. -->
            <div class="pane">
              <div class="pane-head"><span class="title">extra_model_paths.yaml</span></div>
              <div class="scroll"><pre class="console">cpo_shared_0:
  base_path: D:/AI/_shared/models
  is_default: true
  checkpoints: checkpoints/
  loras: loras/
  vae: vae/
  text_encoders: |
    text_encoders/
    clip/</pre></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Реальный объём данных

**Прокрутка.** Двадцать пять категорий; корень, тумблер и конфиг
закреплены — скроллится только список.

Двадцать пять категорий утаскивали тумблер и конфиг под нижний край: они
относятся к настройке целиком, а не к строке списка, поэтому из прокрутки
вынесены вовсе. На узком экране они стоят над списком, на мониторе —
справа от него.

<Window :fixed="true" scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
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
  <div class="content flush">
    <div class="settings-split">
      <nav class="settings-sections">
        <div class="nav-item"><Palette class="ico" /><span>Внешний вид</span></div>
        <div class="nav-item on"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content framed no-foot">
        <div class="pinned">
          <h3>Общие модели</h3>
          <div class="field">
            <div class="path-row">
              <div class="input mono"><span>D:\AI\_shared\models</span></div>
              <span class="btn secondary">Обзор</span>
            </div>
            <div class="hint">Распознано 25 категорий · 231 ГБ · подключено 8 инстансов</div>
          </div>
        </div>
        <div class="split-master shared">
          <div class="pane">
            <div class="pane-head"><span class="title">Папки категорий</span></div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
            <div class="cat"><code>checkpoints</code><span class="n">14 файлов · 187 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>loras</code><span class="n">126 файлов · 41 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>vae</code><span class="n">6 файлов · 2.1 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>controlnet</code><span class="n">9 файлов · 4.8 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>text_encoders</code><span class="n">4 файла · 3.2 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>diffusion_models</code><span class="n">3 файла · 22 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>clip_vision</code><span class="n">2 файла · 1.1 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>upscale_models</code><span class="n">11 файлов · 0.9 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>embeddings</code><span class="n">38 файлов · 0.1 ГБ</span><span class="tag">распознано</span></div>
            <div class="cat"><code>style_models</code><span class="n">пусто</span><span class="tag">распознано</span></div>
            <div class="cat"><code>hypernetworks</code><span class="n">пусто</span><span class="tag">распознано</span></div>
            <div class="cat"><code>photomaker</code><span class="n">пусто</span><span class="tag">распознано</span></div>
            <div class="cat unknown"><code>my_experiments</code><span class="n">3 файла · 0.4 ГБ</span><span class="tag warn">не распознано</span></div>
            <div class="cat blocked"><code>custom_nodes</code><span class="n">не шарится</span><span class="tag stop">исключено</span></div>
            </div></div>
          </div>
          <div class="side">
            <div class="toggle-row">
              <span class="toggle"></span>
              <div>
                <div class="t-base">Скачивать новые модели в общую папку</div>
                <div class="hint">Модель, загруженная в одной сборке, станет видна всем.</div>
                <div class="hint">Вступает в силу при следующем запуске сборки.</div>
              </div>
            </div>
            <div class="row"><span class="btn ghost" aria-pressed="true">Скрыть сгенерированный конфиг</span></div>
            <div class="pane">
              <div class="pane-head"><span class="title">extra_model_paths.yaml</span></div>
              <div class="scroll"><pre class="console">cpo_shared_0:
  base_path: D:/AI/_shared/models
  is_default: true
  checkpoints: checkpoints/
  loras: loras/
  vae: vae/
  controlnet: controlnet/
  upscale_models: upscale_models/
  embeddings: embeddings/
  style_models: style_models/
  hypernetworks: hypernetworks/
  photomaker: photomaker/
  diffusion_models: diffusion_models/
  clip_vision: clip_vision/
  text_encoders: |
    text_encoders/
    clip/</pre></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>
