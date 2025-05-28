<script setup lang="ts">
import { reactive } from "vue";
/** 搖桿控制模式設定 */
const joystickSettings = reactive({
  /** 搖桿靈敏度 */
  sensitivity: 0.2,
  /** 搖桿大小 */
  size: 150,
});

// const onChange = (value: number) => showToast("当前值：" + value);
</script>
<template>
  <!-- <van-slider v-model="value" @change="onChange" /> -->
  <section class="setting-content">
    <h1 class="setting-title">控制 ControlMode</h1>
    <van-cell-group class="setting-content-group" inset>
      <van-cell title="搖桿靈敏度">
        <template #value>
          <van-slider
            v-model="joystickSettings.sensitivity"
            :step="0.01"
            :min="0"
            :max="1"
          />
          <div class="setting-slider-value">
            {{ joystickSettings.sensitivity }}
          </div>
        </template>
      </van-cell>
      <van-cell title="搖桿大小">
        <template #value>
          <van-slider
            v-model="joystickSettings.size"
            :step="10"
            :min="50"
            :max="500"
          />
          <div class="setting-slider-value">{{ joystickSettings.size }}</div>
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
