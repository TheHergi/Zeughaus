<template>
  <v-row>
    <v-col>
      <CharacterWidgetGeneral/>
    </v-col>
  </v-row>
  <v-row>
    <v-col>
      <CharacterWidgetCareer></CharacterWidgetCareer>
    </v-col>
  </v-row>
</template>

<script setup>
import CharacterWidgetGeneral from '../components/CharacterWidgetGeneral.vue'
import CharacterWidgetCareer from '../components/CharacterWidgetCareer.vue'

</script>
