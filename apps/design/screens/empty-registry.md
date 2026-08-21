<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Пустой реестр

<!-- J-01 · шаг 1 · US-ONB-01 -->

Первое, что видит человек, у которого ещё ничего нет. Навигация доступна
сразу — приложение не прячет структуру за приветственным экраном.
Разделы, требующие инстансов, объясняют, что сделать сначала.

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
    <div class="row"><h3>Инстансы</h3></div>
    <div class="empty">
      <h4>Инстансов пока нет</h4>
      <p>Приложение запускает портабл-сборки ComfyUI, показывает логи старта и открывает интерфейс прямо в этом окне.</p>
      <!-- Ровно те же две дорожки, что и на экране «Добавление»:
           один компонент на два места. Пока они были описаны
           по месту, одна развилка разъехалась на три формулировки. -->
      <div class="forks">
        <div class="fork">
          <b>Папка уже есть</b>
          <p>Зарегистрировать распакованную сборку. Папка остаётся нетронутой.</p>
          <span class="btn secondary">Выбрать папку</span>
        </div>
        <div class="fork">
          <b>Распаковать из архива</b>
          <p>Распаковать портабл-сборку в одно или несколько мест и сразу их зарегистрировать.</p>
          <span class="btn primary">Выбрать архив .7z…</span>
        </div>
      </div>
    </div>
  </div>
</Window>
