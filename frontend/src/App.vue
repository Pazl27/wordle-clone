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

<script>
  import api from './api/backend-api';  

  export default {
    name: 'App',
    data() {
      return {
        users: [],       
        loading: false, 
        error: null    
      };
    },
    created() {
      this.fetchUsers();
    },
    methods: {
      async fetchUsers() {
        this.loading = true;
        this.error = null; 

        try {
          const response = await api.getUsers();  
          this.users = response.data;  
          console.log(response.data); 
        } catch (err) {
          this.error = 'Failed to fetch users: ' + err.message;  
        } finally {
          this.loading = false;  
        }
      }
    }
  };
</script>

<style scoped>
</style>

