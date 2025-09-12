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
        <h2
          style="
            margin-left: 20px;
            background: transparent;
            -webkit-background-clip: text;
            color: transparent;
            background-image: linear-gradient(120deg, #84fab0 0%, #8fd3f4 100%);
          "
        >
          {{ packageData.title }}
        </h2>
      </div>
      <div
        class="right"
        style="transform: translateX(-30px); text-align: center"
      >
        <div class="the-right" style="margin-right: 5px">
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
        <div class="theme-switch">
          <!-- 客户端关闭按钮、全屏、刷新和最小化按钮 -->
          <NButton text @click="handleToRefresh">
            <NIcon size="20">
              <RefreshOutline />
            </NIcon>
          </NButton>
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
          <NButton text @click="ToClose = true">
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
import { h, ref, inject, computed, Ref, onMounted, onUnmounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import {
  NLayoutHeader,
  NIcon,
  NButton,
  NDropdown,
  useDialog,
  NSwitch,
  NPopover,
  NMenu,
  NDrawer,
  NDrawerContent,
  NScrollbar,
  NModal,
  useMessage,
} from "naive-ui";
import {
  PersonCircleOutline,
  LogOutOutline,
  SunnyOutline,
  MoonOutline,
  MenuOutline,
  CloseOutline,
  ScanOutline,
  RemoveOutline,
  RefreshOutline,
} from "@vicons/ionicons5";
import { switchButtonRailStyle } from "../constants/theme.ts";
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

// 从 localStorage 获取头像链接
avatarUrl.value = localStorage.getItem("avatar") || "";

// 注入主题相关函数
const { isDarkMode, toggleTheme } = inject("theme") as {
  isDarkMode: Ref<boolean>;
  toggleTheme: () => void;
};

// 渲染下拉菜单中的主题切换选项
const renderThemeOption = () => {
  return h(
    "div",
    {
      style:
        "display: flex; align-items: center; padding: 8px 12px; height: 20px;",
    },
    [
      h(
        "span",
        {
          style: "flex: 1; margin-right: 12px; font-size: 14px;",
        },
        "主题切换",
      ),
      h(
        NSwitch,
        {
          value: isDarkMode.value,
          "onUpdate:value": handleThemeChange,
          railStyle: switchButtonRailStyle,
          size: "small",
        },
        {
          checked: () => h(NIcon, null, { default: () => h(MoonOutline) }),
          unchecked: () => h(NIcon, null, { default: () => h(SunnyOutline) }),
        },
      ),
    ],
  );
};

const options = [
  {
    key: "theme",
    type: "render",
    render: renderThemeOption,
  },
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

// 处理主题切换
const handleThemeChange = () => {
  toggleTheme();
  localStorage.setItem("theme", isDarkMode.value ? "dark" : "light");
};

function userLogout() {
  userApi.get("/user/logout", accessHandle(), () => {});
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
  ToClose.value = false;
  await invoke("hide_to_tray");
};
const handleResize = () => {
  isMobile.value = window.innerWidth <= 768;
};

onMounted(() => {
  window.addEventListener("resize", handleResize);
});

onUnmounted(() => {
  window.removeEventListener("resize", handleResize);
});
</script>

<style lang="scss" scoped>
.avatar {
  --size: 36px;
  width: var(--size);
  height: var(--size);
  border-radius: 50%;
  transform: translateY(-8px) translateX(-12px);
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
