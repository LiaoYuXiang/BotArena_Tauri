<template>
  <div class="intro-page">
    <van-empty
      image="/public/BotArena.png"
      description="請從左側選擇一個設定項目"
    />
  </div>
</template>

<style lang="scss" scoped>
@use "/src/assets/styles/colors.scss" as *;
.intro-page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;

  .van-empty {
    margin-top: -50px;

    .van-empty__description {
      font-size: 16px;
      color: $secondary-text;
    }
  }
}
</style>
