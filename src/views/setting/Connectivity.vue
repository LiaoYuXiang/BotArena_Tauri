<script setup lang="ts">
import { onMounted, reactive, watch } from "vue";
import { setting_api, Settings } from "@/assets/ts/tauri_api.ts";
/** 網路連線設定 */
const networkConfig = reactive<Settings>({
  control: {
    joystick_sensitivity: 1.0,
    joystick_size: 100,
  },
  connect: {
    url: "",
    port: 0,
  },
});
// 載入設定
const loadSettings = async () => {
  const result = await setting_api.getSetting();
  Object.assign(networkConfig, result);
};

// 即時儲存設定（防抖可加）
const saveSettings = async () => {
  await setting_api.setSetting(networkConfig);
};

// 初始化時載入設定
onMounted(() => {
  loadSettings();
});

// 監聽 networkConfig，深層 watch 自動儲存
watch(
  () => networkConfig,
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
        v-model="networkConfig.connect.url"
        label="連線網址"
        class=".setting-cell"
      />
      <van-field
        v-model.number="networkConfig.connect.port"
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
