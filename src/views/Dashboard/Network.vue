<template>
  <div class="network-container">
    <div class="main-content" style="margin-top: 15px">
      <n-tabs
        v-model:value="activeTab"
        type="line"
        animated
        class="network-tabs"
      >
        <!-- 当前连接 -->
        <n-tab-pane name="status" tab="当前通道">
          <div class="tab-content">
            <!-- 有活动网络时显示详细信息 -->
            <div v-if="currentNetwork">
              <n-card class="status-card">
                <template #header>
                  <div class="status-header">
                    <div class="status-title">
                      <h3>{{ currentNetwork.config.name }}</h3>
                      <n-tag type="success" size="small">运行中</n-tag>
                    </div>
                    <div class="status-actions">
                      <n-button
                        type="error"
                        size="small"
                        @click="leaveNetwork"
                        :loading="leaving"
                      >
                        {{ leaving ? "断开中..." : "断开连接" }}
                      </n-button>
                    </div>
                  </div>
                </template>

                <div class="status-content">
                  <!-- 虚拟网络信息卡片 -->
                  <n-card title="虚拟网络信息" class="info-card">
                    <div class="connection-info-grid">
                      <div class="info-item">
                        <div class="info-label">网络名称</div>
                        <div class="info-value">
                          <span class="address-text">{{
                            currentNetwork.config.name
                          }}</span>
                        </div>
                      </div>
                      <div class="info-item">
                        <div class="info-label">本地IP</div>
                        <div class="info-value">
                          <span class="address-text">{{
                            currentNetwork.config.localIp
                          }}</span>
                        </div>
                      </div>
                      <div class="info-item">
                        <div class="info-label">连接状态</div>
                        <div class="info-value">
                          <n-tag type="success" size="small">已连接</n-tag>
                        </div>
                      </div>
                    </div>
                    <div class="connection-tip">
                      <n-alert type="info" size="small">
                        <template #icon>
                          <n-icon><InformationCircleOutline /></n-icon>
                        </template>
                        虚拟网络已建立
                      </n-alert>
                    </div>
                  </n-card>

                  <!-- 通道信息卡片 -->
                  <n-card title="通道信息" class="channel-card">
                    <div class="channel-info">
                      <div class="channel-item">
                        <div class="channel-label">创建时间</div>
                        <div class="channel-value">
                          <span class="channel-time">{{
                            currentNetwork.config.create_time || "刚刚"
                          }}</span>
                        </div>
                      </div>
                    </div>
                  </n-card>

                  <!-- 运行日志卡片 -->
                  <n-card title="运行日志" class="log-card">
                    <template #header>
                      <div class="log-header">
                        <span class="log-title">运行日志</span>
                        <div class="log-actions">
                          <n-button text size="small" @click="clearLogs">
                            <template #icon>
                              <n-icon><TrashOutline /></n-icon>
                            </template>
                            清空
                          </n-button>
                          <n-button text size="small" @click="refreshLogs">
                            <template #icon>
                              <n-icon><RefreshOutline /></n-icon>
                            </template>
                            刷新
                          </n-button>
                        </div>
                      </div>
                    </template>
                    <div class="log-container">
                      <div v-if="logs.length === 0" class="log-empty">
                        <n-empty description="暂无日志" size="small" />
                      </div>
                      <div v-else class="log-list">
                        <div
                          v-for="(log, index) in logs"
                          :key="index"
                          class="log-item"
                          :class="log.type"
                        >
                          <div class="log-time">{{ log.time }}</div>
                          <div class="log-content">
                            <n-tag
                              :type="getLogTagType(log.type)"
                              size="tiny"
                              class="log-type"
                            >
                              {{ getLogTypeText(log.type) }}
                            </n-tag>
                            <span class="log-message">{{ log.message }}</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </n-card>
                </div>
              </n-card>
            </div>

            <!-- 没有活动网络时显示空状态 -->
            <div v-else>
              <n-card title="当前虚拟网络状态" class="status-card">
                <div class="empty-state">
                  <n-empty description="暂无活动的虚拟网络">
                    <template #icon>
                      <n-icon size="48" color="#d9d9d9">
                        <WifiOutline />
                      </n-icon>
                    </template>
                  </n-empty>
                </div>
              </n-card>
            </div>
          </div>
        </n-tab-pane>
        <!-- 创建网络 -->
        <n-tab-pane name="create" tab="创建网络">
          <div class="tab-content">
            <n-card title="创建新网络" class="create-network-card">
              <n-space vertical size="large">
                <n-form-item label="网络名称">
                  <n-input
                    v-model:value="networkName"
                    placeholder="请输入网络名称"
                    :maxlength="20"
                    show-count
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>
                <n-form-item label="网络密钥">
                  <n-input
                    v-model:value="networkPassword"
                    placeholder="请输入网络密钥"
                    :maxlength="20"
                    show-count
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>
                <n-form-item label="本地IP">
                  <n-input
                    v-model:value="localIp"
                    placeholder="请输入本地IP地址"
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>
                <n-form-item label="备注">
                  <n-input
                    v-model:value="networkRemark"
                    placeholder="请输入网络备注（可选）"
                    type="textarea"
                    :rows="3"
                    :maxlength="200"
                    show-count
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>

                <n-button
                  type="primary"
                  @click="createNetwork"
                  :loading="creating"
                  :disabled="!networkName || !!currentNetwork"
                  block
                  size="large"
                >
                  {{
                    creating
                      ? "创建中..."
                      : currentNetwork
                        ? "已连接网络"
                        : "创建网络"
                  }}
                </n-button>
              </n-space>
            </n-card>
          </div>
        </n-tab-pane>
        <!-- 我的网络列表 -->
        <n-tab-pane name="my-networks" tab="我的通道">
          <div class="tab-content">
            <n-card class="networks-card">
              <template #header>
                <div class="card-header">
                  <span class="card-title">我的虚拟网络</span>
                  <n-button
                    type="primary"
                    size="small"
                    @click="refreshMyNetworks"
                    :loading="loadingMyNetworks"
                  >
                    刷新
                  </n-button>
                </div>
              </template>
              <div v-if="myNetworks.length === 0" class="empty-state">
                <n-empty description="暂无创建的虚拟网络" size="small" />
              </div>
              <div v-else class="networks-container">
                <div class="network-list">
                  <n-card
                    v-for="network in myNetworks"
                    :key="network.id"
                    class="network-item-card"
                    :class="{
                      disabled: !!currentNetwork,
                    }"
                    @click="!currentNetwork && joinMyNetwork(network)"
                  >
                    <div class="network-header">
                      <span class="network-name">{{ network.name }}</span>
                      <div class="network-actions">
                        <n-button
                          text
                          size="tiny"
                          @click.stop="copyNetworkId(network.id)"
                        >
                          <template #icon>
                            <n-icon><CopyOutline /></n-icon>
                          </template>
                        </n-button>
                        <n-button
                          text
                          size="tiny"
                          @click.stop="deleteNetwork(network.id)"
                        >
                          <template #icon>
                            <n-icon><TrashOutline /></n-icon>
                          </template>
                        </n-button>
                      </div>
                    </div>
                    <div class="network-meta">
                      <div class="meta-row">
                        <n-tag type="info" size="small" class="address-tag">
                          <template #icon>
                            <n-icon><WifiOutline /></n-icon>
                          </template>
                          {{ network.localIp }}
                        </n-tag>
                      </div>
                      <div class="meta-row">
                        <n-tag type="default" size="small" class="time-tag">
                          <template #icon>
                            <n-icon><InformationCircleOutline /></n-icon>
                          </template>
                          {{ network.createTime }}
                        </n-tag>
                      </div>
                    </div>
                    <div v-if="network.remark" class="network-remark">
                      <n-tag type="warning" size="small" class="remark-tag">
                        <template #icon>
                          <n-icon><InformationCircleOutline /></n-icon>
                        </template>
                        {{ network.remark }}
                      </n-tag>
                    </div>
                  </n-card>
                </div>
              </div>
            </n-card>
          </div>
        </n-tab-pane>

        <!-- 日志页面 -->
        <n-tab-pane name="logs" tab="日志">
          <div class="tab-content">
            <n-card class="logs-card">
              <template #header>
                <div class="logs-header">
                  <div class="logs-title-section">
                    <h3>系统日志</h3>
                    <p class="logs-subtitle">
                      查看虚拟网络的运行日志和操作记录
                    </p>
                  </div>
                  <div class="logs-actions">
                    <n-button type="primary" size="small" @click="exportLogs">
                      <template #icon>
                        <n-icon><CopyOutline /></n-icon>
                      </template>
                      导出日志
                    </n-button>
                    <n-button type="warning" size="small" @click="clearLogs">
                      <template #icon>
                        <n-icon><TrashOutline /></n-icon>
                      </template>
                      清空日志
                    </n-button>
                    <n-button type="info" size="small" @click="refreshLogs">
                      <template #icon>
                        <n-icon><RefreshOutline /></n-icon>
                      </template>
                      刷新
                    </n-button>
                  </div>
                </div>
              </template>

              <!-- 日志筛选器 -->
              <div class="logs-filter">
                <div class="filter-section">
                  <span class="filter-label">日志类型：</span>
                  <n-space>
                    <n-tag
                      v-for="type in logTypes"
                      :key="type.value"
                      :type="
                        selectedLogType === type.value ? 'primary' : 'default'
                      "
                      :bordered="false"
                      clickable
                      @click="selectedLogType = type.value"
                      class="filter-tag"
                    >
                      {{ type.label }}
                    </n-tag>
                  </n-space>
                </div>
                <div class="filter-section">
                  <span class="filter-label">时间范围：</span>
                  <n-space>
                    <n-button
                      v-for="range in timeRanges"
                      :key="range.value"
                      :type="
                        selectedTimeRange === range.value
                          ? 'primary'
                          : 'default'
                      "
                      size="small"
                      @click="selectedTimeRange = range.value"
                    >
                      {{ range.label }}
                    </n-button>
                  </n-space>
                </div>
              </div>

              <!-- 日志统计 -->
              <div class="logs-stats">
                <div class="stat-item">
                  <div class="stat-number">{{ totalLogs }}</div>
                  <div class="stat-label">总日志数</div>
                </div>
                <div class="stat-item">
                  <div class="stat-number">{{ infoLogs }}</div>
                  <div class="stat-label">信息</div>
                </div>
                <div class="stat-item">
                  <div class="stat-number">{{ successLogs }}</div>
                  <div class="stat-label">成功</div>
                </div>
                <div class="stat-item">
                  <div class="stat-number">{{ warningLogs }}</div>
                  <div class="stat-label">警告</div>
                </div>
                <div class="stat-item">
                  <div class="stat-number">{{ errorLogs }}</div>
                  <div class="stat-label">错误</div>
                </div>
              </div>

              <!-- 日志列表 -->
              <div class="logs-container">
                <div v-if="filteredLogs.length === 0" class="logs-empty">
                  <n-empty description="暂无日志记录" size="large">
                    <template #icon>
                      <n-icon size="45" color="#d9d9d9">
                        <InformationCircleOutline />
                      </n-icon>
                    </template>
                  </n-empty>
                </div>
                <div v-else class="logs-list">
                  <div
                    v-for="(log, index) in filteredLogs"
                    :key="index"
                    class="log-item"
                    :class="log.type"
                  >
                    <div class="log-time">{{ log.time }}</div>
                    <div class="log-content">
                      <n-tag
                        :type="getLogTagType(log.type)"
                        size="small"
                        class="log-type"
                      >
                        {{ getLogTypeText(log.type) }}
                      </n-tag>
                      <span class="log-message">{{ log.message }}</span>
                    </div>
                    <div class="log-actions">
                      <n-button
                        text
                        size="tiny"
                        @click="copyLogMessage(log.message)"
                      >
                        <template #icon>
                          <n-icon><CopyOutline /></n-icon>
                        </template>
                      </n-button>
                    </div>
                  </div>
                </div>
              </div>
            </n-card>
          </div>
        </n-tab-pane>
      </n-tabs>
    </div>

    <!-- 帮助信息 -->
    <n-card title="使用说明" class="help-card" style="margin-top: 20px">
      <div class="help-content">
        <div class="help-item">
          <h4>🌐 如何创建虚拟网络？</h4>
          <p>1. 输入网络名称</p>
          <p>2. 设置网络密钥</p>
          <p>3. 设置本地IP地址（默认127.0.0.1）</p>
          <p>4. 可选择添加备注信息</p>
          <p>5. 点击"创建网络"按钮</p>
        </div>
        <div class="help-item">
          <h4>📋 管理我的网络</h4>
          <p>在"我的网络"标签页中可以查看和管理您创建的所有虚拟网络。</p>
        </div>
        <div class="help-item">
          <h4>🔗 启动虚拟网络</h4>
          <p>
            创建网络后，在我的列表点击刚刚创建的网络，会弹出一个是否启动虚拟网络的弹窗，点击确定后会启动网络，点击取消则不启动。
          </p>
          <p>启动网络后，虚拟网络即可使用</p>
        </div>
        <div class="help-item">
          <h4>📊 查看系统日志</h4>
          <p>在"日志"标签页中可以查看所有虚拟网络的操作记录和运行状态。</p>
          <p>
            支持按类型和时间范围筛选，可以导出日志文件，便于问题排查和系统监控。
          </p>
        </div>
      </div>
    </n-card>

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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from "vue";
import {
  NCard,
  NButton,
  NInput,
  NFormItem,
  NSpace,
  NTag,
  NAlert,
  NEmpty,
  NIcon,
  useMessage,
  useDialog,
  NTabs,
  NTabPane,
} from "naive-ui";
import {
  WifiOutline,
  RefreshOutline,
  CopyOutline,
  InformationCircleOutline,
  TrashOutline,
} from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import router from "../../router";
import { loadAppSystemInfo, loadAppVersion } from "../../utils/localInfo";

