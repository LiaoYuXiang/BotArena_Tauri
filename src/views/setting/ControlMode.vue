<script setup lang="ts">
// import { reactive } from "vue";
/** 搖桿控制模式設定 */
// const joystickSettings = reactive({
//   /** 搖桿靈敏度 */
//   sensitivity: 0.2,
//   /** 搖桿大小 */
//   size: 150,
// });
import { reactive, watch, onMounted } from "vue";
import { setting_api, Settings } from "@/assets/ts/tauri_api.ts";

// 取得整體設定物件
const networkConfig = reactive<Settings>({
  control: {
    joystick_sensitivity: 0.2,
    joystick_size: 150,
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

// 儲存設定（保險：轉型 port）
const saveSettings = async () => {
  await setting_api.setSetting({
    control: networkConfig.control,
    connect: {
      ...networkConfig.connect,
      port: Number(networkConfig.connect.port),
    },
  });
};

// 掛載時載入設定
onMounted(() => {
  loadSettings();
});

// 監聽搖桿控制部分變化，自動儲存
watch(
  () => networkConfig.control,
  () => {
    saveSettings();
  },
  { deep: true }
);
</script>
<template>
  <!-- <van-slider v-model="value" @change="onChange" /> -->
  <section class="setting-content">
    <h1 class="setting-title">控制 ControlMode</h1>
    <van-cell-group class="setting-content-group" inset>
      <van-cell title="搖桿靈敏度">
        <template #value>
          <van-slider
            v-model="networkConfig.control.joystick_sensitivity"
            :step="0.01"
            :min="0"
            :max="1"
          />
          <div class="setting-slider-value">
            {{ networkConfig.control.joystick_sensitivity }}
          </div>
        </template>
      </van-cell>
      <van-cell title="搖桿大小">
        <template #value>
          <van-slider
            v-model="networkConfig.control.joystick_size"
            :step="10"
            :min="50"
            :max="500"
          />
          <div class="setting-slider-value">
            {{ networkConfig.control.joystick_size }}
          </div>
        </template>
      </van-cell>
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
