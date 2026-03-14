<template>
  <div class="left-menu-wrapper">
    <div class="left-menu-container" :class="{ 'compact-mode': !showUserInfo }">
      <!-- 主菜单区域 -->
      <div class="menu-main-area">
        <NMenu
          :collapsed-width="64"
          :collapsed-icon-size="24"
          :options="menuOptions"
          :value="selectedKey"
          :icon-size="22"
          @update:value="handleMenuSelect"
          style="user-select: none"
          :default-expanded-keys="defaultExpandedKeys"
        />
      </div>

      <!-- 用户信息区域（仅在 showUserInfo 模式下显示） -->
      <div v-if="showUserInfo" class="user-info-section">
        <div class="divider"></div>
        <div class="user-info-content">
          <div class="user-avatar">
            <img
              v-if="userAvatar"
              :src="userAvatar"
              alt="用户头像"
              class="avatar-img"
            />
            <div v-else class="avatar-placeholder">
              <n-icon :component="PersonOutline" size="24" />
            </div>
          </div>
          <div class="user-details">
            <div class="user-name">{{ userNickname }}</div>
            <div class="user-email">{{ userEmail }}</div>
          </div>
          <n-button
            quaternary
            circle
            size="small"
            class="settings-btn"
            @click="goToSettings"
          >
            <template #icon>
              <n-icon :component="SettingsOutline" size="16" />
            </template>
          </n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { NMenu, NButton, NIcon } from "naive-ui";
import { useRouter, useRoute } from "vue-router";
import { getMenuOptions, defaultExpandedKeys } from "../shared/menuOptions.ts";
import type { MenuOption } from "../types/menu";
import { PersonOutline, SettingsOutline } from "@vicons/ionicons5";

// 定义 props
interface Props {
  showUserInfo?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showUserInfo: false,
});

const emit = defineEmits(["select"]);
const router = useRouter();
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

const goToSettings = () => {
  router.push("/dashboard/settings");
  emit("select");
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

<style scoped lang="scss">
.left-menu-wrapper {
  height: 100%;
}

.left-menu-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.menu-main-area {
  flex: 1;
  overflow-y: auto;
}

.user-info-section {
  flex-shrink: 0;
  padding: 12px 16px;
  background: var(--n-color, #fff);

  .divider {
    height: 1px;
    background: linear-gradient(
      90deg,
      transparent,
      var(--n-border-color, #e0e0e0),
      transparent
    );
    margin-bottom: 16px;
    margin-top: 8px;
  }

  .user-info-content {
    display: flex;
    align-items: center;
    gap: 12px;

    .user-avatar {
      flex-shrink: 0;

      .avatar-img {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        object-fit: cover;
        border: 2px solid var(--n-border-color, #e0e0e0);
      }

      .avatar-placeholder {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        background: var(--n-tag-color, #f5f5f5);
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--n-text-color-3, #999);
      }
    }

    .user-details {
      flex: 1;
      min-width: 0;
      overflow: hidden;

      .user-name {
        font-size: 14px;
        font-weight: 600;
        color: var(--n-text-color, #333);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      .user-email {
        font-size: 12px;
        color: var(--n-text-color-3, #999);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        margin-top: 2px;
      }
    }

    .settings-btn {
      flex-shrink: 0;
      opacity: 0.7;
      transition: opacity 0.2s;

      &:hover {
        opacity: 1;
      }
    }
  }
}

// 折叠状态下的样式调整
:deep(.n-menu--collapsed) {
  & + .user-info-section {
    .user-info-content {
      justify-content: center;

      .user-details,
      .settings-btn {
        display: none;
      }

      .user-avatar {
        .avatar-img,
        .avatar-placeholder {
          width: 36px;
          height: 36px;
        }
      }
    }
  }
}
</style>
