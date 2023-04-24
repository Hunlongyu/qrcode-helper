<template>
  <div class="nav d-flex justify-center align-center">
    <v-sheet class="rounded-pill d-flex flex-column align-center justify-center" :height="250" :width="80" :elevation="1">
      <v-btn :class="active === 'generate' ? 'bg-primary' : ''" icon="mdi-format-text"
        :variant="active === 'generate' ? 'tonal' : 'elevated'" @click="changeViewTo('generate')"></v-btn>
      <v-btn :class="active === 'parse' ? 'ma-4 bg-primary' : 'ma-4'" icon="mdi-qrcode"
        :variant="active === 'parse' ? 'tonal' : 'elevated'" @click="changeViewTo('parse')"></v-btn>
      <v-btn :class="active === 'settings' ? 'bg-primary' : ''" icon="mdi-cog-outline"
        :variant="active === 'settings' ? 'tonal' : 'elevated'" @click="changeViewTo('settings')"></v-btn>
    </v-sheet>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useTheme } from 'vuetify'

const active = ref('generate')
const theme = useTheme()

const emit = defineEmits(["changeView"])

function changeViewTo(view: string) {
  active.value = view
  emit("changeView", view)
  theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
}
</script>

<style lang="scss">
.nav {
  width: 140px;
  height: 100%;
}
</style>