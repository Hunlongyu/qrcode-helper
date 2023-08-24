<template>
  <v-container class="settings overflow-hide">
    <v-row class="mt-8">
      主题
      <v-divider></v-divider>
    </v-row>
    <v-row class="mt-4">
      <v-btn-toggle v-model="db.theme" borderless variant="outlined" @click="changeTheme">
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
      </v-btn-toggle>
    </v-row>
    <v-row class="mt-8">
      二维码<v-divider></v-divider>
    </v-row>
    <v-row class="mt-6">
      <v-text-field label="二维码默认保存路径" class="mr-4" v-model="db.qrcode.save" variant="solo-inverted"
        @click="changeQrcodeSave"></v-text-field>
      <v-select label="二维码尺寸" :items="[32, 64, 128, 256, 512, 1024]" v-model="db.qrcode.size" variant="solo-inverted"
        @update:model-value="changeQrcodeSize"></v-select>
    </v-row>
    <v-row class="d-flex justify-space-between mt-2">
      <v-text-field label="二维码方块颜色" class="mr-4" v-model="db.qrcode.color" variant="solo-inverted"
        @update:model-value="changeQrcodeColor"></v-text-field>
      <v-text-field label="二维码背景颜色" v-model="db.qrcode.bg_color" variant="solo-inverted"
        @update:model-value="changeQrcodeBgColor"></v-text-field>
    </v-row>
    <v-row class="mt-8">
      快捷键<v-divider></v-divider>
    </v-row>
    <v-row class="mt-6">
      <v-text-field label="截图识别快捷键" class="mr-4" v-model="db.shortcut.screenshot" variant="solo-inverted"
        @update:model-value="changeShortcutScreenshot"></v-text-field>
      <v-text-field label="软件唤醒快捷键" v-model="db.shortcut.focus" variant="solo-inverted"
        @update:model-value="changeShortcutFocus"></v-text-field>
    </v-row>
    <v-row class="mt-8">
      其他<v-divider></v-divider>
    </v-row>
    <v-row class="mt-6">
      <v-autocomplete label="软件关闭时" v-model="db.app.close" :items="close" item-text="title" variant="solo-inverted"
        @update:model-value="changeAppClose" item-value="value"></v-autocomplete>
      <v-autocomplete class="mx-4" label="软件开机自启" v-model="db.app.autoLaunch" :items="autoLaunch" item-text="title"
        variant="solo-inverted" @update:model-value="changeAppAutoLaunch" item-value="value"></v-autocomplete>
      <v-autocomplete label="自动更新" v-model="db.app.update" :items="update" item-text="title" variant="solo-inverted"
        @update:model-value="changeAppUpdate" item-value="value"></v-autocomplete>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { dialog, invoke, path, globalShortcut } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event';
import localforage from 'localforage'
import { reactive, onMounted } from 'vue'
import { useTheme } from 'vuetify'

const theme = useTheme()

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
  db.theme = await localforage.getItem('theme') || 'dark'
  db.theme === 'dark' ? theme.global.name.value = 'dark' : theme.global.name.value = 'light'
  db.qrcode.save = await localforage.getItem('qrcode.save') || ''
  db.qrcode.size = await localforage.getItem('qrcode.size') || 256
  db.qrcode.color = await localforage.getItem('qrcode.color') || '#000000'
  db.qrcode.bg_color = await localforage.getItem('qrcode.bg_color') || '#ffffff'
  db.shortcut.screenshot = await localforage.getItem('shortcut.screenshot') || 'Alt+S'
  db.shortcut.focus = await localforage.getItem('shortcut.focus') || 'Alt+F'
  db.app.close = await localforage.getItem('app.close') || 'mini'
  db.app.autoLaunch = await localforage.getItem('app.autoLaunch') || false
  db.app.update = await localforage.getItem('app.update') || true
}

// 改变主题
const changeTheme = async () => {
  theme.global.name.value = db.theme
  await localforage.setItem('theme', db.theme)
}

// 获取用户桌面路径
const getDesktopPath = async () => {
  const desktopDir = await path.desktopDir()
  if (db.qrcode.save === '' && desktopDir) {
    db.qrcode.save = desktopDir
    await localforage.setItem('qrcode.save', desktopDir)
  }
}

// 改变二维码默认保存路径
const changeQrcodeSave = async () => {
  const savePath = await dialog.open({ directory: true, multiple: false }) as string;
  if (savePath) {
    db.qrcode.save = savePath
    await localforage.setItem('qrcode.save', savePath)
  }
}

// 改变二维码尺寸
const changeQrcodeSize = async () => {
  await localforage.setItem('qrcode.size', db.qrcode.size)
}

// 检查 hex 颜色值
const checkColorValue = (item: string) => {
  const colorRegExp = /^#([0-9A-Fa-f]{6})$/
  return colorRegExp.test(item)
}

// 改变二维码颜色
const changeQrcodeColor = async () => {
  const isOk = checkColorValue(db.qrcode.color)
  if (isOk) {
    await localforage.setItem('qrcode.color', db.qrcode.color)
  } else {
    db.qrcode.color = '#000000'
  }
}

// 改变二维码背景色
const changeQrcodeBgColor = async () => {
  const isOk = checkColorValue(db.qrcode.bg_color)
  if (isOk) {
    await localforage.setItem('qrcode.bg_color', db.qrcode.bg_color)
  } else {
    db.qrcode.bg_color = '#FFFFFF'
  }
}

// 改变屏幕识别快捷键
const changeShortcutScreenshot = async () => {
  // await localforage.setItem('app.shortcut.screenshot', db.shortcut.screenshot)
  console.log('=== lala ===')
}

// 改变软件唤醒快捷键
const changeShortcutFocus = async () => {
  await localforage.setItem('app.shortcut.focus', db.shortcut.focus)
}

// 改变软件关闭时选项
const changeAppClose = async () => {
  await localforage.setItem('app.close', db.app.close)
}

// 改变软件开机是否开机自启选项
const changeAppAutoLaunch = async () => {
  await localforage.setItem('app.autoLaunch', db.app.autoLaunch)
  invoke('_toggle_auto_launch', { enable: db.app.autoLaunch })
}

// 改变软件开机是否开机自动更新选项
const changeAppUpdate = async () => {
  console.log('=== up ===')
  await localforage.setItem('app.update', db.app.update)
}

// 注册 屏幕识别快捷键
const registeredGlobalShortcut = async () => {
  const sreen = db.shortcut.screenshot;
  const isRegistered = await globalShortcut.isRegistered(sreen)
  console.log('=== is ===', isRegistered)
}

onMounted(async () => {
  await getDB()
  await getDesktopPath()
  await registeredGlobalShortcut()
})
</script>

<style lang="scss"></style>