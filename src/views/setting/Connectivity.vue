<script setup lang="ts">
import { reactive } from "vue";
/** 搖桿控制模式設定 */
const networkConfig = reactive({
  /** 連線url */
  url: "http://localhost",
  /** 連線port */
  port: 8080,
});

// const onChange = (value: number) => showToast("当前值：" + value);
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
      <van-field v-model="networkConfig.port" type="digit" label="連線 port" />
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
