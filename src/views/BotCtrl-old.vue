<script setup lang="ts">
import { ref, onMounted } from "vue";
import Joystick from "../components/Joystick.vue";
// import ActionButton from "../components/ActionButton.vue";
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
/** 記錄最後移動的 腳部搖桿資料 */
let lastPayload: {
  angle: number;
  direction: "up" | "down" | "left" | "right";
  force: number;
} | null = null;
/** 記錄最後移動的 手部搖桿資料 */
let lastPayloadArm: {
  angle: number;
  direction: "up" | "down" | "left" | "right";
  force: number;
} | null = null;
/** 腳部間隔判斷用計時器 */
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
/** 手部間隔判斷用計時器 */
let debounceTimerArm: ReturnType<typeof setTimeout> | null = null;
/** 防抖間隔 ms 必須持續這段時間才會觸發 */
let debounceDuration = 200;
/** 機器人持續移動計時器 */
let keepActionTimer: ReturnType<typeof setTimeout> | null = null;
/** 機器人持續移動狀態 */
let keepActionState: boolean = false;
/** 傳送延遲 ms 機器人持續移動間隔時間 */
let keepActionDuration = 200;

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
      keepActionState = true;
      /** 最終方向 */
      const endDirection = payload.direction;
      keepAction(endDirection, forceValue);
    }, debounceDuration);
  }
};

/** 搖桿結束事件處理 */
const onEnd = async () => {
  keepActionState = false;
  if (keepActionTimer) {
    clearTimeout(keepActionTimer);
    keepActionTimer = null;
  }
  if (debounceTimer) {
    clearTimeout(debounceTimer);
    debounceTimer = null;
  }
  webSocketConnetState.value = await actionControl_api.stopAction({
    position: "feet",
  });
  lastPayload = null;
};
/** 執行手部動作 */
const onMove_arm = async (payload: {
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
    if (debounceTimerArm) {
      clearTimeout(debounceTimerArm);
      debounceTimerArm = null;
    }
    return;
  }
  // 與上次記錄的(搖桿資料)相同 並且力道差異<0.05
  const isSamePayload =
    lastPayloadArm &&
    lastPayloadArm.direction === payload.direction &&
    Math.abs(lastPayloadArm.force - forceValue) < 0.05;
  // 不同則更新(搖桿資料)
  if (!isSamePayload) {
    lastPayloadArm = { ...payload, force: forceValue };
    // 重設計時器
    if (debounceTimerArm) {
      clearTimeout(debounceTimerArm);
    }
    debounceTimerArm = setTimeout(async () => {
      // angle.value = payload.angle;
      // direction.value = payload.direction;
      // force.value = payload.force;
      /** 最終方向 */
      const endDirection = payload.direction;
      console.log(`方向:${payload.force}力道:${payload.direction}`);
      webSocketConnetState.value = await actionControl_api.controlAction({
        position: "arm",
        direction: endDirection,
        force: forceValue,
      });
    }, debounceDuration);
  }

  // window.alert(direction);
  // console.log("按下方向：", direction);
  // 你可以在這裡處理移動邏輯
};
/** 載入搖桿大小 */
const loadJoystickSize = async () => {
  const result = await setting_api.getSetting();
  joystickThreshold.value = result.control.joystick_sensitivity;
  joystickSize.value = result.control.joystick_size;
  keepActionDuration = result.control.joystick_send_interval;
  debounceDuration = result.control.joystick_debounce_interval;
};
/** 持續移動 */
const keepAction = async (
  direction: "up" | "down" | "left" | "right",
  force: number
) => {
  keepActionTimer = setTimeout(async () => {
    if (keepActionState) {
      webSocketConnetState.value = await actionControl_api.controlAction({
        position: "feet",
        direction: direction,
        force: force,
      });
      if (keepActionTimer) {
        clearTimeout(keepActionTimer);
        keepActionTimer = null;
      }
      // console.log(`方向:${direction} 力道:${force} 狀態:${keepActionState}`);

      keepAction(direction, force);
    }
  }, keepActionDuration);
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
    <Joystick
      v-if="joystickSize !== null && joystickThreshold !== null"
      :color="'#00e'"
      :size="joystickSize"
      :threshold="joystickThreshold"
      @move="onMove_arm"
    />
    <!-- <ActionButton :size="15" @click="onArrowClick" /> -->
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
