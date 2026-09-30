<template>
  <HomeMenu v-if="!isDashboard" />
  <!-- 路由出口不加整页过渡，页面切换的过渡由各页面自己控制（如 Dashboard 只在右侧内容区做过渡） -->
  <RouterView />
  <NGlobalStyle v-if="!isTrayMenu" />
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute, RouterView, useRouter } from "vue-router";
import {
  NGlobalStyle,
  useLoadingBar,
  useDialog,
  useNotification,
  useMessage,
} from "naive-ui";
import HomeMenu from "./HomeMenu.vue";
const dialog = useDialog();
const notification = useNotification();

const route = useRoute();
const router = useRouter();
const loadingBar = useLoadingBar();
const message = useMessage();

const isDashboard = computed(() => {
  return route.path.startsWith("/dashboard");
});

const isTrayMenu = computed(() => {
  return route.path === "/tray-menu";
});

onMounted(async () => {
  await router.isReady();

  window.$loadingBar = loadingBar;
  window.$message = message;
  window.$dialog = dialog;
  window.$notification = notification;
});
</script>
