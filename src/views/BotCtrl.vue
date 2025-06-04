<script setup lang="ts">
import { ref, onMounted } from "vue";
import Joystick from "../components/Joystick.vue";
import ActionButton from "../components/ActionButton.vue";
import {
  actionControl_api,
  setting_api,
  webSocket_api,
} from "@/assets/ts/tauri_api.ts";

/** 搖桿輸出資訊 */
/** 搖桿大小 */
const joystickSize = ref<number | null>(null);
/** 搖桿靈敏度 */
const joystickThreshold = ref<number | null>(null);
/** webSocket 連線狀態 */
const webSocketConnetState = ref<boolean>(false);
/** 搖桿角度 */
// const angle = ref<number | null>(null);
/** 搖桿方向 */
// const direction = ref<string | null>(null);
/** 搖桿力道 */
// const force = ref<number | null>(null);
/** 記錄最後移動的 搖桿資料 */
let lastPayload: {
  angle: number;
  direction: "up" | "down" | "left" | "right";
  force: number;
} | null = null;

/** 間隔判斷用計時器 */
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
/** ms 必須持續這段時間才會觸發 */
const debounceDuration = 200;

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
  direction: "up" | "down" | "left" | "right";
  force: number;
}) => {
  /** 搖桿靈敏度 */
  const threshold = joystickThreshold.value ?? 0.2;
  /** 限制最大輸出 */
  const forceValue = payload.force < 1 ? payload.force : 1;

  // 力道變化率太低不處理 同時清除計時器
  if (forceValue < threshold) {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    return;
  }
  // 與上次記錄的(搖桿資料)相同 並且力道差異<0.05
  const isSamePayload =
    lastPayload &&
    lastPayload.direction === payload.direction &&
    Math.abs(lastPayload.force - forceValue) < 0.05;
  // 不同則更新(搖桿資料)
  if (!isSamePayload) {
    lastPayload = { ...payload, force: forceValue };
    // 重設計時器
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
    debounceTimer = setTimeout(async () => {
      // angle.value = payload.angle;
      // direction.value = payload.direction;
      // force.value = payload.force;
      const endDirection = payload.direction;
      webSocketConnetState.value = await actionControl_api.controlAction({
        position: "feet",
        direction: endDirection,
        force: forceValue,
      });
    }, debounceDuration);
  }
};

/** 搖桿結束事件處理 */
const onEnd = async () => {
  webSocketConnetState.value = await actionControl_api.stopAction({
    position: "feet",
  });

  if (debounceTimer) {
    clearTimeout(debounceTimer);
    debounceTimer = null;
  }
  lastPayload = null;
};
const onArrowClick = async (direction: "up" | "down" | "left" | "right") => {
  // 停止上一項動作
  webSocketConnetState.value = await actionControl_api.stopAction({
    position: "arm",
  });
  // 執行手部動作
  webSocketConnetState.value = await actionControl_api.controlAction({
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
/** 確認WebSocketConnet連線狀態 */
const chackWebSocketConnet = async () => {
  setTimeout(async () => {
    webSocketConnetState.value = await webSocket_api.wsIsConnected();
    chackWebSocketConnet();
  }, 5000);
};
// 初始化時載入設定
onMounted(() => {
  loadJoystickSize();
  chackWebSocketConnet();
});
</script>

<template>
  <div class="main">
    <div class="container">
      <!-- 角度顯示等內容顯示 -->
      <!-- <div class="status"> -->
      <!-- <p>角度：{{ angle }}</p>
        <p>方向：{{ direction }}</p>
        <p>力道：{{ force }}</p> -->
      <!-- 遊戲畫面容器 -->
      <!-- </div> -->
    </div>
    <ActionButton :size="15" @click="onArrowClick" />
    <!-- 搖桿 -->
    <div class="game-container">
      <Joystick
        v-if="joystickSize !== null && joystickThreshold !== null"
        :color="'#aaa'"
        :size="joystickSize"
        :threshold="joystickThreshold"
        @start="onStart"
        @move="onMove"
        @end="onEnd"
      />
    </div>
    <div class="loding" v-if="!webSocketConnetState">
      <van-loading size="40px" color="#7f8c8d">連線中...</van-loading>
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
  .loding {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translateX(-50%) translateY(-50%);
  }
}
</style>
