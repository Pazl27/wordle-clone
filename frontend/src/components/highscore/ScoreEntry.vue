<template>
  <div class="container">
    <div class="position">#{{ props.position }}</div>
    <div class="user-name">{{ props.user.name }}</div>
    <div class="user-score">{{ props.user.score }}</div>
    <div class="user-word">{{ props.user.word }}</div>
    <div class="user-attempts">{{ props.user.attempts }}</div>
  </div>
</template>

<script lang="ts" setup>
import { defineProps } from 'vue'

const props = defineProps({
  position: { type: Number, required: true },
  user: { type: Object, required: true }
})
</script>

<style scoped>
.container {
  display: grid;
  grid-template-columns: 50px 1fr 1fr 1fr 1fr;
  gap: 15px;
  margin: 0.5rem;
  padding: 15px;
  border-radius: 10px;
  border: 2px solid white;
  color: white;
}

.position {
  text-align: center;
  font-size: 1.2rem;
}

.user-name,
.user-score,
.user-word,
.user-attempts {
  font-size: 1.2rem;
  text-align: center;
}
</style>

