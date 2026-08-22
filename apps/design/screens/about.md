<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, FolderOpen } from '@lucide/vue';
</script>

# О приложении

<!-- J-06 · шаг 1 · US-DATA-01 · US-DATA-03 -->

Требование сформулировано прямо: удалил приложение — исчезло всё лишнее,
но не модели и не библиотека. Чтобы этому можно было доверять, границу
видно заранее, а не после удаления. Отдельным блоком названы файлы,
которые приложение по команде пользователя записало внутрь чужих папок:
они останутся, и это честнее скрыть нельзя.

Блоков четыре, и каждый устроен одинаково: подпись, обведённый список
строк, одна строка объяснения под ним. Различать их размером заголовка
не выходило — экран читался одним полотном одинаковых полей, и главное
на нём, границу между «нашим» и «вашим», приходилось вычитывать.
Цветом уцелевшее не помечено: зелёный в этой палитре — цвет живого
процесса, и на списке папок он обещал бы состояние, которого у папки
нет. Границу держат заголовки блоков и строка под каждым списком.

Обновление живёт здесь же, над сеткой и во всю ширину. Версия уже стоит
в шапке этого экрана, и вопрос «что у меня установлено и есть ли новее» —
один вопрос, а не два в разных разделах. В сетку пятым блоком он не встаёт:
остальные четыре отвечают, где что лежит, а этот предлагает действие.

Новую версию видно и снаружи экрана — точкой у раздела в рейле. Тост для
этого не годится: проверка идёт при запуске, тост пришлось бы либо закрыть,
либо потерять, и узнать о версии второй раз было бы неоткуда. Точка синяя,
из палитры акцентов: зелёная обещала бы работающий процесс, красная —
падение, а речь про новость.

Сборки в реестре — свой блок, а не список внутри «Останется на диске»:
их бывает восемь, и вперемешку с общими папками имена сборок читались бы
ещё двумя видами общего добра. В приложении раскладка — CSS-грид
с именованными областями: широкая панель ставит «Что исчезнет» в пару
с «Останется на диске» строкой выше и «Что приложение записало» в пару
со «Сборками в реестре» строкой ниже, а порог зависит от ширины самой
панели (`@container`), не окна. Здесь, в статичной витрине, четыре блока —
прямые дети авто-грида `.cols` в том же DOM-порядке: на широкой панели
он расставляет их по два в ряд той же парой, а на узкой складывает в одну
колонку в том же порядке, что и живой грид приложения. Расхождение
с приложением только в механизме (авто-грид против именованных областей),
не в результате.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item on"><Info class="ico" /><span>О приложении</span><i class="dot nav-mark"></i></div>
    </nav>
  </template>
  <div class="content">
    <!-- Версия — один факт, а не блок: она стоит рядом с названием
         раздела, а место на экране отдано тому, что о папках.
         Название приложения здесь не повторяется: оно и так
         в заголовке окна. -->
    <div class="row"><h3>О приложении</h3><span class="hint">Версия 0.1.0</span></div>
    <!-- Обновление над сеткой и во всю ширину: остальные четыре блока
         о том, где что лежит, а этот предлагает действие, и в пару
         ни к одному из них не встаёт. Номер версии в заголовке
         обязателен — «доступно обновление» без него не даёт решить,
         нужно ли оно прямо сейчас. -->
    <div class="group">
      <span class="t-label">Обновления</span>
      <div class="banner">
        <div><b>Доступна версия 0.2.0</b><p class="t-sm">У вас 0.1.0 · Выпущена 22 авг. 2026 г.</p></div>
        <span class="spacer"></span>
        <span class="btn primary">Скачать и установить</span>
      </div>
      <div>
        <span class="t-label">Что изменилось</span>
        <p class="t-sm">Библиотека воркфлоу переехала в «Настройки». Отмена распаковки больше не выглядит зависанием.</p>
      </div>
      <div class="row"><span class="btn secondary">Проверить сейчас</span></div>
      <div class="toggle-row">
        <span class="toggle"></span>
        <div>
          <div class="t-base">Проверять обновления при запуске</div>
          <p class="hint">Единственное, что приложение отправляет наружу: номер версии, чтобы спросить, вышла ли новее.</p>
        </div>
      </div>
    </div>
    <!-- Каждый блок — подпись, обведённый список и одна строка
         объяснения под ним. Раньше блоки различались только
         размером заголовка, и экран читался одним полотном.
         Четыре блока — прямые дети `.cols`, в том же DOM-порядке,
         что и в приложении: свернувшись в одну колонку на узкой
         панели, порядок остаётся тем же, что и в живом гриде,
         а не только раскладка на широкой. -->
    <div class="cols">
      <div class="group">
        <span class="t-label">Что исчезнет при удалении</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Настройки и реестр сборок<span class="hint">C:\Users\andrew\AppData\Roaming\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Roaming\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Кэш и данные браузера<span class="hint">C:\Users\andrew\AppData\Local\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Local\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Удаление уносит само приложение и его настройки. Модели, общие шаблоны и папки ComfyUI остаются на месте.</p>
      </div>
      <div class="group">
        <span class="t-label">Останется на диске</span>
        <!-- Ради этого блока экран и существует: сотни гигабайт
             не должны зависеть от веры в то, что деинсталлятор
             поступит правильно. Отвечает за это заголовок блока
             и строка под списком, а не подкраска строк. -->
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Общие модели<span class="hint">D:\AI\_shared\models</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Библиотека воркфлоу<span class="hint">D:\AI\_shared\workflows</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\workflows"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Эти папки принадлежат вам. Удаление приложения их не трогает.</p>
      </div>
      <!-- Отдельный блок, а не строка среди уцелевшего: тот
           отвечает на вопрос «где моё», а этот — «что вы тронули
           в моём». Два вида файлов названы по одному, строками:
           абзацем они читались как оговорка. -->
      <div class="group">
        <span class="t-label">Что приложение записало внутрь ваших сборок</span>
        <div class="paths">
          <div class="path-item"><span class="lbl">extra_model_paths.yaml</span><span class="val">Только в режиме «файл внутри сборки»</span></div>
          <div class="path-item"><span class="lbl">Копии воркфлоу</span><span class="val">Только те, что вы добавили в сборку</span></div>
        </div>
        <p class="hint">Оба остаются после удаления приложения — они лежат в чужой установке, и убирать их мы не вправе.</p>
      </div>
      <!-- Сборки своим блоком, а не списком внутри «Останется на диске»:
           их бывает восемь, и вперемешку с общими папками имена сборок
           читались бы как ещё два вида общего добра. Отдельный блок
           заодно встаёт в пару с «Что приложение записало» на широкой
           панели — оба про то, что приходится на каждую сборку. -->
      <div class="group">
        <span class="t-label">Сборки в реестре</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">SDXL стабильная<span class="hint">D:\AI\comfy-sdxl</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-sdxl"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Flux тест<span class="hint">D:\AI\comfy-flux</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-flux"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Анимация<span class="hint">D:\AI\comfy-anim</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-anim"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Реестр только запоминает, где лежит каждая сборка. Удаление приложения стирает записи, а не сами сборки.</p>
      </div>
    </div>
  </div>
