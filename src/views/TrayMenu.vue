<template>
  <div
    class="tray-menu-container relative z-999999 w-full[240px] overflow-hidden border border-black/5 p-3 text-[13px] select-none shadow-[0_8px_32px_rgba(0,0,0,0.15),0_2px_8px_rgba(0,0,0,0.1)] [-webkit-app-region:no-drag] pointer-events-auto"
    :class="[
      isDark ? 'border-white/10 text-white' : '',
      frostedGlassMode ? 'border-white/20' : '',
      frostedGlassMode && isDark ? 'border-white/15' : '',
    ]"
    :style="trayContainerStyle"
  >
    <div class="menu-header mb-2 flex items-center gap-2.5 px-3 py-2">
      <img
        src="/favicon.ico"
        alt="logo"
        class="logo h-6 w-6 rounded-md transition-transform duration-200 hover:rotate-12 hover:scale-110"
      />
      <span
        class="app-name text-sm font-semibold"
        :class="isDark ? 'text-white' : 'text-[#333]'"
        >LingYunFRP</span
      >
    </div>

    <div
      class="menu-divider my-2 h-px"
      :class="
        isDark
          ? 'bg-[linear-gradient(90deg,transparent,rgba(255,255,255,0.1),transparent)]'
          : 'bg-[linear-gradient(90deg,transparent,rgba(0,0,0,0.1),transparent)]'
      "
    ></div>

    <div class="menu-list flex flex-col gap-0.5">
      <div
        class="menu-item flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 transition-[transform,background-color] duration-200 hover:translate-x-1"
        :class="
          isDark
            ? 'text-[#e0e0e0] hover:bg-white/10'
            : 'text-[#333] hover:bg-black/5'
        "
        @click="handleToggleWindow"
      >
        <div class="menu-icon flex h-5 w-5 items-center justify-center opacity-80">
          <n-icon v-if="!isWindowVisible" :component="EyeOutline" size="18" />
          <n-icon v-else :component="EyeOffOutline" size="18" />
        </div>
        <span class="menu-label flex-1 text-[13px]">{{
          isWindowVisible ? "隐藏主窗口" : "显示主窗口"
        }}</span>
      </div>
    </div>

    <div
      class="menu-divider my-2 h-px"
      :class="
        isDark
          ? 'bg-[linear-gradient(90deg,transparent,rgba(255,255,255,0.1),transparent)]'
          : 'bg-[linear-gradient(90deg,transparent,rgba(0,0,0,0.1),transparent)]'
      "
    ></div>

    <div class="menu-list flex flex-col gap-0.5">
      <div
        class="menu-item flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 transition-[transform,background-color] duration-200 hover:translate-x-1"
        :class="
          isDark
            ? 'text-[#e0e0e0] hover:bg-white/10'
            : 'text-[#333] hover:bg-black/5'
        "
        @click="handleSettings"
      >
        <div class="menu-icon flex h-5 w-5 items-center justify-center opacity-80">
          <n-icon :component="SettingsOutline" size="18" />
        </div>
        <span class="menu-label flex-1 text-[13px]">设置</span>
      </div>

      <div
        class="menu-item flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 transition-[transform,background-color] duration-200 hover:translate-x-1"
        :class="
          isDark
            ? 'text-[#e0e0e0] hover:bg-white/10'
            : 'text-[#333] hover:bg-black/5'
        "
        @click="handleAutoStart"
      >
        <div class="menu-icon flex h-5 w-5 items-center justify-center opacity-80">
          <n-icon :component="CheckmarkOutline" size="18" />
        </div>
        <span class="menu-label flex-1 text-[13px]">开/关闭自启</span>
        <span
          class="status-badge rounded-[10px] px-2 py-0.5 text-[11px] transition-colors duration-200"
          :class="
            autoStartEnabled
              ? 'bg-[rgba(82,196,26,0.15)] text-[#52c41a]'
              : isDark
                ? 'bg-white/10 text-[#999]'
                : 'bg-black/6 text-[#999]'
          "
        >
          {{ autoStartEnabled ? "已开启" : "已关闭" }}
        </span>
      </div>

      <div
        class="menu-item flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 transition-[transform,background-color] duration-200 hover:translate-x-1"
        :class="
          isDark
            ? 'text-[#e0e0e0] hover:bg-white/10'
            : 'text-[#333] hover:bg-black/5'
        "
        @click="handleOpenDataDir"
      >
        <div class="menu-icon flex h-5 w-5 items-center justify-center opacity-80">
          <n-icon :component="FolderOpenOutline" size="18" />
        </div>
        <span class="menu-label flex-1 text-[13px]">打开数据目录</span>
      </div>
    </div>

    <div
      class="menu-divider my-2 h-px"
      :class="
        isDark
          ? 'bg-[linear-gradient(90deg,transparent,rgba(255,255,255,0.1),transparent)]'
          : 'bg-[linear-gradient(90deg,transparent,rgba(0,0,0,0.1),transparent)]'
      "
    ></div>

    <div class="menu-list flex flex-col gap-0.5">
      <div
        class="menu-item flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 text-[#faad14] transition-[transform,background-color] duration-200 hover:translate-x-1 hover:bg-[rgba(250,173,20,0.1)]"
        @click="handleQuitWithoutFrpc"
      >
        <div class="menu-icon flex h-5 w-5 items-center justify-center opacity-80">
          <n-icon :component="PowerOutline" size="18" />
        </div>
        <span class="menu-label flex-1 text-[13px]">退出但不关闭FRPC</span>
      </div>

      <div
        class="menu-item flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2.5 text-[#ff4d4f] transition-[transform,background-color] duration-200 hover:translate-x-1 hover:bg-[rgba(255,77,79,0.1)]"
        @click="handleQuit"
      >
        <div class="menu-icon flex h-5 w-5 items-center justify-center opacity-80">
          <n-icon :component="ExitOutline" size="18" />
        </div>
        <span class="menu-label flex-1 text-[13px]">完全退出</span>
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

