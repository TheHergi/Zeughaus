<template>
  <v-container>
    <v-row >

      <v-col cols="2">
        <v-card class="pa-2">
          <CharacterMenu></CharacterMenu>
        </v-card>
        <v-card class="pa-2 my-2">

          <v-list
            :lines="false"
            density="compact"
            nav
          >
            <div v-for="item in links" :key="item.path">
              <div v-if="item.children.length == 0">
                <v-list-item :title="item.name" :to="item.path"></v-list-item>
              </div>
              <div v-else>
                <v-list-group :title="item.name">
                  <template v-slot:activator="{ props }">
                    <v-list-item
                      v-bind="props"
                      :title="item.name"
                    ></v-list-item>
                  </template>
                  <div v-for="child in item.children" :key="child.path">
                    <v-list-item
                      v-bind="props"
                      :title="child.name"
                      :to="child.path"
                    ></v-list-item>
                  </div>
                </v-list-group>
              </div>
            </div>

          </v-list>

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
import { ref } from 'vue'

const props = ref({})
const links = [
  {
    name: 'Allgemein',
    path: '/character/general',
    icon: '',
    children: []
  },
  {
    name: 'Spielwerte',
    path: '/character/attributes',
    icon: '',
    children: [
    ]
  },
  {
    name: 'Fähigkeiten',
    path: '/character/skills',
    icon: '',
    children: [{
      name: 'Grundfähigkeiten',
      path: '/character/skills/basic',
      icon: ''
    },
    {
      name: 'Ausbaufähigkeiten',
      path: '/character/skills/advanced',
      icon: ''
    }]
  },
  {
    name: 'Talente',
    path: '/character/talents',
    icon: '',
    children: []
  },
  {
    name: 'Zauber',
    path: '/character/spells',
    icon: '',
    children: []
  },
  {
    name: 'Gebete',
    path: '/character/prayer',
    icon: '',
    children: []
  },
  {
    name: 'Ausrüstung',
    path: '/character/trappings',
    icon: '',
    children: []
  }
]
</script>

<style scoped></style>
