<!--
  Copyright (c) 2025, TechCat-Team
  本部分代码来源于：https://github.com/TechCat-Team/ChmlFrp-Panel-v3/blob/main/src/components/Layout/ThemeSwitcher.vue
  遵循 Apache 2.0 许可证
  修改者：eefen
  修改时间：2025-12-05
-->

<template>
  <div class="theme-switcher h-full [&_.n-scrollbar]:h-full [&_.n-scrollbar]:overflow-visible">
    <n-scrollbar
      class="container-scrollbar"
      :vertical-rail-style="{ right: '-15px' }"
    >
      <div class="container flex flex-col gap-4 py-2 [&>.n-card]:animate-rise-in [&>:nth-child(1)]:[animation-delay:0ms] [&>:nth-child(2)]:[animation-delay:80ms] [&>:nth-child(3)]:[animation-delay:160ms] [&>:nth-child(4)]:[animation-delay:240ms] [&>:nth-child(5)]:[animation-delay:320ms] [&>:nth-child(6)]:[animation-delay:400ms]">
        <!-- 主题设置卡片 -->
        <n-card class="setting-card transition-all duration-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]" size="small">
          <template #header>
            <div class="card-header flex items-center gap-2 text-[15px] font-semibold">
              <n-icon :component="ColorPaletteOutline" :size="20" />
              <span>主题设置</span>
            </div>
          </template>
          <div class="setting-content flex flex-col gap-4 py-1">
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon :component="SyncOutline" :size="18" />
                <span>自动切换主题</span>
              </div>
              <n-switch
                size="large"
                v-model:value="isAutoTheme"
                @click="changeTheme"
                :checked-value="true"
                :unchecked-value="false"
              >
                <template #checked>自动切换</template>
                <template #unchecked>手动切换</template>
              </n-switch>
            </div>
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1" v-if="!isAutoTheme">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon
                  :component="isDarkTheme ? Sparkles : Sunny"
                  :size="18"
                />
                <span>主题模式</span>
              </div>
              <n-switch
                size="large"
                v-model:value="isDarkTheme"
                :rail-style="railStyle"
                :checked-value="true"
                :unchecked-value="false"
                @click="changeTheme"
                :loading="isTransitioning"
              >
                <template #checked-icon>
                  <n-icon :component="Sparkles" color="#9f9f9c" />
                </template>
                <template #unchecked-icon>
                  <n-icon :component="Sunny" color="#E6A23C" />
                </template>
                <template #checked>月映万川</template>
                <template #unchecked>日照千里</template>
              </n-switch>
            </div>
          </div>
        </n-card>

        <!-- 主题色设置卡片 -->
        <n-card class="setting-card transition-all duration-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]" size="small">
          <template #header>
            <div class="card-header flex items-center gap-2 text-[15px] font-semibold">
              <n-icon :component="BrushOutline" :size="20" />
              <span>主题色</span>
            </div>
          </template>
          <div class="setting-content flex flex-col gap-4 py-1">
            <div class="color-picker-wrapper mb-3 flex justify-center">
              <n-color-picker
                v-model:value="primaryColor"
                :show-preview="true"
                :modes="['hex']"
                size="large"
              />
            </div>
            <div class="preset-colors grid max-w-full grid-cols-5 justify-center gap-3 py-2 max-md:gap-2.5">
              <motion.div
                v-for="(color, index) in presetColors"
                :key="color"
                :style="{ backgroundColor: color }"
                class="preset-color relative flex h-[25px] w-[25px] cursor-pointer items-center justify-center overflow-hidden rounded-full border-[3px] transition-all duration-300 before:pointer-events-none before:absolute before:inset-0 before:rounded-full before:bg-[linear-gradient(135deg,rgba(255,255,255,0.3),rgba(0,0,0,0.1))] before:opacity-0 before:transition-opacity before:duration-300 max-md:h-8 max-md:w-8"
                :class="
                  primaryColor === color
                    ? 'scale-110 border-[var(--primary-color,#18a058)] shadow-[0_0_0_2px_var(--primary-color,#18a058),0_4px_12px_rgba(0,0,0,0.25)]'
                    : 'border-transparent shadow-[0_2px_8px_rgba(0,0,0,0.15)] hover:shadow-[0_4px_12px_rgba(0,0,0,0.25)] hover:before:opacity-100'
                "
                :initial="{ opacity: 0, scale: 0.4 }"
                :animate="{ opacity: 1, scale: primaryColor === color ? 1.1 : 1 }"
                :transition="{ delay: index * 0.04, type: 'spring', stiffness: 320, damping: 20 }"
                :while-hover="{ scale: 1.18, rotate: 8 }"
                :while-tap="{ scale: 0.9 }"
                @click="setPresetColor(color)"
              >
                <n-icon
                  v-if="primaryColor === color"
                  :component="CheckmarkCircleOutline"
                  :size="16"
                  color="#fff"
                />
              </motion.div>
            </div>
          </div>
        </n-card>

        <!-- 视觉效果设置卡片 -->
        <n-card class="setting-card transition-all duration-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]" size="small">
          <template #header>
            <div class="card-header flex items-center gap-2 text-[15px] font-semibold">
              <n-icon :component="EyeOutline" :size="20" />
              <span>视觉效果</span>
            </div>
          </template>
          <div class="setting-content flex flex-col gap-4 py-1">
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon :component="ColorFilterOutline" :size="18" />
                <span>RGB模式</span>
              </div>
              <n-switch
                size="large"
                v-model:value="isRGBMode"
                :checked-value="true"
                :unchecked-value="false"
              />
            </div>
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon :component="LayersOutline" :size="18" />
                <span>对话框模糊</span>
              </div>
              <n-switch
                size="large"
                v-model:value="isDialogBoxHairGlass"
                :checked-value="true"
                :unchecked-value="false"
              />
            </div>
          </div>
        </n-card>

        <!-- 侧边栏设置卡片 -->
        <n-card class="setting-card transition-all duration-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]" size="small">
          <template #header>
            <div class="card-header flex items-center gap-2 text-[15px] font-semibold">
              <n-icon :component="MenuOutline" :size="20" />
              <span>侧边栏</span>
            </div>
          </template>
          <div class="setting-content flex flex-col gap-4 py-1">
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon :component="PersonOutline" :size="18" />
                <span>经典模式</span>
              </div>
              <n-switch
                size="large"
                v-model:value="sidebarUserInfoMode"
                :checked-value="true"
                :unchecked-value="false"
              >
                <template #checked>经典</template>
                <template #unchecked>简洁</template>
              </n-switch>
            </div>
          </div>
        </n-card>

        <!-- 无障碍设置卡片 -->
        <n-card class="setting-card transition-all duration-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]" size="small">
          <template #header>
            <div class="card-header flex items-center gap-2 text-[15px] font-semibold">
              <n-icon :component="AccessibilityOutline" :size="20" />
              <span>无障碍</span>
            </div>
          </template>
          <div class="setting-content flex flex-col gap-4 py-1">
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon :component="ColorWandOutline" :size="18" />
                <span>色弱模式</span>
              </div>
              <n-switch
                size="large"
                v-model:value="colorBlindMode"
                :checked-value="true"
                :unchecked-value="false"
              />
            </div>
            <div class="setting-item flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
              <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                <n-icon :component="ContrastOutline" :size="18" />
                <span>高对比度模式</span>
              </div>
              <n-switch
                size="large"
                v-model:value="highContrastMode"
                :checked-value="true"
                :unchecked-value="false"
              />
            </div>
          </div>
        </n-card>

        <!-- 背景图设置卡片 -->
        <n-card class="setting-card transition-all duration-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.08)]" size="small">
          <template #header>
            <div class="card-header flex items-center gap-2 text-[15px] font-semibold">
              <n-icon :component="ImageOutline" :size="20" />
              <span>背景图</span>
            </div>
          </template>
          <div class="setting-content flex flex-col gap-4 py-1">
            <div class="background-settings flex w-full flex-col gap-3">
              <n-upload
                :file-list="[]"
                :show-file-list="false"
                accept="image/*"
                @change="handleFileChange"
                :max="1"
              >
                <n-button block type="primary" ghost>
                  <template #icon>
                    <n-icon :component="CloudUploadOutline" />
                  </template>
                  选择本地图片
                </n-button>
              </n-upload>
              <n-input
                v-model:value="backgroundImageUrl"
                placeholder="输入网络图片链接"
                class="background-input mt-0"
                @update:value="handleImageUrlChange"
                clearable
              >
                <template #prefix>
                  <n-icon :component="LinkOutline" />
                </template>
              </n-input>
              <div
                v-if="backgroundImageUrl || backgroundImage"
                class="image-preview mt-2 flex w-full flex-col gap-4"
              >
                <div class="preview-wrapper group relative w-full overflow-hidden rounded-xl shadow-[0_2px_8px_rgba(0,0,0,0.1)] transition-all duration-300 hover:shadow-[0_4px_16px_rgba(0,0,0,0.15)]">
                  <img
                    :src="backgroundImageUrl || backgroundImage"
                    alt="背景预览"
                    class="preview-image block max-h-[180px] w-full object-cover transition-transform duration-300 group-hover:scale-[1.02]"
                  />
                  <div class="preview-overlay absolute inset-0 flex items-start justify-end bg-[linear-gradient(to_bottom,rgba(0,0,0,0.4),transparent)] p-3 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                    <n-button
                      size="small"
                      type="error"
                      @click="clearBackgroundImage"
                    >
                      <template #icon>
                        <n-icon :component="TrashOutline" />
                      </template>
                      清除
                    </n-button>
                  </div>
                </div>
                <div class="slider-control flex w-full flex-col gap-2" v-if="!frostedGlassMode">
                  <div class="slider-label flex items-center gap-1.5 text-[13px] font-medium text-[var(--text-color-2)]">
                    <n-icon :component="LayersOutline" :size="16" />
                    <span>模糊深度: {{ backgroundBlur }}px</span>
                  </div>
                  <n-slider
                    v-model:value="backgroundBlur"
                    :min="0"
                    :max="20"
                    :step="1"
                    @update:value="handleBlurChange"
                  />
                </div>
                <div class="slider-control flex w-full flex-col gap-2" v-if="!frostedGlassMode">
                  <div class="slider-label flex items-center gap-1.5 text-[13px] font-medium text-[var(--text-color-2)]">
                    <n-icon :component="WaterOutline" :size="16" />
                    <span>元素不透明度: {{ backgroundOpacity || 100 }}%</span>
                  </div>
                  <n-slider
                    v-model:value="backgroundOpacity"
                    :min="20"
                    :max="100"
                    :step="1"
                    @update:value="handleOpacityChange"
                  />
                </div>
                <div class="slider-control flex w-full flex-col gap-2" v-else>
                  <div class="slider-label flex items-center gap-1.5 text-[13px] font-medium text-[var(--text-color-2)]">
                    <n-icon :component="WaterOutline" :size="16" />
                    <span>毛玻璃强度: {{ backgroundOpacity }}px</span>
                  </div>
                  <n-slider
                    v-model:value="backgroundOpacity"
                    :min="5"
                    :max="30"
                    :step="1"
                    @update:value="handleOpacityChange"
                  />
                </div>
                <div class="setting-item mt-3 flex items-center justify-between py-2 transition-all duration-200 hover:pl-1">
                  <div class="setting-label flex items-center gap-2 text-sm text-[var(--text-color-1)]">
                    <n-icon :component="LayersOutline" :size="18" />
                    <span>毛玻璃模式</span>
                  </div>
                  <n-switch
                    size="large"
                    v-model:value="frostedGlassMode"
                    :checked-value="true"
                    :unchecked-value="false"
                    @update:value="handleFrostedGlassChange"
                  />
                </div>
              </div>
            </div>
          </div>
        </n-card>
      </div>
    </n-scrollbar>
  </div>
