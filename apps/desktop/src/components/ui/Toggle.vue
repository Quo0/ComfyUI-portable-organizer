<script setup lang="ts">

//

//

import { computed } from 'vue';

const props = withDefaults(defineProps<{
  checked: boolean;
  mixed?: boolean;
  touched?: boolean;
  role?: 'switch' | 'checkbox';
  ariaLabel?: string;
  disabled?: boolean;
}>(), { mixed: false, touched: false, role: 'switch', ariaLabel: undefined, disabled: false });

defineEmits<{ click: [] }>();

const ariaChecked = computed<'true' | 'false' | 'mixed'>(() =>
  props.mixed ? 'mixed' : props.checked ? 'true' : 'false');
</script>

<template>
  <var class="Toggle">
    <button
      type="button"
      class="toggle"
      :class="{ off: !checked, mixed, 'is-init': touched }"
      :role="role"
      :aria-checked="ariaChecked"
      :aria-label="ariaLabel"
      :disabled="disabled"
      @click="$emit('click')"
    ></button>
  </var>
</template>

<style src="./Toggle.css" />
