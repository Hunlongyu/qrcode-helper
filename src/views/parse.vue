<template>
  <div class="parse">
    <div class="parse-left">
      <input id="input-images" type="file" multiple accept="image/*" placeholder="" />
      <div class="tips">拖拽或打开图片文件</div>
      <div><img src="../assets/01.png" alt="" srcset=""></div>
    </div>
    <div class="parse-right">
      <div class="header">
        <Delete /><Clear />
      </div>
      <div class="wrapper">
        <div class="list">
          <div class="item">
            <button @click="parseEvent">parse</button>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" size="14" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字这里是解析后文字这里是解析后文字这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
          <div class="item">
            <div class="text">这里是解析后文字</div>
            <div class="btn"><Copy theme="outline" fill="#fff" /></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Clear, Copy, Delete } from '@icon-park/vue-next'
import { invoke } from '@tauri-apps/api'

async function parseEvent() {
  const result = await invoke('', { path: "E:\\qrcode-helper\\src\\assets\\01.png" })
  console.log('=== result ===', result)
}

</script>

<style lang="scss" scoped>
.parse{
  flex-grow: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  .parse-left{
    flex-grow: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    z-index: 1;
    &::before{
      position: absolute;
      content: "";
      width: 100px;
      height: 6px;
      background-color: #c8dbf3;
      z-index: 2;
    }
    &::after{
      position: absolute;
      content: "";
      width: 6px;
      height: 100px;
      background-color: #c8dbf3;
      z-index: 3;
    }
    #input-images{
      width: 400px;
      height: 200px;
      opacity: 0;
      cursor: pointer;
      z-index: 4;
    }
    .tips{
      color: #889fc4;
    }
  }
  .parse-right{
    width: 360px;
    background-color: #00288a;
    margin: 20px 25px;
    border-radius: 20px;
    display: flex;
    justify-content: center;
    padding: 20px 0px;
    align-items: center;
    box-shadow: 0 8px 30px rgb(0,0,0,0.12);
    flex-direction: column;
    .wrapper{
      display: flex;
      width: 100%;
      height: 100%;
      min-height: 400px;
      position: relative;
      overflow-y: scroll;
      &::-webkit-scrollbar{
        width: 10px;
      }
      &::-webkit-scrollbar-thumb {
        border-radius: 5px;
        background: #00000077;
      }
      .list{
        position: absolute;
        .item{
          margin-bottom: 10px;
          background-color: #113a9f;
          border-radius: 8px;
          font-size: 14px;
          display: flex;
          flex-direction: row;
          justify-content: space-between;
          overflow: hidden;
          margin-left: 20px;
          margin-right: 10px;
          .text{
            display: flex;
            justify-content: start;
            padding: 10px 20px;
          }
          .btn{
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
            padding: 10px 20px 8px;
            &:hover{
              background-color: #ff9b05;
            }
          }
        }
      }
    }
  }
}
</style>