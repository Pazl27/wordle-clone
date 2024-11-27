<template>
  <div class="home">
    <h1>Welcome to the Game</h1>
    <div class="buttons">
      <router-link to="/play">
        <button class="play-button">Play</button>
      </router-link>
      <router-link to="/highscores">
        <button class="highscores-button">High Scores</button>
      </router-link>
    </div>
  </div>
</template>

<script lang="ts" >
</script>

<style scoped>
.home {
  text-align: center;
  margin-top: 50px;
}

.buttons {
  margin-top: 20px;
}

button {
  padding: 10px 20px;
  margin: 10px;
  font-size: 16px;
  cursor: pointer;
}

.play-button {
  background-color: #4CAF50;
  color: white;
}

.highscores-button {
  background-color: #008CBA;
  color: white;
}
</style>

