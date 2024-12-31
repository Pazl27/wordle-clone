<template>
  <div class="user-list">
    <div v-for="(user, index) in aroundUser" :key="user.id" class="user-card">
      <ScoreEntry
        :position="getPosition(user.id)"
        :user="user"
        :highlight="user.id === props.user.id"
        :small="true"
      ></ScoreEntry>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps, onMounted } from 'vue';
import api from '../../api/backend-api';
import ScoreEntry from './ScoreEntrySmall.vue';

const props = defineProps({
  user: { type: Object, required: true },
});

const allUsers = ref<any[]>([]);
const aroundUser = ref<any[]>([]);

const getUsers = async () => {
  try {
    const response = await api.getUsers();
    allUsers.value = response.data;
    return response.data;
  } catch (error) {
    console.error(error);
  }
};

const filterUsers = (users: any[]) => {
  const index = users.findIndex(user => user.id === props.user.id);
  if (index === -1) {
    return users.slice(-4);
  }

  const start = Math.max(0, index - 2);
  const end = Math.min(users.length, index + 3);

  return users.slice(start, end);
};

const getPosition = (userId: number) => {
  const userIndex = allUsers.value.findIndex(user => user.id === userId);
  if (userIndex === -1) return -1;
  return userIndex + 1;
};

onMounted(async () => {
  const users = await getUsers();
  aroundUser.value = filterUsers(users);
});
</script>

<style scoped>
</style>

