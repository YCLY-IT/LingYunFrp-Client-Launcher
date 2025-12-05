<template>
  <Suspense>
    <NConfigProvider :theme="theme" :theme-overrides="themeOverrides">
      <NDialogProvider>
        <NMessageProvider>
          <NNotificationProvider>
            <NLoadingBarProvider>
              <AppContent />
              <CustomContextMenu ref="contextMenuRef" />
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
import { ref, computed, provide, onMounted, h, watch, onUnmounted } from "vue";
import { initLogService } from "./utils/log.ts";

initLogService();
import {
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NNotificationProvider,
  NLoadingBarProvider,
  darkTheme,
  lightTheme,
} from "naive-ui";
import AppContent from "./components/AppContent.vue";
import { invoke } from "@tauri-apps/api/core";
import { loadAppVersion, loadAppSystemInfo } from "./utils/localInfo";
import { userApi } from "./net";
import { accessHandle } from "./net/base";
import CustomContextMenu from "./components/CustomContextMenu.vue";
import { useThemeStore } from "./stores/theme.ts";

const theme = computed(() =>
  themeStore.theme === "dark" ? darkTheme : lightTheme,
);

const themeStore = useThemeStore();

const toggleTheme = () => {
  themeStore.theme = themeStore.theme === "dark" ? "light" : "dark";
};
// 更新检查相关
const updateCheckInProgress = ref(false);
const lastUpdateCheck = ref<number>(0);
const UPDATE_CHECK_INTERVAL = 3600000; // 1小时
const isAppReady = ref(false);
const hasShownUpdateNotification = ref(false);
const MAX_RETRY_COUNT = 50; // 最大重试次数
const RETRY_INTERVAL = 200; // 重试间隔（毫秒）

// 版本号比较函数，返回1表示a>b，0表示相等，-1表示a<b
function compareVersion(a: string, b: string): number {
  const aParts = a.split(".").map(Number);
  const bParts = b.split(".").map(Number);
  const len = Math.max(aParts.length, bParts.length);
  for (let i = 0; i < len; i++) {
    const aNum = aParts[i] || 0;
    const bNum = bParts[i] || 0;
    if (aNum > bNum) return 1;
    if (aNum < bNum) return -1;
  }
  return 0;
}

const checkForUpdates = async () => {
  if (localStorage.getItem("suppressUpdateNotification") === "true") {
    return;
  }

  // 如果正在检查更新，直接返回
  if (updateCheckInProgress.value) return;

  // 如果距离上次检查时间不足，直接返回
  const now = Date.now();
  if (now - lastUpdateCheck.value < UPDATE_CHECK_INTERVAL) return;

  try {
    updateCheckInProgress.value = true;
    const clientVersion = await loadAppVersion();
    const systemInfo = await loadAppSystemInfo();
    let system = systemInfo.split(" ")[0];
    let arch = systemInfo.split(" ")[1];
    console.log(`客户端版本: ${clientVersion}, 系统: ${system}, 架构: ${arch}`);
    userApi.get(
      `/frp/updates/latest?software=LingYunFrpClient&system=${system}&arch=${arch}&version=${clientVersion}`,
      accessHandle(),
      (data: any) => {
        if (
          compareVersion(data.data.latest_info.version, clientVersion) === 1 &&
          !hasShownUpdateNotification.value
        ) {
          // 只在页面刷新时显示通知
          if (performance.navigation.type === 1) {
            let retryCount = 0;

            // 等待应用准备就绪
            const showNotification = () => {
              if (isAppReady.value && (window as any).$notification) {
                (window as any).$notification.info({
                  title: `新版本 ${data.data.latest_info.version} 可用`,
                  content: data.data.latest_info.release_notes,
                  duration: 0,
                  action: () =>
                    h(
                      "button",
                      {
                        style: `
                      margin-left: 16px;
                      color: #409eff;
                      background: none;
                      border: none;
                      cursor: pointer;
                      font-size: 14px;
                      padding: 0;
                      text-decoration: underline transparent;
                      transition: text-decoration-color 0.2s;
                    `,
                        onmouseenter: (e: MouseEvent) => {
                          (e.target as HTMLElement).style.textDecorationColor =
                            "#409eff";
                        },
                        onmouseleave: (e: MouseEvent) => {
                          (e.target as HTMLElement).style.textDecorationColor =
                            "transparent";
                        },
                        onclick: () => {
                          localStorage.setItem(
                            "suppressUpdateNotification",
                            "true",
                          );
                          (window as any).$notification.destroyAll();
                        },
                      },
                      "以后不再提示",
                    ),
                });
                hasShownUpdateNotification.value = true;
              } else if (retryCount < MAX_RETRY_COUNT) {
                // 如果还没准备好且未超过最大重试次数，继续重试
                retryCount++;
                setTimeout(showNotification, RETRY_INTERVAL);
              } else {
                // 超过最大重试次数，记录错误
                console.error("显示更新通知失败：组件未就绪");
              }
            };

            // 延迟一段时间后开始尝试显示通知
            setTimeout(showNotification, 1000);
          }
        }

        // 记录日志
        setTimeout(async () => {
          await invoke("emit_event", {
            event: "log",
            payload: {
              type: "info",
              message: `新版本 ${data.data.latest_info.version} 可用`,
            },
          });
        }, 50);

        // 更新最后检查时间
        lastUpdateCheck.value = now;
      },
      (message: string) => {
        console.error(message);
      },
    );
  } catch (error) {
    console.error("检查更新失败:", error);
  } finally {
    updateCheckInProgress.value = false;
  }
};

