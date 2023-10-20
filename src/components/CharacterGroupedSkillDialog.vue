<template>

  <v-dialog v-model="show" width="550">
    <v-card>
      <v-card-text>

        <v-data-table :headers="headers" :items="specItems" item-value="id"
                      density="compact" items-per-page="50">

          <template v-slot:top>
            <v-toolbar flat density="compact">
              <v-toolbar-title>{{ skillItem.title }}</v-toolbar-title>
              <v-spacer></v-spacer>
              <v-dialog
                v-model="newSpecDialog"
                max-width="500px"
              >
                <template v-slot:activator="{ props }">
                  <v-btn v-bind="props" prepend-icon="mdi-plus-thick">
                    Neue Spezialisierung
                  </v-btn>
                </template>
                <v-card>
                  <v-card-title>
                    <span class="text-h5">Neue Spezialisierung</span>
                  </v-card-title>

                  <v-card-text>
                    <v-text-field density="compact"
                                  v-model="newSpecItem"
                                  hide-details variant="underlined"
                                  label="Name">
                    </v-text-field>
                  </v-card-text>
                  <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn @click="newSpecDialog=false">Abbrechen</v-btn>
                    <v-btn @click="newSpecDialog=false; addNewSkillSpec()" prepend-icon="mdi-content-save">Speichern</v-btn>

                  </v-card-actions>
                </v-card>
              </v-dialog>
            </v-toolbar>
          </template>

          <template v-for="header in headers" v-slot:[`item.${header.key}`]="props" :key="header.key">
            <div v-if="header.key == 'title'">
              {{ props.item[header.key] }}
            </div>
            <div v-else-if="header.key == 'attribute' ">
              <v-row no-gutters>
                <v-col>
                  {{ $t('characteristics.shortName.' + skillItem[header.key] )}}
                </v-col>
                <v-col>
                  {{ charStore.attributes.characteristics.total[skillItem[header.key]] }}
                </v-col>
              </v-row>
            </div>
            <div v-else-if="header.key == 'advances' && !props.item.is_grouped">
              <v-text-field density="compact"
                            type="number"
                            v-model="charStore.skillSpecs[props.item.id].advances"
                            hide-details variant="underlined">
              </v-text-field>
            </div>
            <div v-else-if="header.key == 'total'">
              1
            </div>
            <div v-else-if="header.key == 'expand'">
              <v-btn icon="mdi-plus"
                     size="small"
                     v-if="props.item.is_grouped"
                     @click="specItemRequest(props.item.skill_id);"
                     density="compact">
                <v-icon size="small" icon="mdi-plus"></v-icon>
              </v-btn>

            </div>

          </template>

          <template #bottom></template>
        </v-data-table>

      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn @click="show = false" variant="outlined">Schließen</v-btn>
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
const newSpecDialog = ref(false)
const newSpecItem = ref('')

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

function addNewSkillSpec () {
  console.log('add new spec')
  console.log(newSpecItem)
}
</script>
