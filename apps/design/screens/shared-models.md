<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft, FolderOpen, RotateCw, ChevronRight } from '@lucide/vue';
</script>

# Подключение общих моделей

<!-- J-03 · шаг 4 · US-SHARED-03 · US-SHARED-05 -->

Предупреждение про перезапуск обязательно: конфиг читается только при
старте, и без него пользователь решит, что функция сломана
`US-SHARED-03/AC-7`. Стоит оно прямо под тумблером — там,
где только что нажали, — а не отдельной плашкой наверху экрана.
Способ применения по умолчанию не пишет в папку инстанса ничего.

Тумблер живёт на вкладке «Модели» и ровно в одном месте. На «Обзоре»
подключение только показано строкой: два переключателя одного и того же
неизбежно расходятся.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
      <div class="nav-sep"></div>
      <div class="nav-note">Запущены</div>
      <div class="nav-run"><span class="chip" style="--instance-accent:var(--accent-teal)">S</span><em>SDXL стабильная</em><i class="dot" style="background:var(--state-running)"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-teal)">S</span>
      <h3>SDXL стабильная</h3>
      <span class="spacer"></span>
      <span class="pill running"><i></i>Работает</span>
      <span class="t-mono">:8188</span>
      <span class="btn primary lg">Открыть ComfyUI</span>
      <span class="btn secondary">Перезапустить</span>
      <span class="btn danger">Остановить</span>
    </div>
    <div class="tabs">
      <span>Обзор</span>
      <span aria-selected="true">Модели</span>
      <span>Воркфлоу</span>
      <span>Параметры</span>
    </div>
    <!-- Вкладка это два отдела: общая папка и модели самой сборки.
         Заголовок у них одной формы — подпись, значок своей папки,
         дальше содержимое: иначе верх вкладки читался набором
         разрозненных строк без границы между отделами. -->
    <div class="row">
      <span class="t-label">Общие модели</span>
      <!-- Тот же значок папки, что у моделей сборки ниже, но ведёт
           в общую: разбираться руками с тем, что уже перенесено,
           приходится именно там. -->
      <span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span>
    </div>
    <!-- Тумблер отдельной строкой, а подпись справа от него говорит,
         что он делает. Название отдела ушло в заголовок, и повторять
         его у тумблера незачем. -->
    <div class="toggle-row">
      <span class="toggle"></span>
      <div>
        <div class="t-base">Дать этой сборке доступ к общей папке, ничего в неё не копируя.</div>
        <div class="hint">D:\AI\_shared\models · папка сборки не тронута</div>
      </div>
    </div>
    <div class="group">
      <span class="t-label">Способ применения</span>
      <div class="seg">
        <span aria-pressed="true">Не трогать папку сборки</span>
        <span>Записать файл в сборку</span>
      </div>
      <!-- Подсказка называет приложение по имени: «у приложения»
           читалось и как органайзер, и как сама сборка ComfyUI.
           И говорит цену выбора: файла в сборке нет, но и общие
           модели тогда есть только при запуске отсюда. -->
      <p class="hint">Список общих папок хранится у ComfyUI Portable Organizer и передаётся сборке аргументом при запуске. В папку сборки не пишется ничего, но общие модели будут у неё только тогда, когда её запускает это приложение.</p>
    </div>
    <p class="hint bad">Сборка читает эту настройку только при старте. Перезапустите её, чтобы изменение подействовало.</p>
    <!-- Перенос моделей сборки в общую папку — вторая половина той же
         вкладки: подключение и перенос это разные действия, и делать
         их приходится подряд. -->
    <div class="row">
      <span class="t-label">Модели этой сборки</span>
      <!-- Тот же значок папки, что в «Архивах установщика»: путь
           к моделям в панели больше нигде не показан, а идти
           разбираться руками приходится именно туда. -->
      <span class="btn ghost icon" title="D:\AI\ComfyUI_windows_portable\ComfyUI\models"><FolderOpen class="ico" /></span>
      <span class="spacer"></span>
      <!-- «Обновить» с иконкой обновления: кнопка пересматривает
           папки сборки, а не повторяет неудавшееся действие. -->
      <span class="btn ghost"><RotateCw class="ico" />Обновить</span>
    </div>
    <!-- Один список на всё: категория раскрывается, и модели внутри
         показаны со своими вердиктами. Раньше дубликаты и «совпало
         имя» шли отдельными перечнями внизу, и связь с категорией
         приходилось восстанавливать по имени в строке.
         Тумблер стоит у каждой модели: одна лора из двадцати может
         быть нужна сборке локально. У категории он распоряжается
         всем содержимым и умеет промежуточное состояние. У строки
         с чужим содержимым тумблера нет вовсе — её не переносят
         и не убирают никогда.
         Метка на категории остаётся ради свёрнутого вида: иначе
         такая сборка выглядела тупиком — строки на месте, а
         в сводке ноль файлов. -->
    <div class="cats">
      <div class="cat marked">
        <button class="disclose" aria-expanded="true"><ChevronRight class="ico" /><code>checkpoints</code></button>
        <span class="n">6 элементов · 38,2 ГБ</span><span class="tag warn">2 уже в общей</span><span class="toggle mixed"></span>
      </div>
      <div class="cat model"><code>model-a.safetensors</code><span class="n">7,4 ГБ</span><span class="toggle"></span></div>
      <!-- У вердикта подсказка на наведении: из двух слов не видно,
           на чём он основан, а разница существенная. У файла сверены
           байты, у папки — только объём и число файлов внутри,
           и «похоже, тот же» ровно об этом. -->
      <div class="cat model marked"><code>sd_xl_base_1.0.safetensors</code><span class="n">6,9 ГБ</span><span class="tag" title="Совпал размер и по мегабайту с каждого края — почти наверняка тот же файл. Уборка убирает локальную копию, в общей папке файл остаётся.">тот же файл</span><span class="toggle"></span></div>
      <!-- Снятая строка и показывает промежуточный тумблер выше:
           в категории выбрано не всё. -->
      <div class="cat model marked"><code>refiner</code><span class="n">5,8 ГБ</span><span class="tag warn" title="Это папка: совпали суммарный объём и число файлов внутри, само содержимое не сверялось. Уборка убирает локальную копию, в общей папке папка остаётся.">похоже, тот же</span><span class="toggle off"></span></div>
      <!-- Метки «уже в общей» у этой категории нет: внутри неё занято
           одно имя, но дубликатом оно не признано, а метка считает
           только то, что можно убрать. -->
      <div class="cat">
        <button class="disclose" aria-expanded="true"><ChevronRight class="ico" /><code>loras</code></button>
        <span class="n">21 элемент · 4,7 ГБ</span><span class="toggle"></span>
      </div>
      <div class="cat model marked"><code>detail_tweaker.safetensors</code><span class="n">144 МБ</span><span class="tag stop" title="В общей папке есть кое-что под этим именем, но содержимое разное. Не переносится и не убирается никогда.">содержимое другое</span><span class="no-toggle"></span></div>
      <div class="cat model"><code>style_v2.safetensors</code><span class="n">320 МБ</span><span class="toggle"></span></div>
      <div class="cat">
        <button class="disclose" aria-expanded="false"><ChevronRight class="ico" /><code>vae</code></button>
        <span class="n">3 элемента · 1,6 ГБ</span><span class="toggle off"></span>
      </div>
    </div>
    <p class="hint">1 совпадение имени, но не дубликат: в общей папке есть файл с таким именем, но содержимое разное. Он остаётся в сборке и к уборке не предлагается никогда.</p>
    <p class="hint bad">Сначала остановите сборку: забирать её файлы на ходу нельзя.</p>
    <!-- Каждая кнопка стоит рядом со своим описанием: одна удаляет
         файлы, другая переносит, и путать их нельзя. Уборка идёт
         первой — она про то, что уже перенесено, а перенос про то,
         что ещё нет.
         Сетка одна на оба ряда, обёрток на ряд нет: от этого
         зависит, что колонка кнопок берёт ширину самой широкой
         надписи и кнопки выходят одного размера на любом языке. -->
    <div class="act-grid">
      <span class="btn danger">Убрать локальные копии</span>
      <!-- Найдено — всё, что признано дубликатом; освободится —
           только по отмеченным строкам. `refiner` снят, и его
           5,8 ГБ в счёт не идут. -->
      <p class="hint">Найдено 2 дубликата: те же файлы уже лежат в общей папке, и сборка берёт их оттуда. Уборка локальных копий освободит 6,9 ГБ.</p>
      <span class="btn primary" aria-disabled="true">Перенести в общую папку</span>
      <p class="hint">Перенесётся 24 файла, 30,1 ГБ. То, что пришло вместе со сборкой, остаётся: файлы-маркеры и папка с конфигами моделей.</p>
    </div>
  </div>
