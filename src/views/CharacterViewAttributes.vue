<template>
  <v-row>
    <v-col>
      <CharacterWidgetCharacteristics></CharacterWidgetCharacteristics>
    </v-col>
  </v-row>
  <v-row>
    <v-col cols="3">
      <CharacterWidgetFate></CharacterWidgetFate>
    </v-col>
    <v-col cols="3">
      <CharacterWidgetResilience></CharacterWidgetResilience>
    </v-col>
    <v-col cols="3">
      <CharacterWidgetMovement></CharacterWidgetMovement>
    </v-col>
    <v-col cols="3">
      <CharacterWidgetExperience></CharacterWidgetExperience>
    </v-col>
  </v-row>

</template>

<script setup>

import CharacterWidgetCharacteristics from '../components/CharacterWidgetCharacteristics.vue'
import CharacterWidgetExperience from '../components/CharacterWidgetExperience.vue'
import CharacterWidgetFate from '../components/CharacterWidgetFate.vue'
import CharacterWidgetResilience from '../components/CharacterWidgetResilience.vue'
import CharacterWidgetMovement from '../components/CharacterWidgetMovement.vue'

</script>
