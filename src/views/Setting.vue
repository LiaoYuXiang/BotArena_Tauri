<script>
import settingMenu from "@/components/settingMenu.vue";
</script>
<template>
  <main class="main-content">
    <div class="setting-list">
      <h1 class="setting-title">設定</h1>
      <!-- 設定頁面切換 -->
      <settingMenu />
    </div>
    <div class="setting-content">
      <!-- 設定頁面內容 -->
      <router-view />
    </div>
  </main>
</template>
<style lang="scss" scoped>
@use "/src/assets/styles/colors.scss" as *;
.main-content {
  height: 100%;
  width: 100%;
  padding: 1rem;
  display: grid;
  grid-template-columns: 1fr 3fr;
  grid-template-areas: "setList content";
}
.setting-list {
  grid-area: setList;
  // height: 100%;
  .setting-list-group {
    width: 100%;
  }
}
.setting-content {
  grid-area: content;
  height: 100%;
  padding: 0 1rem;
}
.setting-title {
  font-size: 0.875rem;
  line-height: 1rem;
  padding: 1rem 1.8rem;
  color: $secondary-text;
}
</style>