// 定义网络项类型
interface NetworkItem {
  id: string;
  name: string;
  password: string;
  createTime: string;
  localIp: string;
  remark?: string;
}

interface LogItem {
  time: string;
  type: "info" | "success" | "warning" | "error";
  message: string;
}

// 基础响应式数据
const message = useMessage();
const dialog = useDialog();

// 表单数据
const networkName = ref("");
const networkPassword = ref("");
const localIp = ref("10.114.114.1");
const networkRemark = ref("");

const clientVersion = await loadAppVersion();
const systemInfo = await loadAppSystemInfo();
let system = systemInfo.split(" ")[0];
let arch = systemInfo.split(" ")[1];

// 状态
const creating = ref(false);
const leaving = ref(false);
const loadingMyNetworks = ref(false);
const activeTab = ref("create");

// 当前网络
const currentNetwork = ref<any>(null);

// 我的网络列表
const myNetworks = ref<NetworkItem[]>([]);

// 日志列表
const logs = ref<LogItem[]>([]);

// 清理函数数组
const cleanupFunctions = ref<(() => void)[]>([]);

// 日志筛选相关
const selectedLogType = ref<string>("all");
const selectedTimeRange = ref<string>("all");

// 日志类型选项
const logTypes = [
  { value: "all", label: "全部" },
  { value: "info", label: "信息" },
  { value: "success", label: "成功" },
  { value: "warning", label: "警告" },
  { value: "error", label: "错误" },
];

