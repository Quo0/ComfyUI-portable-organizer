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

Блоков три, и каждый устроен одинаково: подпись, обведённый список
строк, одна строка объяснения под ним. Различать их размером заголовка
не выходило — экран читался одним полотном одинаковых полей, и главное
на нём, границу между «нашим» и «вашим», приходилось вычитывать.
Цветом уцелевшее не помечено: зелёный в этой палитре — цвет живого
процесса, и на списке папок он обещал бы состояние, которого у папки
нет. Границу держат заголовки блоков и строка под каждым списком.

<Window>
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item on"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <!-- Версия — один факт, а не блок: она стоит рядом с названием
         раздела, а место на экране отдано тому, что о папках.
         Название приложения здесь не повторяется: оно и так
         в заголовке окна. -->
    <div class="row"><h3>О приложении</h3><span class="hint">Версия 0.1.0</span></div>
    <!-- Каждый блок — подпись, обведённый список и одна строка
         объяснения под ним. Раньше блоки различались только
         размером заголовка, и экран читался одним полотном. -->
    <div class="cols">
      <div>
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
      </div>
      <div>
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
          <!-- Сборки отдельным списком под своей подписью: их бывает
               восемь, и вперемешку с общими папками имена сборок
               читались бы как ещё два вида общего добра. -->
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
          <p class="hint">Эти папки принадлежат вам. Удаление приложения их не трогает.</p>
        </div>
      </div>
    </div>
  </div>
</Window>

## На мониторе

Окно 1920×1080, уменьшено до 55%: колонки расходятся, «наше» и «ваше»
встают рядом.

<Window :fixed="true" :hd="true">
  <template #nav>
    <nav class="nav in-win">
      <div class="nav-item"><Layers class="ico" /><span>Инстансы</span></div>
      <div class="nav-item"><FolderPlus class="ico" /><span>Добавление</span></div>
      <div class="nav-item"><SlidersHorizontal class="ico" /><span>Настройки</span></div>
      <div class="nav-item on"><Info class="ico" /><span>О приложении</span></div>
    </nav>
  </template>
  <div class="content">
    <div class="row"><h3>О приложении</h3><span class="hint">Версия 0.1.0</span></div>
    <div class="cols">
      <div>
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
          <span class="t-label">Что приложение записало внутрь ваших сборок</span>
          <div class="paths">
            <div class="path-item"><span class="lbl">extra_model_paths.yaml</span><span class="val">Только в режиме «файл внутри сборки»</span></div>
            <div class="path-item"><span class="lbl">Копии воркфлоу</span><span class="val">Только те, что вы добавили в сборку</span></div>
          </div>
          <p class="hint">Оба остаются после удаления приложения — они лежат в чужой установке, и убирать их мы не вправе.</p>
        </div>
      </div>
      <div>
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
          <p class="hint">Эти папки принадлежат вам. Удаление приложения их не трогает.</p>
        </div>
      </div>
    </div>
  </div>
</Window>
