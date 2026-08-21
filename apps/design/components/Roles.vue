<script setup lang="ts">
// Не через ThemePair: подпись под каждой ролью — конкретный hex,
// а не то, что доедет через var() сама CSS-каскадом. Обеим сторонам
// нужны РАЗНЫЕ строки текста одновременно, поэтому пара здесь своя,
// а не общий компонент, который просто дважды рендерит один слот.
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
