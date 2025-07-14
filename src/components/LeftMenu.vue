<template>
  <div class="left-menu-wrapper">
    <div
      class="menu-scroll-area"
      :style="{ height: menuAreaHeight + 'px', overflow: 'hidden' }"
    >
      <NMenu
        :collapsed-width="64"
        :collapsed-icon-size="24"
        :options="mainMenuOptions"
        :value="selectedKey"
        :icon-size="22"
        @update:value="handleMenuSelect"
        style="user-select: none"
        :default-expanded-keys="defaultExpandedKeys"
        :scrollbar-props="{ style: { display: 'none' } }"
      />
    </div>
    <n-divider />
    <div class="bottom-menu" style="margin-top: 10px">
      <NMenu
        :collapsed-width="64"
        :collapsed-icon-size="24"
        :options="bottomMenuOptions"
        :value="selectedKey"
        :icon-size="22"
        @update:value="handleMenuSelect"
        style="user-select: none"
        :scrollbar-props="{ style: { display: 'none' } }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { NMenu, NDivider, useMessage, useDialog } from "naive-ui";
import { useRouter } from "vue-router";
import { getMenuOptions, defaultExpandedKeys } from "../shared/menuOptions.ts";
import type { MenuOption } from "../types/menu";
import { invoke } from "@tauri-apps/api/core";
import { emitTo } from "@tauri-apps/api/event";
import { window as tauriWindow } from "@tauri-apps/api";

const emit = defineEmits(["select"]);
const router = useRouter();
const message = useMessage();
const dialog = useDialog();
const allMenuOptions = getMenuOptions();
const mainMenuOptions = allMenuOptions.filter(
  (opt) =>
    typeof opt.key === "string" && !["user", "settings"].includes(opt.key),
);
const bottomMenuOptions = allMenuOptions.filter(
  (opt) =>
    typeof opt.key === "string" && ["user", "settings"].includes(opt.key),
);

const menuAreaHeight = ref(0);

const calcMenuHeight = async () => {
  const currentWindow = tauriWindow.Window.getCurrent();
  const size = await currentWindow.innerSize();
  // 这里假设底部菜单+分割线高度为 110px，可根据实际情况微调
  menuAreaHeight.value = size.height - 195;
};

onMounted(() => {
  calcMenuHeight();
  window.addEventListener("resize", calcMenuHeight);
});
onBeforeUnmount(() => {
  window.removeEventListener("resize", calcMenuHeight);
});

// 管理员权限检查函数
const checkAdminPermission = async (): Promise<boolean> => {
  try {
    const isAdmin = await invoke<boolean>("is_admin");
    return isAdmin;
  } catch (e) {
    console.error("管理员权限检测失败:", e);
    return false;
  }
};

// 处理虚拟网络菜单点击
const handleNetworkMenuClick = async (): Promise<boolean> => {
  const isAdmin = await checkAdminPermission();
  if (!isAdmin) {
    dialog.warning({
      title: "需要管理员权限",
      content: "虚拟网络功能需要以管理员权限运行，是否以管理员权限重启？",
      positiveText: "以管理员权限重启",
      negativeText: "取消",
      onPositiveClick: async () => {
        try {
          await emitTo("main", "request_admin");
        } catch (e) {
          message.error("重启失败，请手动以管理员权限运行");
        }
      },
      onNegativeClick: () => {
        // 用户取消，不做任何操作
      },
    });
    return false; // 阻止默认导航
  }

  // 有管理员权限，正常导航
  router.push("/dashboard/network");
  return true;
};

const handleMenuSelect = async (key: string, _option: MenuOption) => {
  // 递归查找选中的菜单项
  function findOption(
    options: MenuOption[],
    key: string,
  ): MenuOption | undefined {
    for (const opt of options) {
      if (opt.key === key) return opt;
      if (opt.children) {
        const found = findOption(opt.children as MenuOption[], key);
        if (found) return found;
      }
    }
    return undefined;
  }
  const opt = findOption(allMenuOptions, key);
  if (!opt) return;

  if (opt.key === "network") {
    // 只有校验通过并且页面切换后才激活菜单
    const ok = await handleNetworkMenuClick();
    if (ok) {
      selectedKey.value = key;
      emit("select");
    }
  } else if (opt.link) {
    router.push(opt.link);
    selectedKey.value = key;
    emit("select");
  }
};

const selectedKey = ref("dashboardIndex");
</script>

<style scoped>
.left-menu-wrapper {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.menu-scroll-area {
  flex: 1 1 auto;
  min-height: 0;
  overflow-y: hidden !important;
}
.menu-scroll-area::-webkit-scrollbar {
  display: none !important;
  width: 0 !important;
  height: 0 !important;
  background: transparent !important;
}
.menu-scroll-area::-webkit-scrollbar-thumb {
  background: #e5e5e5;
  border-radius: 3px;
}
.menu-scroll-area::-webkit-scrollbar-track {
  background: #fff;
}
.bottom-menu {
  flex-shrink: 0;
  padding-bottom: 14px;
}
.n-divider {
  flex-shrink: 0;
  margin: 0 12px;
}
</style>
