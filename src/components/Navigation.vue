<template>
  <div class="nav d-flex justify-center align-center">
    <v-sheet class="rounded-pill d-flex flex-column align-center justify-center" :height="250" :width="80" :elevation="1">
      <v-tooltip location="end" text="Generate">
        <template v-slot:activator="{ props }">
          <v-btn :class="active === 'generate' ? 'bg-primary' : ''" icon="mdi-format-text" v-bind="props"
            :variant="active === 'generate' ? 'tonal' : 'elevated'" @click="changeViewTo('generate')">
          </v-btn>
        </template>
      </v-tooltip>
      <v-tooltip location="end" text="Parse">
        <template v-slot:activator="{ props }">
          <v-btn :class="active === 'parse' ? 'ma-4 bg-primary' : 'ma-4'" icon="mdi-qrcode" v-bind="props"
            :variant="active === 'parse' ? 'tonal' : 'elevated'" @click="changeViewTo('parse')"></v-btn>
        </template>
      </v-tooltip>
      <v-tooltip location="end" text="Settings">
        <template v-slot:activator="{ props }">
          <v-btn :class="active === 'settings' ? 'bg-primary' : ''" icon="mdi-cog-outline" v-bind="props"
            :variant="active === 'settings' ? 'tonal' : 'elevated'" @click="changeViewTo('settings')"></v-btn>
        </template>
      </v-tooltip>
    </v-sheet>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const active = ref('settings')

const emit = defineEmits(["changeView"])

function changeViewTo(view: string) {
  active.value = view
  emit("changeView", view)
}
</script>

<style lang="scss">
.nav {
  width: 140px;
  height: 100%;
}
</style>