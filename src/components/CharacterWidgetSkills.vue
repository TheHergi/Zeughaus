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
              {{ $t('characteristics.shortName.' + props.item[header.key] )}}
            </v-col>
            <v-col>
              {{ charStore.attributes.characteristics.total[props.item[header.key]] }}
            </v-col>
          </v-row>
        </div>
        <div v-else-if="header.key == 'advances' && !props.item.is_grouped">
          <v-text-field density="compact"
                        type="number"
                        variant="underlined"
                        v-model.number="charStore.skills[props.item.skill_id].advances"
                        hide-details
                        @input="charStore.updateSkill(props.item.skill_id)">
          </v-text-field>
        </div>
        <div v-else-if="header.key == 'total'">
          <v-text-field density="compact"
                        type="number"
                        v-model.number="charStore.skills[props.item.skill_id].total"
                        hide-details
                        variant="underlined"
                        readonly>
          </v-text-field>
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
  </v-card>
  <CharacterGroupedSkillDialog v-model="showGroupDialog" :skill_id="groupID"></CharacterGroupedSkillDialog>
</template>

<script setup>
import { ref } from 'vue'
import { useCharacterStore } from '../stores/CharacterStore'
import CharacterGroupedSkillDialog from '../components/CharacterGroupedSkillDialog.vue'

const charStore = useCharacterStore()
const props = defineProps(['skillsData'])
const expanded = ref([])
const showGroupDialog = ref(false)
const groupID = ref(-1)

const headers = ref([
  { title: '', key: 'expand', align: 'center' },
  { title: 'Name', key: 'title', sortable: false, align: 'begin' },
  { title: 'Spielwert', key: 'attribute', sortable: false, align: 'begin' },
  { title: 'Steig.', key: 'advances', sortable: false, align: 'center' },
  { title: 'Wert', key: 'total', sortable: false, align: 'center' }

])

function specItemRequest (id) {
  groupID.value = id
  showGroupDialog.value = true
}

function updateCharacteristics () {
  console.log('Update Skills')
}
updateCharacteristics()

</script>

<style scoped>

</style>
