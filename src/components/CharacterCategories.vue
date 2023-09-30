<template>
  <v-row dense>
    <v-col>
      <!-- <v-tabs color="grey-darken-2" direction="vertical" :disabled="!appStore.isCharacterAvailable"> -->
      <v-tabs color="grey-darken-2" direction="vertical">
        <v-tab v-for="item in links" :key="item.path" :text="item.name" :to="item.path"></v-tab>
      </v-tabs>
    </v-col>

  </v-row>
</template>

<script setup>
// import { useAppStore } from '../stores/AppStore'
// const appStore = useAppStore()

const links = [
  {
    name: 'Allgemein',
    path: '/character/',
    icon: ''
  },
  {
    name: 'Spielwerte',
    path: '/character/attributes',
    icon: ''
  },
  {
    name: 'Fähigkeiten',
    path: '/character/skills',
    icon: ''
  },
  {
    name: 'Talente',
    path: '/character/talents',
    icon: ''
  },
  {
    name: 'Zauber',
    path: '/character/spells',
    icon: ''
  },
  {
    name: 'Gebete',
    path: '/character/prayer',
    icon: ''
  },
  {
    name: 'Ausrüstung',
    path: '/character/trappings',
    icon: ''
  }
]

</script>
