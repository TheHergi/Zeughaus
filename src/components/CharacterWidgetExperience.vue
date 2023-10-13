<template>
  <v-card class="pa-2" title="Erfahrung">
    <v-text-field type="number"
                  v-model="charStore.attributes.experience.total"
                  @input="charStore.updateExperience"
                  :min=0
                  hide-details
                  label="Gesamt">
    </v-text-field>
    <v-text-field type="number"
                  v-model="charStore.attributes.experience.spent"
                  @input="charStore.updateExperience"
                  :min=0
                  hide-details
                  label="Ausgegeben">
    </v-text-field>
    <v-text-field type="number"
                  v-model="charStore.attributes.experience.current"
                  :min=0
                  hide-details
                  readonly
                  label="Aktuell">
    </v-text-field>
  </v-card>
</template>

<script setup>
import { useCharacterStore } from '../stores/CharacterStore'

const charStore = useCharacterStore()
</script>
