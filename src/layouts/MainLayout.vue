<template>
    <v-app>
        <v-app-bar>
            <v-container class="mx-auto d-flex">
                <v-avatar class="me-4 " color="grey-darken-1" size="32" href="/index"></v-avatar>

                <div class="d-flex justify-center">
                    <a>
                        <v-btn text="About" href="/about" />
                    </a>
                </div>
                <!-- <v-btn v-for="link in links" :key="link" :text="link" variant="text"></v-btn> -->

                <!-- <v-spacer></v-spacer> -->

                <!-- <v-responsive max-width="160">
            <v-text-field density="compact" flat hide-details label="Search" rounded="lg" single-line
              variant="solo-filled"></v-text-field>
          </v-responsive> -->
            </v-container>
        </v-app-bar>

        <v-main class="bg-grey-lighten-3">
            <router-view />
        </v-main>
    </v-app>
</template>
  