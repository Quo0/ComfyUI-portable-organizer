<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Две версии рядом

<!-- J-04 · шаг 5 · US-REG-03 · US-INST-07 -->

Приложение никогда не обновляет сборку на месте: новая версия
разворачивается рядом, старая остаётся нетронутой. Чтобы версии
различались без открывания папок, в карточке видно, из какого архива
развёрнут инстанс `US-INST-07/AC-5`.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Запущены</div>
      <div class="nav-run alert"><span class="chip" style="--instance-accent:var(--accent-ember)">A</span><em>Анимация</em><span class="badge">!</span></div>
    </nav>
  </template>
  <div class="content">
    <!-- Кнопки «Добавить» здесь нет: сборки заводит раздел
         «Добавление», и двух дверей в одно место быть не должно. -->
    <div class="row">
      <h3>Инстансы</h3>
      <span class="t-sm">3</span>
      <span class="spacer"></span>
      <div class="input" style="width:220px">Поиск по имени</div>
      <div class="seg">
        <span aria-pressed="true">по имени</span>
        <span>по запуску</span>
        <span>по размеру</span>
      </div>
    </div>
    <div class="cards grid">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
            <div class="card-name">SDXL стабильная</div>
            <span class="pill stopped"><i></i>Остановлен</span>
          </div>
          <div class="card-desc">Рабочая сборка, ноды не трогаю</div>
          <div class="meta"><span>0.30.2</span><span>:8188</span><span>52,3 ГБ</span></div>
          <div class="src">
            <div>Распакован из ComfyUI_windows_portable_nvidia_0.30.2.7z, 4 августа</div>
            <div>Последний запуск: вчера, 19:40</div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">S</span>
            <div class="card-name">SDXL новая версия</div>
            <span class="pill stopped"><i></i>Остановлен</span>
          </div>
          <div class="card-desc">Проверяю обновление перед переездом</div>
          <div class="meta"><span>0.31.0</span><span>:8189</span><span>9,6 ГБ</span></div>
          <div class="src">
            <div>Распакован из ComfyUI_windows_portable_nvidia_0.31.0.7z, сегодня</div>
            <div>Отсюда ещё не запускали</div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-ember)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-ember)">А</span>
            <div class="card-name">Анимация</div>
            <span class="pill crashed"><i></i>Аварийно завершён</span>
          </div>
          <div class="card-desc">Не хватило видеопамяти при втором запуске</div>
          <div class="meta"><span>0.30.2</span><span>:8190</span><span>41,0 ГБ</span></div>
          <div class="src">
            <div>Распакован из ComfyUI_windows_portable_nvidia_0.30.2.7z, 12 марта</div>
            <div>Последний запуск: сегодня, 11:05</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Реальный объём данных

*прокрутка · тринадцать инстансов; рейл и заголовок закреплены*

