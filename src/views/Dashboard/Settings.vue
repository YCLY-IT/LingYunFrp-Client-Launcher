<script setup lang="ts">
import packageData from "../../../package.json";
import { ref, onMounted, computed, onUnmounted } from "vue";
import {
  useMessage,
  useDialog,
  NButton,
  NCard,
  NLog,
  NSpace,
  NAlert,
  NFormItem,
  NSwitch,
  NInput,
  NDrawer,
  NDrawerContent,
  NIcon,
  NSelect,
  NProgress,
  NText,
} from "naive-ui";
import {
  RefreshOutline,
  FolderOpenOutline,
  DownloadOutline,
  TerminalOutline,
  SettingsOutline,
  SkullOutline,
  EyeOutline,
  EyeOffOutline,
  CheckmarkOutline,
  PinOutline,
  ServerOutline,
  PowerOutline,
  WifiOutline,
} from "@vicons/ionicons5";
import { onBeforeRouteLeave } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { checkUpdate } from "../../utils/update";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { loadAppSystemInfo, loadAppVersion } from "../../utils/localInfo";
import { useSystemStore, type LogLevel } from "../../stores/system";
import { useUpdateManager } from "../../composables/useUpdateManager";

const message = useMessage();
const dialog = useDialog();
const downloading = ref(false);
const logs = ref("");

const currentVersion = ref("获取中...");
const checking = ref(false);
const downloadProgress = ref(0);
const downloadedBytes = ref(0);
const totalBytes = ref(0);
const buildTime = ref(__BUILD_TIME__ || "开发模式");
const buildFingerprint = ref(__BUILD_FINGERPRINT__ || "dev");

const isAppReady = ref(true);
const {
  performAutoUpdate,
  updateModalVisible,
  updateProgress,
  updateStatus,
  updateDownloaded,
  updateTotal,
} = useUpdateManager(isAppReady);

// 使用系统store
const systemStore = useSystemStore();
const autoStart = computed({
  get: () => systemStore.autoStart,
  set: (value) => (systemStore.autoStart = !value),
});
const autoRestoreTunnels = computed({
  get: () => systemStore.autoRestoreTunnels,
  set: (value) => systemStore.setAutoRestoreTunnels(value),
});
const saveToTray = computed({
  get: () => systemStore.saveToTray,
  set: (value) => systemStore.setSaveToTray(value),
});
const skipSystemProxy = computed({
  get: () => systemStore.skipSystemProxy,
  set: (value) => systemStore.setSkipSystemProxy(value),
});

const consoleLogLevel = computed({
  get: () => systemStore.consoleLogLevel,
  set: (value: LogLevel) => systemStore.setConsoleLogLevel(value),
});

const logLevelOptions = [
  { label: "调试 - 显示所有日志", value: "debug" },
  { label: "信息 - 显示 info 及以上", value: "info" },
  { label: "警告 - 显示 warn 及以上", value: "warn" },
  { label: "错误 - 仅显示错误", value: "error" },
];

// 存储事件监听器的取消函数
let unlistenAutoStartChange: UnlistenFn | null = null;
let unlistenSettingsChange: UnlistenFn | null = null;

const clientVersion = await loadAppVersion();
const systemInfo = await loadAppSystemInfo();
let system = systemInfo.split(" ")[0];
let arch = systemInfo.split(" ")[1];

const getCurrentVersion = async () => {
  try {
    const version = (await loadAppVersion()) as string;
    currentVersion.value = version;
  } catch (e) {
    currentVersion.value = "获取失败";
    console.error("获取版本失败:", e);
  }
};

const checkAppUpdate = async () => {
  checking.value = true;
  try {
    const result = await checkUpdate(
      "LingYunFrpClient",
      system,
      arch,
      clientVersion,
      clientVersion,
    );
    if (!result) {
      message.error("检查时出现了一些问题");
      return;
    }
    if (result.success) {
      const downloadUrl = result.url;
      const fileName = downloadUrl.split("/").pop() || "update.exe";
      dialog.success({
        title: `发现新版本 ${result.version}`,
        content: result.message,
        positiveText: "立即更新",
        negativeText: "暂不更新",
        onPositiveClick: () => {
          setTimeout(() => {
            performAutoUpdate(downloadUrl, fileName);
          }, 100);
        },
      });
    } else {
      dialog.info({
        title: "检查更新",
        content: result.message,
        positiveText: "确定",
      });
    }
  } catch (e) {
    message.error(e);
  } finally {
    checking.value = false;
  }
};

