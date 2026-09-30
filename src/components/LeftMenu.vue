<template>
  <div class="left-menu-wrapper h-full overflow-hidden">
    <div class="left-menu-container flex h-full flex-col overflow-hidden">
      <!-- 主菜单区域 -->
      <div
        class="menu-main-area flex-1 overflow-x-hidden overflow-y-auto [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:hidden [&_.n-menu-item]:transition-colors [&_.n-menu-item]:duration-200"
      >
        <NMenu
          :collapsed-width="64"
          :collapsed-icon-size="24"
          :options="menuOptions"
          :value="selectedKey"
          :icon-size="22"
          @update:value="handleMenuSelect"
          class="select-none"
          :default-expanded-keys="defaultExpandedKeys"
        />
      </div>

      <!-- 用户信息区域（仅在 showUserInfo 模式下显示） -->
      <motion.div
        v-if="showUserInfo"
        class="user-info-section shrink-0 bg-[var(--n-color,#fff)] px-4 py-3"
        :initial="{ opacity: 0, y: 24 }"
        :animate="{ opacity: 1, y: 0 }"
        :transition="{ type: 'spring', stiffness: 220, damping: 24 }"
      >
        <div
          v-if="!collapsed"
          class="divider mb-4 mt-2 h-px bg-[linear-gradient(90deg,transparent,var(--n-border-color,#e0e0e0),transparent)]"
        />
        <div
          class="user-info-content flex items-center gap-3"
          :class="collapsed ? 'justify-center py-2' : ''"
        >
          <motion.div
            class="user-avatar shrink-0"
            :while-hover="{ scale: 1.1, rotate: 4 }"
            :transition="{ type: 'spring', stiffness: 300, damping: 18 }"
          >
            <img
              v-if="userAvatar"
              :src="userAvatar"
              alt="用户头像"
              class="avatar-img rounded-full border-2 border-[var(--n-border-color,#e0e0e0)] object-cover"
              :class="collapsed ? 'h-9 w-9' : 'h-10 w-10'"
            />
            <div
              v-else
              class="avatar-placeholder flex items-center justify-center rounded-full bg-[var(--n-tag-color,#f5f5f5)] text-[var(--n-text-color-3,#999)]"
              :class="collapsed ? 'h-9 w-9' : 'h-10 w-10'"
            >
              <n-icon :component="PersonOutline" size="24" />
            </div>
          </motion.div>
          <div
            v-if="!collapsed"
            class="user-details min-w-0 flex-1 overflow-hidden"
          >
            <div
              class="user-name truncate text-sm font-semibold text-[var(--n-text-color,#333)]"
            >
              {{ userNickname }}
            </div>
            <div
              class="user-email mt-0.5 truncate text-xs text-[var(--n-text-color-3,#999)]"
            >
              {{ userEmail }}
            </div>
          </div>
          <n-button
            v-if="!collapsed"
            quaternary
            circle
            size="small"
            class="logout-btn shrink-0 opacity-70 transition-opacity duration-200 hover:text-[var(--n-error-color,#d03050)]! hover:opacity-100"
            @click="handleLogout"
          >
            <template #icon>
              <n-icon :component="LogOutOutline" size="16" />
            </template>
          </n-button>
        </div>
      </motion.div>

      <!-- 简洁模式退出登录 -->
      <motion.div
        v-else
        class="logout-section shrink-0 px-4 py-3"
        :initial="{ opacity: 0, y: 24 }"
        :animate="{ opacity: 1, y: 0 }"
        :transition="{ type: 'spring', stiffness: 220, damping: 24 }"
      >
        <div
          v-if="!collapsed"
          class="divider mb-3 h-px bg-[linear-gradient(90deg,transparent,var(--n-border-color,#e0e0e0),transparent)]"
        />
        <n-button
          quaternary
          block
          size="medium"
          class="logout-btn opacity-80 transition-opacity duration-200 hover:opacity-100"
          @click="handleLogout"
        >
          <template #icon>
            <n-icon :component="LogOutOutline" size="18" />
          </template>
          <span v-if="!collapsed">退出登录</span>
        </n-button>
      </motion.div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { NMenu, NButton, NIcon, useDialog, useMessage } from "naive-ui";
import { motion } from "motion-v";
import { useRouter, useRoute } from "vue-router";
import { getMenuOptions, defaultExpandedKeys } from "../shared/menuOptions.ts";
import type { MenuOption } from "../types/menu";
import { PersonOutline, LogOutOutline } from "@vicons/ionicons5";
import { userApi } from "../net";
import { accessHandle, removeToken } from "../net/base.ts";

// 定义 props
interface Props {
  showUserInfo?: boolean;
  collapsed?: boolean;
}

withDefaults(defineProps<Props>(), {
  showUserInfo: false,
  collapsed: false,
});

const emit = defineEmits(["select"]);
const router = useRouter();
const dialog = useDialog();
const message = useMessage();
const menuOptions: MenuOption[] = getMenuOptions();

// 从 localStorage 获取用户信息
const userNickname = computed(
  () => localStorage.getItem("nickname") || "未登录",
);
const userEmail = computed(() => localStorage.getItem("email") || "");
const userAvatar = computed(() => localStorage.getItem("avatar") || "");

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
  const opt = findOption(menuOptions, key);
  router.push(opt.link);
  selectedKey.value = key;
  emit("select");
};

const handleLogout = () => {
  dialog.warning({
    title: "提示",
    content: "确定要退出登录吗？",
    positiveText: "确定",
    negativeText: "取消",
    onPositiveClick: () => {
      if (localStorage.getItem("isDeepLinkLogin") !== "true") {
        userApi.post("/auth/logout", {}, accessHandle(), () => {});
      }
      removeToken();
      message.success("已退出登录");
      emit("select");
      router.push({ name: "login" });
    },
  });
};

const selectedKey = ref("dashboardIndex");

const route = useRoute();

watch(
  () => route.path,
  (newPath) => {
    // 递归查找当前路由对应的菜单 key
    function findKeyByLink(
      options: MenuOption[],
      link: string,
    ): string | undefined {
      for (const opt of options as MenuOption[]) {
        if (opt.link === link) return opt.key as string;
        if (opt.children) {
          const found = findKeyByLink(opt.children as MenuOption[], link);
          if (found) return found;
        }
      }
      return undefined;
    }
    const key = findKeyByLink(menuOptions, newPath);
    if (key) {
      selectedKey.value = key;
    }
  },
  { immediate: true },
);
</script>
