<script setup lang="ts">
import { ref } from "vue";
import SideMenu from "@/components/SideMenu.vue";

/**
 * 切換頁面彈出顯示狀態
 */
const popupList_show = ref(false);

/**
 * 切換頁面彈出顯示狀態切換
 */
function togglePopup(): void {
  popupList_show.value = !popupList_show.value;
}
</script>

<template>
  <!-- 頁首導航欄 -->
  <div class="header-bar">
    <van-icon name="wap-nav" @click="togglePopup" class="icon" />
  </div>
  <!-- 主要顯示區域 vue-router 自動切換 -->
  <main class="main-content">
    <router-view></router-view>
  </main>
  <!-- 頁面切換彈出 -->
  <SideMenu v-model:show="popupList_show" />
</template>

<style scoped lang="scss">
@use "@/assets/styles/colors.scss" as *;
@use "/src/assets/styles/colors.scss" as *;
$herder-bar-height: 3.125rem;
.main-content {
  height: calc(100vh - $herder-bar-height);
  background-color: $bg-color;
}

.header-bar {
  height: $herder-bar-height;
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
</style>