</template>

<script lang="ts" setup>
import { CSSProperties, ref, onMounted, watch } from "vue";
import { motion } from "motion-v";
import { invoke } from "@tauri-apps/api/core";
import {
  readFile,
  writeFile,
  mkdir,
  exists,
  remove,
} from "@tauri-apps/plugin-fs";
import { appDataDir } from "@tauri-apps/api/path";
import { useThemeStore } from "../stores/theme";
import { useThemeTransition } from "../composables/useThemeTransition.ts";
import {
  Sparkles,
  Sunny,
  ColorPaletteOutline,
  SyncOutline,
  BrushOutline,
  EyeOutline,
  ColorFilterOutline,
  LayersOutline,
  AccessibilityOutline,
  ColorWandOutline,
  ContrastOutline,
  ImageOutline,
  CloudUploadOutline,
  LinkOutline,
  TrashOutline,
  WaterOutline,
  CheckmarkCircleOutline,
  MenuOutline,
  PersonOutline,
} from "@vicons/ionicons5";

const themeStore = useThemeStore();
const { toggleThemeWithDualCircle, isTransitioning } = useThemeTransition();
const isDarkTheme = ref(themeStore.theme === "dark");
const primaryColor = ref(themeStore.primaryColor);
const isAutoTheme = ref(themeStore.isAutoTheme);
const isRGBMode = ref(themeStore.isRGBMode);
const isDialogBoxHairGlass = ref(themeStore.isDialogBoxHairGlass);
const backgroundImage = ref(themeStore.backgroundImage);
const backgroundImageUrl = ref(themeStore.backgroundImage);
const backgroundBlur = ref(themeStore.backgroundBlur);
const backgroundOpacity = ref(themeStore.backgroundOpacity || 100);
const colorBlindMode = ref(themeStore.colorBlindMode);
const highContrastMode = ref(themeStore.highContrastMode);
const frostedGlassMode = ref(themeStore.frostedGlassMode);
const frostedGlassIntensity = ref(themeStore.frostedGlassIntensity || 15);
const sidebarUserInfoMode = ref(themeStore.sidebarUserInfoMode);
const isBackgroundLoading = ref(false);
const BACKGROUND_IMAGE_FILENAME = "background_image";

