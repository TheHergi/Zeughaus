<template>
  <v-app-bar class="px-12" density="compact">
    <v-spacer></v-spacer>

    <v-tabs>
      <v-tab v-for="item in links" :key="item.link" :text="item.text" :to="item.link"></v-tab>
    </v-tabs>
    <!-- <v-btn
      v-for="item in links"
      :key="item.link"
      :text="item.text"
      :to="item.link"
      variant="text"
    ></v-btn> -->

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
    text: 'Charakter',
    link: '/character/general'
  },
  {
    text: 'Wiki',
    link: '/wiki'
  },
  {
    text: 'Hilfe',
    link: '/about'
  }
]
</script>
