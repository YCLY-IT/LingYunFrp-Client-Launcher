<template>
  <Teleport to="body">
    <div
      class="route-progress"
      :style="{
        width: `${progress}%`,
        opacity: visible ? 1 : 0,
        background: `linear-gradient(90deg, ${fadeColor}, ${barColor})`,
        boxShadow: `0 0 10px ${barColor}, 0 0 4px ${barColor}`,
      }"
      aria-hidden="true"
    />
  </Teleport>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import router from "../router";
import { useThemeStore } from "../stores/theme";

const themeStore = useThemeStore();

const baseColor = computed(() => themeStore.primaryColor.slice(0, 7));
const barColor = computed(
  () => `color-mix(in srgb, ${baseColor.value} 92%, #000)`,
);
const fadeColor = computed(
  () => `color-mix(in srgb, ${baseColor.value} 20%, transparent)`,
);

const progress = ref(0);
const visible = ref(false);

let tickTimer: ReturnType<typeof setInterval> | null = null;
let offBefore: (() => void) | null = null;
let offAfter: (() => void) | null = null;
let offError: (() => void) | null = null;

const stopTick = () => {
  if (tickTimer) {
    clearInterval(tickTimer);
    tickTimer = null;
  }
};

const start = () => {
  stopTick();
  progress.value = 8;
  visible.value = true;

  tickTimer = setInterval(() => {
    const remain = 95 - progress.value;
    progress.value += Math.max(0.35, remain * 0.08);
  }, 180);
};

const finish = () => {
  stopTick();
  progress.value = 100;

  window.setTimeout(() => {
    visible.value = false;
    window.setTimeout(() => {
      progress.value = 0;
    }, 300);
  }, 400);
};

onMounted(() => {
  offBefore = router.beforeEach(() => {
    start();
  });
  offAfter = router.afterEach(() => {
    finish();
  });
  offError = router.onError(() => {
    finish();
  });
});

onUnmounted(() => {
  stopTick();
  offBefore?.();
  offAfter?.();
  offError?.();
});
</script>
