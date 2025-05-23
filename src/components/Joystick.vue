<template>
  <div ref="zone" class="joystick-zone"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, defineEmits, defineProps } from "vue";
import nipplejs, { JoystickManager, JoystickOutputData } from "nipplejs";

/**
 * Props 型別定義，提供搖桿初始化時的可調參數
 */
interface JoystickProps {
  /** 搖桿顏色，例如 "blue", "red", "#00ff00" */
  color?: string;

  /** 搖桿尺寸（像素），預設 250 */
  size?: number;

  /** 靈敏度門檻，0~1，越小越靈敏，預設 0.1 */
  threshold?: number;

  /** 是否靜止時保留搖桿樣式 */
  restJoystick?: boolean;

  /** 靜止時搖桿透明度，預設 0.5 */
  restOpacity?: number;

  /** 是否啟用多點觸控搖桿（通常關閉） */
  multitouch?: boolean;

  /** 搖桿模式，可為 'dynamic' 或 'static' */
  mode?: "dynamic" | "static";
}

/** 傳入參數（支援調整搖桿外觀與行為） */
const props = defineProps<JoystickProps>();

/**
 * 對外發出事件
 * - start：搖桿觸控啟動
 * - move：搖桿移動中，包含角度、方向與力道
 * - end：搖桿釋放
 */
const emits = defineEmits<{
  (e: "start"): void;
  (
    e: "move",
    payload: { angle: number; direction: string; force: number }
  ): void;
  (e: "end"): void;
}>();

/** 綁定搖桿掛載區域 */
const zone = ref<HTMLDivElement | null>(null);

/** nipplejs 搖桿管理器實例 */
let manager: JoystickManager | null = null;

/** 元件掛載後建立搖桿 */
onMounted(() => {
  manager = nipplejs.create({
    zone: zone.value!,
    mode: props.mode ?? "dynamic",
    color: props.color ?? "blue",
    size: props.size ?? 250,
    threshold: props.threshold ?? 0.1,
    restJoystick: props.restJoystick ?? false,
    restOpacity: props.restOpacity ?? 0.5,
    multitouch: props.multitouch ?? false,
  });

  manager.on("start", () => emits("start"));

  manager.on("move", (_, data: JoystickOutputData) => {
    if (data?.direction && data.angle) {
      emits("move", {
        angle: data.angle.degree,
        direction: data.direction.angle,
        force: data.force,
      });
    }
  });

  manager.on("end", () => emits("end"));
});

/** 卸載元件時銷毀搖桿 */
onBeforeUnmount(() => {
  if (manager) {
    manager.destroy();
    manager = null;
  }
});
</script>

<style scoped>
.joystick-zone {
  width: 100%;
  height: 100%;
  touch-action: none; /* 禁止預設觸控行為，避免捲動等干擾 */
}
</style>
