<template>
  <v-app>
    <div class="main">
      <Navigation @changeView="changeViews" />
      <keep-alive>
        <component :is="active"></component>
      </keep-alive>
    </div>
  </v-app>
</template>

<script setup lang="ts">
import Navigation from './components/Navigation.vue';
import generate from './views/generate/generate.vue';
import parse from './views/parse/parse.vue';
import settings from './views/settings.vue';
import { listen } from '@tauri-apps/api/event'
import { shallowRef, markRaw, onMounted } from 'vue'
import { window } from '@tauri-apps/api';

interface Views {
  [key: string]: any
}

const views: Views = markRaw({ generate: generate, parse: parse, settings: settings })
const active = shallowRef(settings)

function changeViews(_view: string) {
  if (_view === 'generate') {
    active.value = views.generate
  }
  if (_view === 'parse') {
    active.value = views.parse
  }
  if (_view === 'settings') {
    active.value = views.settings
  }
}

async function windowFocus() {
  const ctx = await navigator.clipboard.read()
  console.log('=== ctx ===', ctx)
}

onMounted(() => {
  console.log('=== lala on mounted ===')
  // listen('tauri://focus', windowFocus)
  // listen("scan_screen", val => {
  //   console.log('=== val ===', val.payload)
  // })
  // listen('tauri://file-drop', event => {
  //   console.log(event)
  // })
})

</script>

<style lang="scss">
html,
body,
#app {
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
}

.main {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
}
</style>