const toggleAutoStart = async () => {
  try {
    await systemStore.toggleAutoStart();
    message.success(`${systemStore.autoStart ? "启用" : "禁用"}开机自启动成功`);
  } catch (e) {
    message.error(`设置开机自启动失败: ${e}`);
  }
};

const toggleAutoRestoreTunnels = (value: boolean) => {
  autoRestoreTunnels.value = value;
  message.success(`${value ? "启用" : "禁用"}开机恢复隧道成功`);
  if (!value && autoStart.value) {
    setTimeout(() => {
      message.warning("已禁用恢复隧道，程序将在启动后不会自动启动隧道");
    }, 500);
  }
};

const toggleSaveToTray = (value: boolean) => {
  saveToTray.value = value;
  message.success(`${value ? "启用" : "禁用"}保存到托盘成功`);
};

const toggleSkipSystemProxy = (value: boolean) => {
  skipSystemProxy.value = value;
  message.success(`${value ? "启用" : "禁用"}跳过系统代理成功`);
};

onBeforeRouteLeave((_to, _from, next) => {
  if (downloading.value) {
    dialog.warning({
      title: "提示",
      content: "正在下载 frpc，离开页面将中断下载。确定要离开吗？",
      positiveText: "继续下载",
      negativeText: "离开",
      onPositiveClick: () => {
        next(false);
      },
      onNegativeClick: () => {
        next();
      },
    });
  } else {
    next();
  }
});

onMounted(async () => {
  getCurrentVersion();
});

async function downloadAndReplaceFrpc(url: string, system: string) {
  const fileName = system === "windows" ? "frpc.exe" : "frpc";
  try {
    await invoke("delete_file", { fileName });
  } catch (e) {
    console.error(e);
  }

  downloading.value = true;
  try {
    await invoke("download_file", {
      url,
      fileName,
      needExtract: false,
      extractInnerFolder: null,
      extractRenameTo: null,
    });
    message.success("更新成功");
  } catch (e) {
    message.error(`下载失败: ${e}`);
  } finally {
    downloading.value = false;
  }
}

async function checkHasFrpcAndUpdate() {
  try {
    const result = await invoke<string>("get_frpc_cli_version");
    const frpcInfo = JSON.parse(result);

    if (!frpcInfo?.version || typeof frpcInfo.version !== "string") {
      throw new Error("无效的版本信息格式");
    }
    const updateInfo = await checkUpdate(
      "Frpc",
      system,
      arch,
      frpcInfo.version,
      currentVersion.value,
    );
    if (!updateInfo.success) {
      message.success("当前已是最新版本");
      return;
    }

    const dialogInstance = dialog.info({
      title: "提示",
      content: `检测到新版本 ${updateInfo.version}，是否更新？`,
      positiveText: "更新",
      negativeText: "取消",
      onPositiveClick: () => {
        dialogInstance.destroy();
        downloadAndReplaceFrpc(updateInfo.url, system);
      },
    });
  } catch {
    // 无法解析本地版本
    const updateInfo = await checkUpdate(
      "Frpc",
      system,
      arch,
      clientVersion,
      currentVersion.value,
    );
    console.log(updateInfo);
    if (updateInfo.success) {
      await downloadAndReplaceFrpc(updateInfo.url, system);
    } else {
      message.info(updateInfo.message);
    }
  }
}

const getFrpcVersion = async () => {
  try {
    const result = await invoke("get_frpc_cli_version");
    let frpcInfo;
    try {
      frpcInfo = JSON.parse(result as string);
    } catch (e) {
      console.error("解析失败，原始数据:", result); // 记录原始响应
      throw new Error(
        `数据解析失败: ${e instanceof Error ? e.message : String(e)}`,
      );
    }

    // 添加字段验证
    if (!frpcInfo?.version || typeof frpcInfo.version !== "string") {
      throw new Error("无效的版本信息格式");
    }
    logs.value += `${new Date().toLocaleTimeString()} [系统] 检测到Frpc版本: ${frpcInfo.version}\n`;
    message.success(`当前版本: ${frpcInfo.version}`);
  } catch (e) {
    if (e.includes("系统找不到指定的文件")) {
      logs.value += `${new Date().toLocaleTimeString()} [系统] 您并未下载FRPC,可点击旁边的"自动下载/更新Frpc"按钮\n`;
      message.warning("Frpc可执行文件不存在，请配置或下载");
      return;
    }
    logs.value += `${new Date().toLocaleTimeString()} [系统] 获取Frpc版本失败: ${e}\n`;
    message.error(`获取版本失败: ${e}`);
  }
};

