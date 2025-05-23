<script setup lang="ts">
import { ref } from "vue";
import Joystick from "../components/Joystick.vue";
/** 搖桿輸出資訊 */
const angle = ref<number | null>(null);
const direction = ref<string | null>(null);
const force = ref<number | null>(null);

/** 搖桿事件處理 */
const onStart = () => {
  console.log("搖桿啟動");
};

const onMove = (payload: {
  angle: number;
  direction: string;
  force: number;
}) => {
  angle.value = payload.angle;
  direction.value = payload.direction;
  force.value = payload.force;
};

const onEnd = () => {
  console.log("搖桿結束");
  angle.value = null;
  direction.value = null;
  force.value = null;
};
</script>

<template>
  <div class="main">
    <div class="container">
      <div class="status">
        <p>角度：{{ angle }}</p>
        <p>方向：{{ direction }}</p>
        <p>力道：{{ force }}</p>
      </div>
    </div>

    <div class="game-container">
      <Joystick
        :color="'#aaa'"
        :size="150"
        :threshold="0.2"
        @start="onStart"
        @move="onMove"
        @end="onEnd"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
.main {
  width: 100%;
  height: 100vh;
  // overflow: hidden;
  position: relative;
  display: flex;

  .container {
    z-index: 1;
    .status {
      text-align: center;
    }
  }
  .game-container {
    width: 50%;
    height: 100vh;
    position: absolute;
    top: 0;
    left: 0;
    z-index: 0;
    // background-color: #ccc;
  }
}
</style>