const presetColors = [
  "#18a058",
  "#2080f0",
  "#f5222d",
  "#fa541c",
  "#faad14",
  "#13c2c2",
  "#52c41a",
  "#eb2f96",
  "#722ed1",
  "#2f54eb",
];

const changeTheme = async (event?: MouseEvent) => {
  await toggleThemeWithDualCircle(event, {
    duration: 600,
    easing: "cubic-bezier(0.4, 0, 0.2, 1)",
  });
};

const changePrimaryColor = (color: string) => {
  themeStore.setPrimaryColor(color);
};

const setPresetColor = (color: string) => {
  primaryColor.value = color;
  changePrimaryColor(color);
};

const setAutoTheme = (isAuto: boolean) => {
  isAutoTheme.value = isAuto;
  themeStore.setAutoTheme(isAuto);
};

const setRGBMode = (isRGB: boolean) => {
  isRGBMode.value = isRGB;
  themeStore.setRGBMode(isRGB);
};

const setDialogBoxHairGlass = (isDBH: boolean) => {
  isDialogBoxHairGlass.value = isDBH;
  themeStore.setDialogBoxHairGlass(isDBH);
};

watch(isDialogBoxHairGlass, (newVal) => {
  setDialogBoxHairGlass(newVal);
  if (newVal) {
    document.documentElement.style.setProperty("--modal-filter", "10px");
  } else {
    document.documentElement.style.setProperty("--modal-filter", "0px");
  }
});

