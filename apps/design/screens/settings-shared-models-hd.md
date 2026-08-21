<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Общие модели на мониторе 1920×1080

<!-- J-03 · монитор · US-SHARED-01 · US-SHARED-02 -->

Кадры выше нарисованы в 940 на 560 — это нижняя граница, при которой
экран обязан работать. Живёт он не там: сборки ComfyUI держат на машине
с большим монитором, и окно на ней развёрнуто. Этот кадр — настоящие
1920 на 1080, вписанные в страницу уменьшением, поэтому соотношение
текста и места здесь честное. Подписи читаются приближением страницы.

Что доказывает именно этот размер: место, которое даёт монитор, уходит
списку категорий, а не растянутым на всю ширину строкам. Корень остаётся
на месте сверху, тумблер загрузок и конфиг — справа, и ни один из них
не уезжает, сколько бы папок ни было в общей папке. Прокручиваются
ровно две области: список и сам конфиг.

**Прокрутка.** Тридцать четыре категории; окно 1920×1080, уменьшено до 55%.

<Window :fixed="true" :hd="true" scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item on"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Запущены · 4</div>
      <div class="nav-runs">
        <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL стабильная</em><i class="dot" style="background:var(--state-running)"></i></div>
        <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><em>Flux тест</em><i class="dot" style="background:var(--state-starting)"></i></div>
        <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Анимация</em><span class="badge">!</span></div>
        <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-moss)">Э</span><em>Эксперименты</em><i class="dot" style="background:var(--state-running)"></i></div>
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
            <label>Общий корень моделей</label>
            <div class="path-row">
              <div class="input mono"><span>D:\AI\_shared\models</span></div>
              <span class="btn secondary">Обзор</span>
            </div>
            <div class="hint">Распознано 32 категории · 604 ГБ · подключено 4 инстанса</div>
          </div>
        </div>
        <div class="split-master shared">
          <div class="pane">
            <div class="pane-head"><span class="title">Папки категорий</span></div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
              <div class="cat"><code>checkpoints</code><span class="n">21 файл · 274 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>loras</code><span class="n">312 файлов · 96 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>vae</code><span class="n">9 файлов · 3.4 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>controlnet</code><span class="n">17 файлов · 12 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>text_encoders</code><span class="n">6 файлов · 18 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>clip_vision</code><span class="n">4 файла · 2.6 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>diffusion_models</code><span class="n">8 файлов · 141 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>unet</code><span class="n">2 файла · 11 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>upscale_models</code><span class="n">14 файлов · 1.2 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>embeddings</code><span class="n">61 файл · 0.2 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>style_models</code><span class="n">3 файла · 1.7 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>hypernetworks</code><span class="n">пусто</span><span class="tag">распознано</span></div>
              <div class="cat"><code>photomaker</code><span class="n">1 файл · 0.9 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>gligen</code><span class="n">пусто</span><span class="tag">распознано</span></div>
              <div class="cat"><code>diffusers</code><span class="n">2 файла · 14 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>configs</code><span class="n">7 файлов · 0.1 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>vae_approx</code><span class="n">4 файла · 0.1 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>audio_encoders</code><span class="n">2 файла · 1.4 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>model_patches</code><span class="n">пусто</span><span class="tag">распознано</span></div>
              <div class="cat"><code>ipadapter</code><span class="n">11 файлов · 6.3 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>insightface</code><span class="n">3 файла · 0.7 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>instantid</code><span class="n">2 файла · 1.9 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>animatediff_models</code><span class="n">5 файлов · 8.1 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>animatediff_motion_lora</code><span class="n">12 файлов · 0.9 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>facerestore_models</code><span class="n">4 файла · 1.1 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>sams</code><span class="n">3 файла · 2.4 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>ultralytics</code><span class="n">9 файлов · 0.3 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>grounding-dino</code><span class="n">2 файла · 0.8 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>onnx</code><span class="n">6 файлов · 0.5 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>LLM</code><span class="n">1 файл · 4.1 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat"><code>clip</code><span class="n">пусто</span><span class="tag">распознано</span></div>
              <div class="cat"><code>xlabs</code><span class="n">2 файла · 0.6 ГБ</span><span class="tag">распознано</span></div>
              <div class="cat unknown"><code>my_experiments</code><span class="n">7 файлов · 1.3 ГБ</span><span class="tag warn">не распознано</span></div>
              <div class="cat blocked"><code>custom_nodes</code><span class="n">не шарится</span><span class="tag stop">исключено</span></div>
              <!-- Объяснение стоит под списком, которого касается,
                   и внутри его прокрутки: это сноска к строке,
                   а не постоянная надпись на экране. -->
              <div class="hint">Кастомные ноды не шарятся никогда: отдельные сборки и заведены потому, что ноды конфликтуют между собой.</div>
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
  clip_vision: clip_vision/
  diffusion_models: diffusion_models/
  unet: unet/
  upscale_models: upscale_models/
  embeddings: embeddings/
  style_models: style_models/
  hypernetworks: hypernetworks/
  photomaker: photomaker/
  gligen: gligen/
  diffusers: diffusers/
  configs: configs/
  vae_approx: vae_approx/
  audio_encoders: audio_encoders/
  model_patches: model_patches/
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
