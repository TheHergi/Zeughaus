import { defineStore } from 'pinia'

export const useCharacterStore = defineStore('Character', {
  state: () => ({
    name: 'Herbert'
  }),
  getters: {}
})