// 时间范围选项
const timeRanges = [
  { value: "all", label: "全部时间" },
  { value: "today", label: "今天" },
  { value: "yesterday", label: "昨天" },
  { value: "week", label: "最近7天" },
  { value: "month", label: "最近30天" },
];

// 计算属性
const filteredLogs = computed(() => {
  let filtered = logs.value;

  // 按类型筛选
  if (selectedLogType.value !== "all") {
    filtered = filtered.filter((log) => log.type === selectedLogType.value);
  }

  // 按时间范围筛选
  if (selectedTimeRange.value !== "all") {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000);
    const weekAgo = new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000);
    const monthAgo = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000);

    filtered = filtered.filter((log) => {
      const logTime = new Date(log.time);
      switch (selectedTimeRange.value) {
        case "today":
          return logTime >= today;
        case "yesterday":
          return logTime >= yesterday && logTime < today;
        case "week":
          return logTime >= weekAgo;
        case "month":
          return logTime >= monthAgo;
        default:
          return true;
      }
    });
  }

  return filtered;
});

// 日志统计
const totalLogs = computed(() => logs.value.length);
const infoLogs = computed(
  () => logs.value.filter((log) => log.type === "info").length,
);
const successLogs = computed(
  () => logs.value.filter((log) => log.type === "success").length,
);
const warningLogs = computed(
  () => logs.value.filter((log) => log.type === "warning").length,
);
const errorLogs = computed(
  () => logs.value.filter((log) => log.type === "error").length,
);