watch(isDarkTheme, async () => {
  if (!isAutoTheme.value && !isTransitioning.value) {
    await changeTheme();
  }
});

watch(primaryColor, (newColor) => {
  changePrimaryColor(newColor);
});

watch(isAutoTheme, (newVal) => {
  setAutoTheme(newVal);
  if (newVal) {
    const systemDarkTheme = window.matchMedia("(prefers-color-scheme: dark)");
    isDarkTheme.value = systemDarkTheme.matches;
    changeTheme();
    systemDarkTheme.addEventListener("change", handleSystemThemeChange);
  } else {
    const systemDarkTheme = window.matchMedia("(prefers-color-scheme: dark)");
    systemDarkTheme.removeEventListener("change", handleSystemThemeChange);
  }
});

watch(isRGBMode, (newVal) => {
  setRGBMode(newVal);
});

watch(sidebarUserInfoMode, (newVal) => {
  themeStore.setSidebarUserInfoMode(newVal);
});

const setColorBlindMode = (enabled: boolean) => {
  colorBlindMode.value = enabled;
  themeStore.setColorBlindMode(enabled);
  updateAccessibilityStyles();
};

const setHighContrastMode = (enabled: boolean) => {
  highContrastMode.value = enabled;
  themeStore.setHighContrastMode(enabled);
  updateAccessibilityStyles();
};

