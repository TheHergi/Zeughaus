import { defineStore } from 'pinia'

export const useCounterStore = defineStore('Character', {
  state: () => ({
    name: 'Herbert'
  }),
  getters: {}
})