// 本地存储相关
const STORAGE_KEY = "my_networks";

// 保存网络到本地存储
const saveNetworksToLocal = (networks: NetworkItem[]) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(networks));
  } catch (error) {
    console.error("保存网络到本地存储失败:", error);
    message.error("保存网络失败");
  }
};

// 从本地存储加载网络
const loadNetworksFromLocal = (): NetworkItem[] => {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch (error) {
    console.error("从本地存储加载网络失败:", error);
    message.error("加载网络失败");
  }
  return [];
};

// 生成网络ID
const generateNetworkId = (): string => {
  return (
    "LingYunFrp_NET" +
    Date.now().toString(36).toUpperCase() +
    Math.random().toString(36).substr(2, 5).toUpperCase()
  );
};

// 占位函数
const createNetwork = () => {
  if (!networkName.value.trim()) {
    message.warning("请输入网络名称");
    addLog("warning", "创建失败：网络名称不能为空");
    return;
  }

  creating.value = true;
  addLog("info", `开始创建虚拟网络: ${networkName.value.trim()}`);

  setTimeout(() => {
    const newNetwork: NetworkItem = {
      id: generateNetworkId(),
      name: networkName.value.trim(),
      password: networkPassword.value.trim(),
      createTime: new Date().toLocaleString("zh-CN"),
      localIp: localIp.value,
      remark: networkRemark.value.trim() || undefined,
    };

    myNetworks.value.unshift(newNetwork);

    saveNetworksToLocal(myNetworks.value);

    networkName.value = "";
    networkPassword.value = "";
    localIp.value = "10.144.144.1";
    networkRemark.value = "";

    creating.value = false;
    addLog("success", `虚拟网络 "${newNetwork.name}" 创建成功`);
    message.success("网络创建成功！");

    dialog.info({
      title: "网络创建成功",
      content: "是否现在启动这个网络？",
      positiveText: "启动网络",
      negativeText: "稍后启动",
      onPositiveClick: () => {
        startNetwork(newNetwork);
        activeTab.value = "status";
      },
      onNegativeClick: () => {
        activeTab.value = "my-networks";
      },
    });
  }, 1000);
};

