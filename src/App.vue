<template>
  <Suspense>
    <NConfigProvider :theme="theme" :theme-overrides="themeOverrides">
      <NDialogProvider>
        <NMessageProvider>
          <NNotificationProvider>
            <NLoadingBarProvider>
              <AppContent />
              <CustomContextMenu ref="contextMenuRef" />

              <NModal
                v-model:show="updateModalVisible"
                :mask-closable="false"
                :closable="false"
              >
                <NSpace vertical style="padding: 24px; min-width: 400px">
                  <NText style="font-size: 18px; font-weight: bold"
                    >正在更新应用</NText
                  >
                  <NText>{{ updateStatus }}</NText>
                  <NProgress
                    type="line"
                    :percentage="updateProgress"
                    :show-indicator="true"
                  />
                  <NSpace justify="space-between">
                    <NText style="font-size: 12px; color: #999">
                      已下载:
                      {{ (updateDownloaded / 1024 / 1024).toFixed(2) }} MB
                    </NText>
                    <NText style="font-size: 12px; color: #999">
                      总大小:
                      {{
                        updateTotal > 0
                          ? (updateTotal / 1024 / 1024).toFixed(2) + " MB"
                          : "未知"
                      }}
                    </NText>
                  </NSpace>
                </NSpace>
              </NModal>
            </NLoadingBarProvider>
          </NNotificationProvider>
        </NMessageProvider>
      </NDialogProvider>
    </NConfigProvider>
  </Suspense>
  <svg class="defs-only" aria-hidden="true">
    <defs>
      <filter id="colorblind" color-interpolation-filters="sRGB">
        <feColorMatrix
          type="matrix"
          values="0.567 0.433 0 0 0 0.558 0.442 0 0 0 0 0.242 0.758 0 0 0 0 0 1 0"
        />
      </filter>
    </defs>
  </svg>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { initLogService } from "./utils/log.ts";

initLogService();

import {
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NNotificationProvider,
  NLoadingBarProvider,
  NModal,
  NSpace,
  NText,
  NProgress,
} from "naive-ui";
import AppContent from "./components/AppContent.vue";
import CustomContextMenu from "./components/CustomContextMenu.vue";
import { useAppInitialization } from "./composables/useAppInitialization";

const {
  theme,
  themeOverrides,
  updateModalVisible,
  updateProgress,
  updateStatus,
  updateDownloaded,
  updateTotal,
  initializeApp,
  cleanup,
} = useAppInitialization();

onMounted(() => {
  initializeApp();
});

onUnmounted(() => {
  cleanup();
});
</script>

<style lang="scss">
@use "./assets/styles/transitions.scss";
@use "./assets/styles/index.scss";
input,
textarea,
select {
  font-size: 16px !important;
}

@media screen and (max-width: 768px) {
  input,
  textarea,
  select {
    font-size: 16px !important;
  }
}
#app {
  font-family: "Lato", "Fira Code", sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 确保SVG滤镜不占用空间 */
.defs-only {
  position: absolute;
  width: 0;
  height: 0;
  overflow: hidden;
  pointer-events: none;
}
</style>