<Window fixed scroll>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
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
  <div class="content framed no-foot">
    <div class="pinned">
      <div class="row">
        <h3>Инстансы</h3>
        <span class="t-sm">13</span>
        <span class="spacer"></span>
        <div class="input" style="width:220px">Поиск по имени</div>
        <div class="seg">
          <span aria-pressed="true">по имени</span>
          <span>по запуску</span>
          <span>по размеру</span>
        </div>
      </div>
    </div>
    <div class="data">
      <div class="scroll"><div class="scroll-pad">
        <div class="cards grid">
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-teal)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><div class="card-name">SDXL стабильная</div><span class="pill running"><i></i>Работает</span></div>
          <div class="card-desc">Рабочая сборка, ноды не трогаю</div>
          <div class="meta"><span>0.30.2</span><span>:8188</span><span>52,3 ГБ</span><span class="tag">общие модели</span></div>
          <div class="src"><div>Последний запуск: сегодня, 14:20</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-indigo)">F</span><div class="card-name">Flux тест</div><span class="pill starting"><i></i>Стартует</span></div>
          <div class="card-desc">Новые ноды проверяю здесь</div>
          <div class="meta"><span>0.31.0</span><span>:8189</span><span>18,7 ГБ</span><span class="tag">общие модели</span></div>
          <div class="src"><div>Последний запуск: только что</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-ember)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-ember)">А</span><div class="card-name">Анимация</div><span class="pill crashed"><i></i>Аварийно завершён</span></div>
          <div class="card-desc">Не хватило видеопамяти при втором запуске</div>
          <div class="meta"><span>0.30.2</span><span>:8190</span><span>41,0 ГБ</span></div>
          <div class="src"><div>Последний запуск: сегодня, 11:05</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-moss)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-moss)">Э</span><div class="card-name">Эксперименты</div><span class="pill stopped"><i></i>Остановлен</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.29.4</span><span>:8191</span><span>9,4 ГБ</span></div>
          <div class="src"><div>Последний запуск: 3 дня назад</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-azure)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-azure)">В</span><div class="card-name">Видео</div><span class="pill running"><i></i>Работает</span></div>
          <div class="card-desc">Длинные ролики, отдельный набор нод</div>
          <div class="meta"><span>0.30.2</span><span>:8192</span><span>77,1 ГБ</span></div>
          <div class="src"><div>Последний запуск: сегодня, 09:12</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-orchid)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-orchid)">И</span><div class="card-name">Инпейнт</div><span class="pill stopped"><i></i>Остановлен</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.30.0</span><span>:8193</span><span>12,8 ГБ</span></div>
          <div class="src"><div>Последний запуск: 12 марта</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-rose)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-rose)">У</span><div class="card-name">Апскейл</div><span class="pill stopped"><i></i>Остановлен</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.29.1</span><span>:8194</span><span>6,2 ГБ</span></div>
          <div class="src"><div>Отсюда ещё не запускали</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-amber)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-amber)">Т</span><div class="card-name">Тесты нод</div><span class="pill running"><i></i>Работает</span></div>
          <div class="card-desc">Сюда ставлю всё подряд, не жалко</div>
          <div class="meta"><span>0.31.0</span><span>:8195</span><span>15,0 ГБ</span></div>
          <div class="src"><div>Последний запуск: вчера, 22:41</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-teal)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><div class="card-name">SDXL новая версия</div><span class="pill stopped"><i></i>Остановлен</span></div>
          <div class="card-desc">Проверяю обновление перед переездом</div>
          <div class="meta"><span>0.31.0</span><span>:8196</span><span>9,6 ГБ</span></div>
          <div class="src"><div>Отсюда ещё не запускали</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-moss)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-moss)">Л</span><div class="card-name">Лоры</div><span class="pill stopped"><i></i>Остановлен</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.30.2</span><span>:8197</span><span>21,4 ГБ</span><span class="tag">общие модели</span></div>
          <div class="src"><div>Последний запуск: 4 августа</div></div>
        </div></div>
        <div class="card"><div class="card-accent" style="--instance-accent:var(--accent-azure)"></div><div class="card-in">
          <div class="card-top"><span class="chip" style="--instance-accent:var(--accent-azure)">К</span><div class="card-name">Контролнеты</div><span class="pill stopped"><i></i>Остановлен</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>0.29.4</span><span>:8198</span><span>7,9 ГБ</span></div>
          <div class="src"><div>Последний запуск: 28 июля</div></div>
        </div></div>
        <div class="card gone"><div class="card-accent"></div><div class="card-in">
          <div class="card-top"><span class="chip">В</span><div class="card-name">Видео (внешний диск)</div><span class="pill gone"><i></i>Папка не найдена</span></div>
          <div class="card-desc"></div>
          <div class="meta"><span>E:\comfy\video</span></div>
          <div class="src"><div>Последний запуск: 12 марта</div></div>
        </div></div>
        </div>
      </div></div>
    </div>
  </div>
</Window>