const killAllProcesses = async () => {
  dialog.warning({
    title: "终止所有 Frpc 进程",
    content:
      "此操作将终止所有正在运行的 Frpc 进程，这将会断开所有隧道连接。确定要继续吗？",
    positiveText: "确定终止",
    negativeText: "取消",
    onPositiveClick: async () => {
      try {
        await invoke("kill_all_processes", { processes: ["frpc.exe"] });
        message.success("已终止所有 frpc 进程");
      } catch (e) {
        message.error(`操作失败: ${e}`);
      }
    },
  });
};

const openAppDataDir = async () => {
  try {
    await invoke("open_app_data_dir");
    message.success("已打开数据目录");
  } catch (e) {
    console.error("打开数据目录失败:", e);
    message.error(`打开数据目录失败: ${e}`);
  }
};

const manualModeVisible = ref(false);
const showManualMode = () => {
  manualModeVisible.value = true;
};

const appDataDir = ref("");
const expectedFrpcFilename = ref("");
const getExpectedFrpcInfo = async () => {
  try {
    const frpcVersion = (await invoke("get_frpc_cli_version")) as {
      version: string;
      path: string;
      filename: string;
    };
    if (frpcVersion && frpcVersion.filename) {
      expectedFrpcFilename.value = frpcVersion.filename;
      return;
    }
  } catch (e) {
    console.error("获取frpc信息失败:", e);
  }
  const isWindows = navigator.platform.toLowerCase().includes("win");
  if (isWindows) {
    expectedFrpcFilename.value = "frpc.exe";
  } else {
    const isMac = navigator.platform.toLowerCase().includes("mac");
    if (isMac) {
      const isArm =
        /arm|aarch/i.test(navigator.platform) ||
        (/Mac/.test(navigator.userAgent) && navigator.maxTouchPoints > 1);
      expectedFrpcFilename.value = isArm
        ? "frpc_darwin_arm64"
        : "frpc_darwin_amd64";
    } else {
      const isLinuxArm = /arm|aarch/i.test(navigator.platform);
      expectedFrpcFilename.value = isLinuxArm
        ? "frpc_linux_arm64"
        : "frpc_linux_amd64";
    }
  }
};

onMounted(async () => {
  try {
    // 从系统获取自启动状态并加载所有设置
    await systemStore.loadAutoStartStatus();
  } catch {
    /* 忽略解析错误 */
  }
  try {
    appDataDir.value = (await invoke("get_app_data_dir")) as string;
    await getExpectedFrpcInfo();
  } catch (e) {
    console.error("获取应用数据目录失败:", e);
    message.error(`获取应用数据目录失败: ${e}`);
  }
  try {
    await listen("download-progress-frpc", (e: any) => {
      const { bytes_downloaded, total_bytes } = e.payload;

      // 1) 原始字节
      downloadedBytes.value = bytes_downloaded;
      totalBytes.value = total_bytes;

      // 2) 百分比（保留整数）
      downloadProgress.value = total_bytes
        ? Math.round((bytes_downloaded / total_bytes) * 100)
        : 0;
    });
  } catch (e) {
    console.error("监听下载进度失败:", e);
  }

  try {
    unlistenAutoStartChange = await listen(
      "system-auto-start-changed",
      (e: any) => {
        if (e.payload && e.payload.autoStart !== undefined) {
          systemStore.$patch({
            autoStart: e.payload.autoStart,
          });
        }
      },
    );
  } catch (e) {
    console.error("监听自启动状态变化失败:", e);
  }

  try {
    unlistenSettingsChange = await listen(
      "system-settings-changed",
      (e: any) => {
        if (e.payload) {
          systemStore.$patch({
            autoStart: e.payload.autoStart,
            autoRestoreTunnels: e.payload.autoRestoreTunnels,
            saveToTray: e.payload.saveToTray,
            skipSystemProxy: e.payload.skipSystemProxy,
          });
        }
      },
    );
  } catch (e) {
    console.error("监听设置变化失败:", e);
  }
});

onUnmounted(() => {
  // 移除自启动状态变化事件监听
  if (unlistenAutoStartChange) {
    unlistenAutoStartChange();
    unlistenAutoStartChange = null;
  }
  // 移除设置变化事件监听
  if (unlistenSettingsChange) {
    unlistenSettingsChange();
    unlistenSettingsChange = null;
  }
});

