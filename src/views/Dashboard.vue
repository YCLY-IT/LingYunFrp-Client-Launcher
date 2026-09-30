<script setup lang="ts">
import { darkTheme } from "naive-ui";
import { ref, computed, onMounted, onUnmounted } from "vue";
import { RouterView } from "vue-router";
import { AnimatePresence, motion } from "motion-v";
import LeftMenu from "../components/LeftMenu.vue";
import TopMenu from "../components/TopMenu.vue";
import { useThemeStore } from "../stores/theme";

const themeStore = useThemeStore();
const collapsed = ref(false);
const isMobile = ref(window.innerWidth <= 768);

// 侧边栏显示模式：从 themeStore 读取
const showUserInfoInSidebar = computed(() => themeStore.sidebarUserInfoMode);

const handleResize = () => {
  isMobile.value = window.innerWidth <= 768;
};

onMounted(() => {
  window.addEventListener("resize", handleResize);
});

onUnmounted(() => {
  window.removeEventListener("resize", handleResize);
});

defineExpose({
  darkTheme,
});
</script>

<template>
  <div class="h-full">
    <NLayout position="absolute">
      <NLayoutHeader bordered class="h-16! p-0!">
        <TopMenu />
      </NLayoutHeader>
      <NLayout has-sider position="absolute" class="top-16!">
        <NLayoutSider
          v-if="!isMobile"
          bordered
          collapse-mode="width"
          :collapsed-width="64"
          :width="240"
          :collapsed="collapsed"
          :native-scrollbar="true"
          show-trigger
          class="h-[calc(100vh-64px)]!"
          @update:collapsed="collapsed = $event"
        >
          <LeftMenu
            :show-user-info="showUserInfoInSidebar"
            :collapsed="collapsed"
          />
        </NLayoutSider>
        <NLayout :native-scrollbar="false">
          <NLayoutContent class="min-h-[calc(100vh-64px)] p-4! md:p-6!">
            <RouterView v-slot="{ Component, route }">
              <AnimatePresence mode="wait">
                <motion.div
                  :key="route.path"
                  :initial="{ opacity: 0, y: 16, scale: 0.985 }"
                  :animate="{ opacity: 1, y: 0, scale: 1 }"
                  :exit="{ opacity: 0, y: -16, scale: 0.985 }"
                  :transition="{ duration: 0.28, ease: [0.4, 0, 0.2, 1] }"
                >
                  <component :is="Component" />
                </motion.div>
              </AnimatePresence>
            </RouterView>
          </NLayoutContent>
        </NLayout>
      </NLayout>
    </NLayout>
  </div>
</template>
