<template>
  <div class="nav">
    <div class="navList">
      <span :class="active === 'generate' ? 'navBtn active' : 'navBtn'" @click="changeViewTo('generate')">
        <AddText theme="outline" :fill="active === 'generate' ? '#fff' : '#00288a'" />
      </span>
      <span :class="active === 'parse' ? 'navBtn active' : 'navBtn'" @click="changeViewTo('parse')">
        <ScanCode theme="outline" :fill="active === 'parse' ? '#fff' : '#00288a'" />
      </span>
      <span :class="active === 'settings' ? 'navBtn active' : 'navBtn'" @click="changeViewTo('settings')">
        <Config theme="outline" :fill="active === 'settings' ? '#fff' : '#00288a'"/>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AddText, ScanCode, Config } from '@icon-park/vue-next'
import { ref } from 'vue';

const active = ref('generate')

const emit = defineEmits(["changeView"])

function changeViewTo(view: string) {
  active.value = view
  emit("changeView", view)
}
</script>

<style lang="scss">
.nav{
  width: 140px;
  display: flex;
  justify-content: center;
  align-items: center;
  .navList{
    display: flex;
    flex-direction: column;
    background-color: #fff;
    padding: 40px 12px;
    border-radius: 45px;
    box-shadow: 0 8px 30px rgb(0,0,0,0.12);
    justify-content: center;
    align-items: center;
    span{
      width: 50px;
      height: 50px;
      display: flex;
      justify-content: center;
      align-items: center;
      border-radius: 50%;
      margin: 5px 0;
      cursor: pointer;
      transition: all 0.3s ease;
      &.active{
        background-color: #00288a;
        box-shadow: 0 0 1px 1px #e0e0ea;
      }
      &:hover{
        box-shadow: 0 10px 10px -7px rgba(255, 0, 166, 0.25), 0 -10px 10px -7px rgba(0, 162, 255, 0.25), -10px 0 10px -7px rgba(255, 98, 0, 0.25), 10px 0 10px -7px rgba(0, 255, 47, 0.25);
      }
    }
  }
}
</style>