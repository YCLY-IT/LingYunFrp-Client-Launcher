<template>
  <NLayoutHeader
    bordered
    position="absolute"
    style="height: 64px; z-index: 999; user-select: none"
  >
    <!-- 确保拖动区域覆盖整个header -->
    <div class="header-content" data-tauri-drag-region>
      <div class="left">
        <NPopover
          trigger="click"
          placement="bottom-start"
          :show="showMenu"
          @update:show="showMenu = $event"
        >
          <template #trigger>
            <NButton text class="menu-trigger">
              <NIcon size="24">
                <MenuOutline />
              </NIcon>
            </NButton>
          </template>
          <div class="mobile-menu">
            <NScrollbar style="max-height: 500px">
              <NMenu
                :options="menuOptions"
                :value="currentKey"
                @update:value="handleMenuSelect"
                :default-expanded-keys="defaultExpandedKeys"
              />
            </NScrollbar>
          </div>
        </NPopover>
        <h2 :style="{ marginLeft: '20px', color: themeStore.primaryColor }">
          {{ packageData.title }}
        </h2>
      </div>
      <div
        class="right"
        style="transform: translateX(-30px); text-align: center"
      >
        <div class="the-right" style="margin-right: 5px">
          <n-button
            quaternary
            style="font-size: 18px; transform: translateX(-30px)"
            @click="ThemeSwitcherDrawer('right')"
          >
            <n-icon
              :component="SettingsOutline"
              style="cursor: pointer"
            ></n-icon>
          </n-button>
          <NDropdown
            style="margin-top: 12px"
            :options="options"
            @select="handleUserMenuSelect"
            trigger="hover"
          >
            <NButton text>
              <template #icon>
                <NIcon>
                  <div class="avatar">
                    <img :src="avatarUrl" alt="avatar" />
                  </div>
                </NIcon>
              </template>
              <span class="nikename">{{ nickname }}</span>
            </NButton>
          </NDropdown>
        </div>
        <n-button
          quaternary
          circle
          size="small"
          @click="handleThemeToggle"
          class="theme-toggle-btn"
          style="transform: translateX(-3px)"
        >
          <NIcon
            size="20"
            :component="themeStore.theme === 'dark' ? Sunny : Moon"
          />
        </n-button>
        <div class="theme-switch">
          <!-- 客户端关闭按钮、全屏和最小化按钮 -->
          <NButton text @click="handleToMinimize" style="margin-right: 2px">
            <NIcon size="20">
              <RemoveOutline />
            </NIcon>
          </NButton>
          <NButton text style="margin-right: 2px" @click="handleToMaximize">
            <NIcon size="20">
              <ScanOutline />
            </NIcon>
          </NButton>
          <NButton text @click="handleCloseButtonClick">
            <NIcon size="20">
              <CloseOutline />
            </NIcon>
          </NButton>
        </div>
      </div>
    </div>
  </NLayoutHeader>

  <!-- 移动端菜单抽屉 -->
  <NDrawer v-model:show="showMobileMenu" :width="280" placement="left">
    <NDrawerContent title="菜单">
      <LeftMenu @select="showMobileMenu = false" />
    </NDrawerContent>
  </NDrawer>

  <NModal v-model:show="ToClose" preset="dialog" style="width: 400px">
    <template #header> 你确定要关闭吗? </template>
    这样会关闭所有隧道, 你也可以同样点击右上角的X图标来关闭当前弹窗。
    <br />
    <div style="margin-top: 15px">
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
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import {
  NLayoutHeader,
  NIcon,
  NButton,
  NDropdown,
  useDialog,
  NPopover,
  NMenu,
  NDrawer,
  NDrawerContent,
  NScrollbar,
  NModal,
  useMessage,
  DrawerPlacement,
  NCheckbox,
} from "naive-ui";
import {
  PersonCircleOutline,
  LogOutOutline,
  Sunny,
  Moon,
  SettingsOutline,
  MenuOutline,
  CloseOutline,
  ScanOutline,
  RemoveOutline,
} from "@vicons/ionicons5";
import {
  getMenuOptions,
  renderIcon,
  defaultExpandedKeys,
} from "../shared/menuOptions.ts";
import LeftMenu from "./LeftMenu.vue";
import { userApi } from "../net";
import { accessHandle, removeToken } from "../net/base.ts";
import { invoke } from "@tauri-apps/api/core";
import type { MenuOption } from "../types/menu";
import { useThemeStore } from "../stores/theme.ts";
import { useThemeTransition } from "../composables/useThemeTransition.ts";

