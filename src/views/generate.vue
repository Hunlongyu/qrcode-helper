<template>
  <div class="generate">
    <div class="text">text</div>
    <div class="qrcode">qrcode</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const txt = ref("this is a txt")
</script>

<style lang="scss">
.generate{
  flex-grow: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #fff;
  .text{
    background-color: red;
    flex-grow: 1;
    height: 100%;
  }
  .qrcode{
    background-color: blue;
    width: 440px;
    height: 100%;
  }
}
</style>