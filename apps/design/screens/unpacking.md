<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Распаковка

<!-- J-01 · шаг 6 · US-INST-05 -->

Операция долгая, поэтому прогресс детерминантный и показывает текущий файл.
Уйти в другой раздел можно — распаковка не прервётся. Отмена не оставит
папку, которую приложение потом приняло бы за рабочий инстанс
`US-INST-05/AC-6`.

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
      <span class="step done"><u>✓</u>Назначения</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step now"><u>4</u>Распаковка</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Готово</span>
    </div>
    <div class="step-bar">
      <h3>Распаковка</h3>
      <span class="t-label">1 из 2</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn danger">Отменить</span>
      </span>
    </div>
    <!-- Полоса на каждое назначение, а не одна общая: общая при
         нескольких целях трижды проходит путь от нуля до ста,
         и понять по ней, сколько работы осталось, невозможно. -->
    <div class="prog">
      <div class="prog-head"><span>D:\AI\Flux</span><span class="count">64%</span></div>
      <div class="track"><i style="width:64%"></i></div>
      <div class="prog-file">python_embeded\Lib\site-packages\torch\_inductor\kernel\mm.py</div>
      <div class="hint">27 906 из 61 895 файлов · 4 ГБ из 9,7 ГБ</div>
    </div>
    <div class="prog">
      <div class="prog-head"><span>E:\AI\Flux_clean</span><span class="count">в очереди</span></div>
      <div class="track"><i style="width:0"></i></div>
    </div>
  </div>
</Window>

## Реальный объём данных

*прокрутка · шесть целей; «Отменить установку» достижимо всегда*

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
        <span class="step done"><u>✓</u>Назначения</span><span class="step-sep"></span>
        <span class="step done"><u>✓</u>Общие ресурсы</span><span class="step-sep"></span>
        <span class="step now"><u>4</u>Распаковка</span><span class="step-sep"></span>
        <span class="step"><u>5</u>Готово</span>
      </div>
      <div class="step-bar">
        <h3>Распаковка</h3>
        <span class="t-label">2 из 6</span>
        <span class="spacer"></span>
        <span class="acts"><span class="btn danger">Отменить</span></span>
      </div>
    </div>
    <div class="scroll"><div class="scroll-pad" style="gap:var(--space-4)">
      <div class="prog"><div class="prog-head"><span>D:\AI\Flux</span><span class="count">готово</span></div><div class="track"><i style="width:100%"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Flux_clean</span><span class="count">64%</span></div><div class="track"><i style="width:64%"></i></div><div class="prog-file">python_embeded\Lib\site-packages\torch\_inductor\kernel\mm.py</div><div class="hint">27 906 из 61 895 файлов · 4 ГБ из 9,7 ГБ</div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\SDXL_new</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Sandbox</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\Video</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Archive_030</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\SDXL_new</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Sandbox</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\Video</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Archive_030</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\SDXL_new</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Sandbox</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>D:\AI\Video</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
      <div class="prog"><div class="prog-head"><span>E:\AI\Archive_030</span><span class="count">в очереди</span></div><div class="track"><i style="width:0"></i></div></div>
    </div></div>
  </div>
</Window>

**Фаза без доли выполненного** — считать нечего, но молчать нельзя.

У подготовки, уборки следов прерванной попытки и регистрации доли
выполненного нет. Полоса бежит вместо того, чтобы стоять на нуле,
вместо процента стоит прочерк, а вместо пути файла — название фазы:
пауза без подписи и без движения читается как зависание.

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
      <span class="step done"><u>✓</u>Назначения</span><span class="step-sep"></span>
      <span class="step done"><u>✓</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step now"><u>4</u>Распаковка</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Готово</span>
    </div>
    <div class="step-bar">
      <h3>Распаковка</h3>
      <span class="t-label">1 из 2</span>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn danger">Отменить</span>
      </span>
    </div>
    <div class="prog">
      <div class="prog-head"><span>D:\AI\Flux</span><span class="count">—</span></div>
      <div class="track indet"><i></i></div>
      <div class="prog-file">Проверяем папки и открываем архив…</div>
    </div>
    <div class="prog">
      <div class="prog-head"><span>E:\AI\Flux_clean</span><span class="count">в очереди</span></div>
      <div class="track"><i style="width:0"></i></div>
    </div>
  </div>
</Window>
