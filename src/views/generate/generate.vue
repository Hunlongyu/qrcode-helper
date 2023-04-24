<template>
  <v-container class="generate overflow-hide">
    <v-tabs v-model="tab" align-tabs="end">
      <v-tab :value="1">单个生成</v-tab>
      <v-tab :value="2">批量生成</v-tab>
    </v-tabs>
    <v-window v-model="tab">
      <v-window-item :value="1">
        <single />
      </v-window-item>
      <v-window-item :value="2">
        <multiple />
      </v-window-item>
    </v-window>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useDebounceFn } from '@vueuse/core'
import { invoke } from '@tauri-apps/api'
import single from './single.vue'
import multiple from './multiple.vue'

const tab = ref(1)


function textChange() {
  document.querySelector('.textarea')?.addEventListener("DOMSubtreeModified", (e) => {
    const target = e.target as HTMLOListElement
    const val = target.textContent
    generateQrcode(val)
  }, false)
}

const code = ref('')
const generateQrcode = useDebounceFn(async (val) => {
  const img = await invoke('_generate', { data: val, color: '#000000', bgColor: '#ffffff' })
  code.value = img as string
}, 500, { maxWait: 3000 })

onMounted(async () => {
  textChange()
})

const expend = ref('')
function changeExpend(e: string) {
  if (expend.value === e) {
    expend.value = ''
  } else {
    expend.value = e
  }
}

</script>

<style lang="scss" scoped>
.generate {
  flex-grow: 1;

  .text {
    flex-grow: 1;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .textarea {
      font-size: 16px;
      color: #012889 !important;
      max-width: 480px;
      border: none;
      outline: none;
      word-break: break-all;
      min-width: 400px;
      display: flex;
      cursor: text;

      &:empty {
        &::before {
          content: "输入内容将自动生成二维码。";
          color: #889fc4;
        }
      }
    }
  }

  .qrcode {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .qrcode_box {
      background-color: #00288a;
      border-radius: 20px;
      overflow: hidden;
      margin: 20px 25px;
      display: flex;
      justify-content: center;
      align-items: center;
      flex-direction: column;
      box-shadow: 0 8px 30px rgb(0, 0, 0, 0.12);

      // height: 94%;
      .code {
        margin: 60px 100px;
        width: 160px;
        height: 160px;
      }

      .style {
        background-color: #113a9f;
        border-radius: 8px;

        .style-content {
          display: flex;
          flex-direction: column;
          justify-content: center;
        }
      }

      .logoBox {
        background-color: #113a9f;
        border-radius: 8px;
        margin-top: 10px;
      }

      .header {
        padding: 10px 20px;
        display: flex;
        justify-content: space-between;
        width: 200px;
        font-size: 14px;
        cursor: pointer;
      }

      .download {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        width: 240px;
        margin: 20px 0;

        .jpg {
          background-color: #3dbcf9;
          border-radius: 8px;
          padding: 6px 20px;
          font-size: 12px;
          cursor: pointer;

          &:hover {
            background-color: #ff9b05;
          }
        }

        .png {
          background-color: #3dbcf9;
          border-radius: 8px;
          padding: 6px 20px;
          font-size: 12px;
          cursor: pointer;

          &:hover {
            background-color: #ff9b05;
          }
        }
      }
    }
  }
}
</style>