const router = useRouter();
const route = useRoute();
const showMenu = ref(false);
const menuOptions = ref(getMenuOptions());
const dialog = useDialog();
const message = useMessage();
const showMobileMenu = ref(false);
const ToClose = ref(false);
const isMobile = ref(window.innerWidth <= 768);
const nickname = ref("");
const avatarUrl = ref("");
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

// 从 localStorage 获取头像链接
avatarUrl.value = localStorage.getItem("avatar") || "";

const options = [
  {
    label: "个人资料",
    key: "profile",
    icon: renderIcon(PersonCircleOutline),
  },
  {
    label: "退出登录",
    key: "logout",
    icon: renderIcon(LogOutOutline),
  },
];

function userLogout() {
  if (localStorage.getItem("isDeepLinkLogin") !== "true") {
    userApi.post("/auth/logout", {}, accessHandle(), () => {});
  }
  removeToken();
  router.push({ name: "login" });
}

const handleUserMenuSelect = (key: string) => {
  switch (key) {
    case "logout":
      dialog.warning({
        title: "提示",
        content: "确定要退出登录吗？",
        positiveText: "确定",
        negativeText: "取消",
        onPositiveClick: () => {
          userLogout();
          message.success("已退出登录");
          router.push("/login");
        },
      });
      break;
    case "profile":
      router.push("/dashboard/user/my-profile");
      break;
    case "home":
      router.push("/");
      break;
  }
};
const handleMenuSelect = async (_: any, item: MenuOption) => {
  // 检查是否有自定义的 onClick 处理函数
  if (item.onClick) {
    if (typeof item.onClick === "function") {
      const result = await item.onClick();
      if (result === false) {
        // 如果返回 false，表示阻止默认导航
        return;
      }
    }
  } else if (item.link) {
    // 默认导航行为
    router.push(item.link as string);
  }

  showMenu.value = false;
};

const currentKey = computed(() => {
  const key = route.path.replace("/dashboard/", "").replace("/", "-");
  if (key === "home") return "dashboardIndex";
  return key;
});

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

<style lang="scss" scoped>
:deep(.theme-toggle-btn) {
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;

  &:hover {
    transform: rotate(30deg) scale(1.1);
    background-color: var(--n-color-hover);
  }

  &:active {
    transform: rotate(30deg) scale(0.95);
  }

  .n-icon {
    transition: all 0.3s ease;
  }
}

.avatar {
  --size: 35px;
  width: var(--size);
  height: var(--size);
  border-radius: 25%;
  transform: translateY(-8px) translateX(-23px);
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}
.nikename {
  margin-left: 3px;
  max-width: 70px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.header-content {
  /* 确保拖动区域有足够的面积 */
  height: 100%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  -webkit-app-region: drag;
}

/* 确保按钮等可交互元素不被拖动区域覆盖 */
.n-button,
.n-popover,
.n-dropdown {
  -webkit-app-region: no-drag;
}

.theme-switch {
  gap: 2px;

  /* 每个按钮统一尺寸 */
  .n-button {
    --size: 36px;
    width: var(--size);
    height: var(--size);
    padding: 0;
    align-items: center;
    justify-content: center;
    background: transparent;
    transition: background 0.15s;
    border-radius: 10px;
    & > span {
      color: inherit;
    }

    /* 默认悬停：浅灰 */
    &:hover:not(:last-of-type) {
      background: rgba(128, 128, 128, 0.15);
      border-radius: 10px;
    }

    /* 最后一个按钮 = 关闭按钮，悬停红色 */
    &:last-of-type:hover {
      border-radius: 10px;
      background: #e81123;
    }
  }
}
</style>
