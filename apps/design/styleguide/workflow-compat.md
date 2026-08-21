# Совместимость воркфлоу

<!-- US-WF-05 -->

То, ради чего перенос воркфлоу живёт в приложении, а не в проводнике.

Перед добавлением видно, все ли нужные ноды есть в целевой сборке.
Состояний три, а не два: у остановленного инстанса точного ответа нет,
и честнее пометить его **неизвестным**, чем показать
зелёную галочку `US-WF-05/AC-6`. Нехватка нод предупреждает,
но не запрещает — доустановить их можно потом `AC-4`.

<ThemePair>
  <div class="compat">
    <div class="compat-row ok">
      <span class="chip" style="--instance-accent:var(--accent-teal)"></span>
      <span>SDXL стабильная</span>
      <span class="compat-note">все ноды на месте</span>
    </div>
    <div class="compat-row warn">
      <span class="chip" style="--instance-accent:var(--accent-indigo)"></span>
      <span>Flux тест</span>
      <span class="compat-note">нет 2 нод</span>
    </div>
    <div class="missing">IPAdapterUnifiedLoader · ReActorFaceSwap</div>
    <div class="compat-row">
      <span class="chip" style="--instance-accent:var(--accent-moss)"></span>
      <span>Эксперименты</span>
      <span class="compat-note">по данным последнего запуска</span>
    </div>
  </div>
</ThemePair>
