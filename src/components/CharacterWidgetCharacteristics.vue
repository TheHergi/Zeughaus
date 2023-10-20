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
import { useI18n } from 'vue-i18n'
const { t } = useI18n()

const headers = ref([
  { title: t('characteristics.characteristics'), align: 'begin', key: 'title', width: '10%', sortable: false },
  { title: t('characteristics.shortName.WS'), key: 'WS', width: '9%', sortable: false },
  { title: t('characteristics.shortName.BS'), key: 'BS', width: '9%', sortable: false },
  { title: t('characteristics.shortName.S'), key: 'S', width: '9%', sortable: false },
  { title: t('characteristics.shortName.T'), key: 'T', width: '9%', sortable: false },
  { title: t('characteristics.shortName.I'), key: 'I', width: '9%', sortable: false },
  { title: t('characteristics.shortName.Ag'), key: 'Ag', width: '9%', sortable: false },
  { title: t('characteristics.shortName.Dex'), key: 'Dex', width: '9%', sortable: false },
  { title: t('characteristics.shortName.Int'), key: 'Int', width: '9%', sortable: false },
  { title: t('characteristics.shortName.WP'), key: 'WP', width: '9%', sortable: false },
  { title: t('characteristics.shortName.Fel'), key: 'Fel', width: '9%', sortable: false }
])

const charStore = useCharacterStore()
const tableValues = computed(() =>
  Object.values(charStore.attributes.characteristics)
)

</script>
