<template>
  <v-container>
    <v-row class="flex-wrap">

      <v-col cols="2" class="d-flex flex-column">
        <v-sheet rounded>
          <v-list rounded>
            <v-list-item v-for="n in 5" :key="n" link :title="`List Item ${n}`"></v-list-item>

            <v-divider class="my-2"></v-divider>

            <v-list-item color="grey-lighten-4" link title="Refresh"></v-list-item>
          </v-list>
        </v-sheet>
      </v-col>

      <v-col cols="10" class="d-flex flex-column">
        <v-sheet min-height="20vh" min-width="20vh" rounded>
          <!--  -->
        </v-sheet>
      </v-col>
    </v-row>
  </v-container>
</template>
  
<style scoped>
home {
  background-color: aqua;
}
</style>