const updateAccessibilityStyles = () => {
  const root = document.documentElement;
  if (colorBlindMode.value) {
    // 应用色弱模式滤镜（红绿色盲辅助）
    root.style.setProperty("--color-blind-filter", "url(#colorblind)");
    root.classList.add("color-blind-mode");
  } else {
    root.style.removeProperty("--color-blind-filter");
    root.classList.remove("color-blind-mode");
  }

  if (highContrastMode.value) {
    root.classList.add("high-contrast-mode");
  } else {
    root.classList.remove("high-contrast-mode");
  }
};

const updateFrostedGlassStyle = () => {
  const root = document.documentElement;
  if (frostedGlassMode.value && backgroundImage.value) {
    root.classList.add("frosted-glass-mode");
    root.style.setProperty("--frosted-glass-transition", "all 0.3s ease");
  } else {
    root.classList.remove("frosted-glass-mode");
    root.style.removeProperty("--frosted-glass-transition");
  }
};

const updateElementOpacityStyle = () => {
  const root = document.documentElement;
  if (backgroundImage.value && !frostedGlassMode.value) {
    root.classList.add("element-opacity-mode");
    const opacity = Math.max(20, backgroundOpacity.value || 100);
    root.style.setProperty("--element-opacity", `${opacity / 100}`);
  } else {
    root.classList.remove("element-opacity-mode");
    root.style.removeProperty("--element-opacity");
  }
};

watch(colorBlindMode, (newVal) => {
  setColorBlindMode(newVal);
});

watch(highContrastMode, (newVal) => {
  setHighContrastMode(newVal);
});

const railStyle = ({
  focused,
  checked,
}: {
  focused: boolean;
  checked: boolean;
}) => {
  const style: CSSProperties = {};
  if (checked) {
    style.background = "#000000";
    if (focused) {
      style.boxShadow = "0 0 0 2px #00000040";
    }
  }
  return style;
};

const handleSystemThemeChange = (e: MediaQueryListEvent) => {
  isDarkTheme.value = e.matches;
  changeTheme();
};

// 压缩图片函数 - 返回 Uint8Array 用于文件系统存储
const compressImage = async (
  file: File,
  maxWidth: number = 1920,
  maxHeight: number = 1080,
  quality: number = 0.8,
): Promise<Uint8Array> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => {
      const img = new Image();
      img.onload = () => {
        let width = img.width;
        let height = img.height;

        if (width > maxWidth || height > maxHeight) {
          const ratio = Math.min(maxWidth / width, maxHeight / height);
          width = Math.round(width * ratio);
          height = Math.round(height * ratio);
        }

        const canvas = document.createElement("canvas");
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext("2d");

        if (!ctx) {
          reject(new Error("无法创建 canvas 上下文"));
          return;
        }

        ctx.drawImage(img, 0, 0, width, height);

        canvas.toBlob(
          (blob) => {
            if (!blob) {
              reject(new Error("图片压缩失败"));
              return;
            }
            blob.arrayBuffer().then((buffer) => {
              resolve(new Uint8Array(buffer));
            });
          },
          "image/jpeg",
          quality,
        );
      };
      img.onerror = () => reject(new Error("图片加载失败"));
      img.src = e.target?.result as string;
    };
    reader.onerror = () => reject(new Error("文件读取失败"));
    reader.readAsDataURL(file);
  });
};

const getBackgroundImagePath = async (): Promise<string> => {
  const dataDir = await appDataDir();
  const separator = dataDir.endsWith("/") || dataDir.endsWith("\\") ? "" : "/";
  return `${dataDir}${separator}${BACKGROUND_IMAGE_FILENAME}`;
};

const saveBackgroundToFile = async (imageData: Uint8Array): Promise<string> => {
  const dataDir = await appDataDir();
  const dirExists = await exists(dataDir);
  if (!dirExists) {
    await mkdir(dataDir, { recursive: true });
  }
  const filePath = await getBackgroundImagePath();
  await writeFile(filePath, imageData);
  return filePath;
};

