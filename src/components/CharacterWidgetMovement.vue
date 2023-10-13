<template>
  <v-card class="pa-4" title="Bewegung">
    <v-text-field type="number"
                  v-model="charStore.attributes.movement.movement"
                  @input="charStore.updateMovement"
                  :min=0
                  :max=99
                  hide-details
                  label="Bewegung">
    </v-text-field>
    <v-text-field type="number"
                  v-model="charStore.attributes.movement.walk"
                  hide-details
                  readonly
                  label="Gehen">
    </v-text-field>
    <v-text-field type="number"
                  v-model="charStore.attributes.movement.run"
                  hide-details
                  readonly
                  label="Rennen">
    </v-text-field>
  </v-card>
</template>

<script setup>
import { useCharacterStore } from '../stores/CharacterStore'

const charStore = useCharacterStore()
</script>
