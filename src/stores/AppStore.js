import { defineStore } from 'pinia'

export const useAppStore = defineStore('App', {
  state: () => ({
    characterAvailable: false
  }),
  getters: {
    isCharacterAvailable: (state) => state.characterAvailable
  },
  actions: {
    makeCharacterAvailable (en) {
      this.characterAvailable = en
    }
  }
})
