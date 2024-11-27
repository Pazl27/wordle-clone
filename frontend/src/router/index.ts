import { createRouter, createWebHistory } from 'vue-router'
import HomePage from '../views/HomePage.vue'
import PlayGame from '../views/PlayGame.vue'
import HighScores from '../views/HighScores.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomePage,
    },
    {
      path: '/play',
      name: 'play',
      component: PlayGame,
    },
    {
      path: '/highscores',
      name: 'highscores',
      component: HighScores,
    },
  ],
})

export default router