</Window>

## На мониторе

Окно 1920×1080, уменьшено до 55%: панель шире порога, и в приложении это
включило бы вторую колонку именованной сетки. У авто-грида витрины порог
свой (`minmax(480px, 1fr)`), но на этой ширине оба срабатывают одинаково:
те же четыре блока в том же DOM-порядке встают по два в ряд — «что
исчезнет» с «останется на диске» строкой выше, «что дописано в сборки»
со «сборками в реестре» строкой ниже.

<Window :fixed="true" :hd="true">
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item on"><Info class="ico" /><span>О приложении</span><i class="dot nav-mark"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row"><h3>О приложении</h3><span class="hint">Версия 0.1.0</span></div>
    <!-- Обновление над сеткой и во всю ширину: остальные четыре блока
         о том, где что лежит, а этот предлагает действие, и в пару
         ни к одному из них не встаёт. Номер версии в заголовке
         обязателен — «доступно обновление» без него не даёт решить,
         нужно ли оно прямо сейчас. -->
    <div class="group">
      <span class="t-label">Обновления</span>
      <div class="banner">
        <div><b>Доступна версия 0.2.0</b><p class="t-sm">У вас 0.1.0 · Выпущена 22 авг. 2026 г.</p></div>
        <span class="spacer"></span>
        <span class="btn primary">Скачать и установить</span>
      </div>
      <div>
        <span class="t-label">Что изменилось</span>
        <p class="t-sm">Библиотека воркфлоу переехала в «Настройки». Отмена распаковки больше не выглядит зависанием.</p>
      </div>
      <div class="row"><span class="btn secondary">Проверить сейчас</span></div>
      <div class="toggle-row">
        <span class="toggle"></span>
        <div>
          <div class="t-base">Проверять обновления при запуске</div>
          <p class="hint">Единственное, что приложение отправляет наружу: номер версии, чтобы спросить, вышла ли новее.</p>
        </div>
      </div>
    </div>
    <div class="cols">
      <div class="group">
        <span class="t-label">Что исчезнет при удалении</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Настройки и реестр сборок<span class="hint">C:\Users\andrew\AppData\Roaming\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Roaming\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Кэш и данные браузера<span class="hint">C:\Users\andrew\AppData\Local\com.cpo.organizer</span></span>
            <span class="acts"><span class="btn ghost icon" title="C:\Users\andrew\AppData\Local\com.cpo.organizer"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Удаление уносит само приложение и его настройки. Модели, общие шаблоны и папки ComfyUI остаются на месте.</p>
      </div>
      <div class="group">
        <span class="t-label">Останется на диске</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">Общие модели<span class="hint">D:\AI\_shared\models</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\models"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Библиотека воркфлоу<span class="hint">D:\AI\_shared\workflows</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\_shared\workflows"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Эти папки принадлежат вам. Удаление приложения их не трогает.</p>
      </div>
      <div class="group">
        <span class="t-label">Что приложение записало внутрь ваших сборок</span>
        <div class="paths">
          <div class="path-item"><span class="lbl">extra_model_paths.yaml</span><span class="val">Только в режиме «файл внутри сборки»</span></div>
          <div class="path-item"><span class="lbl">Копии воркфлоу</span><span class="val">Только те, что вы добавили в сборку</span></div>
        </div>
        <p class="hint">Оба остаются после удаления приложения — они лежат в чужой установке, и убирать их мы не вправе.</p>
      </div>
      <div class="group">
        <span class="t-label">Сборки в реестре</span>
        <div class="paths with-acts">
          <div class="path-item">
            <span class="lbl">SDXL стабильная<span class="hint">D:\AI\comfy-sdxl</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-sdxl"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Flux тест<span class="hint">D:\AI\comfy-flux</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-flux"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Анимация<span class="hint">D:\AI\comfy-anim</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-anim"><FolderOpen class="ico" /></span></span>
          </div>
          <div class="path-item">
            <span class="lbl">Эксперименты<span class="hint">D:\AI\comfy-lab</span></span>
            <span class="acts"><span class="btn ghost icon" title="D:\AI\comfy-lab"><FolderOpen class="ico" /></span></span>
          </div>
        </div>
        <p class="hint">Реестр только запоминает, где лежит каждая сборка. Удаление приложения стирает записи, а не сами сборки.</p>
      </div>
    </div>
  </div>
