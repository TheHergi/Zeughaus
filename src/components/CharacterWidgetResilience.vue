<template>
  <v-card class="pa-2" title="Zähigkeit">
    <v-text-field type="number"
                  v-model.number="charStore.attributes.resilience.resilience"
                  :min=0
                  :max=99
                  hide-details
                  label="Zähigkeit">
    </v-text-field>
    <v-text-field type="number"
                  v-model.number="charStore.attributes.resilience.resolve"
                  :min=0
                  :max=99
                  hide-details
                  label="Mut">
    </v-text-field>
    <v-text-field v-model="charStore.attributes.resilience.motivation"
                  hide-details
                  label="Motivation">
    </v-text-field>
  </v-card>
</template>

<script setup>
import { useCharacterStore } from '../stores/CharacterStore'

const charStore = useCharacterStore()
</script>
