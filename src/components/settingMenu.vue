<!-- src/components/SideMenu.vue -->
<template>
  <van-cell-group class="setting-content-group" inset>
    <van-cell
      title="控制"
      to="/setting/ControlMode"
      is-link
      class="setting-list-itom"
    />
    <van-cell
      title="網路連接"
      to="/setting/Connectivity"
      is-link
      class="setting-list-itom"
    />
    <van-cell
      title="設定檔"
      to="/setting/Profiles"
      is-link
      class="setting-list-itom"
    />
    <van-cell
      title="關於"
      to="/setting/About"
      is-link
      class="setting-list-itom"
    />
  </van-cell-group>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss"></style>
