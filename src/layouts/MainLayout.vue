<template>
    <v-app>
        <MenuBar />

        <v-main class="bg-grey-lighten-3">
            <router-view />
        </v-main>
    </v-app>
</template>
  

<script setup>
import MenuBar from '../components/MenuBar.vue';
</script>