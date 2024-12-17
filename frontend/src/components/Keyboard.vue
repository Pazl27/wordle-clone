<template>
  <div class="keyboard">
    <div class="row" v-for="(row, index) in keyboardLayout" :key="index">
      <button
        v-for="(key, index) in row"
        :key="index"
        class="key"
        :id="key"
        @click="keyClicked(key)"
      >
        {{ key }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';

const keyboardLayout: string[][] = [
  ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
  ['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
  ['Z', 'X', 'C', 'V', 'B', 'N', 'M']
];

const activeKey = ref<string | null>(null);

const keyClicked = (key: string) => {
  console.log(key);
  simulateHapticFeedback(key);
};

const simulateHapticFeedback = (key: string) => {
  const button = document.getElementById(key);
  if (button) {
    button.classList.add('pressed');
    setTimeout(() => {
      button.classList.remove('pressed');
    }, 200);
  }
};

const onKeydown = (event: KeyboardEvent) => {
  if (keyboardLayout.flat().includes(event.key.toUpperCase())) {
    simulateHapticFeedback(event.key.toUpperCase());
  }
}

window.addEventListener('keydown', onKeydown);
</script>

<style scoped>
.keyboard {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 20px;
}

.row {
  display: flex;
  justify-content: center;
  margin-bottom: 10px;
}

.key {
  background-color: #f0f0f0;
  border: 1px solid #ccc;
  border-radius: 5px;
  padding: 15px;
  margin: 5px;
  font-size: 18px;
  cursor: pointer;
  width: 50px;
  text-align: center;
  transition: all 0.1s ease-in-out;
}

.key.pressed {
  background-color: #ddd;
  transform: scale(0.95);
}

.key:active {
  background-color: #ddd;
}
</style>

