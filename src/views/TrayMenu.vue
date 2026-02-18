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
          <svg
            v-if="!isWindowVisible"
            viewBox="0 0 24 24"
            width="18"
            height="18"
          >
            <path fill="currentColor" d="M4 4h16v12H4V4zm0 14h16v2H4v-2z" />
          </svg>
          <svg v-else viewBox="0 0 24 24" width="18" height="18">
            <path
              fill="currentColor"
              d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
            />
          </svg>
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
          <svg viewBox="0 0 24 24" width="18" height="18">
            <path
              fill="currentColor"
              d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L3.16 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"
            />
          </svg>
        </div>
        <span class="menu-label">设置</span>
      </div>

      <div class="menu-item" @click="handleAutoStart">
        <div class="menu-icon">
          <svg viewBox="0 0 24 24" width="18" height="18">
            <path
              fill="currentColor"
              d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
            />
          </svg>
        </div>
        <span class="menu-label">开/关闭自启</span>
        <span class="status-badge" :class="{ active: autoStartEnabled }">
          {{ autoStartEnabled ? "已开启" : "已关闭" }}
        </span>
      </div>

      <div class="menu-item" @click="handleOpenDataDir">
        <div class="menu-icon">
          <svg viewBox="0 0 24 24" width="18" height="18">
            <path
              fill="currentColor"
              d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z"
            />
          </svg>
        </div>
        <span class="menu-label">打开数据目录</span>
      </div>
    </div>

    <div class="menu-divider"></div>

    <div class="menu-list">
      <div class="menu-item warning" @click="handleQuitWithoutFrpc">
        <div class="menu-icon">
          <svg viewBox="0 0 24 24" width="18" height="18">
            <path
              fill="currentColor"
              d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z"
            />
          </svg>
        </div>
        <span class="menu-label">退出但不关闭FRPC</span>
      </div>

      <div class="menu-item danger" @click="handleQuit">
        <div class="menu-icon">
          <svg viewBox="0 0 24 24" width="18" height="18">
            <path
              fill="currentColor"
              d="M10.09 15.59L11.5 17l5-5-5-5-1.41 1.41L12.67 11H3v2h9.67l-2.58 2.59zM19 3H5c-1.11 0-2 .9-2 2v4h2V5h14v14H5v-4H3v4c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z"
            />
          </svg>
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

const isDark = ref(false);
const autoStartEnabled = ref(false);
const isWindowVisible = ref(false);
const frostedGlassMode = ref(false);
const frostedGlassIntensity = ref(15);

let unlistenFrostedGlass: UnlistenFn | null = null;
let unlistenFrostedIntensity: UnlistenFn | null = null;

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

  checkAutoStart();
  checkWindowVisibility();

  const currentWindow = getCurrentWindow();
  const unlistenFocus = currentWindow.onFocusChanged(({ payload: focused }) => {
    if (focused) {
      checkWindowVisibility();
      updateFrostedGlass();
    }
  });

  (window as any).__unlistenFocus = unlistenFocus;
});

onUnmounted(() => {
  window.removeEventListener("theme-change", updateTheme);
  if (unlistenFrostedGlass) unlistenFrostedGlass();
  if (unlistenFrostedIntensity) unlistenFrostedIntensity();
  if ((window as any).__unlistenFocus) {
    (window as any).__unlistenFocus();
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

const checkAutoStart = async () => {
  try {
    autoStartEnabled.value = await invoke("check_auto_start_status");
  } catch (e) {
    console.error("检查自启状态失败:", e);
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
    const newStatus = !autoStartEnabled.value;
    await invoke("toggle_auto_start", { enable: newStatus });
    autoStartEnabled.value = newStatus;
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
