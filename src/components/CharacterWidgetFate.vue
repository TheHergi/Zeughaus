<template>
  <v-card class="pa-2" title="Schicksal">
    <v-text-field type="number"
                  v-model="charStore.attributes.fate.fate"
                  :min=0
                  :max=99
                  hide-details
                  label="Schicksal">
    </v-text-field>
    <v-text-field type="number"
                  v-model="charStore.attributes.fate.fortune"
                  :min=0
                  :max=99
                  hide-details
                  label="Glück">
    </v-text-field>
  </v-card>
</template>

<script setup>
import { useCharacterStore } from '../stores/CharacterStore'

const charStore = useCharacterStore()
</script>
