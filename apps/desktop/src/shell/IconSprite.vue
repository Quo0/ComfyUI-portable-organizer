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
