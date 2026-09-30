<template>
  <!-- PC端导航栏 -->
  <motion.div
    :initial="{ y: -64, opacity: 0 }"
    :animate="{ y: 0, opacity: 1 }"
    :transition="{ type: 'spring', stiffness: 170, damping: 22 }"
    class="sticky top-0 z-100"
  >
    <NLayoutHeader
      bordered
      class="bg-[var(--n-color)]! backdrop-blur-[8px] select-none max-md:hidden [-webkit-app-region:drag] [&_.n-button]:[-webkit-app-region:no-drag]"
      style="user-select: none"
    >
      <div
        class="mx-auto flex max-w-[1200px] items-center justify-between gap-8 px-6 py-3 max-md:gap-2 max-md:px-4"
      >
        <div class="logo flex-1 max-md:flex-none">
          <RouterLink to="/" class="logo-link block text-inherit no-underline">
            <h2
              class="m-0 bg-gradient-to-r from-primary to-primary-hover bg-clip-text text-[1.1rem] font-semibold text-transparent max-md:bg-none max-md:text-[var(--n-text-color)]"
            >
              {{ packageData.title }}
            </h2>
          </RouterLink>
        </div>

        <!-- 桌面端菜单 -->
        <div class="window-controls flex translate-y-[2px] scale-[1.15]">
          <NSpace>
            <NButton
              quaternary
              circle
              size="small"
              class="theme-toggle-btn overflow-hidden transition-all duration-300 hover:rotate-[30deg] hover:bg-[var(--n-color-hover)] [&_.n-icon]:transition-all [&_.n-icon]:duration-300"
              @click="handleThemeToggle"
            >
              <motion.span
                :key="isDarkMode ? 'sun' : 'moon'"
                class="inline-flex"
                :initial="{ rotate: -90, opacity: 0, scale: 0.6 }"
                :animate="{ rotate: 0, opacity: 1, scale: 1 }"
                :transition="{ type: 'spring', stiffness: 320, damping: 20 }"
              >
                <NIcon size="19" :component="isDarkMode ? Sunny : Moon" />
              </motion.span>
            </NButton>
            <NButton
              quaternary
              circle
              size="small"
              class="transition-colors duration-200 hover:bg-[rgba(128,128,128,0.15)]!"
              @click="handleToMinimize"
            >
              <NIcon size="23"><RemoveOutline /></NIcon>
            </NButton>
            <NButton
              quaternary
              circle
              size="small"
              class="transition-transform duration-200 hover:scale-110 hover:bg-[rgba(128,128,128,0.15)]!"
              @click="handleToMaximize"
            >
              <NIcon size="20"><ScanOutline /></NIcon>
            </NButton>
            <NButton
              quaternary
              circle
              size="small"
              class="transition-colors duration-200 hover:bg-[#e81123]! hover:text-white!"
              @click="ToShow = true"
            >
              <NIcon size="23"><CloseOutline /></NIcon>
            </NButton>
          </NSpace>
        </div>
      </div>
    </NLayoutHeader>
  </motion.div>

  <!-- 弹窗：是否关闭到托盘 -->
  <NModal v-model:show="ToShow" preset="dialog" style="width: 400px">
    <template #header> 你确定要关闭吗? </template>
    这样会关闭所有隧道, 你也可以同样点击右上角的X图标来关闭当前弹窗。
    <br />
    <template #action>
      <NButton size="small" type="error" @click="handleToClose(false)"
        >确定</NButton
      >
      <NButton size="small" type="warning" @click="handleToClose(true)"
        >保留隧道</NButton
      >
      <NButton size="small" type="primary" @click="handleToCloseToPanel"
        >最小化托盘</NButton
      >
    </template>
  </NModal>
</template>

<script setup lang="ts">
import packageData from "../../package.json";
import { inject, Ref, ref } from "vue";
import { RouterLink } from "vue-router";
import { NLayoutHeader, NButton, NSpace, NIcon } from "naive-ui";
import { motion } from "motion-v";
import {
  Moon,
  Sunny,
  RemoveOutline,
  ScanOutline,
  CloseOutline,
} from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/core";
import { useThemeTransition } from "../composables/useThemeTransition.ts";

const { toggleThemeWithDualCircle } = useThemeTransition();
const ToShow = ref(false);
const { isDarkMode } = inject("theme", {
  isDarkMode: ref(false),
  toggleTheme: () => {},
}) as {
  isDarkMode: Ref<boolean>;
  toggleTheme: () => void;
};

const handleToClose = async (isKeep: boolean) => {
  await invoke("quit_window", { isKeep: isKeep });
};

const handleToMinimize = async () => {
  await invoke("minimize_window");
};

const handleToMaximize = async () => {
  await invoke("toggle_maximize");
};

const handleToCloseToPanel = async () => {
  ToShow.value = false;
  await invoke("hide_to_tray");
};

const handleThemeToggle = async (event: MouseEvent) => {
  await toggleThemeWithDualCircle(event, {
    duration: 600,
    easing: "cubic-bezier(0.4, 0, 0.2, 1)",
  });
};
</script>
