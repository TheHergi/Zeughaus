<template>
    <v-app-bar class="px-3" density="compact">
        <v-avatar color="grey-darken-1" class="hidden-md-and-up" size="32"></v-avatar>

        <v-spacer></v-spacer>

        <v-tabs centered color="grey-darken-2">
            <v-tab v-for="item in links" :key="item.link" :text="item.text" :to="item.link"></v-tab>
        </v-tabs>
        <v-spacer></v-spacer>

        <v-avatar class="hidden-sm-and-down" color="grey-darken-1" size="32"></v-avatar>
    </v-app-bar>
</template>

<script setup>
const links = [
    {
        text: "Character",
        link: "/"
    },
    {
        text: "Wiki",
        link: "/wiki"
    },
    {
        text: "About",
        link: "/about"
    },
]
</script>