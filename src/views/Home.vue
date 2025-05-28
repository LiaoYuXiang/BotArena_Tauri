<template>
  <h1 class="main_title">歡迎來到BotArena_Tauri</h1>
  <img src="/public/BotArena.png" class="mian_img" />
</template>
<style lang="scss" scoped>
@use "/src/assets/styles/colors.scss" as *;
.main_title {
  font-size: 2rem;
  color: $text-color;
  text-align: center;
  margin-top: 2rem;
}
.mian_img {
  display: block;
  margin: 0 auto;
  width: calc(80vh - 4rem);
  height: auto;
}
</style>

<script setup lang="ts"></script>
