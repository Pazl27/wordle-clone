<template>
  <div
    class="letter"
    :id="'letter-' + index"
    :class="{ empty: !letter, active: isActive }"
    tabindex="0"
  >
    {{ letter || ' ' }}
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

const props = defineProps({
  letter: {
    type: String,
    required: false,
    default: '',
  },
  isActive: {
    type: Boolean,
    required: true,
  },
  index: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits();

</script>

<style scoped>
.letter {
  width: 7rem;
  height: 7rem;
  background-color: lightgray;
  color: black;
  text-align: center;
  font-size: 4rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  outline: none;
}

</style>

