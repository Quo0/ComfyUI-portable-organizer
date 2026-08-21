<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive, Plus, ListChecks, Check, ExternalLink } from '@lucide/vue';
</script>

# Настройки: библиотека воркфлоу

<!-- J-05 · шаги 4-5 · US-WF-02 · US-WF-04 · US-WF-05 -->

Библиотека — раздел настроек, а не раздел рейла. Она устроена ровно как
общие модели: одна папка снаружи сборок и её содержимое. Настройка папки
и то, что в ней лежит, стояли в разных местах — путь в настройках, список
в рейле, — и на вопрос «а куда это складывается» приходилось отвечать
переходом в другой раздел. Теперь и путь, и содержимое на одном экране,
а в рейле остались четыре постоянных места: сборки, откуда взять новую,
настройки, о программе.

Проверка совместимости — то, ради чего перенос живёт в приложении, а не в
проводнике. У остановленного инстанса точного ответа нет, и честнее
пометить его неизвестным, чем показать зелёную галочку.

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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <!-- Поля даёт содержимое: у шапки и у мастер-детейла они свои. -->
      <div class="content flush">
        <!-- Путь стоит в шапке раздела, а не отдельным полем над
             списком: это одна строка, которую задают один раз, и
             отводить под неё блок в полэкрана значило бы говорить
             о ней громче, чем о самой библиотеке. -->
        <!-- Способы пополнить библиотеку собраны в одну кнопку
             с меню и отжаты к правому краю. Их два, они об одном,
             и рядом с путём читались как три равноправных органа;
             освободившееся место отдано пути — ради него ряд
             и существует. -->
        <div class="lib-head">
          <h3>Библиотека воркфлоу</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Добавить</span>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <!-- Множественный выбор объявляется отсюда, а не
                   заводится сам от первой отметки. Кнопка стоит
                   до поля поиска: она меняет, чем список является,
                   а поиск и избранное лишь сужают показанное. -->
              <span class="btn ghost icon"><ListChecks class="ico" /></span>
              <div class="input search"><span>Поиск по имени и тегам</span></div>
              <span class="btn ghost icon" aria-pressed="true"><span class="star">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad" style="gap:1px">
              <!-- Отметок в обычном режиме нет: держать под них
                   пустую колонку постоянно значило бы отодвигать
                   имена ради органа, которого на экране нет. -->
              <div class="wf-row"><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span><span class="tag">upscale</span></span></div>
              <div class="wf-row on"><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
              <div class="wf-row"><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
              <div class="wf-row lost"><span class="star off">★</span><span class="nm">video / ltx-draft.json</span><span class="tags"><span class="tag stop">файла нет</span></span></div>
            </div></div>
            <!-- Счётчик внизу списка, а не в его шапке: шапка занята
                 поиском, а число отвечает на вопрос о том, что под
                 ним, — и отвечает после фильтра, а не до него. -->
            <div class="pane-foot"><span class="t-label">14 воркфлоу</span></div>
          </div>
          <div class="pane">
            <div class="pane-head">
              <span class="title">portrait-v3.json</span>
              <span class="star lg">★</span>
            </div>
            <div class="tabs">
              <span aria-selected="true">Где откроется</span>
              <span>Заметка</span>
              <span>Теги <span class="n">1</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="compat">
                <!-- Уход на страницу сборки — в правом углу, в одну
                     строку с именем: это переход к другому объекту,
                     а не действие над воркфлоу, и путать его с «положить»
                     нельзя. -->
                <div class="compat-row ok">
                  <span class="chip" style="--instance-accent:var(--accent-teal)"></span>
                  <span class="act on"><Check class="ico" /></span>
                  <span class="nm">SDXL стабильная</span><span class="compat-note">все ноды на месте</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
                <div class="compat-row warn">
                  <span class="chip" style="--instance-accent:var(--accent-indigo)"></span>
                  <span class="act"><Plus class="ico" /></span>
                  <span class="nm">Flux тест</span><span class="compat-note">нет 2 нод</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
                <div class="missing">IPAdapterUnifiedLoader · ReActorFaceSwap</div>
                <div class="compat-row">
                  <span class="chip" style="--instance-accent:var(--accent-moss)"></span>
                  <span class="act"><Plus class="ico" /></span>
                  <span class="nm">Эксперименты</span><span class="compat-note">по данным последнего запуска</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
              </div>
            </div></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Папка ещё не выбрана

