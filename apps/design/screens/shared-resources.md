<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info, ArrowLeft } from '@lucide/vue';
</script>

# Общие ресурсы

<!-- J-01 · шаг 5 · US-SHARED-01 · US-WF-02 -->

Ресурса два, и они про разное: модели и воркфлоу. Раньше всё шло одним
списком полей, и понять, к чему относится очередной путь, можно было
только по подписи — отсюда две отдельные панели.

Настраивается прямо здесь, теми же компонентами, что в «Настройках»:
уводить за одной папкой в другой раздел посреди установки незачем
`US-SHARED-01/AC-4`.

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
      <span class="step now"><u>3</u>Общие ресурсы</span><span class="step-sep"></span>
      <span class="step"><u>4</u>Распаковка</span><span class="step-sep"></span>
      <span class="step"><u>5</u>Готово</span>
    </div>
    <div class="step-bar">
      <h3>Подключить новые сборки к общим моделям</h3>
      <span class="spacer"></span>
      <span class="acts">
        <span class="btn ghost"><ArrowLeft class="ico" />Назад</span>
        <span class="btn primary lg">Распаковать</span>
      </span>
    </div>
    <div class="pane">
      <div class="pane-head"><span class="title">Общие модели</span></div>
      <div class="scroll-pad">
        <div class="field">
          <span class="t-label">Общий корень моделей</span>
          <div class="path-row">
            <div class="input mono"><span>D:\AI\_shared\models</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <div class="hint">распознано 5 категорий · 231 ГБ</div>
        </div>
        <div class="toggle-row">
          <span class="toggle"></span>
          <div>
            <div class="t-base">Подключить обе новые сборки</div>
            <div class="hint">Модель, загруженная в одной сборке, станет видна всем.</div>
          </div>
        </div>
        <!-- Способ применения показывается только при включённом
             тумблере: без подключения выбирать нечего. -->
        <div class="field">
          <span class="t-label">Способ применения</span>
          <div class="seg">
            <span aria-pressed="true">Не трогать папку сборки</span>
            <span>Записать файл в сборку</span>
          </div>
          <div class="hint">Конфиг живёт у приложения и передаётся сборке при запуске. В папку сборки не пишется ничего.</div>
        </div>
      </div>
    </div>
    <div class="pane">
      <div class="pane-head"><span class="title">Библиотека воркфлоу</span></div>
      <div class="scroll-pad">
        <div class="field">
          <span class="t-label">Папка библиотеки</span>
          <div class="path-row">
            <div class="input mono"><span>D:\AI\_shared\workflows</span></div>
            <span class="btn secondary">Обзор</span>
          </div>
          <div class="hint">14 воркфлоу</div>
        </div>
      </div>
    </div>
  </div>
</Window>
