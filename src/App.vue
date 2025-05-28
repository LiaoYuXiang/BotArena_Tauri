<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import SideMenu from "@/components/SideMenu.vue";

/** 路由 */
const route = useRoute();
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
    <h1 class="header-title">{{ route.meta.title || "BotArena_Tauri" }}</h1>
  </div>
  <!-- 主要顯示區域 vue-router 自動切換 -->
  <main class="main-content">
    <router-view></router-view>
  </main>
  <!-- 頁面切換彈出 -->
  <SideMenu v-model:show="popupList_show" />
</template>

<style scoped lang="scss">
@use "/src/assets/styles/colors.scss" as *;
$herder-bar-height: 3rem;
.main-content {
  position: relative;
  color: $text-color;
  display: flex;
  flex-direction: column;
  height: calc(100vh - $herder-bar-height);
  background-color: $bg-color;
}

.header-bar {
  display: flex;
  position: relative;
  color: $text-color;
  height: $herder-bar-height;
  background-color: $primary;
  box-shadow: $primary-dark 0px 0px 5px;
  .header-title {
    position: absolute;
    right: 50%;
    transform: translateX(50%);
    text-align: center;
    line-height: $herder-bar-height;
    font-size: 1.25rem;
  }
}

.icon {
  font-size: 3rem;
  transition: color 0.3s;
}

.icon:hover {
  color: $secondary-text;
}
</style>