const leaveNetwork = async () => {
  if (!currentNetwork.value) {
    message.warning("没有活动的网络连接");
    return;
  }

  leaving.value = true;
  const networkId = currentNetwork.value.config.networkId;

  try {
    const result = await invoke("stop_easytire", { networkId });
    console.log("停止虚拟网络结果:", result);

    if (result === true) {
      addLog(
        "success",
        `虚拟网络 "${currentNetwork.value.config.name}" 已断开`,
      );
      message.success("网络连接已断开");

      currentNetwork.value = null;

      activeTab.value = "create";
    } else {
      throw new Error("停止失败");
    }
  } catch (error) {
    console.error("断开网络连接失败:", error);
    addLog("error", `断开虚拟网络失败: ${error}`);
    message.error(`断开失败: ${error}`);
  } finally {
    leaving.value = false;
  }
};

const copyToClipboard = (text: string) => {
  // 复制到剪贴板
  navigator.clipboard
    .writeText(text)
    .then(() => {
      message.success(`已复制到剪贴板：${text}`);
    })
    .catch(() => {
      message.error("复制失败");
    });
};

// 我的网络相关函数
const refreshMyNetworks = () => {
  loadingMyNetworks.value = true;
  setTimeout(() => {
    // 从本地存储重新加载
    myNetworks.value = loadNetworksFromLocal();
    loadingMyNetworks.value = false;
    message.success("网络列表已刷新");
  }, 500);
};

const joinMyNetwork = (network: NetworkItem) => {
  dialog.info({
    title: "启动虚拟网络",
    content: `是否启动虚拟网络 "${network.name}"？`,
    positiveText: "启动网络",
    negativeText: "取消",
    onPositiveClick: () => {
      startNetwork(network);
    },
  });
};

const startNetwork = async (network: NetworkItem) => {
  try {
    const result = await invoke("start_easytire", {
      name: network.name,
      password: network.password,
      id: network.id,
      localIp: network.localIp,
    });
    console.log(result);

    if (Array.isArray(result) && result[0] === "true") {
      addLog("success", `虚拟网络 "${network.name}" 启动成功`);
      message.success(`虚拟网络 "${network.name}" 启动成功`);

      currentNetwork.value = {
        config: {
          name: network.name,
          networkId: network.id,
          localIp: network.localIp,
          create_time: new Date().toLocaleString(),
        },
        status: "Active",
        nodes: {},
      };
    } else {
      throw new Error("启动失败");
    }
  } catch (error) {
    console.error("启动虚拟网络失败:", error);
    addLog("error", `虚拟网络 "${network.name}" 启动失败: ${error}`);
    message.error(`启动失败: ${error}`);
    return;
  }

  activeTab.value = "status";

  setTimeout(() => {
    dialog.info({
      title: "虚拟网络已启动",
      content: `虚拟网络 "${network.name}" 已成功启动\n本地地址：${network.localIp}`,
      positiveText: "复制本地地址",
      negativeText: "关闭",
      onPositiveClick: () => {
        copyToClipboard(network.localIp);
        addLog("info", "本地地址已复制到剪贴板");
      },
    });
  }, 500);
};

const copyNetworkId = (networkId: string) => {
  copyToClipboard(networkId);
};

const deleteNetwork = (networkId: string) => {
  const network = myNetworks.value.find((n) => n.id === networkId);
  dialog.warning({
    title: "删除虚拟网络",
    content: `确定要删除虚拟网络 ${networkId} 吗？此操作不可恢复。`,
    positiveText: "确定删除",
    negativeText: "取消",
    onPositiveClick: () => {
      myNetworks.value = myNetworks.value.filter(
        (network) => network.id !== networkId,
      );

      saveNetworksToLocal(myNetworks.value);

      addLog("warning", `虚拟网络 "${network?.name || networkId}" 已删除`);
      message.success(`虚拟网络 ${networkId} 已删除`);
    },
  });
};

// 日志相关函数
const addLog = (
  type: "info" | "success" | "warning" | "error",
  message: string,
) => {
  // 检查最后一条日志是否相同
  if (logs.value.length > 0 && logs.value[0].message === message) {
    return; // 相同则不添加
  }

  const log: LogItem = {
    time: new Date().toLocaleTimeString("zh-CN"),
    type,
    message,
  };
  logs.value.unshift(log);

  // 限制日志数量，最多保留100条
  if (logs.value.length > 100) {
    logs.value = logs.value.slice(0, 100);
  }
};

const clearLogs = () => {
  dialog.warning({
    title: "清空日志",
    content: "确定要清空所有日志吗？此操作不可恢复。",
    positiveText: "确定清空",
    negativeText: "取消",
    onPositiveClick: () => {
      logs.value = [];
      message.success("日志已清空");
    },
  });
};

const refreshLogs = () => {
  // 这里可以添加从服务器获取最新日志的逻辑
  addLog("info", "日志已刷新");
  message.success("日志已刷新");
};

const getLogTagType = (type: string) => {
  switch (type) {
    case "success":
      return "success";
    case "warning":
      return "warning";
    case "error":
      return "error";
    default:
      return "info";
  }
};

const getLogTypeText = (type: string) => {
  switch (type) {
    case "success":
      return "成功";
    case "warning":
      return "警告";
    case "error":
      return "错误";
    default:
      return "信息";
  }
};