const themeOverrides = computed(() => {
  const commonColors = {
    primaryColor: themeStore.primaryColor,
    primaryColorHover: themeStore.primaryColor,
    primaryColorPressed: themeStore.primaryColor,
    primaryColorSuppl: themeStore.primaryColor,
  };

  // 如果有背景图，则让背景色透明，否则使用默认背景色
  const hasBackgroundImage = !!themeStore.backgroundImage;
  const bodyColor = hasBackgroundImage
    ? "transparent"
    : themeStore.theme === "light"
      ? "#f5f5f5"
      : undefined;

  const lightThemeOverrides =
    themeStore.theme === "light"
      ? {
          bodyColor: bodyColor || "#f5f5f5", // 更改亮色主题下的背景颜色
        }
      : {};

  return {
    common: {
      ...commonColors,
      ...lightThemeOverrides,
      // 如果有背景图，强制设置 bodyColor 为透明
      ...(hasBackgroundImage ? { bodyColor: "transparent" } : {}),
    },
    Button: {
      // 调整Primary按钮，让它看起来更合适
      textColorPrimary: "#fff",
      textColorHoverPrimary: "#fff",
      textColorPressedPrimary: "#fff",
      textColorFocusPrimary: "#fff",
      textColorDisabledPrimary: "#fff",
      colorPrimary: themeStore.primaryColor,
      colorHoverPrimary: themeStore.primaryColor,
      colorPressedPrimary: themeStore.primaryColor,
      colorFocusPrimary: themeStore.primaryColor,
      colorDisabledPrimary: themeStore.primaryColor,
    },
  };
});

let animationFrameId: number | null = null;
let isRGBRunning = false;

const animatePrimaryColor = () => {
  if (isRGBRunning) return;
  isRGBRunning = true;

  let r = 255,
    g = 0,
    b = 0;
  let dr = -5,
    dg = 5,
    db = 0;

  const step = () => {
    if (!themeStore.isRGBMode) {
      isRGBRunning = false;
      if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
      }
      return;
    }
    if (r <= 0 && g >= 255) {
      dr = 0;
      dg = -5;
      db = 5;
    }
    if (g <= 0 && b >= 255) {
      dr = 5;
      dg = 0;
      db = -5;
    }
    if (b <= 0 && r >= 255) {
      dr = -5;
      dg = 5;
      db = 0;
    }
    r += dr;
    g += dg;
    b += db;

    themeStore.primaryColor = `rgb(${r}, ${g}, ${b})`;
    animationFrameId = requestAnimationFrame(step);
  };

  step();
};

watch(
  () => themeStore.isRGBMode,
  (newVal) => {
    if (newVal) {
      animatePrimaryColor();
    } else {
      // 完全停止RGB动画
      if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
      }
      isRGBRunning = false;

      // 重置主色调为默认值
      const defaultColor =
        localStorage.getItem("app-primary-color") || "#722ed1";
      themeStore.setPrimaryColor(defaultColor);
    }
  },
);

// 监听背景图变化，更新主题覆盖
watch(
  () => themeStore.backgroundImage,
  () => {
    // 触发 themeOverrides 重新计算
    // 由于 themeOverrides 是 computed，会自动更新
  },
);

