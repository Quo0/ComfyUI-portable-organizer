<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Готово

<!-- J-01 · шаг 7 · US-INST-06 · US-INST-07 -->

Итог показывает то, что появилось, а не то, что произошло: карточки
созданных сборок кликабельны и ведут на их экраны. Отсюда же —
сразу второй прогон, если ставили не всё, что собирались.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item on"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="steps">
      <span class="step done"><u>✓</u>Архив</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Назначения</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Распаковка</span><span class="step-sep"></span>
      <span class="step now"><u>5</u>Готово</span>
    </div>
    <div class="step-bar">
      <h3>Готово</h3>
      <span class="t-label">Добавлено 2 инстанса</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost">Добавить ещё</span>
        <span class="btn primary lg">К инстансам</span>
      </span>
    </div>
    <div class="cards grid">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
            <div class="card-name">Flux тест</div>
            <span class="pill stopped"><i></i>Остановлен</span>
          </div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.31.0</span><span>:8188</span><span class="tag">общие модели</span></div>
          <div class="src">D:\AI\Flux</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-moss)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-moss)">F</span>
            <div class="card-name">Flux чистый</div>
            <span class="pill stopped"><i></i>Остановлен</span>
          </div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.31.0</span><span>:8189</span><span class="tag">общие модели</span></div>
          <div class="src">E:\AI\Flux_clean</div>
        </div>
      </div>
    </div>
  </div>
</Window>
