<template>
  <!-- PC端导航栏 -->
  <NLayoutHeader
    bordered
    class="navbar pc-navbar drag-bar"
    style="user-select: none"
  >
    <div class="navbar-content">
      <div class="logo">
        <RouterLink to="/" class="logo-link">
          <h2
            style="
              background: transparent;
              -webkit-background-clip: text;
              color: transparent;
              background-image: linear-gradient(
                -225deg,
                #7de2fc 0%,
                #b9b6e5 100%
              );
            "
          >
            {{ packageData.title }}
          </h2>
        </RouterLink>
      </div>

      <!-- 桌面端菜单 -->
      <div class="window-controls">
        <NSpace>
          <NButton
            quaternary
            circle
            size="small"
            @click="toggleTheme"
            class="theme-toggle-btn no-drag"
          >
            <NIcon size="19" :component="isDarkMode ? Sunny : Moon" />
          </NButton>
          <n-button
            quaternary
            circle
            size="small"
            class="no-drag"
            @click="handleToRefresh"
          >
            <NIcon size="20"><RefreshOutline /></NIcon>
          </n-button>
          <NButton
            quaternary
            circle
            size="small"
            class="no-drag"
            @click="handleToMinimize"
          >
            <NIcon size="23"><RemoveOutline /></NIcon>
          </NButton>
          <NButton
            quaternary
            circle
            size="small"
            class="no-drag"
            @click="handleToMaximize"
          >
            <NIcon size="20"><ScanOutline /></NIcon>
          </NButton>
          <NButton
            quaternary
            circle
            size="small"
            class="no-drag"
            @click="ToShow = true"
          >
            <NIcon size="23"><CloseOutline /></NIcon>
          </NButton>
        </NSpace>
      </div>
    </div>
  </NLayoutHeader>

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
import {
  Moon,
  Sunny,
  RemoveOutline,
  ScanOutline,
  CloseOutline,
  RefreshOutline,
} from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/core";

const ToShow = ref(false);
const { isDarkMode, toggleTheme } = inject("theme", {
  isDarkMode: ref(false),
  toggleTheme: () => {},
}) as {
  isDarkMode: Ref<boolean>;
  toggleTheme: () => void;
};

const handleToRefresh = () => {
  window.location.reload();
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
</script>

<style lang="scss" scoped>
@use "../assets/styles/components/homeMenu.scss" as *;

.theme-toggle-btn {
  transition: all 0.3s ease;
  &:hover {
    transform: rotate(30deg);
    background-color: var(--n-color-hover);
  }
  .n-icon {
    transition: all 0.3s ease;
  }
}

.drag-bar {
  -webkit-app-region: drag;
}
.no-drag {
  -webkit-app-region: no-drag;
}

.navbar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.window-controls {
  display: flex;
  transform: translateY(2px) scale(1.15);
}
</style>
