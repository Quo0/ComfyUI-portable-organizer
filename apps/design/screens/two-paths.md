<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Две дорожки

<!-- ревизия · US-ONB-02 · US-REG-01 -->

Раздел постоянный, а не только для первого запуска, и это единственное
место, где заводят сборки: со списка инстансов кнопка «Добавить»
убрана, двух дверей в одну комнату быть не должно.

Дорожки идут слева направо, а не одна под другой: это развилка, и обе
ветки должны быть видны разом, чтобы выбирать было из чего.

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
    <h3>Добавление сборки</h3>
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
</Window>
