<script setup lang="ts"></script>
<template>
  <section class="setting-content">
    <h1 class="setting-title">關於 About</h1>
    <van-cell-group class="setting-content-group" inset>
      <van-cell title="版本" value="beta-0" />
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
