<script setup lang="ts">
import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
/** 設定資料格式 */
interface SettingDt {
  url: string;
  port: number;
}
/** 網路連線設定 */
const networkConfig = reactive<SettingDt>({
  /** 連線url */
  url: "http://localhost",
  /** 連線port */
  port: 8080,
});

/** 取得後端資料 */
const fetchData = async () => {
  const result = await invoke<SettingDt>("get_Setting_data");
  // 將回傳資料灌進 reactive 容器
  Object.assign(networkConfig, result);
};
</script>
<template>
  <section class="setting-content">
    <h1 class="setting-title">網路連接 Connectivity</h1>
    <van-cell-group class="setting-content-group" inset>
      <van-field
        v-model="networkConfig.url"
        label="連線網址"
        class=".setting-cell"
      />
      <!-- <van-field v-model="networkConfig.port" type="digit" label="連線 port" /> -->
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
