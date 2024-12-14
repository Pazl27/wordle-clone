<template>
  <div>
    <div class="gameBoard">
      <Guess
        v-for="(guess, index) in guesses"
        :key="index"
        :letters="guess"
        :isActive="index === activeGuessIndex"
        :rightPlace="guessesRightPlace[index]"
        :rightLetter="guessesRightLetter[index]"
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
import api from '../api/backend-api';

const guesses = ref([
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
]);

const guessesRightPlace = ref([
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
]);

const guessesRightLetter = ref([
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
  ['', '', '', '', ''],
]);

let user = ref({
  id: '',
  word: '',
});

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
    const isValid = await makeGuess(guess);

    if (isValid && activeGuessIndex.value < guesses.value.length - 1) {
      activeGuessIndex.value += 1;
    } else if (!isValid) {
      guesses.value[activeGuessIndex.value] = ['', '', '', '', ''];
    }
  }
};

const makeGuess = async (guess: string) => {
  const data = {
    guess: guess,
    id: user.id,
  };
  try {
    const response = await api.guessWord(data);
    const { valid_word, correct_word, in_word, right_place, attempts } = response.data;

    if (valid_word == "Fail") {
      return false;
    }
    if (attempts >= 5) {
      //TODO: Pop up modal
      alert("Game Over!");
    }
    if (correct_word == "Pass") {
      //TODO: Pop up modal
      alert("You win!");
    }

    for (const [index, char] of Object.entries(right_place)) {
      guessesRightPlace.value[activeGuessIndex.value][index] = char;
    }

    for (const [index, char] of Object.entries(in_word)) {
      guessesRightLetter.value[activeGuessIndex.value][index] = char;
    }

  } catch (error) {
    console.error(error);
    return false;
  }

  return true;
};

const startGame = async () => {
  try {
    const response = await api.startGame();
    user = response.data;
    console.log("User: ", user);
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

