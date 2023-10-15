<template>
  <v-card class="">
    <v-data-table v-model:expanded="expanded" :headers="headers" :items="props.skillsData" item-value="skill_id"
                  density="compact" items-per-page="50">
      <template v-for="header in headers" v-slot:[`item.${header.key}`]="props" :key="header.key">
        <div v-if="header.key == 'title'">
          {{ props.item[header.key] }}
        </div>
        <div v-else-if="header.key == 'attribute' ">
          <v-row no-gutters>
            <v-col>
              {{ props.item[header.key] }}
            </v-col>
            <v-col>
              {{ charStore.attributes.characteristics.total[props.item[header.key]] }}
            </v-col>
          </v-row>
        </div>
        <div v-else-if="header.key == 'advances' && !props.item.is_grouped">
          <v-text-field density="compact" type="number" v-model="props.item[header.key]" hide-details variant="underlined"></v-text-field>
        </div>
        <div v-else-if="header.key == 'total'">
          1
        </div>
        <div v-else-if="header.key == 'expand'">
          <v-btn icon="mdi-chevron-down" size="small"
                 v-if="props.item.is_grouped && !expanded.includes(props.item.skill_id)"
                 @click="expanded.push(props.item.skill_id); specItemRequest(props.item.skill_id)" density="compact">
            <v-icon size="small" icon="mdi-chevron-down"></v-icon>
          </v-btn>
          <v-btn icon="mdi-chevron-up" v-if="props.item.is_grouped && expanded.includes(props.item.skill_id)"
                 @click="expanded = expanded.filter(e => e != props.item.skill_id)" density="compact">
            <v-icon xs icon="mdi-chevron-up"></v-icon>
          </v-btn>
          <!-- <v-btn icon="mdi-plus" size="small"
                 v-if="props.item.is_grouped"
                 @click="specItemRequest(props.item.skill_id)" density="compact">
            <v-icon size="small" icon="mdi-plus"></v-icon>
          </v-btn> -->
        </div>

        <!-- <div v-if="expanded.includes(props.item.skill_id)">

            </div> -->
      </template>
      <template v-slot:expanded-row="{ item }">
        <tr v-for="spec in specItems" :key="spec.id">
          <td></td>
          <td>
            {{ spec.title }}
          </td>
          <td>
            {{item.attribute}}
          </td>
          <td>
            0
          </td>
          <td>
            0
          </td>

        </tr>
      </template>

      <template #bottom></template>
    </v-data-table>
  </v-card>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useCharacterStore } from '../stores/CharacterStore'

const charStore = useCharacterStore()
const props = defineProps(['skillsData'])
const expanded = ref([])
const specItems = ref([])

const headers = ref([
  { title: '', key: 'expand', align: 'center' },
  { title: 'Name', key: 'title', sortable: false, align: 'begin' },
  { title: 'Spielwert', key: 'attribute', sortable: false, align: 'begin' },
  { title: 'Steig.', key: 'advances', sortable: false, align: 'center' },
  { title: 'Wert', key: 'total', sortable: false, align: 'center' }

])

function specItemRequest (id) {
  loadSpecItems(id)
}

async function loadSpecItems (id) {
  console.log(id)
  specItems.value = await invoke('get_skill_spec', { id })
  // tableItem.value.push(...specItems.value)
  console.log(specItems.value)
}

</script>

<style scoped>
/* .v-table:deep() .v-table__wrapper > table > tbody > tr > td {
    padding: 0px 2px;
} */

</style>
