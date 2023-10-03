<template>
  <v-card title="Karriere" class="pa-2">
    <v-row dense>
      <v-col>
        <v-select label="Klasse" density="compact"></v-select>
      </v-col>
      <v-col>
        <v-select label="Karriere" density="compact"></v-select>
      </v-col>
      <v-col>
        <v-select label="Karrierestufe" density="compact"></v-select>
      </v-col>
      <v-col cols="1">
        <v-btn block @click="greet">
          <v-icon icon="mdi-plus-thick"></v-icon>
          <v-tooltip activator="parent" location="end">Hinzufügen</v-tooltip>
        </v-btn>
      </v-col>
    </v-row>
    <v-row dense>
      <v-col></v-col>

    </v-row>
  </v-card>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri'

async function greet () {
  console.log('invoke')
  await invoke('create_talent', { title: 'sersyyy' })

  const x = await invoke('get_talent', { id: 123 })
  console.log(x)

  const xs = await invoke('get_talents')
  console.log(xs)
}
</script>
