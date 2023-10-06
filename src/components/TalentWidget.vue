<template>
  <v-card class="pa-2" v-show="talentID >= 0">

    <template v-slot:title>
      {{ talent.title }}
    </template>

    <v-card-text>
      <div v-if="talent.tests">
        <p class="font-weight-bold">Tests: </p>
        <p class="text--primary"> {{ talent.tests }} </p>
        <br>
      </div>

      <div v-if="talent.maximum">
        <p class="font-weight-bold">Maximum: </p>
        <p class="text--primary"> {{ talent.maximum }} </p>
        <br>
      </div>

      <p class="font-weight-bold">Beschreibung: </p>
      <p class="text--primary text-justify">
        {{ talent.talent_description }}
      </p>

    </v-card-text>
    <v-card-actions>

      <v-btn prepend-icon="mdi-plus-thick" variant="text" outlined>
        Hinzufügen
      </v-btn>

      <v-spacer></v-spacer>
      <v-btn prepend-icon="mdi-pencil" variant="text">
        Editieren
      </v-btn>

      <v-btn prepend-icon="mdi-trash-can-outline" variant="text">
        Löschen
      </v-btn>

    </v-card-actions>
  </v-card>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri'
import { ref, watch } from 'vue'

const props = defineProps(['talentID'])
const talent = ref({})

watch(() => props.talentID, (newID, oldID) => {
  loadItem()
})

const loadItem = async () => {
  talent.value = await invoke('get_talent', { talentId: props.talentID })
}

</script>