const restoreUpdateNotification = () => {
  dialog.warning({
    title: "恢复更新提示",
    content: "确定要恢复更新提示吗？恢复后，软件将重新显示更新通知。",
    positiveText: "确定恢复",
    negativeText: "取消",
    onPositiveClick: () => {
      localStorage.removeItem("suppressUpdateNotification");
      message.success("已恢复更新提示");
    },
  });
};

const disableUpdateNotification = () => {
  dialog.warning({
    title: "禁用更新提示",
    content:
      "确定要禁用更新提示吗？禁用后，软件将不再显示更新通知。您可以在设置中重新启用。",
    positiveText: "确定禁用",
    negativeText: "取消",
    onPositiveClick: () => {
      localStorage.setItem("suppressUpdateNotification", "true");
      message.success("已禁用更新提示");
    },
  });
};
</script>

<template>
  <div class="settings">
    <n-scrollbar>
      <n-space vertical :size="20">
        <!-- 版本信息卡片 -->
        <n-card>
          <template #header>
            <n-space>
              <n-icon :component="PinOutline" />
              <span>版本信息</span>
            </n-space>
          </template>
          <n-space vertical>
            <n-text>当前版本：Beta v{{ currentVersion }}</n-text>
            <n-text>构建时间：{{ buildTime }}</n-text>
            <n-text>构建指纹：{{ buildFingerprint }}</n-text>
            <n-space>
              <n-button @click="checkAppUpdate()" :loading="checking">
                <template #icon>
                  <n-icon :component="RefreshOutline" />
                </template>
                {{ checking ? "检查中..." : "检查更新" }}
              </n-button>
              <n-button @click="openAppDataDir">
                <template #icon>
                  <n-icon :component="FolderOpenOutline" />
                </template>
                打开软件数据目录
              </n-button>
              <n-button
                tertiary
                type="warning"
                @click="disableUpdateNotification"
              >
                <template #icon>
                  <n-icon :component="EyeOffOutline" />
                </template>
                禁用更新提示
              </n-button>
              <n-button tertiary type="info" @click="restoreUpdateNotification">
                <template #icon>
                  <n-icon :component="EyeOutline" />
                </template>
                恢复更新提示
              </n-button>
            </n-space>
          </n-space>
        </n-card>

        <!-- Frpc 管理卡片 -->
        <n-card>
          <template #header>
            <n-space>
              <n-icon :component="ServerOutline" />
              <span>Frpc 管理</span>
            </n-space>
          </template>
          <template #header-extra> 首次使用请在这里下载或配置 Frpc </template>
          <n-space>
            <n-button
              @click="checkHasFrpcAndUpdate"
              :loading="downloading"
              :disabled="downloading"
            >
              <template #icon>
                <n-icon :component="DownloadOutline" />
              </template>
              {{ downloading ? "正在进行操作..." : "自动下载/更新 Frpc" }}
            </n-button>
            <n-button @click="getFrpcVersion" :disabled="downloading">
              <template #icon>
                <n-icon :component="TerminalOutline" />
              </template>
              获取本地 Frpc 版本
            </n-button>
            <n-button @click="showManualMode" :disabled="downloading">
              <template #icon>
                <n-icon :component="SettingsOutline" />
              </template>
              手动配置 Frpc 可执行文件
            </n-button>
            <n-button
              type="warning"
              :disabled="downloading"
              @click="killAllProcesses"
            >
              <template #icon>
                <n-icon :component="SkullOutline" />
              </template>
              终止所有 Frpc 进程
            </n-button>
          </n-space>
          <br />
          <n-card class="mt-4">
            <template #header>
              <n-space>
                <n-icon :component="TerminalOutline" />
                <span>运行日志</span>
              </n-space>
            </template>
            <n-log :rows="10" :log="logs" :loading="false" trim />
          </n-card>
        </n-card>

        <!-- 启动设置卡片 -->
        <n-card>
          <template #header>
            <n-space>
              <n-icon :component="PowerOutline" />
              <span>启动设置</span>
            </n-space>
          </template>
          <n-space vertical>
            <n-space
              style="
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 10px;
                margin-top: 5px;
              "
            >
              <span>开机自启动</span>
              <n-switch
                v-model:value="autoStart"
                @update:value="toggleAutoStart"
              />
            </n-space>
            <n-space
              style="
                display: flex;
                justify-content: space-between;
                align-items: center;
              "
            >
              <span>打开上次未关闭的隧道</span>
              <n-switch
                v-model:value="autoRestoreTunnels"
                @update:value="toggleAutoRestoreTunnels"
              />
            </n-space>
            <n-space
              style="
                display: flex;
                justify-content: space-between;
                align-items: center;
              "
            >
              <span>关闭时保存到托盘</span>
              <n-switch
                v-model:value="saveToTray"
                @update:value="toggleSaveToTray"
              />
            </n-space>
          </n-space>
        </n-card>

        <!-- 网络设置卡片 -->
        <n-card>
          <template #header>
            <n-space>
              <n-icon :component="WifiOutline" />
              <span>网络设置</span>
            </n-space>
          </template>
          <n-space vertical>
            <n-space
              style="
                display: flex;
                justify-content: space-between;
                align-items: center;
              "
            >
              <span>跳过系统代理</span>
              <n-switch
                v-model:value="skipSystemProxy"
                @update:value="toggleSkipSystemProxy"
              />
            </n-space>
            <n-space
              style="
                display: flex;
                justify-content: space-between;
                align-items: center;
              "
            >
              <span>日志级别</span>
              <n-select
                v-model:value="consoleLogLevel"
                :options="logLevelOptions"
                style="width: 200px"
              />
            </n-space>
          </n-space>
        </n-card>
      </n-space>

      <n-drawer v-model:show="manualModeVisible" :width="600" placement="right">
        <n-drawer-content title="手动配置 Frpc 可执行文件" closable>
          <n-space vertical>
            <n-alert type="info">
              如果自动下载失败，您可以手动下载 Frpc
              可执行文件并放置到程序数据目录
            </n-alert>
            <n-alert type="warning" title="注意">
              请在 {{ packageData.title }} 管理面板 - 下载中心
              下载对应操作系统和对应平台的 Frpc 可执行文件。 <br />
              <br />
              1. Windows 系统请下载 Windows 64 位版本的 Frpc 可执行文件 <br />
              2. Mac 系统请下载 Mac 64 位版本的 Frpc 可执行文件 <br />
              3. Linux 系统请下载 Linux 64 位版本的 Frpc 可执行文件 <br />
              <br />
              4. 下载完成后，将 Frpc 可执行文件放置到应用数据目录 <br />
              5. 打开应用数据目录，找到 Frpc 可执行文件并将其重命名为 frpc.exe
              <br />
            </n-alert>
            <n-form-item label="应用数据目录">
              <n-input v-model:value="appDataDir" readonly />
              <n-button @click="openAppDataDir"> 打开数据目录 </n-button>
            </n-form-item>
            <n-form-item label="Frpc 可执行文件名称">
              <n-input v-model:value="expectedFrpcFilename" readonly />
            </n-form-item>
          </n-space>
          <template #footer
            ><n-space justify="end">
              <n-button @click="manualModeVisible = false">
                <template #icon>
                  <n-icon :component="EyeOffOutline" />
                </template>
                关闭
              </n-button>
              <n-button
                type="primary"
                @click="
                  getFrpcVersion();
                  manualModeVisible = false;
                "
              >
                <template #icon>
                  <n-icon :component="CheckmarkOutline" />
                </template>
                完成并检查
              </n-button>
            </n-space></template
          >
        </n-drawer-content>
      </n-drawer>
    </n-scrollbar>
    <n-modal
      v-model:show="downloading"
      title="下载进度"
      preset="card"
      style="width: 400px"
      :closable="false"
      :mask-closable="false"
    >
      <n-progress
        type="line"
        :percentage="downloadProgress"
        :show-indicator="true"
      />
      <div style="margin-top: 10px">
        已下载: {{ downloadedBytes }} / {{ totalBytes }} 字节 ({{
          downloadProgress
        }}%)
      </div>
    </n-modal>
    <n-modal
      v-model:show="updateModalVisible"
      :mask-closable="false"
      :closable="false"
      :trap-focus="false"
      :auto-focus="false"
    >
      <n-space vertical style="padding: 24px; min-width: 400px">
        <n-text style="font-size: 18px; font-weight: bold">正在更新应用</n-text>
        <n-text>{{ updateStatus }}</n-text>
        <n-progress
          type="line"
          :percentage="updateProgress"
          :show-indicator="true"
        />
        <n-space justify="space-between">
          <n-text style="font-size: 12px; color: #999">
            已下载: {{ (updateDownloaded / 1024 / 1024).toFixed(2) }} MB
          </n-text>
          <n-text style="font-size: 12px; color: #999">
            总大小:
            {{
              updateTotal > 0
                ? (updateTotal / 1024 / 1024).toFixed(2) + " MB"
                : "未知"
            }}
          </n-text>
        </n-space>
      </n-space>
    </n-modal>
  </div>
</template>