const exportLogs = () => {
  const logText = logs.value
    .map((log) => `[${log.time}] [${getLogTypeText(log.type)}] ${log.message}`)
    .join("\n");

  const blob = new Blob([logText], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `virtual-network-logs-${new Date().toISOString().split("T")[0]}.txt`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);

  addLog("info", "日志已导出到本地文件");
  message.success("日志导出成功");
};

const copyLogMessage = (message: string) => {
  copyToClipboard(message);
  addLog("info", "日志消息已复制到剪贴板");
};

// 初始化时加载本地网络
const initLocalNetworks = () => {
  myNetworks.value = loadNetworksFromLocal();
};

const checkHasEasyTire = async () => {
  try {
    const result = (await invoke("check_easy_tire_exists")) as boolean;
    if (!result) {
      throw new Error("EasyTire未下载");
    }
  } catch (error) {
    let dialogInstance = dialog.warning({
      title: "EasyTire未下载",
      content: "是否下载EasyTire",
      positiveText: "下载",
      negativeText: "返回上一页",
      onPositiveClick: async () => {
        dialogInstance.destroy(); // 立即关闭警告对话
        const updateInfo = await checkUpdate(
          "easytier",
          system,
          arch,
          clientVersion,
          "0.0.0",
        );
        if (!updateInfo.success) {
          message.warning("获取版本失败");
          return;
        }
        let fileName = "easytier.zip";
        downloading.value = true;
        try {
          await invoke("download_file", {
            url: updateInfo.url,
            fileName,
          });
          message.success("easytier下载成功");
          downloading.value = false;
        } catch (error) {
          message.error("easytier下载失败");
          downloading.value = false;
          checkHasEasyTire(); // 失败时重新弹出警告
        }
      },
      onNegativeClick: () => {
        router.back();
      },
      closable: false,
      maskClosable: false,
    });
  }
};

// 组件挂载时初始化
import { NModal, NProgress } from "naive-ui";
import { checkUpdate } from "../../utils/update";
const downloading = ref(false);
const downloadProgress = ref(0);
const downloadedBytes = ref(0);
const totalBytes = ref(0);

onMounted(async () => {
  console.log("Network.vue 组件开始挂载");
  initLocalNetworks();
  checkHasEasyTire();
  addLog("info", "虚拟网络管理系统已启动");

  try {
    console.log("开始检查 easytier-core 进程状态...");
    const processRunning = (await invoke(
      "check_easytire_process_running",
    )) as boolean;
    console.log("进程运行状态:", processRunning);

    const networkId = (await invoke("get_active_easytire")) as string | null;
    console.log("记录的网络ID:", networkId);

    if (processRunning && networkId) {
      console.log("检测到活动网络:", networkId);
      const network = myNetworks.value.find((n) => n.id === networkId);
      if (network) {
        currentNetwork.value = {
          config: {
            name: network.name,
            networkId: network.id,
            localIp: network.localIp,
            create_time: new Date().toLocaleString(),
          },
          status: "Active",
          nodes: {},
        };
        activeTab.value = "status";
        addLog("info", `已恢复虚拟网络 "${network.name}" 的状态`);
      } else {
        console.log("未找到对应的网络配置");
        addLog("warning", "检测到进程运行，但未找到对应的网络配置");
      }
    } else if (processRunning && !networkId) {
      console.log("检测到 easytier-core 进程正在运行，但没有记录的网络ID");
      addLog(
        "warning",
        "检测到 easytier-core 进程正在运行，但没有记录的网络ID",
      );
    } else if (!processRunning && networkId) {
      console.log("检测到记录的网络ID，但进程未运行，清除状态");
      await invoke("stop_easytire", { networkId });
      addLog("warning", "检测到记录的网络ID，但进程未运行，已清除状态");
    } else {
      console.log("没有检测到活动的虚拟网络");
    }
  } catch (error) {
    console.error("检查活动网络失败:", error);
    addLog("error", `检查活动网络失败: ${error}`);
  }

  try {
    console.log("开始监听natter下载进度事件");
    const unlistenProgress = await listen(
      "download-progress-easytire",
      (e: any) => {
        const { bytes_downloaded, total_bytes } = e.payload;
        downloadedBytes.value = bytes_downloaded;
        totalBytes.value = total_bytes;
        downloadProgress.value = total_bytes
          ? Math.round((bytes_downloaded / total_bytes) * 100)
          : 0;
      },
    );
    cleanupFunctions.value.push(unlistenProgress);
  } catch (error) {
    console.error("监听EasyTire下载进度事件失败:", error);
  }

  console.log("Network.vue 组件挂载完成");
});

// 组件卸载时清理资源
onUnmounted(() => {
  // 清理事件监听器
  cleanupFunctions.value.forEach((cleanup) => cleanup());
  cleanupFunctions.value = [];
});
</script>

