<template>
  <v-container class="settings overflow-hide">
    <v-row class="mt-8">
      主题
      <v-divider></v-divider>
    </v-row>
    <v-row class="mt-4">
      <v-btn-toggle v-model="db.theme" borderless variant="outlined">
        <v-btn value="light">
          <span class="hidden-sm-and-down">light</span>
          <v-icon end>
            mdi-white-balance-sunny
          </v-icon>
        </v-btn>
        <v-btn value="dark">
          <span class="hidden-sm-and-down">dark</span>
          <v-icon end>
            mdi-weather-night
          </v-icon>
        </v-btn>
        <v-btn value="system">
          <span class="hidden-sm-and-down">System</span>
          <v-icon end>
            mdi-theme-light-dark
          </v-icon>
        </v-btn>
      </v-btn-toggle>
    </v-row>
    <v-row class="mt-8">
      二维码<v-divider></v-divider>
    </v-row>
    <v-row class="mt-6">
      <v-text-field label="二维码默认保存路径" class="mr-4" v-model="db.qrcode.save" variant="solo-inverted"></v-text-field>
      <v-text-field label="二维码尺寸" v-model="db.qrcode.size" variant="solo-inverted"></v-text-field>
    </v-row>
    <v-row class="d-flex justify-space-between mt-2">
      <v-text-field label="二维码方块颜色" class="mr-4" v-model="db.qrcode.color" variant="solo-inverted"></v-text-field>
      <v-text-field label="二维码背景颜色" v-model="db.qrcode.bg_color" variant="solo-inverted"></v-text-field>
    </v-row>
    <v-row class="mt-8">
      快捷键<v-divider></v-divider>
    </v-row>
    <v-row class="mt-6">
      <v-text-field label="截图识别快捷键" class="mr-4" v-model="db.shortcut.screenshot" variant="solo-inverted"></v-text-field>
      <v-text-field label="软件唤醒快捷键" v-model="db.shortcut.focus" variant="solo-inverted"></v-text-field>
    </v-row>
    <v-row class="mt-8">
      其他<v-divider></v-divider>
    </v-row>
    <v-row class="mt-6">
      <v-autocomplete label="软件关闭时" v-model="db.app.close" :items="close" item-text="title"
        item-value="value"></v-autocomplete>
      <v-autocomplete class="mx-4" label="软件开机自启" v-model="db.app.autoLaunch" :items="autoLaunch" item-text="title"
        item-value="value"></v-autocomplete>
      <v-autocomplete label="自动更新" v-model="db.app.update" :items="update" item-text="title"
        item-value="value"></v-autocomplete>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import localforage from 'localforage';
import { onMounted } from 'vue';
import { reactive } from 'vue'

const db = reactive({
  theme: 'system',
  qrcode: {
    save: './',
    size: 512,
    color: '#000000',
    bg_color: '#ffffff'
  },
  shortcut: {
    screenshot: 'alt + s',
    focus: 'alt + f'
  },
  app: {
    close: 'mini',
    autoLaunch: false,
    update: true
  }
})

const close = [
  {
    id: 1,
    value: 'mini',
    title: '最小化到任务栏'
  },
  {
    id: 2,
    value: 'quit',
    title: '退出'
  }
]

const autoLaunch = [
  {
    id: 1,
    value: false,
    title: '关闭'
  },
  {
    id: 2,
    value: true,
    title: '开启'
  }
]

const update = [
  {
    id: 1,
    value: true,
    title: '开启'
  },
  {
    id: 2,
    value: false,
    title: '关闭'
  }
]

const getDB = async () => {
  db.theme = await localforage.getItem('theme') || 'system'
  db.qrcode.save = await localforage.getItem('qrcode.save') || './'
  db.qrcode.size = await localforage.getItem('qrcode.size') || 256
  db.qrcode.color = await localforage.getItem('qrcode.color') || '#000000'
  db.qrcode.bg_color = await localforage.getItem('qrcode.bg_color') || '#ffffff'
  db.shortcut.screenshot = await localforage.getItem('shortcut.screenshot') || 'alt + s'
  db.shortcut.focus = await localforage.getItem('shortcut.focus') || 'alt + f'
  db.app.close = await localforage.getItem('app.close') || 'mini'
  db.app.autoLaunch = await localforage.getItem('app.autoLaunch') || false
  db.app.update = await localforage.getItem('app.update') || true
}

onMounted(async () => {
  await getDB()
})
</script>

<style lang="scss"></style>