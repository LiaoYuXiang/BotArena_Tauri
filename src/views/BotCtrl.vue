<script setup lang="ts">
import { ref, onMounted } from "vue";
import Joystick from "../components/Joystick.vue";
import {
  setting_api,
  webSocket_api,
} from "@/assets/ts/tauri_api.ts";
import {JoyStickControl, JoyStickData} from "@/assets/ts/JoyStickControl.ts";

/** 搖桿輸出資訊 */
/** 搖桿大小 */
const joystickSize = ref<number | null>(null);
/** 搖桿靈敏度 */
const joystickThreshold = ref<number | null>(null);
/** webSocket 連線狀態 */
const webSocketConnetState = ref<boolean>(false);
/** 防抖間隔 ms 必須持續這段時間才會觸發 */
let debounceDuration = 200;
/** 傳送延遲 ms 機器人持續移動間隔時間 */
let keepActionDuration = 200;
/** 搖桿角度 */
// const angle = ref<number | null>(null);
/** 搖桿方向 */
// const direction = ref<string | null>(null);
/** 搖桿力道 */
// const force = ref<number | null>(null);
/** 腳部搖桿控制器 */
let joyStickFeetControl: JoyStickControl
/** 手部搖桿控制器 */
let joyStickArmControl: JoyStickControl

/** 腳步搖桿事件處理 */
const onFeetStart = () => {
  joyStickFeetControl.onStart()
};
/* 搖桿移動事件處理
 * @param joyStickData 搖桿輸出資訊
 */
const onFeetMove = async (joyStickData : JoyStickData) => {
  await joyStickFeetControl.onMove(joyStickData)
  webSocketConnetState.value = joyStickFeetControl.webSocketState
}
const onFeetEnd = () => {
  joyStickFeetControl.onEnd()
}

/** 手部搖桿事件處理 */
const onArmStart = () => {
  joyStickArmControl.onStart()
};
/* 搖桿移動事件處理
 * @param joyStickData 搖桿輸出資訊
 */
const onArmMove = async (joyStickData : JoyStickData) => {
  await joyStickArmControl.onMove(joyStickData)
  webSocketConnetState.value = joyStickArmControl.webSocketState
}
const onArmEnd = () => {
  joyStickArmControl.onEnd()
}

/** 載入搖桿大小 */
const loadJoystickSize = async () => {
  const result = await setting_api.getSetting();
  // console.log(result)
  joystickThreshold.value = result.control.joystick_sensitivity;
  joystickSize.value = result.control.joystick_size;
  // 需要 * 1000 ，將秒轉成毫秒
  keepActionDuration = result.control.joystick_send_interval * 1000;
  debounceDuration = result.control.joystick_debounce_interval * 1000;
};

/** 確認WebSocketConnet連線狀態 */
const chackWebSocketConnet = async () => {
  setTimeout(async () => {
    webSocketConnetState.value = await webSocket_api.wsIsConnected();
    await chackWebSocketConnet();
  }, 200);
};
// 初始化時載入設定
onMounted(async () => {
  await loadJoystickSize();
  await chackWebSocketConnet();
  // console.log(debounceDuration)
  // console.log(keepActionDuration)
  joyStickFeetControl = new JoyStickControl(
      "feet",
      debounceDuration,
      keepActionDuration,
      joystickThreshold.value ?? 0.2
  )
  /** 手部搖桿控制器 */
  joyStickArmControl = new JoyStickControl(
      "arm",
      debounceDuration,
      keepActionDuration,
      joystickThreshold.value ?? 0.2
  )
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
      @start="onArmStart"
      @move="onArmMove"
      @end="onArmEnd"
    />
    <!-- <ActionButton :size="15" @click="onArrowClick" /> -->
    <!-- 搖桿 -->
    <div class="game-container">
      <Joystick
        v-if="joystickSize !== null && joystickThreshold !== null"
        :color="'#aaa'"
        :size="joystickSize"
        :threshold="joystickThreshold"
        @start="onFeetStart"
        @move="onFeetMove"
        @end="onFeetEnd"
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
