<template>
  <v-row>
    <v-col cols="6" >
      <v-card class="pa-2">

      </v-card>
    </v-col>
    <v-col cols="6" >
      <v-card class="pa-2">

      </v-card>
    </v-col>
  </v-row>
</template>

<script setup>
import { ref } from 'vue'

const headers = ref([
  { title: 'Name', align: 'begin', key: 'title' },
  { title: 'Spielwert', key: 'attribute' },
  { title: 'Spielwert Wert', key: 'attribute' },
  { title: 'Steigerungen', key: 'attribute' },
  { title: 'Wert', key: 'attribute' }
])

</script>
