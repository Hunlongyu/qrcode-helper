<template>
  <v-container class="parse overflow-hide">
    <v-tabs v-model="tab" align-tabs="end">
      <v-tab :value="1">单个解析</v-tab>
      <v-tab :value="2">批量解析</v-tab>
    </v-tabs>
    <v-window v-model="tab" style="height: calc(100% - 40px);">
      <v-window-item :value="1" style="height: 100%;">
        <single />
      </v-window-item>
      <v-window-item :value="2" style="height: 100%;">
        <multiple />
      </v-window-item>
    </v-window>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import single from './single.vue'
import multiple from './multiple.vue'

const tab = ref(1)
</script>

<style lang="scss" scoped>
.parse {
  flex-grow: 1;
}
</style>