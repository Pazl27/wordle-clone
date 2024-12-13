<template>
  <div class="home">
    <h1>Woordle</h1>

    <div class="buttons">
      <button class="button play" @click="goToPlay">Play</button>
      <button class="button highscores" @click="goToHighscores">Highscores</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const goToPlay = () => {
  router.push('/play');
};

const goToHighscores = () => {
  router.push('/highscores');
};
</script>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
}

h1 {
  font-size: 3rem;
  margin-bottom: 2rem;
}

.buttons {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.button {
  padding: 1rem 2rem;
  font-size: 1.5rem;
  color: white;
  border: none;
  border-radius: 5px;
  width: 100%;
  cursor: pointer;
}

.play {
  background-color: #4CAF50;
}

.highscores {
  background-color: #2196F3;
}
</style>
