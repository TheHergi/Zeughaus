import { defineStore } from 'pinia'

class Characteristics {
  constructor (title) {
    this.title = title
    this.WS = 0
    this.BS = 0
    this.S = 0
    this.T = 0
    this.I = 0
    this.Ag = 23
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
    attributes: {
      characteristics: new CharacteristicsCollection(),
      fate: {
        fate: 0,
        fortune: 0

      },
      resilience: {
        resilience: 0,
        resolve: 0,
        motivation: ''
      },
      movement: {
        movement: 0,
        walk: 0,
        run: 0
      },
      experience: {
        current: 0,
        spent: 0,
        total: 0
      }
    },
    skills: {},
    skillSpecs: {}
  }),
  getters: {
  },
  actions: {
    updateCharacteristics (type) {
      this.attributes.characteristics.total[type] = parseInt(this.attributes.characteristics.init[type]) + parseInt(this.attributes.characteristics.advances[type])
    },
    updateMovement () {
      this.attributes.movement.walk = parseInt(this.attributes.movement.movement) * 2
      this.attributes.movement.run = parseInt(this.attributes.movement.movement) * 4
    },
    updateExperience () {
      this.attributes.experience.current = parseInt(this.attributes.experience.total) - parseInt(this.attributes.experience.spent)
    },
    createSkillsIfNotExists (skills) {

    }
  }
})