</Window>

## Своих моделей нет

*подключение есть, переносить нечего*

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item on"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="row">
      <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
      <span class="chip" style="--instance-accent:var(--accent-amber)">F</span>
      <h3>Flux эксперименты</h3>
      <span class="spacer"></span>
      <span class="pill stopped"><i></i>Остановлена</span>
      <span class="btn primary lg">Запустить</span>
    </div>
    <div class="tabs">
      <span>Обзор</span>
      <span aria-selected="true">Модели</span>
      <span>Воркфлоу</span>
      <span>Параметры</span>
    </div>
    <div class="row">
      <span class="t-label">Общие модели</span>
      <span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span>
    </div>
    <div class="toggle-row">
      <span class="toggle"></span>
      <div>
        <div class="t-base">Дать этой сборке доступ к общей папке, ничего в неё не копируя.</div>
        <div class="hint">D:\AI\_shared\models · папка сборки не тронута</div>
      </div>
    </div>
    <div class="group">
      <span class="t-label">Способ применения</span>
      <div class="seg">
        <span aria-pressed="true">Не трогать папку сборки</span>
        <span>Записать файл в сборку</span>
      </div>
      <p class="hint">Список общих папок хранится у ComfyUI Portable Organizer и передаётся сборке аргументом при запуске. В папку сборки не пишется ничего, но общие модели будут у неё только тогда, когда её запускает это приложение.</p>
    </div>
    <div class="row">
      <span class="t-label">Модели этой сборки</span>
      <span class="btn ghost icon" title="D:\AI\ComfyUI_flux\ComfyUI\models"><FolderOpen class="ico" /></span>
      <span class="spacer"></span>
      <span class="btn ghost"><RotateCw class="ico" />Обновить</span>
    </div>
    <!-- Отдел на месте: заголовок и путь к папке никуда не деваются,
         меняется только его содержимое. Поэтому строка состояния
         идёт подсказкой, а не крупным пустым блоком: тот выглядел бы
         отдельным экраном, приехавшим в середину вкладки, и рвал бы
         вкладку надвое сильнее, чем сама граница отделов.
         Ни списка, ни кнопок переноса при этом нет — распоряжаться
         нечем. -->
    <!-- `blank`, а не `hint`: это не пояснение к органу рядом,
         а сообщение о том, что списка нет. Вид у всех таких строк
         один и задан в одном месте. -->
    <p class="blank">Переносить нечего: своих моделей у этой сборки нет.</p>
  </div>
</Window>
