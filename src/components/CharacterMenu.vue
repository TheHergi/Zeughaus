<template>
  <v-row dense>
    <v-col>
      <v-row dense class="">
        <v-col cols="12">
          <v-btn block @click="store.makeCharacterAvailable(true)">
            <v-icon icon="mdi-plus-thick"></v-icon>
            <v-tooltip activator="parent" location="end">Neuer Charakter</v-tooltip>
          </v-btn>
        </v-col>
      </v-row>

      <v-row dense class="">

        <v-col cols="6">
          <v-btn block>
            <v-icon icon="mdi-folder-arrow-up"></v-icon>
            <v-tooltip activator="parent" location="end">Laden</v-tooltip>
          </v-btn>
        </v-col>

        <v-col cols="6">
          <v-btn block :disabled="!store.isCharacterAvailable">

            <v-icon icon="mdi-content-save"></v-icon>
            <v-tooltip activator="parent" location="end">Speichern</v-tooltip>
          </v-btn>
        </v-col>
      </v-row>
    </v-col>
  </v-row>
</template>

<script setup>
import { useAppStore } from '../stores/AppStore'

const store = useAppStore()

</script>

<style scoped>
.v-btn {
  text-transform: unset !important;
}
</style>
