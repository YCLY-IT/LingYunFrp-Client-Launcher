<template>
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
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { NMenu } from "naive-ui";
import { useRouter, useRoute } from "vue-router";
import { getMenuOptions, defaultExpandedKeys } from "../shared/menuOptions.ts";
import type { MenuOption } from "../types/menu";

const emit = defineEmits(["select"]);
const router = useRouter();
const menuOptions: MenuOption[] = getMenuOptions();

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
