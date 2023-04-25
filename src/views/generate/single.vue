<template>
  <v-container fluid class="single-container">
    <v-row class="single-wrapper">
      <v-col class="d-flex justify-center align-center">
        <v-textarea auto-grow variant="solo" counter clearable v-model="text"
          @update:model-value="textChange"></v-textarea>
      </v-col>
      <v-col class="d-flex justify-center align-center">
        <v-sheet class="rounded-lg d-flex justify-center flex-column align-center" :width="400" :elevation="4">
          <v-sheet class="image mt-16" :height="140" :width="140" :elevation="4" v-html="code"></v-sheet>
          <v-sheet class="mt-8" :width="300">
            <v-slider v-model="size" class="align-center" :max="512" :min="0" step="1" hide-details>
              <template v-slot:append>
                <v-text-field class="ml-3" v-model="size" hide-details single-line density="compact" variant="solo"
                  @update:model-value="sizeChangeHandle" style="width: 60px"></v-text-field>
              </template>
            </v-slider>
          </v-sheet>
          <v-sheet class="mt-8 d-flex justify-space-between" :width="300">
            <v-dialog v-model="bgDialog" width="auto" variant="outlined">
              <template v-slot:activator="{ props }">
                <v-badge dot :color="bgColor">
                  <v-btn v-bind="props">BgColor</v-btn>
                </v-badge>
              </template>
              <v-sheet>
                <v-color-picker class="ma-3" v-model="bgColor" @update:model-value="textChange"
                  :modes="['hexa']"></v-color-picker>
              </v-sheet>
            </v-dialog>
            <v-dialog v-model="gridDialog" width="auto">
              <template v-slot:activator="{ props }">
                <v-badge dot :color="gridColor">
                  <v-btn v-bind="props">GridColor</v-btn>
                </v-badge>
              </template>
              <v-sheet>
                <v-color-picker class="ma-3" v-model="gridColor" @update:model-value="textChange"
                  :modes="['hexa']"></v-color-picker>
              </v-sheet>
            </v-dialog>
          </v-sheet>
          <v-sheet class="mt-8 mb-12 d-flex flex-row justify-space-between" :width="300">
            <v-btn @click="saveAs('svg')">SVG</v-btn>
            <v-btn>JPG</v-btn>
            <v-btn>PNG</v-btn>
            <v-btn>COPY</v-btn>
          </v-sheet>
        </v-sheet>
      </v-col>
    </v-row>
  </v-container>
  <v-snackbar v-model="snackbar">{{ snkTxt }}</v-snackbar>
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
  const img = await invoke('_generate', { data: val, color: gridColor.value, bgColor: bgColor.value })
  code.value = img as string
}, 500, { maxWait: 3000 })

const size = ref(256)
function isNumber(input: any): boolean {
  if (typeof input === 'number' || typeof input === 'string') {
    return !isNaN(Number(input));
  }
  return false;
}
function sizeChangeHandle() {
  size.value = isNumber(size.value) ? size.value : 256
  size.value = size.value > 512 ? 512 : size.value
  size.value = size.value < 0 ? 0 : size.value
}

const bgDialog = ref(false)
const gridDialog = ref(false)
const bgColor = ref('#ffffffff')
const gridColor = ref('#000000ff')
const snackbar = ref(false)
const snkTxt = ref('lala')

async function saveAs(type: string) {
  console.log('=== type ===', type)
  const isOk = await invoke('_save_svg', { data: text.value, color: gridColor.value, bgColor: bgColor.value, path: 'C:\\Users\\hunlongyu\\Desktop\\test\\test.svg' })
  snkTxt.value = isOk ? '保存成功' : '保存失败'
  snackbar.value = true
}

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