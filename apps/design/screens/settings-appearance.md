<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, Palette, Database, Workflow, HardDrive, Archive } from '@lucide/vue';
</script>

# Настройки: внешний вид

<!-- US-UI-01 · US-UI-02 -->

Первый раздел настроек и самый короткий: тема, язык и образец того,
что от них меняется. Образец здесь не для красоты — язык меняет не только
надписи, но и числа, даты и формы слов, а этого в списке языков не видно.

Тема переключателем, а не списком: вариантов три, они видны разом,
и выбранный виден без раскрытия. «Как в системе» — состояние, а не
отсутствие выбора, поэтому под переключателем сказано, что система
выбрала сейчас. Язык — обычный список: языков будет больше трёх,
и подписи в нём всегда на самом языке, потому что искать свой
в чужом переводе неудобно.

Последняя строка отвечает на вопрос, который иначе задают ошибкой:
у ComfyUI внутри встроенной вкладки своя тема и свой язык, и приложение
их не трогает.

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
        <div class="nav-item on"><Palette class="ico" /><span>Внешний вид</span></div>
        <div class="nav-item"><Database class="ico" /><span>Общие модели</span></div>
        <div class="nav-item"><Workflow class="ico" /><span>Библиотека воркфлоу</span></div>
        <div class="nav-item"><HardDrive class="ico" /><span>Отчёт по диску</span></div>
        <div class="nav-item"><Archive class="ico" /><span>Архивы установщика</span></div>
      </nav>
      <div class="content">
        <h3>Внешний вид</h3>
        <div class="group">
          <span class="t-label">Тема</span>
          <div class="seg">
            <span>Светлая</span>
            <span>Тёмная</span>
            <span aria-pressed="true">Как в системе</span>
          </div>
          <p class="hint">Сейчас в Windows выбрана тёмная.</p>
        </div>
        <div class="group">
          <span class="t-label">Язык</span>
          <div class="input" style="width:240px"><span>Русский</span></div>
        </div>
        <div class="group">
          <span class="t-label">Предпросмотр</span>
          <div class="pane">
            <div class="pane-head">
              <span class="title">Числа, даты и формы слов подчиняются выбранному языку.</span>
            </div>
            <div class="scroll"><div class="scroll-pad">
              <div class="row">
                <span class="pill running"><i></i>Работает</span>
                <span class="pill stopped"><i></i>Остановлен</span>
              </div>
              <div class="meta">
                <span>3 инстанса</span>
                <span>Размер на диске: 52,4 ГБ</span>
                <span>Последний запуск: сегодня, 14:20</span>
              </div>
            </div></div>
          </div>
        </div>
        <p class="hint">У ComfyUI внутри встроенной вкладки своя тема и свой язык — приложение их не меняет.</p>
      </div>
    </div>
  </div>
</Window>
