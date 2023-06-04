<template>
  <v-container fluid class="single-container">
    <v-row class="single-wrapper">
      <v-col class="d-flex justify-center align-center parse_left">
        <div id="input-images" @click="parseImages"></div>
        <div class="tips">拖拽或选择图片文件</div>
      </v-col>
      <v-col class="d-flex justify-center align-center parse_right">
        <v-sheet class="rounded-lg d-flex justify-center flex-column align-center" :width="400" :elevation="4"
          :max-height="600">
          <v-list lines="two" class="content_list">
            <v-list-item v-for="(item, index) in resultList" variant="plain">
              <v-list-item-subtitle @click="handleCheck(item)">
                {{ item.content }}
              </v-list-item-subtitle>
              <template v-slot:append>
                <v-btn size="x-small" icon="mdi-content-copy" variant="text" @click="handleCopy(item)"></v-btn>
                <v-btn size="x-small" icon="mdi-delete" variant="text" @click="handleDelete(index)"></v-btn>
              </template>
            </v-list-item>
          </v-list>
        </v-sheet>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { dialog, invoke, clipboard, shell } from '@tauri-apps/api';
import { ref, Ref } from 'vue';

const resultList: Ref<Result[]> = ref([])

interface Result {
  result: boolean
  content: string
}

// 解析
const parseImages = async () => {
  const path = await dialog.open({ directory: false, multiple: true, filters: [{ name: '二维码图片', extensions: ['png', 'jpg', 'jpeg'] }] }) as string[];
  if (path.length) {
    const res: string = await invoke('_parse_images_with_paths', { path: JSON.stringify(path), lib: 'rxing' }) // rqrr & rxing & all
    const res_json = JSON.parse(res) as Result[]
    resultList.value.unshift(...res_json)
  }
}

// 
const handleCheck = async (item: Result) => {
  const urlRegExp = /^(?:(?:https?|ftp):\/\/)?[\w/\-?=%.]+\.[\w/\-?=%.]+$/
  const isUrl = urlRegExp.test(item.content)
  if (isUrl) {
    shell.open(item.content)
  }
}

// 复制内容到剪贴板
const handleCopy = async (item: Result) => {
  await clipboard.writeText(item.content)
}

// 删除指定序号的数据
const handleDelete = (idx: number) => {
  resultList.value.splice(idx, 1)
}

</script>

<style lang="scss" scoped>
.single-container {
  height: 100%;

  .single-wrapper {
    height: 100%;

    .parse_left {
      height: 100%;
      width: 100%;
      flex-direction: column;
      z-index: 1;

      &::before {
        position: absolute;
        content: "";
        width: 100px;
        height: 6px;
        background-color: #c8dbf3;
        z-index: 2;
      }

      &::after {
        position: absolute;
        content: "";
        width: 6px;
        height: 100px;
        background-color: #c8dbf3;
        z-index: 3;
      }

      #input-images {
        width: 400px;
        height: 200px;
        opacity: 0;
        cursor: pointer;
        z-index: 4;
      }

      .tips {
        color: #889fc4;
      }
    }

    .parse_right {
      .content_list {
        width: 100%;
      }
    }
  }
}
</style>