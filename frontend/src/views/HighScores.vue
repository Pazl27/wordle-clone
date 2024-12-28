<template>
  <div class="highscore-container">
    <h1>Highscore List</h1>
    <div class="header-row">
      <div class="header">Position</div>
      <div class="header">Name</div>
      <div class="header">Score</div>
      <div class="header">Word</div>
      <div class="header">Attempts</div>
    </div>

    <div class="user-list">
      <div v-for="(user, index) in users" :key="user.id" class="user-card">
        <ScoreEntry
          :position="index + 1"
          :user="user"
        ></ScoreEntry>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import ScoreEntry from '../components/ScoreEntry.vue'
import api from '../api/backend-api'
import { ref, onMounted } from 'vue'

const users = ref([])

const getUsers = async () => {
  try {
    const response = await api.getUsers()
    users.value = response.data
    console.log(users.value)
  } catch (error) {
    console.error(error)
  }
}

onMounted(() => {
  getUsers()
})
</script>

<style scoped>
.highscore-container {
  padding: 20px;
}

h1 {
  text-align: center;
  margin-bottom: 20px;
}

.header-row {
  display: grid;
  grid-template-columns: 50px 1fr 1fr 1fr 1fr;
  gap: 15px;
  font-weight: bold;
  padding: 10px 15px;
  color: white;
  border-radius: 10px;
}

.header {
  text-align: center;
  font-size: 1.2rem;
}

.user-card {
  margin-bottom: 10px;
}
.user-list {
  max-height: 500px;
  overflow-y: scroll;
}
.user-list::-webkit-scrollbar {
  width: 8px;
}

.user-list::-webkit-scrollbar-track {
  background-color: #f1f1f1;
  border-radius: 10px;
}

.user-list::-webkit-scrollbar-thumb {
  background-color: #888;
  border-radius: 10px;
  border: 2px solid #f1f1f1;
}

.user-list::-webkit-scrollbar-thumb:hover {
  background-color: #555;
}
</style>
