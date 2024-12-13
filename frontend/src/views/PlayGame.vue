<template>
  <div>
    <div class="gameBoard">
      <Guess
        v-for="(guess, index) in guesses"
        :key="index"
        :letters="guess"
        :isActive="index === activeGuessIndex"
        @letterEntered="onLetterEntered(index, $event)"
        @enterPressed="onEnterPressed"
      />
    </div>

    <div class="keyboard">
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Guess from '../components/play/Guess.vue';
import api from '../api/backend-api'

const guesses = ref([
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', '']
]);

let user = ref({
  id: '',
  word: '',
})

const activeGuessIndex = ref(0);

const onLetterEntered = (index: number, letter: string) => {
  if (index < guesses.value.length) {
    const guess = guesses.value[index];
    const firstEmptySlot = guess.indexOf('');

    if (firstEmptySlot !== -1) {
      guess[firstEmptySlot] = letter;
    }

    if (!guess.includes('') && index < guesses.value.length - 1) {
      activeGuessIndex.value = index + 1;
    }
  }
};

const onEnterPressed = async () => {
  const currentGuess = guesses.value[activeGuessIndex.value];

  if (!currentGuess.includes('')) {
    const guess = currentGuess.join('').toLowerCase();
    await makeGuess(guess);

    if (activeGuessIndex.value < guesses.value.length - 1) {
      activeGuessIndex.value += 1;
    }
  }
};

const makeGuess = async (guess: string) => {
  const data = {
    guess: guess,
    id: user.id
  }
  try {
    const response = await api.guessWord(data);
    console.log(response.data);
  } catch (error) {
    console.error(error);
  }
};

const startGame = async () => {
  try {
    const response = await api.startGame();
    user = response.data;
    console.log(user);
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  startGame();
});
</script>

<style scoped>
.gameBoard {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.keyboard {
  margin-top: 20px;
}
</style>

