<template>
  <v-card title="Allgemein" class="pa-2">
    <v-row dense>
      <v-col>
        <v-text-field label="Name" density="compact" hide-details></v-text-field>
      </v-col>
      <v-col>
        <v-text-field label="Volk" density="compact" hide-details></v-text-field>
      </v-col>
    </v-row>
    <v-row dense>
      <v-col>
        <v-text-field label="Alter" density="compact" type="number" hide-details></v-text-field>
      </v-col>
      <v-col>
        <v-text-field label="Körpergröße" density="compact" type="number" suffix="m" hide-details></v-text-field>
      </v-col>
      <v-col>
        <v-text-field label="Haare" density="compact" hide-details></v-text-field>
      </v-col>
      <v-col>
        <v-text-field label="Augen" density="compact" hide-details></v-text-field>
      </v-col>
    </v-row>
  </v-card>
</template>

<script setup>
</script>
