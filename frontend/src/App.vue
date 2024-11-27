<template>
  <div id="app">
    <h1>Users List</h1>

    <div v-if="loading">Loading...</div>

    <div v-if="error" style="color: red;">Error: {{ error }}</div>

    <div v-if="users.length > 0">
      <h2>Users</h2>
      <ul>
        <li v-for="user in users" :key="user.id">
          <strong>ID:</strong> {{ user.id }}<br>
          <strong>Word:</strong> {{ user.word }}<br>
          <strong>Attempts:</strong> {{ user.attempts }}<br>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
  import { defineComponent, ref } from 'vue';
  import api from './api/backend-api';

  interface User {
    id: number;
    word: string;
    attempts: number;
  }

  export default defineComponent({
    name: 'App',
    setup() {
      const users = ref<User[]>([]);
      const loading = ref<boolean>(false);
      const error = ref<string | null>(null);

      const fetchUsers = async () => {
        loading.value = true;
        error.value = null;

        try {
          const response = await api.getUsers();
          users.value = response.data;
          console.log(response.data);
        } catch (err) {
          error.value = 'Failed to fetch users: ' + (err instanceof Error ? err.message : 'Unknown error');
        } finally {
          loading.value = false;
        }
      };

      fetchUsers();

      return {
        users,
        loading,
        error
      };
    }
  });
</script>

<style scoped>
</style>

