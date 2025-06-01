<script setup lang="ts">
import { ref, onMounted } from "vue";
import Joystick from "../components/Joystick.vue";
import ActionButton from "../components/ActionButton.vue";
import {
  actionControl_api,
  setting_api,
  Settings,
  ActionControl,
  StopAction,
} from "@/assets/ts/tauri_api.ts";

/** 搖桿輸出資訊 */
/** 搖桿大小 */
const joystickSize = ref<number | null>(null);
const joystickThreshold = ref<number | null>(null);
/** 搖桿角度 */
// const angle = ref<number | null>(null);
/** 搖桿方向 */
// const direction = ref<string | null>(null);
/** 搖桿力道 */
// const force = ref<number | null>(null);

/** 搖桿事件處理 */
const onStart = () => {
  // console.log("搖桿啟動");
};

/* 搖桿移動事件處理
 * @param payload 搖桿輸出資訊
 * - angle: 搖桿角度
 * - direction: 搖桿方向
 * - force: 搖桿力道
 */
const onMove = (payload: {
  angle: number;
  direction: string;
  force: number;
}) => {
  const forceValue = payload.force < 1 ? payload.force : 1;
  actionControl_api.controlAction({
    position: "feet",
    direction: payload.direction,
    force: forceValue,
  });
  // angle.value = payload.angle;
  // direction.value = payload.direction;
  // force.value = payload.force;
};

/** 搖桿結束事件處理 */
const onEnd = () => {
  // 停止腳部動作
  actionControl_api.stopAction({ position: "feet" });
  // console.log("搖桿結束");
  // angle.value = null;
  // direction.value = null;
  // force.value = null;
};
const onArrowClick = (direction: "up" | "down" | "left" | "right") => {
  // 停止上一項動作
  actionControl_api.stopAction({ position: "arm" });
  // 執行手部動作
  actionControl_api.controlAction({
    position: "arm",
    direction: direction,
    force: 1,
  });

  // window.alert(direction);
  // console.log("按下方向：", direction);
  // 你可以在這裡處理移動邏輯
};
/** 載入搖桿大小 */
const loadJoystickSize = async () => {
  const result = await setting_api.getSetting();
  joystickThreshold.value = result.control.joystick_sensitivity;
  joystickSize.value = result.control.joystick_size;
};

// 初始化時載入設定
onMounted(() => {
  loadJoystickSize();
});
</script>

<template>
  <div class="main">
    <div class="container">
      <!-- 角度顯示等內容顯示 -->
      <div class="status">
        <!-- <p>角度：{{ angle }}</p>
        <p>方向：{{ direction }}</p>
        <p>力道：{{ force }}</p> -->
        <!-- 遊戲畫面容器 -->
      </div>
    </div>
    <ActionButton :size="15" @click="onArrowClick" />
    <!-- 搖桿 -->
    <div class="game-container">
      <Joystick
        v-if="joystickSize !== null"
        :color="'#aaa'"
        :size="joystickSize"
        :threshold="joystickThreshold"
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
  height: 100%;
  // overflow: hidden;
  position: relative;
  display: flex;

  .container {
    height: fit-content;
    z-index: 1;
    .status {
      text-align: center;
    }
  }
  .game-container {
    width: 50%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    z-index: 0;
    // background-color: #ccc;
  }
}
</style>
