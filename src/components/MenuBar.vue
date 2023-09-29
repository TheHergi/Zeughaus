<template>
  <v-app-bar class="px-12" density="compact">
    <v-spacer></v-spacer>

    <v-tabs color="grey-darken-2">
      <v-tab v-for="item in links" :key="item.link" :text="item.text" :to="item.link"></v-tab>
    </v-tabs>

    <v-spacer></v-spacer>
  </v-app-bar>
</template>

<script setup>
const links = [
  {
    text: 'Zeughaus',
    link: '/'
  },
  {
    text: 'Character',
    link: '/character'
  },
  {
    text: 'Wiki',
    link: '/wiki'
  },
  {
    text: 'About',
    link: '/about'
  }
]
</script>
