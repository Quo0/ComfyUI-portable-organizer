<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Выход при работающих сборках

<!-- US-RUN-08 -->

Серверы — дочерние процессы приложения и уходят вместе с ним. Молча
закрыться значило бы оборвать чужую генерацию и потерять несохранённые
графы, поэтому вместо закрытия показывается этот экран: что именно
работает и два честных выхода.

Развилка настоящая, а не «да / отмена»: свернуть в трей — полноценный
ответ, а не отказ от действия. Красная кнопка называет последствие
словами, а не «ОК». Список работающих не декоративный: он отвечает
на вопрос «что именно я сейчас остановлю» до нажатия, а не после.

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
    <h3>Серверы ещё работают</h3>
    <p class="t-sm">Закрытие приложения останавливает все работающие серверы: они его дочерние процессы и уходят вместе с ним. Несохранённые графы и очередь генерации — тоже.</p>
    <div class="cards">
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-teal)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="card-name">SDXL стабильная</span>
            <span class="pill running"><i></i>Работает</span>
            <span class="t-mono">:8188</span>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="card-name">Flux тест</span>
            <span class="pill starting"><i></i>Стартует</span>
            <span class="t-mono">:8189</span>
          </div>
        </div>
      </div>
    </div>
    <div class="row">
      <span class="btn danger lg">Остановить всё и выйти</span>
      <span class="btn secondary lg">Свернуть в трей</span>
    </div>
    <p class="hint">Приложение останется в области уведомлений — щелчок по значку вернёт его.</p>
  </div>
</Window>
