<template>
  <div
    class="dpad"
    :style="{ '--btnsize-h': size + 'vh', '--btnsize-w': size + 'vw' }"
  >
    <button class="arrow up" aria-label="Up" @click="handleClick('up')">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="32"
        height="32"
        viewBox="0 0 32 32"
      >
        <path
          fill="currentColor"
          d="M18.847 4.684c-1.235-2.242-4.457-2.243-5.693-.001L2.404 24.18C1.21 26.346 2.777 29 5.251 29h21.492c2.473 0 4.04-2.653 2.846-4.819z"
        />
      </svg>
    </button>
    <button class="arrow left" aria-label="Left" @click="handleClick('left')">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="32"
        height="32"
        viewBox="0 0 32 32"
      >
        <path
          fill="currentColor"
          d="M18.847 4.684c-1.235-2.242-4.457-2.243-5.693-.001L2.404 24.18C1.21 26.346 2.777 29 5.251 29h21.492c2.473 0 4.04-2.653 2.846-4.819z"
        />
      </svg>
    </button>
    <button
      class="arrow right"
      aria-label="Right"
      @click="handleClick('right')"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="32"
        height="32"
        viewBox="0 0 32 32"
      >
        <path
          fill="currentColor"
          d="M18.847 4.684c-1.235-2.242-4.457-2.243-5.693-.001L2.404 24.18C1.21 26.346 2.777 29 5.251 29h21.492c2.473 0 4.04-2.653 2.846-4.819z"
        />
      </svg>
    </button>
    <button class="arrow down" aria-label="Down" @click="handleClick('down')">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="32"
        height="32"
        viewBox="0 0 32 32"
      >
        <path
          fill="currentColor"
          d="M18.847 4.684c-1.235-2.242-4.457-2.243-5.693-.001L2.404 24.18C1.21 26.346 2.777 29 5.251 29h21.492c2.473 0 4.04-2.653 2.846-4.819z"
        />
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
// import { Icon } from "@iconify/vue";

defineProps<{
  size: number;
}>();

const emit = defineEmits<{
  (e: "click", direction: "up" | "down" | "left" | "right"): void;
}>();

/** 處理按鈕點擊事件 */
const handleClick = (direction: "up" | "down" | "left" | "right") => {
  emit("click", direction);
};
</script>

<style scoped lang="scss">
// @use "/src/assets/styles/colors.scss" as *;
.dpad {
  --btnsize-w: 2rem;
  --btnsize-h: 2rem;
  --btnsize: min(calc(var(--btnsize-w) / 2), var(--btnsize-h));
  display: grid;
  grid-template-areas:
    ".    up    ."
    "left null right"
    ".   down   .";
  gap: calc(var(--btnsize) / 6.25);

  width: calc(var(--btnsize) * 3.16);
  height: calc(var(--btnsize) * 3.16);
  // height: max-content;
  position: absolute;
  right: 15vb;
  bottom: 25vh;
  z-index: 1;
  .arrow {
    all: unset; // 清除按鈕樣式
    cursor: pointer;
    width: var(--btnsize);
    height: var(--btnsize);
    display: inline-block;
    transition: transform 0.2s;
    svg {
      display: block;
      width: 100%;
      height: 100%;
      pointer-events: none;
    }

    &.up {
      grid-area: up;
      transform: rotate(0deg);
      &:hover {
        transform: scale(1.1);
      }
    }

    &.down {
      grid-area: down;
      transform: rotate(180deg);
      &:hover {
        transform: rotate(180deg) scale(1.1);
      }
    }

    &.left {
      grid-area: left;
      transform: rotate(-90deg);
      &:hover {
        transform: rotate(-90deg) scale(1.1);
      }
    }

    &.right {
      grid-area: right;
      transform: rotate(90deg);
      &:hover {
        transform: rotate(90deg) scale(1.1);
      }
    }
  }
}
</style>