// 触屏识别
const isTouchDevice = ref(false);

provide("isTouchDevice", isTouchDevice);

const detectInputMethod = (event: PointerEvent) => {
  if (event.pointerType === "touch") {
    isTouchDevice.value = true;
  } else if (event.pointerType === "mouse") {
    isTouchDevice.value = false;
  }
};

// 提供给全局使用
provide("theme", {
  theme,
  toggleTheme,
});
const contextMenuRef = ref();

onMounted(async () => {
  if (themeStore.isRGBMode) {
    animatePrimaryColor();
  }
  // 初始化模糊效果
  if (themeStore.isDialogBoxHairGlass) {
    document.documentElement.style.setProperty("--modal-filter", "10px");
  } else {
    document.documentElement.style.setProperty("--modal-filter", "0px");
  }
  // 初始化背景图
  if (themeStore.backgroundImage) {
    // 确保不透明度不低于20%
    const opacity = Math.max(20, themeStore.backgroundOpacity || 100);
    document.documentElement.style.setProperty(
      "--background-image",
      `url(${themeStore.backgroundImage})`,
    );
    document.documentElement.style.setProperty(
      "--background-blur",
      `${themeStore.backgroundBlur}px`,
    );
    document.documentElement.style.setProperty(
      "--background-opacity",
      `${opacity / 100}`,
    );
  } else {
    // 移除 CSS 变量以恢复默认样式
    document.documentElement.style.removeProperty("--background-image");
    document.documentElement.style.removeProperty("--background-blur");
    document.documentElement.style.removeProperty("--background-opacity");
  }
  // 初始化无障碍模式
  if (themeStore.colorBlindMode) {
    document.documentElement.classList.add("color-blind-mode");
    document.documentElement.style.setProperty(
      "--color-blind-filter",
      "url(#colorblind)",
    );
  }
  if (themeStore.highContrastMode) {
    document.documentElement.classList.add("high-contrast-mode");
  }
  // 初始化毛玻璃模式
  if (themeStore.frostedGlassMode && themeStore.backgroundImage) {
    document.documentElement.classList.add("frosted-glass-mode");
    document.documentElement.style.setProperty(
      "--frosted-glass-blur",
      `${themeStore.frostedGlassIntensity}px`,
    );
    document.documentElement.style.setProperty(
      "--frosted-glass-transition",
      "all 0.3s ease",
    );
  }
  // 更新目前的指针方式
  window.addEventListener("pointerdown", detectInputMethod);
  // 初始化日志和检查frpc
  localStorage.setItem("frpcLogs", "");
  await checkFrpcHas();

  // 标记应用已准备就绪
  isAppReady.value = true;

  // 延迟检查更新，确保所有组件都已初始化
  setTimeout(() => {
    checkForUpdates();
  }, 2000);

  // 监听右键菜单事件
  document.addEventListener("contextmenu", (e) => {
    e.preventDefault();
    contextMenuRef.value?.showMenu(e.clientX, e.clientY);
  });
});

const checkFrpcHas = async () => {
  try {
    const hasFrpc = await invoke<boolean>("check_frpc_exists");
    if (!hasFrpc) {
      (window as any).$notification?.error({
        title: "frpc.exe不存在",
        content: "请到系统设置下载frpc.exe",
        duration: 0,
      });
      setTimeout(async () => {
        await invoke("emit_event", {
          event: "log",
          payload: {
            level: "warning",
            message: `frpc.exe不存在，请到系统设置下载frpc.exe`,
          },
        });
      }, 500);
    }
  } catch (error) {
    console.error("检查frpc.exe失败:", error);
  }
};
onUnmounted(() => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
    animationFrameId = null;
  }
  // 移除指针事件监听器
  window.removeEventListener("pointerdown", detectInputMethod);
});
</script>

<style lang="scss">
@use "./assets/styles/transitions.scss";
@use "./assets/styles/index.scss";
input,
textarea,
select {
  font-size: 16px !important;
}

@media screen and (max-width: 768px) {
  input,
  textarea,
  select {
    font-size: 16px !important;
  }
}
#app {
  font-family: "Lato", "Fira Code", sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 确保SVG滤镜不占用空间 */
.defs-only {
  position: absolute;
  width: 0;
  height: 0;
  overflow: hidden;
  pointer-events: none;
}
</style>
