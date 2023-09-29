<template>
  <v-container>
    <v-row>

      <v-col cols="12" sm="4" md="3">

        <v-sheet rounded class="pa-2">
          <CharacterMenu></CharacterMenu>
        </v-sheet>

        <v-sheet rounded class="pa-2 my-2">
          <CharacterCategories></CharacterCategories>
        </v-sheet>

      </v-col>

      <v-col cols="12" sm="8" md="9">

        <v-sheet rounded class="pa-2">
          test
        </v-sheet>

      </v-col>
    </v-row>
  </v-container>
</template>

<script setup>
import CharacterMenu from '../components/CharacterMenu.vue'
import CharacterCategories from '../components/CharacterCategories.vue'
</script>

<style scoped></style>
