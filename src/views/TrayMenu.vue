<template>
  <div
    class="tray-menu-container"
    :class="{ dark: isDark, 'frosted-glass': frostedGlassMode }"
    :style="trayContainerStyle"
  >
    <div class="menu-header">
      <img src="/favicon.ico" alt="logo" class="logo" />
      <span class="app-name">LingYunFRP</span>
    </div>

    <div class="menu-divider"></div>

    <div class="menu-list">
      <div class="menu-item" @click="handleToggleWindow">
        <div class="menu-icon">
          <n-icon v-if="!isWindowVisible" :component="EyeOutline" size="18" />
          <n-icon v-else :component="EyeOffOutline" size="18" />
        </div>
        <span class="menu-label">{{
          isWindowVisible ? "隐藏主窗口" : "显示主窗口"
        }}</span>
      </div>
    </div>

    <div class="menu-divider"></div>

    <div class="menu-list">
      <div class="menu-item" @click="handleSettings">
        <div class="menu-icon">
          <n-icon :component="SettingsOutline" size="18" />
        </div>
        <span class="menu-label">设置</span>
      </div>

      <div class="menu-item" @click="handleAutoStart">
        <div class="menu-icon">
          <n-icon :component="CheckmarkOutline" size="18" />
        </div>
        <span class="menu-label">开/关闭自启</span>
        <span class="status-badge" :class="{ active: autoStartEnabled }">
          {{ autoStartEnabled ? "已开启" : "已关闭" }}
        </span>
      </div>

      <div class="menu-item" @click="handleOpenDataDir">
        <div class="menu-icon">
          <n-icon :component="FolderOpenOutline" size="18" />
        </div>
        <span class="menu-label">打开数据目录</span>
      </div>
    </div>

    <div class="menu-divider"></div>

    <div class="menu-list">
      <div class="menu-item warning" @click="handleQuitWithoutFrpc">
        <div class="menu-icon">
          <n-icon :component="PowerOutline" size="18" />
        </div>
        <span class="menu-label">退出但不关闭FRPC</span>
      </div>

      <div class="menu-item danger" @click="handleQuit">
        <div class="menu-icon">
          <n-icon :component="ExitOutline" size="18" />
        </div>
        <span class="menu-label">完全退出</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useSystemStore } from "../stores/system";
import { NIcon } from "naive-ui";
import {
  EyeOutline,
  EyeOffOutline,
  SettingsOutline,
  CheckmarkOutline,
  FolderOpenOutline,
  PowerOutline,
  ExitOutline,
} from "@vicons/ionicons5";

const isDark = ref(false);
const isWindowVisible = ref(false);
const frostedGlassMode = ref(false);
const frostedGlassIntensity = ref(15);

// 使用系统store
const systemStore = useSystemStore();
const autoStartEnabled = computed(() => systemStore.autoStart);

let unlistenFrostedGlass: UnlistenFn | null = null;
let unlistenFrostedIntensity: UnlistenFn | null = null;
let unlistenAutoStartChange: UnlistenFn | null = null;
let unlistenSettingsChange: UnlistenFn | null = null;

const trayContainerStyle = computed(() => {
  if (frostedGlassMode.value) {
    return {
      background: isDark.value
        ? "rgba(35, 35, 35, 0.7)"
        : "rgba(255, 255, 255, 0.1)",
      backdropFilter: `blur(${frostedGlassIntensity.value}px) saturate(180%)`,
      WebkitBackdropFilter: `blur(${frostedGlassIntensity.value}px) saturate(180%)`,
    };
  }
  return {
    background: isDark.value
      ? "rgba(35, 35, 35, 0.95)"
      : "rgba(255, 255, 255, 0.95)",
    backdropFilter: "blur(20px)",
    WebkitBackdropFilter: "blur(20px)",
  };
});

const getCurrentTheme = () => {
  return localStorage.getItem("app-theme") || "light";
};

const getFrostedGlassMode = () => {
  return localStorage.getItem("app-frosted-glass-mode") === "true";
};

const getFrostedGlassIntensity = () => {
  return Number(localStorage.getItem("app-frosted-glass-intensity")) || 15;
};

const updateTheme = () => {
  isDark.value = getCurrentTheme() === "dark";
};

const updateFrostedGlass = () => {
  frostedGlassMode.value = getFrostedGlassMode();
  frostedGlassIntensity.value = getFrostedGlassIntensity();
};

onMounted(async () => {
  updateTheme();
  updateFrostedGlass();

  window.addEventListener("storage", (e) => {
    if (e.key === "app-theme") {
      updateTheme();
    }
    if (
      e.key === "app-frosted-glass-mode" ||
      e.key === "app-frosted-glass-intensity"
    ) {
      updateFrostedGlass();
    }
  });

  window.addEventListener("theme-change", updateTheme);

  window.addEventListener("menu-shown", () => {
    updateFrostedGlass();
    updateTheme();
    checkSettings();
  });

  try {
    unlistenFrostedGlass = await listen<{
      enabled: boolean;
      intensity: number;
    }>("frosted-glass-change", (e) => {
      frostedGlassMode.value = e.payload.enabled;
      frostedGlassIntensity.value = e.payload.intensity;
    });

    unlistenFrostedIntensity = await listen<{ intensity: number }>(
      "frosted-glass-intensity-change",
      (e) => {
        frostedGlassIntensity.value = e.payload.intensity;
      },
    );
  } catch (e) {
    console.error("事件监听器注册失败:", e);
  }

  checkSettings();
  checkWindowVisibility();

  const currentWindow = getCurrentWindow();
  const unlistenFocus = currentWindow.onFocusChanged(({ payload: focused }) => {
    if (focused) {
      checkWindowVisibility();
      updateFrostedGlass();
    }
  });

  (window as any).__unlistenFocus = unlistenFocus;

  unlistenAutoStartChange = await listen(
    "system-auto-start-changed",
    (e: any) => {
      if (e.payload && e.payload.autoStart !== undefined) {
        systemStore.$patch({
          autoStart: e.payload.autoStart,
        });
      }
    },
  );

  unlistenSettingsChange = await listen("system-settings-changed", (e: any) => {
    if (e.payload) {
      systemStore.$patch({
        autoStart: e.payload.autoStart,
        autoRestoreTunnels: e.payload.autoRestoreTunnels,
        saveToTray: e.payload.saveToTray,
        skipSystemProxy: e.payload.skipSystemProxy,
      });
    }
  });
});

