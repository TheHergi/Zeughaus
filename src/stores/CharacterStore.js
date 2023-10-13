import { defineStore } from 'pinia'

class Characteristics {
  constructor (title) {
    this.title = title
    this.WS = 0
    this.BS = 0
    this.S = 0
    this.T = 0
    this.I = 0
    this.Ag = 0
    this.Dex = 0
    this.Int = 0
    this.WP = 0
    this.Fel = 0
  }
}

class CharacteristicsCollection {
  constructor () {
    this.init = new Characteristics('Anfangswert')
    this.advances = new Characteristics('Steigerungen')
    this.total = new Characteristics('Gesamt')
  }
}

export const useCharacterStore = defineStore('Character', {
  state: () => ({

    name: '',
    careers: [],
    characteristics: new CharacteristicsCollection()

  }),
  getters: {
  },
  actions: {
    updateCharacteristics (type) {
      this.characteristics.total[type] = parseInt(this.characteristics.init[type]) + parseInt(this.characteristics.advances[type])
    }
  }
})
