<template>
  <v-container fluid class="single-container">
    <v-row class="single-wrapper">
      <v-col class="d-flex justify-center align-center">
        <v-textarea auto-grow variant="solo" counter clearable v-model="text"
          @update:model-value="textChange"></v-textarea>
      </v-col>
      <v-col class="d-flex justify-center align-center">
        <v-sheet class="rounded-lg d-flex justify-center flex-column align-center" :width="400" :elevation="4">
          <v-sheet class="image mt-16" :height="200" :width="200" :elevation="4" v-html="code"></v-sheet>
          <v-sheet class="mt-10 mb-12 d-flex justify-center" :width="300">
            <v-btn-toggle divided variant="outlined">
              <v-btn @click="showSettingsHandle">
                <v-icon>mdi-tune</v-icon>
                <v-tooltip activator="parent" location="bottom">设置参数</v-tooltip>
              </v-btn>
              <v-btn @click="saveAs">
                <v-icon>mdi-arrow-down-bold-box-outline</v-icon>
                <v-tooltip activator="parent" location="bottom">保存图片到本地</v-tooltip>
              </v-btn>
              <v-btn @click="copyHandle">
                <v-icon>mdi-content-copy</v-icon>
                <v-tooltip activator="parent" location="bottom">复制到剪贴板</v-tooltip>
              </v-btn>
            </v-btn-toggle>
          </v-sheet>
        </v-sheet>
      </v-col>
    </v-row>
  </v-container>
  <v-dialog v-model="settings" width="auto">
    <v-card>
      <v-card-title>Settings</v-card-title>
      <v-divider></v-divider>
      <v-sheet class="mx-10 my-8" :width="300">
        <v-select label="size" :items="[32, 64, 128, 256, 512, 1024]" v-model="size"></v-select>
      </v-sheet>
      <v-sheet class="mx-10 d-flex justify-space-between" :width="300">
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
      <v-sheet class="mx-10 mt-10 mb-5" :width="300">
        <v-autocomplete :items="nameList" v-model="name" label="name"></v-autocomplete>
      </v-sheet>
    </v-card>
  </v-dialog>
  <v-snackbar v-model="snackbar">{{ snkTxt }}</v-snackbar>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { useDebounceFn, useNow, useDateFormat, useTimestamp, useBase64 } from '@vueuse/core'
import { invoke } from '@tauri-apps/api'

const text = ref('')
function textChange() {
  if (text.value.trim() != '') {
    generateQrcode(text.value)
  } else {
    generateQrcode(' ')
  }
}

function createNameList() {
  nameList.value = []
  const n1 = useDateFormat(useNow(), 'HH:mm:ss')
  const n2 = useDateFormat(useNow(), 'YYYY-MM-DD HH:mm:ss')
  const n3 = useTimestamp().value.toString()
  nameList.value.push(n1.value)
  nameList.value.push(n2.value)
  nameList.value.push(n3)
  name.value = n3
}

const code = ref('')
const generateQrcode = useDebounceFn(async (val) => {
  const img = await invoke('_generate', { data: val, color: gridColor.value, bgColor: bgColor.value })
  code.value = img as string
  createNameList()
}, 500, { maxWait: 3000 })

const settings = ref(false)
const size = ref(256)
const bgDialog = ref(false)
const gridDialog = ref(false)
const bgColor = ref('#ffffffff')
const gridColor = ref('#000000ff')
const name = ref('')
const nameList = ref([''])
const snackbar = ref(false)
const snkTxt = ref('lala')

function showSettingsHandle() {
  settings.value = true
  if (code.value == '') {
    nameList.value = []
  }
}

async function saveAs() {
  const isOk = await invoke('_save_png', { data: text.value, color: gridColor.value, bgColor: bgColor.value, path: 'C:\\Users\\hunlongyu\\Desktop\\test\\' + name.value + '.png', size: size.value })
  snkTxt.value = isOk ? '保存成功' : '保存失败'
  snackbar.value = true
}

async function copyHandle() {
  const isOk = await invoke('_copy_to_clipboard', { data: text.value, color: gridColor.value, bgColor: bgColor.value, size: size.value })
  console.log('=== isOk ===', isOk)
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