const loadBackgroundFromFile = async (): Promise<string | null> => {
  try {
    const filePath = await getBackgroundImagePath();
    const fileExists = await exists(filePath);
    if (!fileExists) return null;
    const imageData = await readFile(filePath);
    const base64 = btoa(
      new Uint8Array(imageData).reduce(
        (data, byte) => data + String.fromCharCode(byte),
        "",
      ),
    );
    return `data:image/jpeg;base64,${base64}`;
  } catch {
    return null;
  }
};

const deleteBackgroundFile = async (): Promise<void> => {
  try {
    const filePath = await getBackgroundImagePath();
    const fileExists = await exists(filePath);
    if (fileExists) {
      await remove(filePath);
    }
  } catch (error) {
    console.error("删除背景图文件失败:", error);
  }
};

const handleFileChange = async (options: { fileList: any[] }) => {
  const file = options.fileList[0]?.file;
  if (file && file.type.startsWith("image/")) {
    isBackgroundLoading.value = true;
    try {
      const compressedData = await compressImage(file, 1920, 1080, 0.8);
      await saveBackgroundToFile(compressedData);
      const base64Url = await loadBackgroundFromFile();
      if (base64Url) {
        backgroundImageUrl.value = base64Url;
        backgroundImage.value = base64Url;
        themeStore.setBackgroundImage("file://" + BACKGROUND_IMAGE_FILENAME);
        updateBackgroundStyle();
      }
    } catch (error) {
      console.error("图片处理失败:", error);
      try {
        const compressedData = await compressImage(file, 1280, 720, 0.6);
        await saveBackgroundToFile(compressedData);
        const base64Url = await loadBackgroundFromFile();
        if (base64Url) {
          backgroundImageUrl.value = base64Url;
          backgroundImage.value = base64Url;
          themeStore.setBackgroundImage("file://" + BACKGROUND_IMAGE_FILENAME);
          updateBackgroundStyle();
        }
      } catch (fallbackError) {
        console.error("降级压缩也失败:", fallbackError);
      }
    } finally {
      isBackgroundLoading.value = false;
    }
  }
};

const handleImageUrlChange = (url: string) => {
  backgroundImageUrl.value = url;
  if (url && url.trim()) {
    backgroundImage.value = url.trim();
    themeStore.setBackgroundImage(url.trim());
  } else {
    backgroundImage.value = "";
    themeStore.setBackgroundImage("");
  }
  updateBackgroundStyle();
};

const handleBlurChange = (blur: number) => {
  backgroundBlur.value = blur;
  themeStore.setBackgroundBlur(blur);
  updateBackgroundStyle();
};

const handleOpacityChange = async (value: number) => {
  if (frostedGlassMode.value) {
    backgroundOpacity.value = value;
    themeStore.setFrostedGlassIntensity(value);
    document.documentElement.style.setProperty(
      "--frosted-glass-blur",
      `${value}px`,
    );
    try {
      await invoke("emit_event", {
        event: "frosted-glass-intensity-change",
        payload: { intensity: value },
      });
    } catch (e) {
      console.error("毛玻璃强度事件发送失败:", e);
    }
  } else {
    const clampedOpacity = Math.max(20, value);
    backgroundOpacity.value = clampedOpacity;
    themeStore.setBackgroundOpacity(clampedOpacity);
    updateElementOpacityStyle();
  }
};

const handleFrostedGlassChange = async (enabled: boolean) => {
  frostedGlassMode.value = enabled;
  themeStore.setFrostedGlassMode(enabled);
  if (enabled) {
    backgroundOpacity.value = frostedGlassIntensity.value;
    document.documentElement.style.setProperty(
      "--frosted-glass-blur",
      `${frostedGlassIntensity.value}px`,
    );
  } else {
    const savedOpacity = themeStore.backgroundOpacity || 100;
    backgroundOpacity.value = Math.max(20, savedOpacity);
  }
  updateBackgroundStyle();
  updateFrostedGlassStyle();
  updateElementOpacityStyle();
  try {
    await invoke("emit_event", {
      event: "frosted-glass-change",
      payload: { enabled, intensity: frostedGlassIntensity.value },
    });
  } catch (e) {
    console.error("毛玻璃事件发送失败:", e);
  }
};

