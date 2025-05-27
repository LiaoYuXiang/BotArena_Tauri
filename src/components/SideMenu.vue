<!-- src/components/SideMenu.vue -->
<template>
  <van-popup
    :show="show"
    position="left"
    @update:show="$emit('update:show', $event)"
    class="popup-list"
  >
    <van-cell-group class="list-group">
      <!-- 頁面切換彈出列表 -->
      <van-cell
        title="主頁"
        to="/"
        icon="wap-home-o"
        is-link
        class="popup-list-itom"
        @click="closeMenu"
      />
      <van-cell
        title="控制"
        to="/ctrl"
        icon="guide-o"
        is-link
        class="popup-list-itom"
        @click="closeMenu"
      />
      <van-cell
        title="設定"
        to="/setting"
        icon="setting-o"
        is-link
        class="popup-list-itom"
        @click="closeMenu"
      />
    </van-cell-group>
  </van-popup>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from "vue";

/** 定義屬性 */
defineProps<{
  show: boolean;
}>();

/** 定義事件 */
const emit = defineEmits<{
  (e: "update:show", value: boolean): void;
}>();

/** 關閉側邊菜單 */
const closeMenu = (): void => {
  emit("update:show", false);
};
</script>

<style scoped lang="scss">
@use "/src/assets/styles/colors.scss" as *;

.popup-list {
  z-index: 999;
  width: 300px;
  height: 100vh;
  background-color: white;
  padding: 2rem 0;

  .list-group {
    margin: 0 1rem;
  }
}
</style>
