<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, ScrollText, FolderOpen, ExternalLink, RotateCw } from '@lucide/vue';
</script>

# Сообщения: тост и баннер

<!-- US-RUN-06 -->

Две поверхности для одного и того же — сказать, что случилось, — и выбор
между ними не вкусовой. Тост всплывает в углу контентной области
и уходит сам. Он годится там, где под ним наша же разметка.

Ошибка приходит с бэкенда кодом, а не текстом: `AppError`
несёт `code` и подстановки, текст живёт в локалях. Поэтому
у тоста есть «Подробности» и «Копировать» — получатель скопированного
опознает ошибку независимо от языка отправителя. Повторы не плодят
стопку до потолка: одинаковые складываются в один со счётчиком.

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
    <div class="row">
      <h3>Инстансы</h3>
      <span class="t-sm">2</span>
      <span class="spacer"></span>
      <div class="input" style="width:220px">Поиск по имени</div>
    </div>
    <div class="cards grid">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
            <div class="card-name">SDXL стабильная</div>
            <span class="pill running"><i></i>Работает</span>
          </div>
          <div class="card-desc">Рабочая сборка, ноды не трогаю</div>
          <div class="meta"><span>0.30.2</span><span>:8188</span><span>52,4 ГБ</span></div>
          <div class="src">Последний запуск: сегодня, 14:20</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
            <div class="card-name">Flux тест</div>
            <span class="pill stopped"><i></i>Остановлен</span>
          </div>
          <div class="card-desc">Новые ноды проверяю здесь</div>
          <div class="meta"><span>0.31.0</span><span>38,1 ГБ</span></div>
          <div class="src">Последний запуск: вчера, 21:03</div>
        </div>
      </div>
    </div>
    <!-- Стопка растёт снизу вверх, свежий тост внизу — у края,
         где курсор, и ближе всего к тому, что только что нажали. -->
    <div class="toasts win-toasts">
      <div class="toast ok">
        <i></i>
        <div class="toast-in">
          <div class="toast-head">
            <b>Воркфлоу положен в сборку</b>
            <span class="close">✕</span>
          </div>
          <p>Сборка перечитает список при обновлении страницы.</p>
        </div>
      </div>
      <!-- Ошибка с подробностями и счётчиком повторов. Подробности
           свёрнуты по умолчанию: код и подстановки нужны тому, кто
           собрался их кому-то показать, а не всем подряд. -->
      <div class="toast err">
        <i></i>
        <div class="toast-in">
          <div class="toast-head">
            <b>Не удалось прочитать папку</b>
            <span class="badge">×2</span>
            <span class="close">✕</span>
          </div>
          <div class="row">
            <span class="btn ghost">Подробности</span>
            <span class="btn ghost">Копировать</span>
          </div>
          <pre class="toast-details">Не удалось прочитать папку D:\AI\_shared\models
[shared.readFailed]
  path: D:\AI\_shared\models</pre>
        </div>
      </div>
    </div>
  </div>
</Window>

## Баннер

*там, где тост физически невозможен*

На экране встроенной вкладки поверх нашего HTML лежит нативное окно
ComfyUI. Тост всплыл бы в углу контентной области — то есть под ним,
и пользователь не увидел бы его никогда. Поэтому сообщение здесь идёт
баннером **над** прямоугольником вкладки, в потоке, отнимая
у холста свою высоту. Прямоугольник пересчитывается: он и так считается
фронтом на каждое изменение раскладки.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content flush">
    <div class="inst-toolbar">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-ember)">A</span>
      <span class="name">Анимация</span>
      <span class="port">127.0.0.1:8190</span>
      <span class="pill crashed"><i></i>Упал</span>
      <span class="spacer"></span>
      <span class="tools">
        <span class="btn ghost icon"><ScrollText class="ico" /></span>
        <span class="btn ghost icon"><FolderOpen class="ico" /></span>
        <span class="btn ghost icon"><ExternalLink class="ico" /></span>
        <span class="btn ghost icon"><RotateCw class="ico" /></span>
      </span>
      <span class="btn secondary">Перезапустить</span>
      <span class="btn danger">Остановить</span>
    </div>
    <div class="banner bad">
      <b>Сервер завершился сам, код 3221225477.</b>
      <p>Последние строки лога — в консоли; вкладка осталась на месте.</p>
      <span class="spacer"></span>
      <span class="btn secondary">Показать лог</span>
    </div>
    <div class="comfy">
      <div class="comfy-node" style="left:10%; top:20%">
        <b>Load Checkpoint</b><span>animatediff_v3</span>
      </div>
      <div class="comfy-node" style="left:52%; top:40%">
        <b>KSampler</b><span>steps 20 · cfg 7</span>
      </div>
      <div class="comfy-wire" style="left:24%; top:30%; width:28%"></div>
      <span class="comfy-label">область встроенной вкладки ComfyUI</span>
    </div>
  </div>
</Window>