const clearBackgroundImage = async () => {
  backgroundImageUrl.value = "";
  backgroundImage.value = "";
  themeStore.setBackgroundImage("");
  await deleteBackgroundFile();
  if (frostedGlassMode.value) {
    frostedGlassMode.value = false;
    themeStore.setFrostedGlassMode(false);
  }
  updateBackgroundStyle();
  updateFrostedGlassStyle();
  updateElementOpacityStyle();
};

const updateBackgroundStyle = () => {
  const root = document.documentElement;
  if (backgroundImage.value) {
    try {
      const imageUrl = `url(${backgroundImage.value})`;
      const opacity = Math.max(20, backgroundOpacity.value || 100);

      root.style.setProperty("--background-image", imageUrl);
      root.style.setProperty("--background-blur", `${backgroundBlur.value}px`);
      root.style.setProperty("--background-opacity", `${opacity / 100}`);

      const setValue = root.style.getPropertyValue("--background-image");
      if (!setValue || setValue === "none") {
        console.warn("背景图 CSS 变量设置可能失败，图片可能太大");
      }

      console.log("背景图已设置:", {
        length: backgroundImage.value.length,
        blur: backgroundBlur.value,
        opacity: opacity,
      });
    } catch (error) {
      console.error("设置背景图失败:", error);
    }
  } else {
    // 移除 CSS 变量以恢复默认样式
    root.style.removeProperty("--background-image");
    root.style.removeProperty("--background-blur");
    root.style.removeProperty("--background-opacity");
    console.log("背景图已清除");
  }
};

// 监听 themeStore 的变化
watch(
  () => themeStore.backgroundImage,
  async (newImage) => {
    if (newImage && newImage.startsWith("file://")) {
      const fileUrl = await loadBackgroundFromFile();
      if (fileUrl && fileUrl !== backgroundImage.value) {
        backgroundImage.value = fileUrl;
        backgroundImageUrl.value = fileUrl;
        updateBackgroundStyle();
      }
    } else if (newImage !== backgroundImage.value) {
      backgroundImage.value = newImage;
      backgroundImageUrl.value = newImage;
      updateBackgroundStyle();
    }
  },
);

watch(
  () => themeStore.backgroundBlur,
  (newBlur) => {
    if (newBlur !== backgroundBlur.value) {
      backgroundBlur.value = newBlur;
      updateBackgroundStyle();
    }
  },
);

watch(
  () => themeStore.backgroundOpacity,
  (newOpacity) => {
    if (newOpacity !== backgroundOpacity.value) {
      // 确保不透明度不低于20%
      const clampedOpacity = Math.max(20, newOpacity);
      backgroundOpacity.value = clampedOpacity;
      if (clampedOpacity !== newOpacity) {
        themeStore.setBackgroundOpacity(clampedOpacity);
      }
      updateBackgroundStyle();
    }
  },
);

watch(
  () => themeStore.frostedGlassMode,
  (newMode) => {
    if (newMode !== frostedGlassMode.value) {
      frostedGlassMode.value = newMode;
      updateFrostedGlassStyle();
    }
  },
);

watch(
  () => backgroundImage.value,
  () => {
    // 当背景图变化时，更新毛玻璃样式
    updateFrostedGlassStyle();
  },
);

// 初始化背景样式
onMounted(async () => {
  if (frostedGlassMode.value) {
    backgroundOpacity.value = frostedGlassIntensity.value;
  } else {
    if (!backgroundOpacity.value || isNaN(backgroundOpacity.value)) {
      backgroundOpacity.value = 100;
      themeStore.setBackgroundOpacity(100);
    } else if (backgroundOpacity.value < 20) {
      backgroundOpacity.value = 20;
      themeStore.setBackgroundOpacity(20);
    }
  }

  if (isDialogBoxHairGlass.value) {
    document.documentElement.style.setProperty("--modal-filter", "10px");
  } else {
    document.documentElement.style.setProperty("--modal-filter", "0px");
  }

  if (
    themeStore.backgroundImage &&
    themeStore.backgroundImage.startsWith("file://")
  ) {
    const fileUrl = await loadBackgroundFromFile();
    if (fileUrl) {
      backgroundImage.value = fileUrl;
      backgroundImageUrl.value = fileUrl;
    }
  }

  updateBackgroundStyle();
  updateAccessibilityStyles();
  updateFrostedGlassStyle();
  updateElementOpacityStyle();
});
</script>
