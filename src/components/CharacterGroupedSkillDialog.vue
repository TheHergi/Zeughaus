<template>

  <v-dialog
    v-model="show"
    width="auto"
  >
    <v-card>
      <v-card-text>
        {{ specItems }}
      </v-card-text>
      <v-card-actions>
        <v-btn color="primary" block @click="show = false">Close Dialog</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

</template>

<script setup>
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useCharacterStore } from '../stores/CharacterStore'

const props = defineProps(['modelValue', 'skill_id'])
const emit = defineEmits(['update:modelValue'])
const specItems = ref({})

const show = computed({
  get () {
    return props.modelValue
  },
  set (newValue) {
    emit('update:modelValue', newValue)
  }
})

async function loadSpecItems () {
  const id = props.skill_id
  console.log(id)
  specItems.value = await invoke('get_skill_spec', { id })
  // tableItem.value.push(...specItems.value)
  console.log(specItems.value)
}
loadSpecItems()
</script>
