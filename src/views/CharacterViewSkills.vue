<template>
  <v-row>
    <v-col cols="12">
      <v-card class="">
        <v-data-table v-model:expanded="expanded" :headers="headers" :items="tableValues" item-value="skill_id"
                      density="compact" items-per-page="30" @update:options="loadItems">
          <template v-for="header in headers" v-slot:[`item.${header.key}`]="props" :key="header.key">
            <div v-if="header.key == 'title'">
              {{ props.item[header.key] }}
            </div>
            <div v-else-if="header.key == 'attribute'">
              <v-row no-gutters>
                <v-col>
                  {{ props.item[header.key] }}
                </v-col>
                <v-col>
                  {{ charStore.attributes.characteristics.total[props.item[header.key]] }}
                </v-col>
              </v-row>
            </div>
            <div v-else-if="header.key == 'advances'">
              <v-text-field density="compact" type="number" v-model="props.item[header.key]" hide-details></v-text-field>
            </div>
            <div v-else-if="header.key == 'total'">1

            </div>
            <div v-else>
              <v-btn icon="mdi-chevron-down"
                     v-if="props.item.is_grouped && !expanded.includes(props.item.skill_id)"
                     @click="expanded = [props.item.skill_id]" density="compact">
                <v-icon xs icon="mdi-chevron-down"></v-icon>
              </v-btn>
              <v-btn icon="mdi-chevron-up" v-if="props.item.is_grouped && expanded.includes(props.item.skill_id)"
                     @click="expanded = []" density="compact">
                <v-icon xs icon="mdi-chevron-up"></v-icon>
              </v-btn>
            </div>

            <!-- <div v-if="expanded.includes(props.item.skill_id)">

            </div> -->
          </template>
          <template v-slot:expanded-row="{ headers, item }">
            <td>
              <pre>{{ item.title }} Spec</pre>
            </td>
            <td>
              <pre> attr</pre>
            </td>
            <td>
              <pre>{{ item.title }} Incr</pre>
            </td>
            <td>
              <pre>{{ item.title }} Total</pre>
            </td>
          </template>

          <template #bottom></template>
        </v-data-table>
      </v-card>
    </v-col>
    <!-- <v-col cols="6">
      <v-card class="pa-2">

      </v-card>
    </v-col> -->
  </v-row>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useCharacterStore } from '../stores/CharacterStore'

const charStore = useCharacterStore()

const headers = ref([
  { title: 'Name', align: 'begin', key: 'title' },
  { title: 'Spielwert', key: 'attribute' },
  { title: 'Steig.', key: 'advances' },
  { title: 'Wert', key: 'total' },
  { title: 'Ac', key: 'expand', align: 'end' }
])
const tableValues = ref([])
const loading = ref(true)
const expanded = ref([])

async function loadItems () {
  loading.value = true
  const x = await invoke('get_basic_skills')
  tableValues.value = x
  console.log(tableValues.value)
  loading.value = false
}

</script>
