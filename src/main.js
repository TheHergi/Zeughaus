import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import { setupI18n } from './i18n'

import './css/styles.css'

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

import {
  VDataTable,
  VDataTableServer,
  VDataTableVirtual
} from 'vuetify/labs/VDataTable'

import '@mdi/font/css/materialdesignicons.css' // Ensure you are using css-loader
import { invoke } from '@tauri-apps/api/tauri'
import Skill, { useCharacterStore } from './stores/CharacterStore'

const vuetify = createVuetify({
  components: {
    ...components,
    VDataTable,
    VDataTableServer,
    VDataTableVirtual
  },
  directives,
  icons: {
    defaultSet: 'mdi'
  },
  theme: {
    defaultTheme: 'dark'
  },
  defaults: {
    global: {
      ripple: false
    },
    VCard: {
      variant: 'flat'
    },
    VBtn: {
      variant: 'outlined',
      style: 'text-transform: none;'
    },
    VTab: {
      density: 'compact'
    },
    VTextField: {
      variant: 'underlined'
    }
  }
})

const pinia = createPinia()

const i18n = setupI18n({
  legacy: false,
  locale: 'de'
})

const app = createApp(App)
app.use(router)
app.use(vuetify)
app.use(pinia)
app.use(i18n)
app.mount('#app')

async function initSkills () {
  const charStore = useCharacterStore()

  const base = await invoke('get_skills', { advanced: false })
  base.forEach((element) => {
    const s = new Skill()
    s.id = element.skill_id
    s.characteristic = element.attribute
    charStore.skills[element.skill_id] = s
  })

  const adv = await invoke('get_skills', { advanced: true })
  adv.forEach((element) => {
    const s = new Skill()
    s.characteristic = element.attribute
    s.id = element.skill_id
    charStore.skills[element.skill_id] = s
  })

  const specs = await invoke('get_all_skill_specs')
  specs.forEach((element) => {
    const s = new Skill()

    s.characteristic = charStore.skills[element.skill_id].characteristic
    s.id = element.skill_spec_id
    charStore.skillSpecs[element.skill_spec_id] = s
  })
}
initSkills()
