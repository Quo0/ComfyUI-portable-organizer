<script setup lang="ts">
// Цвет образца — переменной `--accent-<имя>`, не подставленным hex:
// сетка обязана меняться вместе с палитрой при правке токена. Подписанные
// hex и коэффициент контраста рядом — те самые числа, что tools/check-styles.mjs
// пересчитывает и сверяет отдельно; здесь они только показаны.
import { data } from '../theme-tokens.data.js';
</script>

<template>
  <var class="Swatches">
    <div class="swatches">
      <div class="sw" v-for="a in data.accents" :key="a.key">
        <div class="sw-chips">
          <span class="sw-chip t-light" style="background:var(--ground)">
            <i :style="{ background: `var(--accent-${a.key})` }" />
          </span>
          <span class="sw-chip t-dark" style="background:var(--ground)">
            <i :style="{ background: `var(--accent-${a.key})` }" />
          </span>
        </div>
        <div class="sw-name">{{ a.label }}</div>
        <div class="sw-meta"><code>{{ a.light }}</code> · {{ a.lightRatio }}:1</div>
        <div class="sw-meta"><code>{{ a.dark }}</code> · {{ a.darkRatio }}:1</div>
      </div>
    </div>
  </var>
</template>