onUnmounted(() => {
  window.removeEventListener("theme-change", updateTheme);
  if (unlistenFrostedGlass) unlistenFrostedGlass();
  if (unlistenFrostedIntensity) unlistenFrostedIntensity();
  if (
    (window as any).__unlistenFocus &&
    typeof (window as any).__unlistenFocus === "function"
  ) {
    (window as any).__unlistenFocus();
  }
  if (unlistenAutoStartChange) {
    unlistenAutoStartChange();
    unlistenAutoStartChange = null;
  }
  if (unlistenSettingsChange) {
    unlistenSettingsChange();
    unlistenSettingsChange = null;
  }
});

const checkWindowVisibility = async () => {
  try {
    const mainWindow = await invoke("is_main_window_visible").catch(() => true);
    isWindowVisible.value = mainWindow as boolean;
  } catch {
    isWindowVisible.value = true;
  }
};

const checkSettings = async () => {
  try {
    await systemStore.loadAutoStartStatus();
  } catch (e) {
    console.error("检查设置失败:", e);
  }
};

const handleToggleWindow = async () => {
  try {
    if (isWindowVisible.value) {
      await invoke("hide_main_window");
    } else {
      await invoke("show_main_window");
    }
    isWindowVisible.value = !isWindowVisible.value;
    await hideMenu();
  } catch (e) {
    console.error("切换窗口状态失败:", e);
  }
};

const handleSettings = async () => {
  try {
    await invoke("show_settings");
    isWindowVisible.value = true;
    await hideMenu();
  } catch (e) {
    console.error("打开设置失败:", e);
  }
};

const handleAutoStart = async () => {
  try {
    await systemStore.toggleAutoStart();
  } catch (e) {
    console.error("切换自启失败:", e);
  }
  await hideMenu();
};

const handleOpenDataDir = async () => {
  try {
    await invoke("open_app_data_dir");
    await hideMenu();
  } catch (e) {
    console.error("打开数据目录失败:", e);
  }
};

const handleQuitWithoutFrpc = async () => {
  try {
    await invoke("quit_without_frpc");
  } catch (e) {
    console.error("退出失败:", e);
  }
};

const handleQuit = async () => {
  try {
    await invoke("quit_app");
  } catch (e) {
    console.error("退出失败:", e);
  }
};

const hideMenu = async () => {
  try {
    await invoke("hide_tray_menu");
  } catch (e) {
    console.error("隐藏菜单失败:", e);
  }
};
</script>

<style scoped lang="scss">
:global(html),
:global(body) {
  overflow: hidden;
  margin: 0;
  padding: 0;
}

.tray-menu-container {
  width: 240px;
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.15),
    0 2px 8px rgba(0, 0, 0, 0.1);
  padding: 12px;
  font-size: 13px;
  user-select: none;
  border: 1px solid rgba(0, 0, 0, 0.05);
  overflow: hidden;
  z-index: 999999;
  position: relative;
  /* 确保菜单显示在任务栏之上 */
  -webkit-app-region: no-drag;
  pointer-events: auto;

  &.dark {
    border-color: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  &.frosted-glass {
    border: 1px solid rgba(255, 255, 255, 0.2);

    &.dark {
      border-color: rgba(255, 255, 255, 0.15);
    }
  }
}

.menu-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  margin-bottom: 8px;

  .logo {
    width: 24px;
    height: 24px;
    border-radius: 6px;
  }

  .app-name {
    font-weight: 600;
    font-size: 14px;
    color: #333;

    .dark & {
      color: #fff;
    }
  }
}

.menu-divider {
  height: 1px;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(0, 0, 0, 0.1),
    transparent
  );
  margin: 8px 0;

  .dark & {
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.1),
      transparent
    );
  }
}

.menu-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #333;

  .dark & {
    color: #e0e0e0;
  }

  &:hover {
    background: rgba(0, 0, 0, 0.05);

    .dark & {
      background: rgba(255, 255, 255, 0.1);
    }
  }

  &:active {
    transform: scale(0.98);
  }

  &.warning {
    color: #faad14;

    &:hover {
      background: rgba(250, 173, 20, 0.1);
    }
  }

  &.danger {
    color: #ff4d4f;

    &:hover {
      background: rgba(255, 77, 79, 0.1);
    }
  }
}

.menu-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  opacity: 0.8;
}

.menu-label {
  flex: 1;
  font-size: 13px;
}

.status-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: 10px;
  background: rgba(0, 0, 0, 0.06);
  color: #999;
  transition: all 0.2s;

  .dark & {
    background: rgba(255, 255, 255, 0.1);
  }

  &.active {
    background: rgba(82, 196, 26, 0.15);
    color: #52c41a;
  }
}
</style>