<style lang="scss" scoped>
.network-container {
  padding: 10px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  text-align: center;
  margin-bottom: 30px;

  h2 {
    margin: 0 0 8px 0;
    font-size: 28px;
    font-weight: 600;
    color: var(--text-color-1);
  }

  .subtitle {
    margin: 0;
    font-size: 16px;
    color: var(--text-color-3);
  }
}

.main-content {
  margin-bottom: 20px;
}

.network-tabs {
  .tab-content {
    padding: 20px 0;
  }
}

.create-network-card,
.join-network-card,
.networks-card,
.status-card {
  margin: 0 auto;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;

  // 在网络列表中的空状态样式
  .networks-card & {
    padding: 40px 20px;
    height: 150px;
    display: flex;
    align-items: center;
    justify-content: center;
    max-width: 300px;
    margin: 0 auto;
  }
}

.networks-header {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 16px;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;

  .status-title {
    display: flex;
    align-items: center;
    gap: 12px;

    h3 {
      margin: 0;
      font-size: 18px;
      font-weight: 600;
      color: var(--text-color-1);
    }
  }

  .status-actions {
    display: flex;
    gap: 8px;
  }
}

.status-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.info-card {
  .connection-info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 16px;

    .info-item {
      .info-label {
        font-size: 13px;
        color: var(--text-color-3);
        margin-bottom: 4px;
        font-weight: 500;
      }

      .info-value {
        display: flex;
        align-items: center;
        gap: 8px;

        .address-text,
        .id-text {
          font-family: monospace;
          font-size: 14px;
          color: var(--text-color-1);
          background-color: var(--info-color-suppl);
          padding: 4px 8px;
          border-radius: 4px;
          flex: 1;
        }
      }
    }
  }

  .connection-tip {
    margin-top: 16px;
  }
}

.channel-card {
  .channel-info {
    display: flex;
    flex-direction: column;
    gap: 16px;

    .channel-item {
      .channel-label {
        font-size: 13px;
        color: var(--text-color-3);
        margin-bottom: 4px;
        font-weight: 500;
      }

      .channel-value {
        display: flex;
        align-items: center;
        gap: 8px;

        .channel-id {
          font-family: monospace;
          font-size: 14px;
          color: var(--text-color-1);
          background-color: var(--info-color-suppl);
          padding: 4px 8px;
          border-radius: 4px;
          flex: 1;
        }

        .channel-time {
          font-size: 14px;
          color: var(--text-color-2);
        }
      }
    }
  }
}

.log-card {
  .log-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .log-title {
      font-weight: 500;
      color: var(--text-color-1);
    }

    .log-actions {
      display: flex;
      gap: 8px;
    }
  }

  .log-container {
    .log-empty {
      text-align: center;
      padding: 40px 20px;
    }

    .log-list {
      display: flex;
      flex-direction: column;
      gap: 8px;

      .log-item {
        display: flex;
        align-items: flex-start;
        gap: 12px;
        padding: 8px 12px;
        border-radius: 6px;
        background-color: var(--hover-color);
        transition: all 0.2s ease;

        &:hover {
          background-color: var(--primary-color-suppl);
        }

        .log-time {
          font-size: 11px;
          color: var(--text-color-3);
          font-family: monospace;
          white-space: nowrap;
          min-width: 60px;
        }

        .log-content {
          display: flex;
          align-items: center;
          gap: 8px;
          flex: 1;

          .log-type {
            min-width: 40px;
            text-align: center;
          }

          .log-message {
            font-size: 13px;
            color: var(--text-color-1);
            line-height: 1.4;
          }
        }

        &.success {
          border-left: 3px solid var(--success-color);
        }

        &.warning {
          border-left: 3px solid var(--warning-color);
        }

        &.error {
          border-left: 3px solid var(--error-color);
        }

        &.info {
          border-left: 3px solid var(--info-color);
        }
      }
    }
  }
}

