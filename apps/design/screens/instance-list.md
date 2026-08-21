<script setup lang="ts">
import { Layers, FolderPlus, SlidersHorizontal, Info } from '@lucide/vue';
</script>

# Список сборок

<!-- ревизия · US-REG-04 · US-RUN-05 -->

Карточки идут сеткой: на развёрнутом окне колонка во всю ширину
оставляла справа пустоту, а вниз уходило то, что помещается рядом.
В шапке поиск и порядок — при пяти-восьми сборках список перестаёт
читаться глазами.

Кнопки «Добавить» здесь нет: раздел «Добавление» и есть место, где
сборки заводят, и двух дверей в одно место быть не должно.
В карточке — то, по чему сборки различают: состояние, порт, версия,
размер и когда её запускали в последний раз.

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
      <span class="t-sm">5</span>
      <span class="spacer"></span>
      <div class="input" style="width:220px">Поиск по имени</div>
      <div class="seg">
        <span aria-pressed="true">по имени</span>
        <span>по запуску</span>
        <span>по размеру</span>
      </div>
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
          <div class="meta">
            <span>0.30.2</span><span>:8188</span><span>52,4 ГБ</span>
            <span class="tag">общие модели</span>
          </div>
          <div class="src">Последний запуск: сегодня, 14:20</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-indigo)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-indigo)">F</span>
            <div class="card-name">Flux тест</div>
            <span class="pill starting"><i></i>Стартует</span>
          </div>
          <div class="card-desc">Новые ноды проверяю здесь</div>
          <div class="meta">
            <span>0.31.0</span><span>:8189</span><span>38,1 ГБ</span>
            <span class="tag">общие модели</span>
          </div>
          <div class="src">Последний запуск: только что</div>
        </div>
      </div>
      <div class="card">
        <div class="card-accent" style="--instance-accent:var(--accent-moss)"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip" style="--instance-accent:var(--accent-moss)">Э</span>
            <div class="card-name">Эксперименты</div>
            <span class="pill stopped"><i></i>Остановлен</span>
          </div>
          <!-- Описания нет, но строка есть: без неё версии
               в соседних карточках встали бы на разной высоте. -->
          <div class="card-desc"></div>
          <div class="meta">
            <span>0.29.4</span><span>:8190</span><span>12,8 ГБ</span>
          </div>
          <div class="src">Последний запуск: 3 дня назад</div>
        </div>
      </div>
      <div class="card gone">
        <div class="card-accent"></div>
        <div class="card-in">
          <div class="card-top">
            <span class="chip">В</span>
            <div class="card-name">Видео</div>
            <span class="pill gone"><i></i>Папка не найдена</span>
          </div>
          <div class="card-desc"></div>
          <div class="meta"><span>E:\comfy\video</span></div>
          <div class="src">Последний запуск: 12 марта</div>
        </div>
      </div>
    </div>
  </div>
</Window>
