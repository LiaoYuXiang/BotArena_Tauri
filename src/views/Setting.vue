<template>
  <main class="main-content">
    <div class="setting-list">
      <van-cell-group class="setting-list-group" inset>
        <van-cell title="单元格" value="内容" />
        <van-cell title="单元格" value="内容" label="描述信息" />
      </van-cell-group>
    </div>
    <div class="setting-content">
      <van-cell-group class="setting-content-group" inset>
        <van-cell title="单元格" value="内容" />
        <van-cell title="单元格" value="内容" label="描述信息" />
      </van-cell-group>
    </div>
  </main>
</template>
<style lang="scss" scoped>
.main-content {
  height: 100%;
  width: 100%;
  padding: 1rem;
  display: grid;
  grid-template-columns: 1fr 3fr;
  grid-template-areas: "setList content";
}
.setting-list {
  grid-area: setList;
  // height: 100%;
  .setting-list-group {
    width: 100%;
  }
}
.setting-content {
  grid-area: content;
  height: 100%;
  padding: 0 1rem;
  .setting-content-group {
    height: 100%;
    width: 100%;
  }
}
</style>
<script></script>
