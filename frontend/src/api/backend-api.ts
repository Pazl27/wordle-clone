import axios, { type AxiosResponse } from 'axios'

const axiosApi = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: { 'Content-Type': 'application/json' }
})

export default {
  startGame() {
    return axiosApi.post('/start')
  },
  guessWord(guess: any) {
    return axiosApi.post('/guess', guess)
  },
  getUsers() {
    return axiosApi.get('/users')
  }
}
