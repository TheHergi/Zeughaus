<template>
  <v-layout>
    <MenuBar />
    <v-main>
      <router-view />
    </v-main>
  </v-layout>
</template>

<script setup>
import MenuBar from '../components/MenuBar.vue'
</script>
