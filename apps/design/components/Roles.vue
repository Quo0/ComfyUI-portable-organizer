<script setup lang="ts">
// Not through ThemePair: the caption under each role is a concrete hex, not
// whatever `var()` resolves to in the cascade. Both sides need different text
// at the same time, so the pair here is its own rather than a component that
// renders one slot twice.
import { data } from '../theme-tokens.data.js';

const rows = (theme: 'light' | 'dark') =>
  data.roles.map((r) => ({ ...r, hex: theme === 'light' ? r.light : r.dark }));
</script>

<template>
  <var class="Roles">
    <div class="pair">
      <figure class="panel t-light">
        <figcaption>Светлая</figcaption>
        <div class="app">
          <div class="roles">
            <div class="role" v-for="r in rows('light')" :key="r.name">
              <i :style="{ background: `var(--${r.name})` }" />
              <div><b>{{ r.name }}</b><span>{{ r.what }}</span></div>
              <code>{{ r.hex }}</code>
            </div>
          </div>
        </div>
      </figure>
      <figure class="panel t-dark">
        <figcaption>Тёмная</figcaption>
        <div class="app">
          <div class="roles">
            <div class="role" v-for="r in rows('dark')" :key="r.name">
              <i :style="{ background: `var(--${r.name})` }" />
              <div><b>{{ r.name }}</b><span>{{ r.what }}</span></div>
              <code>{{ r.hex }}</code>
            </div>
          </div>
        </div>
      </figure>
    </div>
  </var>
</template>
