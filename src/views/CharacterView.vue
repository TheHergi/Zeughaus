<template>
  <v-container>
    <v-row >

      <v-col cols="2">
        <v-card class="pa-2">
          <CharacterMenu></CharacterMenu>
        </v-card>
        <v-card class="pa-2 my-2">
          <v-tabs  direction="vertical">
            <v-tab v-for="item in links" :key="item.path" :text="item.name" :value="item.name" :to="item.path"></v-tab>
          </v-tabs>
        </v-card>
      </v-col>

      <v-col>
        <router-view></router-view>
      </v-col>

    </v-row>
  </v-container>
</template>

<script setup>
import CharacterMenu from '../components/CharacterMenu.vue'

const links = [
  {
    name: 'Allgemein',
    path: '/character/general',
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

<style scoped></style>
