<template>
    <v-row dense>
        <v-col>
            <v-row dense class="">
                <v-col cols="12">
                    <v-btn block>
                        Neuer Charakter {{ charStore.name }}
                    </v-btn>
                </v-col>
            </v-row>
            <v-row dense class="">
                <v-col cols="6">
                    <v-btn block>
                        Laden
                    </v-btn>
                </v-col>
                <v-col cols="6">
                    <v-btn block>
                        Speichern
                    </v-btn>
                </v-col>
            </v-row>
        </v-col>
    </v-row>
</template>

<script setup>
import { useCounterStore } from '../stores/CharacterStore';

const charStore = useCounterStore()

</script>

<style scoped>
.v-btn {
    text-transform: unset !important;
}
</style>