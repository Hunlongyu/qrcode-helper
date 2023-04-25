<template>
  <v-container fluid class="single-container">
    <v-row class="single-wrapper">
      <v-col class="d-flex justify-center align-center">
        <v-textarea auto-grow variant="solo" counter clearable v-model="text"
          @update:model-value="textChange"></v-textarea>
      </v-col>
      <v-col class="d-flex justify-center align-center">
        <v-sheet class="rounded d-flex justify-center flex-column align-center" :width="400" :elevation="4">
          <v-sheet class="image mt-8" :height="140" :width="140" :elevation="4" v-html="code"></v-sheet>
          <v-sheet class="mt-8" :width="300">
            <v-slider v-model="size" class="align-center" :max="512" :min="0" step="1" hide-details>
              <template v-slot:append>
                <v-text-field v-model="size" hide-details single-line density="compact" variant="solo" readonly
                  style="width: 60px"></v-text-field>
              </template>
            </v-slider>
          </v-sheet>
          <v-sheet class="mt-8" :width="300">
            <v-color-picker v-model="color" :modes="['hexa']"></v-color-picker>
          </v-sheet>
          <v-sheet>3</v-sheet>
        </v-sheet>
      </v-col>
    </v-row>
  </v-container>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { useDebounceFn } from '@vueuse/core'
import { invoke } from '@tauri-apps/api'

const text = ref('')
function textChange() {
  if (text.value.trim() != '') {
    generateQrcode(text.value)
  } else {
    generateQrcode(' ')
  }
}

const code = ref('')
const generateQrcode = useDebounceFn(async (val) => {
  const img = await invoke('_generate', { data: val, color: '#000000', bgColor: '#ffffff' })
  code.value = img as string
}, 500, { maxWait: 3000 })

const size = ref(120)
const color = ref('')


</script>
<style lang="scss" scoped>
.single-container {
  height: 100%;

  .single-wrapper {
    height: 100%;

    .single-left {
      height: 100%;
    }
  }
}
</style>