</Window>

## Установка при работающих сборках

Инсталлятор Windows закрывает приложение принудительно, а дочерние
процессы живут в Job Object и уходят вместе с ним. Обновиться посреди
генерации — потерять очередь и минуты холодного старта, поэтому решение
принимает пользователь, а не мы.

Развилка раскрывается баннером на месте: модалку над областью контента
положить нельзя (дисциплина z-order), а у тоста не бывает кнопок. Кнопка
установки из верхнего баннера при этом уходит — иначе на экране было бы
два способа начать одно и то же, и один из них в обход только что
заданного вопроса.

Полоса загрузки индетерминантная, пока сервер не прислал длину: доля
скачанного без известного целого — выдумка.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item on"><Info class="ico" /><span>О приложении</span><i class="dot nav-mark"></i></div>
    </nav>
  </template>
  <div class="content">
    <div class="row"><h3>О приложении</h3><span class="hint">Версия 0.1.0</span></div>
    <div class="group">
      <span class="t-label">Обновления</span>
      <div class="banner">
        <div><b>Доступна версия 0.2.0</b><p class="t-sm">У вас 0.1.0 · Выпущена 22 авг. 2026 г.</p></div>
      </div>
      <!-- Имена сборок в вопросе не для красоты: пользователь должен
           узнать в них ту, где сейчас идёт генерация. -->
      <div class="banner">
        <div>
          <b>Сборки ещё работают</b>
          <p class="t-sm">Установка закроет приложение, и работающие серверы уйдут вместе с ним: SDXL стабильная, Flux тест. Несохранённые графы и очередь генерации — тоже.</p>
        </div>
        <span class="spacer"></span>
        <span class="btn secondary">Остановить и установить</span>
        <span class="btn ghost">Отложить до следующего запуска</span>
      </div>
      <div class="track"><i style="width: 42%"></i></div>
      <p class="hint">18,4 МБ из 43,7 МБ</p>
      <div class="row"><span class="btn secondary">Проверить сейчас</span></div>
      <div class="toggle-row">
        <span class="toggle"></span>
        <div>
          <div class="t-base">Проверять обновления при запуске</div>
          <p class="hint">Единственное, что приложение отправляет наружу: номер версии, чтобы спросить, вышла ли новее.</p>
        </div>
      </div>
    </div>
  </div>
</Window>
