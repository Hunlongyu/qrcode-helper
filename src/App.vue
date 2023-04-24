<template>
  <div class="main">
    <Navigation @changeView="changeViews"/>
    <keep-alive>
      <component :is="active"></component>
    </keep-alive>
  </div>
</template>

<script setup lang="ts">
import Navigation from './components/Navigation.vue';
import generate from './views/generate.vue';
import parse from './views/parse.vue';
import settings from './views/settings.vue';
import { listen } from '@tauri-apps/api/event'
import {shallowRef, markRaw, onMounted } from 'vue'

interface Views {
  [key: string]: any
}

const views : Views = markRaw({generate: generate, parse: parse, settings: settings})
const active = shallowRef(parse)

function changeViews (_view: string) {
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

async function windowFocus () {
  const ctx = await navigator.clipboard.read()
  console.log('=== ctx ===', ctx)
}

onMounted(() => {
  // listen('tauri://focus', windowFocus)
})

</script>

<style lang="scss">
html,body, #app{
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
}
.main{
  width: 100%;
  height: 100%;
  background-color: #ecf7ff;
  display: flex;
  flex-direction: row;
}
</style>