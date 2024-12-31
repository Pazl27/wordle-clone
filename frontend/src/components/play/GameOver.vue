<template>
  <div v-if="showModal" class="modal-overlay">
    <div class="modal-content">
      <h1 class="game-over-title">Game Over!</h1>
      <p class="score-text">Better luck next time!</p>
      <div class="word-display">
        <span class="word-label">Your word: </span>
        <span class="word">{{ word }}</span>
      </div>
      <div class="score-display">
        <span class="score-label">Your Score: </span>
        <span class="score">{{ score }}</span>
      </div>

      <div class="input-container">
        <label for="playerName" class="input-label">Enter your Name:</label>
        <input
          v-model="playerName"
          type="text"
          id="playerName"
          class="name-input"
          placeholder="Your name here"
          @keydown.enter="setName"
          @blur="setName"
        />
      </div>

      <div class="playerList">
        <ShortList
          :user="props.user"
          :key="reload"
        ></ShortList>
      </div>

      <div class="button-container">
        <button @click="goToHomePage" class="home-btn">Go to Homepage</button>
        <button @click="goToHighScores" class="highscores-btn">View Highscores</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, defineProps } from 'vue';
import api from '../../api/backend-api';
import { useRouter } from 'vue-router';
import ShortList from '../highscore/ShortList.vue';

const props = defineProps({
  user: { type: Object, required: true },
});

const showModal = ref(true);
const score = ref(0);
const word = ref('');
const playerName = ref('');
const reload = ref(0);

const router = useRouter();

function closeModal() {
  showModal.value = false;
}

const goToHomePage = () => {
  closeModal();
  router.push('/');
}

const goToHighScores = () => {
  closeModal();
  router.push('/highscores');
}

const getScore = async () => {
  try {
    const response = await api.getScore(props.user.id);
    score.value = response.data;
  } catch (error) {
    console.error(error);
  }
}

const getWord = async () => {
  try {
    const response = await api.getWord(props.user.id);
    word.value = response.data;
  } catch (error) {
    console.error(error);
  }
}

const setName = async () => {
  if (!playerName.value) {
    return
  }

  try {
    await api.setName(props.user.id, playerName.value);

    reload.value += 1;

    const inputField = document.getElementById('playerName') as HTMLInputElement;
    inputField.blur();
  } catch (error) {
    console.error(error);
  }
}

onMounted(() => {
  getScore();
  getWord();
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 999;
}

.modal-content {
  background: #2e2e2e;
  color: #fff;
  padding: 30px 40px;
  border-radius: 10px;
  text-align: center;
  box-shadow: 0px 10px 15px rgba(0, 0, 0, 0.2);
  width: 350px;
  max-width: 90%;
}

.game-over-title {
  font-size: 28px;
  font-weight: 700;
  margin-bottom: 10px;
}

.score-text {
  font-size: 20px;
  margin-bottom: 20px;
  font-weight: 600;
}

.score-display, .word-display {
  font-size: 18px;
  margin-bottom: 15px;
}

.score-label, .word-label {
  font-weight: bold;
  color: #ccc;
}

.score {
  color: #fff;
  font-size: 24px;
  font-weight: 700;
}

.word {
  color: #ffcc00;
  font-size: 24px;
  font-weight: 700;
}

.input-container {
  margin-bottom: 20px;
}

.input-label {
  font-size: 16px;
  color: #ccc;
  margin-bottom: 8px;
  display: block;
}

.name-input {
  width: 100%;
  padding: 10px;
  border-radius: 5px;
  font-size: 16px;
  border: 1px solid #ccc;
  background-color: #3b3b3b;
  color: #fff;
  margin-top: 5px;
}

.name-input::placeholder {
  color: #bbb;
}

.button-container {
  display: flex;
  justify-content: space-around;
  gap: 2rem;
}

button {
  padding: 12px 20px;
  border: none;
  border-radius: 5px;
  font-size: 16px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.home-btn {
  background-color: #4CAF50;
  color: white;
}

.highscores-btn {
  background-color: #007BFF;
  color: white;
}

button:hover {
  opacity: 0.8;
}

button:focus {
  outline: none;
}

.playerList {
  border-radius: 5px;
  border: 2px solid white;
  padding: 1rem;
  margin-bottom: 10px;
}
</style>
