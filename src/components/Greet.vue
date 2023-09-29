<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const greetMsg = ref('')
const name = ref('')
const myID = ref('')

async function greet () {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke('greet', { name: name.value })
  await invoke('create_talent', { name: myID.value })
}
</script>

<template>
  <div>
    <form class="row" @submit.prevent="greet">
      <InputText id="greet-input" v-model="name" placeholder="Enter a name..." />
      <InputText id="greet-input_id" v-model="myID" placeholder="Enter a id..." />
      <Button severity="info">Greet</Button>
    </form>

    <div class="flex flex-column md:flex-row md:justify-content-between row-gap-3">
      <Button type="button" label="Welcome to Tauri!"></Button>
      <Button type="button" label="Button 2" severity="secondary"></Button>
      <Button type="button" label="Button 3" severity="help"></Button>
    </div>

    <p>{{ greetMsg }}</p>
  </div>
</template>