.logs-card {
  .logs-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 20px;

    .logs-title-section {
      h3 {
        margin: 0 0 4px 0;
        font-size: 18px;
        font-weight: 600;
        color: var(--text-color-1);
      }

      .logs-subtitle {
        margin: 0;
        font-size: 14px;
        color: var(--text-color-3);
      }
    }

    .logs-actions {
      display: flex;
      gap: 8px;
    }
  }

  .logs-filter {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    background-color: var(--hover-color);
    border-radius: 8px;
    margin-bottom: 20px;

    .filter-section {
      display: flex;
      align-items: center;
      gap: 12px;

      .filter-label {
        font-size: 14px;
        font-weight: 500;
        color: var(--text-color-1);
        white-space: nowrap;
      }

      .filter-tag {
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
          transform: translateY(-1px);
        }
      }
    }
  }

  .logs-stats {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 16px;
    margin-bottom: 20px;

    .stat-item {
      text-align: center;
      padding: 16px;
      background-color: var(--hover-color);
      border-radius: 8px;
      transition: all 0.2s ease;

      &:hover {
        background-color: var(--primary-color-suppl);
        transform: translateY(-2px);
      }

      .stat-number {
        font-size: 24px;
        font-weight: 600;
        color: var(--primary-color);
        margin-bottom: 4px;
      }

      .stat-label {
        font-size: 12px;
        color: var(--text-color-3);
        text-transform: uppercase;
        letter-spacing: 0.5px;
      }
    }
  }

  .logs-container {
    .logs-empty {
      text-align: center;
      padding: 60px 20px;
    }

    .logs-list {
      display: flex;
      flex-direction: column;
      gap: 8px;

      .log-item {
        display: flex;
        align-items: flex-start;
        gap: 12px;
        padding: 12px 16px;
        border-radius: 8px;
        background-color: var(--hover-color);
        transition: all 0.2s ease;

        &:hover {
          background-color: var(--primary-color-suppl);
          transform: translateX(4px);
        }

        .log-time {
          font-size: 12px;
          color: var(--text-color-3);
          font-family: monospace;
          white-space: nowrap;
          min-width: 80px;
        }

        .log-content {
          display: flex;
          align-items: center;
          gap: 12px;
          flex: 1;

          .log-type {
            min-width: 50px;
            text-align: center;
          }

          .log-message {
            font-size: 14px;
            color: var(--text-color-1);
            line-height: 1.5;
          }
        }

        .log-actions {
          opacity: 0;
          transition: opacity 0.2s ease;
        }

        &:hover .log-actions {
          opacity: 1;
        }

        &.success {
          border-left: 4px solid var(--success-color);
        }

        &.warning {
          border-left: 4px solid var(--warning-color);
        }

        &.error {
          border-left: 4px solid var(--error-color);
        }

        &.info {
          border-left: 4px solid var(--info-color);
        }
      }
    }
  }
}
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;

  .card-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-color-1);
  }
}

.network-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 16px;

  .network-item-card {
    cursor: pointer;
    transition: all 0.3s ease;
    overflow: hidden;
    background: linear-gradient(
      135deg,
      var(--card-color) 0%,
      var(--hover-color) 100%
    );

    &:hover {
      box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
      transform: translateY(-3px);
    }

    &.disabled {
      opacity: 0.6;
      cursor: not-allowed;

      &:hover {
        box-shadow: none;
        transform: none;
      }
    }

    .network-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      margin-bottom: 12px;
      padding: 16px 16px 0 16px;

      .network-name {
        font-weight: 700;
        font-size: 18px;
        color: var(--text-color-1);
        flex: 1;
        line-height: 1.4;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
      }

      .network-actions {
        display: flex;
        gap: 4px;
        opacity: 0.7;
        transition: opacity 0.2s ease;

        &:hover {
          opacity: 1;
        }
      }
    }

    .network-meta {
      display: flex;
      flex-direction: column;
      gap: 12px;
      padding: 0 16px 16px 16px;

      .meta-row {
        display: flex;
        gap: 8px;
        flex-wrap: wrap;
        align-items: center;

        .address-tag {
          font-family: monospace;
          font-weight: 500;
        }

        .time-tag {
          font-size: 11px;
        }
      }
    }

    .network-remark {
      margin: 0 16px 16px 16px;

      .remark-tag {
        font-size: 12px;
        line-height: 1.4;
        white-space: pre-wrap;
        max-width: 100%;
        word-break: break-word;
      }
    }
  }
}

.help-card {
  .help-content {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;

    .help-item {
      h4 {
        margin: 0 0 12px 0;
        font-size: 16px;
        font-weight: 500;
        color: var(--text-color-1);
      }

      p {
        margin: 0 0 8px 0;
        color: var(--text-color-3);
        line-height: 1.6;

        &:last-child {
          margin-bottom: 0;
        }
      }
    }
  }
}

// 响应式设计
@media (max-width: 768px) {
  .help-content {
    grid-template-columns: 1fr !important;
  }

  .network-details {
    flex-direction: column;
    align-items: flex-start;
  }

  .connection-details .connection-item {
    flex-direction: column;
    align-items: flex-start;

    .label {
      min-width: auto;
      margin-bottom: 4px;
    }
  }

  .connection-info {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }

  .network-list {
    grid-template-columns: 1fr;
    gap: 12px;
  }

  .card-header {
    flex-direction: column;
    gap: 8px;
    align-items: flex-start;
  }

  .status-header {
    flex-direction: column;
    gap: 12px;
    align-items: flex-start;

    .status-title {
      flex-direction: column;
      align-items: flex-start;
      gap: 8px;
    }
  }

  .info-card .connection-info-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }

  .log-card .log-header {
    flex-direction: column;
    gap: 12px;
    align-items: flex-start;
  }

  .logs-card .logs-header {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;
  }

  .logs-filter {
    flex-direction: column;
    gap: 16px;
  }

  .logs-stats {
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }
}
</style>
