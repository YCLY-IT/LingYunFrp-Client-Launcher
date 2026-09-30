<template>
  <motion.div
    :initial="{ y: -64, opacity: 0 }"
    :animate="{ y: 0, opacity: 1 }"
    :transition="{ type: 'spring', stiffness: 170, damping: 22 }"
    class="h-16 w-full"
  >
    <NLayoutHeader bordered class="h-16! z-[999]! select-none">
      <!-- 确保拖动区域覆盖整个header -->
      <div
        class="header-content flex h-full w-full items-center justify-between px-4 [-webkit-app-region:drag] [&_.n-button]:[-webkit-app-region:no-drag] [&_.n-dropdown]:[-webkit-app-region:no-drag] [&_.n-popover]:[-webkit-app-region:no-drag]"
        data-tauri-drag-region
      >
        <div class="left flex items-center gap-2">
          <h2
            class="ml-5 bg-gradient-to-r from-primary to-primary-hover bg-clip-text text-xl font-semibold text-transparent"
          >
            {{ packageData.title }}
          </h2>
        </div>

        <div class="right flex items-center gap-2">
          <div class="the-right flex items-center gap-1">
            <n-button
              quaternary
              size="medium"
              class="text-lg"
              @click="ThemeSwitcherDrawer('right')"
            >
              <n-icon :component="SettingsOutline" size="medium" class="cursor-pointer" />
            </n-button>
          </div>
          <motion.div
            class="inline-block"
            :animate="{ rotate: [0, 12, -12, 0] }"
            :transition="{
              duration: 6,
              repeat: Infinity,
              ease: 'easeInOut',
              repeatDelay: 2,
            }"
          >
            <n-button
              quaternary
              circle
              size="medium"
              class="theme-toggle-btn relative overflow-hidden transition-all duration-300 hover:rotate-[30deg] hover:scale-110 hover:bg-[var(--n-color-hover)] active:rotate-[30deg] active:scale-95 [&_.n-icon]:transition-all [&_.n-icon]:duration-300"
              @click="handleThemeToggle"
            >
              <NIcon
                size="20"
                :component="themeStore.theme === 'dark' ? Sunny : Moon"
              />
            </n-button>
          </motion.div>
          <div class="theme-switch flex items-center gap-0.5">
            <!-- 客户端关闭按钮、全屏和最小化按钮 -->
            <NButton
              text
              class="h-9! w-9! rounded-[10px]! p-0! transition-colors hover:bg-[rgba(128,128,128,0.15)]!"
              @click="handleToMinimize"
            >
              <NIcon size="20">
                <RemoveOutline />
              </NIcon>
            </NButton>
            <NButton
              text
              class="h-9! w-9! rounded-[10px]! p-0! transition-colors hover:bg-[rgba(128,128,128,0.15)]!"
              @click="handleToMaximize"
            >
              <NIcon size="20">
                <ScanOutline />
              </NIcon>
            </NButton>
            <NButton
              text
              class="h-9! w-9! rounded-[10px]! p-0! transition-colors hover:bg-[#e81123]! hover:text-white!"
              @click="handleCloseButtonClick"
            >
              <NIcon size="20">
                <CloseOutline />
              </NIcon>
            </NButton>
          </div>
        </div>
      </div>
    </NLayoutHeader>
  </motion.div>

  <NModal v-model:show="ToClose" preset="dialog" style="width: 400px">
    <template #header> 你确定要关闭吗? </template>
    这样会关闭所有隧道, 你也可以同样点击右上角的X图标来关闭当前弹窗。
    <br />
    <div class="mt-[15px]">
      <n-checkbox v-model:checked="rememberChoice">记住我的选择</n-checkbox>
    </div>
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
  <n-drawer
    v-model:show="themeSwitcherDrawer"
    :placement="placement"
    :default-width="320"
    resizable
  >
    <n-drawer-content title="面板配置">
      <ThemeSwitcher />
    </n-drawer-content>
  </n-drawer>
</template>

<script setup lang="ts">
import packageData from "../../package.json";
import { ref, onMounted, onUnmounted } from "vue";
import { motion } from "motion-v";
import {
  NLayoutHeader,
  NIcon,
  NButton,
  NDrawer,
  NDrawerContent,
  NModal,
  DrawerPlacement,
  NCheckbox,
} from "naive-ui";
import {
  Sunny,
  Moon,
  SettingsOutline,
  CloseOutline,
  ScanOutline,
  RemoveOutline,
} from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/core";
import { useThemeStore } from "../stores/theme.ts";
import { useThemeTransition } from "../composables/useThemeTransition.ts";

const ToClose = ref(false);
const isMobile = ref(window.innerWidth <= 768);
const rememberChoice = ref(false);
const rememberedAction = ref("");

const themeSwitcherDrawer = ref(false);
const placement = ref<DrawerPlacement>("right");
const themeStore = useThemeStore();
const { toggleThemeWithDualCircle } = useThemeTransition();

const ThemeSwitcherDrawer = (place: DrawerPlacement) => {
  themeSwitcherDrawer.value = true;
  placement.value = place;
};

const handleThemeToggle = async (event: MouseEvent) => {
  await toggleThemeWithDualCircle(event, {
    duration: 600,
    easing: "cubic-bezier(0.4, 0, 0.2, 1)",
  });
};

const handleToClose = async (isKeep: boolean) => {
  if (rememberChoice.value) {
    rememberedAction.value = isKeep ? "keep" : "close";
    localStorage.setItem("remembered_close_action", rememberedAction.value);
  }
  await invoke("quit_window", { isKeep: isKeep });
};

const handleToMinimize = async () => {
  await invoke("minimize_window");
};

const handleToMaximize = async () => {
  await invoke("toggle_maximize");
};

const handleToCloseToPanel = async () => {
  ToClose.value = false;
  if (rememberChoice.value) {
    rememberedAction.value = "tray";
    localStorage.setItem("remembered_close_action", rememberedAction.value);
  }
  await invoke("hide_to_tray");
};
const handleResize = () => {
  isMobile.value = window.innerWidth <= 768;
};

const handleCloseButtonClick = async () => {
  const rememberedAction = localStorage.getItem("remembered_close_action");
  const saveToTray = localStorage.getItem("boot_settings");

  if (saveToTray) {
    try {
      const settings = JSON.parse(saveToTray);
      if (settings.saveToTray) {
        await invoke("hide_to_tray");
        return;
      }
    } catch (e) {
      console.error("解析启动设置失败:", e);
    }
  }

  if (rememberedAction) {
    switch (rememberedAction) {
      case "keep":
        await invoke("quit_window", { isKeep: true });
        break;
      case "close":
        await invoke("quit_window", { isKeep: false });
        break;
      case "tray":
        await invoke("hide_to_tray");
        break;
      default:
        ToClose.value = true;
    }
  } else {
    ToClose.value = true;
  }
};

onMounted(() => {
  window.addEventListener("resize", handleResize);
});

onUnmounted(() => {
  window.removeEventListener("resize", handleResize);
});
</script>
