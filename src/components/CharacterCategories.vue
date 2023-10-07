<template>
  <v-row dense>

    <v-col>
      <!-- <v-tabs color="grey-darken-2" direction="vertical" :disabled="!appStore.isCharacterAvailable"> -->
      <v-spacer></v-spacer>
      <v-tabs color="grey-darken-2">
        <v-tab v-for="item in links" :key="item.path" :text="item.name" :to="item.path"></v-tab>
      </v-tabs>
      <v-spacer></v-spacer>
    </v-col>

  </v-row>
</template>

<script setup>

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