Раздел настройки и есть, поэтому уводить некуда — выбор стоит здесь же.

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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <h3>Библиотека воркфлоу</h3>
        <div class="empty">
          <p>
            Библиотека — папка вне сборок. Воркфлоу в ней переживают
            переустановку ComfyUI и видны всем сборкам сразу.
          </p>
          <div class="field" style="width:100%;max-width:520px">
            <label>Папка библиотеки</label>
            <div class="path-row">
              <div class="input mono"><span>не выбрана</span></div>
              <span class="btn secondary">Обзор</span>
            </div>
            <!-- Подсказка рядом с общими моделями — не правило,
                 а удобство: они обычно на просторном диске. -->
            <div class="hint">Рядом с общими моделями: D:\AI\_shared\workflows</div>
          </div>
          <span class="btn primary">Положить рядом с общими моделями</span>
        </div>
      </div>
    </div>
  </div>
</Window>

## Вставка из буфера

Граф приходит текстом — из чата, с форума, из соседней машины.

Воркфлоу чаще присылают текстом, чем файлом, и сохранять присланное
в файл только ради того, чтобы тут же выбрать его в диалоге, — лишний
круг. Форма занимает место списка, а не всплывает над ним: тот же
компонент встанет на вкладку «Воркфлоу» у сборки, где поверх нашего
HTML лежит нативное окно ComfyUI и всплыть физически не над чем.
Имя спрашивается сразу: у текста из буфера его нет, а придумывать
за пользователя «workflow (3)» значит гарантировать свалку.

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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <div class="row">
          <h3>Библиотека воркфлоу</h3>
          <span class="spacer"></span>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
        </div>
        <div class="paste">
          <div class="field">
            <label>Имя</label>
            <div class="input"><span>portrait-v3</span></div>
            <!-- Расширение дописывается само: набранное руками
                 «.jsn» или «.json.json» — не выбор пользователя,
                 а описка, и ловить её вопросом незачем. -->
            <div class="hint">Сохранится как portrait-v3.json</div>
          </div>
          <div class="field">
            <label>JSON воркфлоу</label>
            <div class="input area mono">{"last_node_id": 42, "last_link_id": 61, "nodes": [{"id": 1, "type": "CheckpointLoaderSimple", "pos": [80, 120], "widgets_values": ["sd_xl_base_1.0.safetensors"]}, {"id": 2, "type": "CLIPTextEncode", …</div>
            <!-- Разбор идёт по ходу набора, а не по нажатию:
                 «Сохранить» на неразобранном тексте — это вопрос,
                 ответ на который уже известен. -->
            <div class="hint">Разобрано: 27 нод · IPAdapterUnifiedLoader и ещё 12 типов</div>
          </div>
          <div class="row">
            <span class="btn primary">Сохранить в библиотеку</span>
            <span class="btn ghost">Отмена</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## В буфере не воркфлоу

Отказ на месте, до записи файла.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <div class="row">
          <h3>Библиотека воркфлоу</h3>
          <span class="spacer"></span>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
        </div>
        <div class="paste">
          <div class="field">
            <label>Имя</label>
            <!-- Занятое имя — не ошибка ввода: файл в библиотеке
                 не перезаписывается никогда, и сказать об этом
                 надо до нажатия, а не после. -->
            <div class="input bad"><span>portrait-v3</span></div>
            <div class="hint bad">portrait-v3.json в библиотеке уже есть. Заменить нечем: выберите другое имя.</div>
          </div>
          <div class="field">
            <label>JSON воркфлоу</label>
            <div class="input area mono bad">{"prompt": "закат над морем", "steps": 30}</div>
            <div class="hint bad">Это JSON, но не воркфлоу: в нём нет узлов. Ничего не сохранено.</div>
          </div>
          <div class="row">
            <span class="btn primary" aria-disabled="true">Сохранить в библиотеку</span>
            <span class="btn ghost">Отмена</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Реальный объём данных

**Прокрутка.** Двести воркфлоу; список разделов и панель деталей не
уезжают вместе со списком.

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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Библиотека воркфлоу</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Добавить</span>
          </div>
        </div>
        <!-- Множественный выбор. Панель справа заменяется целиком:
             раньше она жила в двух режимах разом — заголовок
             со звёздочкой про выбранный воркфлоу, а тело под ним
             про все отмеченные. -->
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon" aria-pressed="true"><ListChecks class="ico" /></span>
              <div class="input search"><span>Поиск по имени и тегам</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list picking">
                <!-- data-repeat="3": исходник повторяет один и тот же
                     блок из восьми строк трижды подряд, изображая
                     двести воркфлоу без двухсот уникальных записей.
                     Ниже — те же три повтора, расписанные вручную. -->
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / img2img-refine.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">flux / lora-stack.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / img2img-refine.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">flux / lora-stack.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">sdxl / controlnet-depth.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / img2img-refine.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">flux / lora-stack.json</span><span class="tags"><span class="tag">flux</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">214 воркфлоу</span></div>
          </div>
          <div class="pane">
            <!-- Кнопка в шапке, счётчик отмеченного в подвале —
                 как и счётчик списка слева: обе панели отвечают
                 «сколько тут» внизу, а действие держат наверху. -->
            <div class="pane-head"><span class="title">Множественный выбор</span><span class="btn primary">Добавить</span></div>
            <div class="scroll"><div class="scroll-pad">
              <!-- Сборки отмечаются, а не запускают запись по клику:
                   файлы пишутся на диск необратимо, и начинать это
                   попаданием мышью в строку нельзя. Кладёт кнопка
                   внизу, которая так и подписана. -->
              <div class="pick-list">
                <div class="pick-head"><span class="check mixed"></span><span>Все сборки</span></div>
                <div class="pick-row"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="chip" style="--instance-accent:var(--accent-teal)"></span><span class="nm">SDXL стабильная</span></div>
                <div class="pick-row"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="chip" style="--instance-accent:var(--accent-indigo)"></span><span class="nm">Flux тест</span></div>
                <div class="pick-row"><span class="check"></span><span class="chip" style="--instance-accent:var(--accent-moss)"></span><span class="nm">Эксперименты</span></div>
                <div class="pick-row"><span class="check"></span><span class="chip" style="--instance-accent:var(--accent-azure)"></span><span class="nm">Видео</span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">отмечено 6 воркфлоу</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Массовая запись идёт

Панель занята ходом операции: выбирать сборки не во что.

Счёт идёт по операциям, а не по файлам: два воркфлоу в две сборки — это
два файла, но четыре независимые записи, и каждая может отказать своей
причиной. В счётчике стоят удавшиеся, а полоса идёт по пройденным:
её вопрос — «сколько ещё ждать», и на отказах она обязана двигаться,
иначе операция выглядит зависшей.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Библиотека воркфлоу</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Добавить</span>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon" aria-pressed="true"><ListChecks class="ico" /></span>
              <div class="input search"><span>Поиск по имени и тегам</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list picking">
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">5 воркфлоу</span></div>
          </div>
          <div class="pane">
            <!-- Кнопки «Добавить» в шапке нет: пока идёт запись,
                 предлагать начать ещё одну поверх неё незачем. -->
            <div class="pane-head"><span class="title">Множественный выбор</span></div>
            <div class="scroll"><div class="scroll-pad">
              <div class="group">
                <p class="t-sm">выполнено 1 из 4 операций</p>
                <div class="bar"><i style="width:50%"></i></div>
                <div class="row"><span class="btn danger">Отмена</span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">отмечено 2 воркфлоу</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Отчёт: не прошло ничего

Оба воркфлоу уже лежат в обеих сборках.

Занятое имя — не ошибка, а отказ по правилу: перезаписи в массовой
операции нет вовсе, и вопросов она не задаёт — двадцать подряд задавать
нельзя. Поэтому у каждой пары своя строка и своя причина: сверху пара
«воркфлоу → сборка» в цвете основного текста, под ней объяснение
вполголоса. Красный оставлен одной строке со счётом: тревога, размазанная
по всему блоку, перестаёт быть тревогой, а четыре одинаковые красные
строки читаются как поломка приложения. Совет стоит один на весь список,
а не повторяется в каждой строке.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Библиотека воркфлоу</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary"><Plus class="ico" />Добавить</span>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon" aria-pressed="true"><ListChecks class="ico" /></span>
              <div class="input search"><span>Поиск по имени и тегам</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list picking">
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="check on"><svg viewBox="0 0 10.1668 10.1668"><path d="M1 5.52 3.92 9.17 9.17 1"/></svg></span><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">video / ltx-basic.json</span><span class="tags"><span class="tag">video</span></span></div>
                <div class="wf-row"><span class="check"></span><span class="star off">★</span><span class="nm">utils / batch-rename.json</span><span class="tags"><span class="tag">utils</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">5 воркфлоу</span></div>
          </div>
          <div class="pane">
            <div class="pane-head"><span class="title">Множественный выбор</span></div>
            <div class="scroll"><div class="scroll-pad">
              <div class="group">
                <p class="t-sm">выполнено 0 из 4 операций</p>
                <div class="bar"><i style="width:100%"></i></div>
                <p class="hint bad">не прошло 4</p>
                <div class="fails">
                  <div class="fail">
                    <span class="fail-pair">sdxl / base-upscale.json → SDXL стабильная</span>
                    <span class="fail-why">в сборке уже есть воркфлоу с таким именем, он не тронут</span>
                  </div>
                  <div class="fail">
                    <span class="fail-pair">sdxl / base-upscale.json → Flux тест</span>
                    <span class="fail-why">в сборке уже есть воркфлоу с таким именем, он не тронут</span>
                  </div>
                  <div class="fail">
                    <span class="fail-pair">flux / portrait-v3.json → SDXL стабильная</span>
                    <span class="fail-why">в сборке уже есть воркфлоу с таким именем, он не тронут</span>
                  </div>
                  <div class="fail">
                    <span class="fail-pair">flux / portrait-v3.json → Flux тест</span>
                    <span class="fail-why">в сборке уже есть воркфлоу с таким именем, он не тронут</span>
                  </div>
                </div>
                <p class="hint">Заменить можно только по одному: перед перезаписью приложение спросит.</p>
                <div class="row"><span class="btn ghost">Закрыть</span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">отмечено 2 воркфлоу</span></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>

## Меню «Добавить» раскрыто

Два способа пополнить библиотеку под одной кнопкой.

Способов два, и они об одном; рядом с путём они читались как три
равноправных органа, и место, которое занимали, отдано пути — ради него
ряд и существует. Это единственная всплывашка в приложении, кроме тостов,
и ставить её на экране встроенной вкладки нельзя: там поверх нашего HTML
лежит нативное окно, и меню просто не будет видно. Поэтому форма вставки
разворачивается на месте списка, а не всплывает над ним.

<Window>
  <template #nav>
    <nav class="nav in-win collapsed">
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
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item on"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content flush framed no-foot">
        <div class="lib-head">
          <h3>Библиотека воркфлоу</h3>
          <div class="path-row grow">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <span class="spacer"></span>
          <div class="menu">
            <span class="btn secondary" aria-expanded="true"><Plus class="ico" />Добавить</span>
            <div class="menu-pop">
              <span>Из файла…</span>
              <span>Вставить текстом</span>
            </div>
          </div>
        </div>
        <div class="split-master">
          <div class="pane">
            <div class="pane-head">
              <span class="btn ghost icon"><ListChecks class="ico" /></span>
              <div class="input search"><span>Поиск по имени и тегам</span></div>
              <span class="btn ghost icon"><span class="star off">★</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="wf-list">
                <div class="wf-row"><span class="star">★</span><span class="nm">sdxl / base-upscale.json</span><span class="tags"><span class="tag">sdxl</span></span></div>
                <div class="wf-row on"><span class="star">★</span><span class="nm">flux / portrait-v3.json</span><span class="tags"><span class="tag">flux</span></span></div>
                <div class="wf-row"><span class="star off">★</span><span class="nm">inpaint / face-fix.json</span><span class="tags"><span class="tag">inpaint</span></span></div>
              </div>
            </div></div>
            <div class="pane-foot"><span class="t-label">14 воркфлоу</span></div>
          </div>
          <div class="pane">
            <div class="pane-head">
              <span class="title">portrait-v3.json</span>
              <span class="star lg">★</span>
            </div>
            <div class="tabs">
              <span aria-selected="true">Где откроется</span>
              <span>Заметка</span>
              <span>Теги <span class="n">1</span></span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="compat">
                <div class="compat-row ok">
                  <span class="chip" style="--instance-accent:var(--accent-teal)"></span>
                  <span class="act on"><Check class="ico" /></span>
                  <span class="nm">SDXL стабильная</span><span class="compat-note">все ноды на месте</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
                <div class="compat-row">
                  <span class="chip" style="--instance-accent:var(--accent-moss)"></span>
                  <span class="act"><Plus class="ico" /></span>
                  <span class="nm">Эксперименты</span><span class="compat-note">по данным последнего запуска</span>
                  <span class="act open"><ExternalLink class="ico" /></span>
                </div>
              </div>
            </div></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</Window>
