<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const popupList_show = ref(false);
const popupList = () => {
  popupList_show.value = !popupList_show.value;
};
</script>

<template>
  <div class="header-bar">
    <van-icon name="wap-nav" @click="popupList" class="icon" />
  </div>
  <router-view></router-view>
  <van-popup
    :show="popupList_show"
    position="left"
    @update:show="popupList_show = $event"
    class="popup-list"
    ><van-cell-group class="list-group">
      <van-cell
        title="主頁"
        to="/"
        icon="wap-home-o"
        size="48px"
        is-link
        class="popup-list-itom"
        @click="popupList"
      />
      <van-cell
        title="控制"
        to="/ctrl"
        icon="guide-o"
        size="48px"
        is-link
        class="popup-list-itom"
        @click="popupList"
      />
      <van-cell
        title="設定"
        to="/setting"
        icon="setting-o"
        size="48px"
        is-link
        class="popup-list-itom"
        @click="popupList"
      /> </van-cell-group
  ></van-popup>
</template>

<style scoped lang="scss">
@import "@/assets/styles/colors.scss";
.header-bar {
  height: 50px;
  background-color: $primary;
}
.icon {
  font-size: 3rem;
  color: #333;
  transition: color 0.3s;
}
.icon:hover {
  color: #666;
}
.popup-list {
  z-index: 999;
  width: 30%;
  height: 100vh;
  .list-group {
    margin: 0 1rem;
  }
}
</style>
