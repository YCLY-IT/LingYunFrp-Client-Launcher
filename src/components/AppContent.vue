<template>
  <HomeMenu v-if="!isDashboard" />
  <RouterView v-slot="{ Component }">
    <transition name="fade" mode="out-in" appear>
      <component :is="Component" />
    </transition>
  </RouterView>
  <NGlobalStyle />
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
const router = useRouter(); // 新增路由实例
const loadingBar = useLoadingBar();
const message = useMessage();

// 修改后的计算属性
const isDashboard = computed(() => {
  return route.path.startsWith("/dashboard");
});

// Window类型在global.d.ts中已定义

// 使用路由的isReady替代setTimeout
onMounted(async () => {
  await router.isReady(); // 等待路由完全解析

  // 挂载全局对象
  window.$loadingBar = loadingBar;
  window.$message = message;
  window.$dialog = dialog;
  window.$notification = notification;
});
</script>
