<template>
  <HomeMenu v-if="!isDashboard" />
  <RouterView v-slot="{ Component }">
    <transition name="fade" mode="out-in" appear>
      <component :is="Component" />
    </transition>
  </RouterView>
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
