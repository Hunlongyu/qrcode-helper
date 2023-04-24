<template>
  <v-container>
    single
    <!-- <div class="generate">
    <div class="text">
      <div class="textarea" contenteditable="true" @change="textChange">baidu.com</div>
    </div>
    <div class="qrcode">
      <div class="qrcode_box">
        <div class="code" v-html="code"></div>
        <div class="style">
          <div class="header" @click="changeExpend('style')">
            <span class="hdTitle">STYLE</span>
            <span>
              <Down v-show="expend == 'style'" />
              <Up v-show="expend !== 'style'" />
            </span>
          </div>
          <div class="style-content" v-show="expend === 'style'">
            <div class="background">
              <div class="bgColor">
                background color
              </div>
              <div class="bgTransparency">
                background transparency
              </div>
            </div>
            <div class="grid">
              <div class="gridColor">
                grid color
              </div>
            </div>
            <div class="size">
              size
            </div>
          </div>
        </div>
        <div class="logoBox">
          <div class="header" @click="changeExpend('logo')">
            <span class="hdTitle">LOGO</span>
            <span>
              <Down v-show="expend === 'logo'" />
              <Up v-show="expend !== 'logo'" />
            </span>
          </div>
          <div class="logo-content" v-show="expend === 'logo'">
            <div class="logoSize">
              logo size
            </div>
          </div>
        </div>
        <div class="download">
          <div class="jpg">
            <Download /> JPG
          </div>
          <div class="png">
            <Download /> PNG
          </div>
        </div>
      </div>
    </div>
  </div> -->
  </v-container>
</template>