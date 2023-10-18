<template >
  <v-card class="pa-2">

    <v-data-table :headers="headers" :items="tableValues" item-value="id" density="compact">
      <template v-for="header in headers" v-slot:[`item.${header.key}`]="props" :key="header.key">
        <div v-if="header.key != 'title'">
          <div v-if="props.index != 2">
            <v-text-field density="comfortable" type="number" v-model.number="props.item[header.key]"
                          @input="charStore.updateCharacteristics(header.key)" :min=0 :max=999 hide-details>
            </v-text-field>
          </div>
          <div v-else>
            <v-text-field readonly density="compact" v-model.number="props.item[header.key]" hide-details>
            </v-text-field>
          </div>
        </div>
        <div v-else>
          {{ props.item[header.key] }}
        </div>
      </template>

      <template #bottom></template>
    </v-data-table>
  </v-card>
</template>

<script setup>
import { useCharacterStore } from '../stores/CharacterStore'
import { ref, computed } from 'vue'

const headers = ref([
  { title: 'Spielwerte', align: 'begin', key: 'title', width: '10%', sortable: false },
  { title: 'KG', key: 'WS', width: '9%', sortable: false },
  { title: 'BF', key: 'BS', width: '9%', sortable: false },
  { title: 'ST', key: 'S', width: '9%', sortable: false },
  { title: 'Wi', key: 'T', width: '9%', sortable: false },
  { title: 'I', key: 'I', width: '9%', sortable: false },
  { title: 'GW', key: 'Ag', width: '9%', sortable: false },
  { title: 'GS', key: 'Dex', width: '9%', sortable: false },
  { title: 'IN', key: 'Int', width: '9%', sortable: false },
  { title: 'WK', key: 'WP', width: '9%', sortable: false },
  { title: 'CH', key: 'Fel', width: '9%', sortable: false }
])

const charStore = useCharacterStore()
const tableValues = computed(() =>
  Object.values(charStore.attributes.characteristics)
)

</script>
