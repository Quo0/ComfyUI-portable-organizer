<script setup lang="ts">
// A swatch takes its colour from `--accent-<name>`, not from a substituted
// hex: the grid has to move with the palette when a token is edited. The hex
// and the contrast ratio printed beside it are the same numbers that
// tools/check-styles.mjs recomputes and checks; here they are only shown.
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
