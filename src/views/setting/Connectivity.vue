<script setup lang="ts">
import { onMounted, onUnmounted, reactive, watch } from "vue";
import { setting_api, Settings, webSocket_api } from "@/assets/ts/tauri_api.ts";
import lodash from "lodash";
/** 網路連線設定 */
const config = reactive<Settings>({
  control: {
    joystick_sensitivity: 0.2,
    joystick_size: 150,
    joystick_send_interval: 0.2,
    joystick_debounce_interval: 0.2,
  },
  connect: {
    url: "I_am_Shabi.com",
    port: 8787,
  },
});
const configConnectOrigin = {};
// 載入設定
const loadSettings = async () => {
  const result = await setting_api.getSetting();
  Object.assign(configConnectOrigin, result.connect);
  Object.assign(config, result);
};

// 即時儲存設定（防抖可加）
const saveSettings = async () => {
  await setting_api.setSetting(config);
};

// 初始化時載入設定
onMounted(() => {
  loadSettings();
});
// 離開網路連線頁面刷新webSocket
onUnmounted(() => {
  if (!lodash.isEqual(configConnectOrigin, config.connect)) {
    webSocket_api.reconnectWs();
  }
});

// 監聽 networkConfig，深層 watch 自動儲存
watch(
  () => config,
  () => {
    saveSettings();
  },
  { deep: true }
);
</script>
<template>
  <section class="setting-content">
    <h1 class="setting-title">網路連接 Connectivity</h1>
    <van-cell-group class="setting-content-group" inset>
      <van-field
        v-model="config.connect.url"
        label="連線網址"
        class=".setting-cell"
      />
      <van-field
        v-model.number="config.connect.port"
        type="digit"
        label="連線 port"
      />
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
