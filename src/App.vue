<template>
  <Suspense>
    <NConfigProvider :theme="theme" :theme-overrides="themeOverrides">
      <NDialogProvider>
        <NMessageProvider>
          <NNotificationProvider>
            <NLoadingBarProvider>
              <AppContent />
              <RouteProgress />
              <CustomContextMenu ref="contextMenuRef" />

              <NModal
                v-model:show="updateModalVisible"
                :mask-closable="false"
                :closable="false"
              >
                <motion.div
                  :initial="{ opacity: 0, scale: 0.9, y: 24 }"
                  :animate="{ opacity: 1, scale: 1, y: 0 }"
                  :transition="{
                    type: 'spring',
                    stiffness: 260,
                    damping: 24,
                  }"
                  class="flex min-w-[400px] flex-col gap-4 p-6"
                >
                  <NText class="text-lg font-bold">正在更新应用</NText>
                  <NText>{{ updateStatus }}</NText>
                  <NProgress
                    type="line"
                    :percentage="updateProgress"
                    :show-indicator="true"
                  />
                  <div class="flex justify-between">
                    <NText class="text-xs text-[#999]">
                      已下载:
                      {{ (updateDownloaded / 1024 / 1024).toFixed(2) }} MB
                    </NText>
                    <NText class="text-xs text-[#999]">
                      总大小:
                      {{
                        updateTotal > 0
                          ? (updateTotal / 1024 / 1024).toFixed(2) + " MB"
                          : "未知"
                      }}
                    </NText>
                  </div>
                </motion.div>
              </NModal>
            </NLoadingBarProvider>
          </NNotificationProvider>
        </NMessageProvider>
      </NDialogProvider>
    </NConfigProvider>
  </Suspense>
  <svg class="defs-only" aria-hidden="true">
    <defs>
      <filter id="colorblind" color-interpolation-filters="sRGB">
        <feColorMatrix
          type="matrix"
          values="0.567 0.433 0 0 0 0.558 0.442 0 0 0 0 0.242 0.758 0 0 0 0 0 1 0"
        />
      </filter>
    </defs>
  </svg>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, watch } from "vue";
import { initLogService } from "./utils/log.ts";

initLogService();

import {
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NNotificationProvider,
  NLoadingBarProvider,
  NModal,
  NText,
  NProgress,
} from "naive-ui";
import { motion } from "motion-v";
import AppContent from "./components/AppContent.vue";
import CustomContextMenu from "./components/CustomContextMenu.vue";
import RouteProgress from "./components/RouteProgress.vue";
import { useAppInitialization } from "./composables/useAppInitialization";
import { useThemeStore } from "./stores/theme";

const {
  theme,
  themeOverrides,
  updateModalVisible,
  updateProgress,
  updateStatus,
  updateDownloaded,
  updateTotal,
  initializeApp,
  cleanup,
  contextMenuRef,
} = useAppInitialization();
void contextMenuRef;

const themeStore = useThemeStore();

onMounted(() => {
  initializeApp();

  // 设置自定义 hover 背景色变量
  const updateHoverColor = () => {
    const hoverColor =
      themeStore.theme === "dark"
        ? "rgba(255, 255, 255, 0.08)"
        : "rgba(0, 0, 0, 0.06)";
    document.documentElement.style.setProperty("--n-color-hover", hoverColor);
  };
  updateHoverColor();

  // 监听主题变化，更新 hover 背景色
  watch(() => themeStore.theme, updateHoverColor);

  // 设置悬浮阴影颜色变量（使用主题色）
  const updateHoverShadowColor = () => {
    const primaryColor = themeStore.primaryColor.replace("FF", ""); // 移除透明度
    document.documentElement.style.setProperty(
      "--n-color-hover-shadow",
      `${primaryColor}1F`, // 12% 透明度
    );
  };
  updateHoverShadowColor();

  // 监听主题色变化，更新阴影颜色
  watch(() => themeStore.primaryColor, updateHoverShadowColor);
});

onUnmounted(() => {
  cleanup();
});
</script>
