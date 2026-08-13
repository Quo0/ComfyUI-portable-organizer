<script setup lang="ts">
// Спрайт иконок рейла. Отрисовывается один раз в оболочке, дальше на него
// ссылаются через <use href="#i-...">. Иконки взяты из design/screens.src.html:
// они часть дизайна и меняются вместе с ним.
</script>

<template>
  <svg aria-hidden="true" focusable="false" style="display: none">
    <symbol
      id="i-inst"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <rect x="2" y="2.5" width="12" height="4" rx="1" />
      <rect x="2" y="9.5" width="12" height="4" rx="1" />
    </symbol>
    <symbol
      id="i-install"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M8 2v7" />
      <path d="M5 6.5 8 9.5l3-3" />
      <path d="M2.5 11.5v1a1.5 1.5 0 0 0 1.5 1.5h8a1.5 1.5 0 0 0 1.5-1.5v-1" />
    </symbol>
    <symbol
      id="i-wf"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <circle cx="3.5" cy="4" r="1.6" />
      <circle cx="12.5" cy="4" r="1.6" />
      <circle cx="8" cy="12" r="1.6" />
      <path d="M3.5 5.6v2a1.5 1.5 0 0 0 1.5 1.5h1.5" />
      <path d="M12.5 5.6v2a1.5 1.5 0 0 1-1.5 1.5H9.5" />
    </symbol>
    <symbol
      id="i-set"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M2.5 4.5h11" />
      <path d="M2.5 11.5h11" />
      <circle cx="6" cy="4.5" r="1.6" />
      <circle cx="10.5" cy="11.5" r="1.6" />
    </symbol>
    <symbol
      id="i-about"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <circle cx="8" cy="8" r="6" />
      <path d="M8 7.2v3.6" />
      <path d="M8 5.2v.1" />
    </symbol>
    <!-- Значки действия в строке совместимости: положить сюда / уже здесь. -->
    <symbol
      id="i-plus"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M8 3.5v9" />
      <path d="M3.5 8h9" />
    </symbol>
    <symbol
      id="i-check"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M3.5 8.5 6.5 11.5 12.5 4.5" />
    </symbol>
    <!-- Возврат. Стрелка с чертой, а не голая шпилька: голую путают
         со свёрткой рейла, у которой ровно такая же форма. -->
    <symbol
      id="i-back"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M12.5 8h-9" />
      <path d="M7 3.5 2.5 8 7 12.5" />
    </symbol>
    <!-- Тулбар встроенной вкладки: лог, папка результатов, браузер,
         перезагрузка страницы. Подписи там не помещаются, поэтому смысл
         несут значки, а слова уходят в подсказку. -->
    <symbol
      id="i-log"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M3 4.5h10" />
      <path d="M3 8h7" />
      <path d="M3 11.5h9" />
    </symbol>
    <symbol
      id="i-folder"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M2 12V4.5a1 1 0 0 1 1-1h3l1.5 2H13a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1z" />
    </symbol>
    <symbol
      id="i-external"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M12.5 9v3a1.5 1.5 0 0 1-1.5 1.5H4A1.5 1.5 0 0 1 2.5 12V5A1.5 1.5 0 0 1 4 3.5h3" />
      <path d="M9.5 2.5h4v4" />
      <path d="M13.5 2.5 7.5 8.5" />
    </symbol>
    <symbol
      id="i-reload"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M13 8a5 5 0 1 1-1.6-3.7" />
      <path d="M13.2 2.5v3h-3" />
    </symbol>
    <symbol
      id="i-collapse"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M9.5 4.5 6 8l3.5 3.5" />
    </symbol>
    <symbol
      id="i-expand"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M6.5 4.5 10 8l-3.5 3.5" />
    </symbol>
  </svg>
</template>
