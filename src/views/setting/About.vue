<script setup lang="ts">
import { ref, onMounted } from "vue";
import { setting_api } from "@/assets/ts/tauri_api.ts";
const version = ref<string>("沒找到所以不想給你");
// 載入設定
const loadVersion = async () => {
  version.value = await setting_api.getVersion();
};
onMounted(() => {
  loadVersion();
});
</script>
<template>
  <section class="setting-content">
    <h1 class="setting-title">關於 About</h1>
    <van-cell-group class="setting-content-group" inset>
      <van-cell title="版本" :value="version" />
    </van-cell-group>
  </section>
</template>
<style scoped lang="scss">
@use "/src/assets/styles/colors.scss" as *;
.setting-content {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  .setting-content-group {
    flex-grow: 1;
  }
}
.setting-title {
  font-size: 0.875rem;
  line-height: 1rem;
  padding: 1rem 1.8rem;
  color: $secondary-text;
}
</style>
