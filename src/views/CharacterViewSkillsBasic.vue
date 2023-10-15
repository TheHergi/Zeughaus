<template>
  <v-row>
    <v-col cols="6">
      <CharacterWidgetSkills :skills-data="table0Values"></CharacterWidgetSkills>
    </v-col>
    <v-col cols="6">
      <CharacterWidgetSkills :skills-data="table1Values"></CharacterWidgetSkills>
    </v-col>
  </v-row>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import CharacterWidgetSkills from '../components/CharacterWidgetSkills.vue'

const table1Values = ref([])
const table0Values = ref([])

async function loadItems () {
  const val = await invoke('get_skills', { advanced: false })
  table0Values.value = val
  // table0Values.value = val.slice(0, val.length / 2 + 1)
  // table1Values.value = val.slice(val.length / 2 + 1)
}
onMounted(() => loadItems())

</script>
