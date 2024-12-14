<template>
  <div class="guess">
    <Letter
      v-for="(letter, index) in letters"
      :key="index"
      :letter="letter"
      :isActive="isActive && !letter"
      :index="index"
      :rightPlace="rightPlace[index]"
      :rightLetter="rightLetter[index]"
    />
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import Letter from './Letter.vue';

const props = defineProps({
  letters: {
    type: Array,
    required: true,
  },
  isActive: {
    type: Boolean,
    required: true,
  },
    rightPlace: {
    type: Array,
    required: true,
  },
  rightLetter: {
    type: Array,
    required: true,
  },
});

const emit = defineEmits();

const emitLetter = (letter: string) => {
  emit('letterEntered', letter);
};

const onKeydown = (event: KeyboardEvent) => {
  if (props.isActive) {
    if (event.key.length === 1 && /[a-zA-Z]/.test(event.key)) {
      const currentGuess = props.letters.find((letter) => letter === '');

      if (currentGuess !== undefined) {
        const index = props.letters.indexOf(currentGuess);
        props.letters[index] = event.key.toUpperCase();
        emit('update:letters', [...props.letters]);
      }
    }

    if (event.key === 'Backspace' && props.letters.length > 0) {
      const currentGuess = props.letters.find((letter) => letter === '');
      const index = props.letters.indexOf(currentGuess) - 1;
      if (index >= 0) {
        props.letters[index] = '';
        emit('update:letters', [...props.letters]);
      } else {
        props.letters[props.letters.length - 1] = '';
        emit('update:letters', [...props.letters]);
      }
    }

    if (event.key === 'Enter') {
      emit('enterPressed');
    }
  }
};

window.addEventListener('keydown', onKeydown);
</script>

<style scoped>
.guess {
  display: flex;
  gap: 5px;
}
</style>

