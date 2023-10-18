<template>

  <v-dialog v-model="show" width="550">
    <v-card>
      <v-card-text>

        <v-data-table :headers="headers" :items="specItems" item-value="id"
                      density="compact" items-per-page="50">
          <template v-for="header in headers" v-slot:[`item.${header.key}`]="props" :key="header.key">
            <div v-if="header.key == 'title'">
              {{ props.item[header.key] }}
            </div>
            <div v-else-if="header.key == 'attribute' ">
              <v-row no-gutters>
                <v-col>
                  {{ skillItem[header.key] }}
                </v-col>
                <v-col>
                  {{ charStore.attributes.characteristics.total[skillItem[header.key]] }}
                </v-col>
              </v-row>
            </div>
            <div v-else-if="header.key == 'advances' && !props.item.is_grouped">
              <v-text-field density="compact" type="number" v-model="charStore.skills[props.item.skill_id]" hide-details variant="underlined"></v-text-field>
            </div>
            <div v-else-if="header.key == 'total'">
              1
            </div>
            <div v-else-if="header.key == 'expand'">
              <v-btn icon="mdi-plus" size="small"
                     v-if="props.item.is_grouped"
                     @click="specItemRequest(props.item.skill_id); " density="compact">
                <v-icon size="small" icon="mdi-plus"></v-icon>
              </v-btn>

            </div>

          </template>

          <template #bottom></template>
        </v-data-table>

      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" @click="show = false" variant="text">Schließen</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

</template>

<script setup>
import { computed, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useCharacterStore } from '../stores/CharacterStore'

const props = defineProps(['modelValue', 'skill_id'])
const emit = defineEmits(['update:modelValue'])
const specItems = ref([])
const skillItem = ref([])
const charStore = useCharacterStore()

const headers = ref([
  { title: 'Spezialisierung', key: 'title', sortable: false, align: 'begin' },
  { title: 'Spielwert', key: 'attribute', sortable: false, align: 'begin' },
  { title: 'Steig.', key: 'advances', sortable: false, align: 'center' },
  { title: 'Wert', key: 'total', sortable: false, align: 'center' }

])

const show = computed({
  get () {
    return props.modelValue
  },
  set (newValue) {
    emit('update:modelValue', newValue)
  }
})

async function loadSpecItems (id) {
  console.log(id)
  specItems.value = await invoke('get_skill_spec', { id })
  skillItem.value = await invoke('get_skill', { id })

  console.log(skillItem.value)
}

watch(() => props.skill_id, (newVal, oldVal) => {
  console.log(
    loadSpecItems(newVal)
  )
})
</script>
