<script setup lang="ts">
// Строка тумблера: сам тумблер и подпись рядом с ним, по левому краю
// (`align-items: flex-start` — подпись бывает в две-три строки, и по
// центру тумблер уезжал бы от её первой строки). Состав строки — забота
// вызывающего экрана (`Toggle` первым, за ним любой текст), сама
// `ToggleRow` — только раскладка, как `Group`.
</script>

<template>
  <var class="ToggleRow">
    <div class="toggle-row">
      <slot />
    </div>
  </var>
</template>

<style src="./ToggleRow.css" />
