<template>
  <v-row dense>
    <v-col>

      <v-list rounded :disabled="!appStore.isCharacterAvailable">
        <v-list-item v-for="n in 5" :key="n" link :title="`List Item ${n}`"></v-list-item>

        <v-divider class="my-2"></v-divider>

        <v-list-item color="grey-lighten-4" link title="Refresh"></v-list-item>
      </v-list>

    </v-col>

  </v-row>
</template>

<script setup>
import { useAppStore } from '../stores/AppStore'

const appStore = useAppStore()

</script>
