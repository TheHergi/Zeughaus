<template>
  <v-row>
    <v-col cols="6" >
      <v-card class="pa-2">
        <v-card-title>
          <v-text-field
            v-model="search"
            append-icon="mdi-magnify"
            label="Suche"
            single-line
            hide-details
            density="compact"
          ></v-text-field>
        </v-card-title>
        <v-data-table
          v-model:page="page"
          :headers="headers"
          :items="serverItems"
          :items-per-page="itemsPerPage"
          :search="search"
          item-value="id"
          density="compact"
          @update:options="loadItems"
          @click:row="handleClick"
        >
          <template v-slot:item.actions="{ item }">
            <v-icon
              size="small"
              class="me-2"
              @click="editItem(item)"
            >
              mdi-pencil
            </v-icon>
            <v-icon
              size="small"
              @click="deleteItem(item)"
            >
              mdi-delete
            </v-icon>
          </template>
          <template v-slot:bottom>
            <div class="text-center pt-2">
              <v-pagination
                v-model="page"
                :length="pageCount"
                density="compact"
              ></v-pagination>
            </div>
          </template>
        </v-data-table>
      </v-card>
    </v-col>
    <v-col cols="6">
      <TalentWidget :talentID="selection" ></TalentWidget>
    </v-col>
  </v-row>

</template>

<script setup>
import { invoke } from '@tauri-apps/api/tauri'
import { ref, computed } from 'vue'
import TalentWidget from '../components/TalentWidget.vue'

const selection = ref(-1)
const itemsPerPage = ref(15)
const page = ref(1)
const search = ref('')
const serverItems = ref([])
const loading = ref(true)
const totalItems = ref(0)
const headers = ref([
  { title: 'Talent', align: 'begin', key: 'title' }
])

function editItem (item) {

}
function deleteItem (item) {

}

function handleClick (item, row) {
  selection.value = row.item.id
}

function loadItems () {
  loading.value = true

  loadItems1()

  totalItems.value = serverItems.value.length
  loading.value = false
}

const loadItems1 = async () => {
  serverItems.value = await invoke('get_talents')
}

const pageCount = computed(() => {
  return Math.ceil(serverItems.value.length / itemsPerPage.value)
})

</script>
