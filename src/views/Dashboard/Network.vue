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
        <!-- 加入网络 -->
        <n-tab-pane name="join" tab="加入网络">
          <div class="tab-content">
            <n-card title="加入现有网络" class="join-network-card">
              <n-space vertical size="large">
                <n-form-item label="网络名称">
                  <n-input
                    v-model:value="joinNetworkName"
                    placeholder="请输入要加入的网络名称"
                    :maxlength="20"
                    show-count
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>
                <n-form-item label="网络密钥">
                  <n-input
                    v-model:value="joinNetworkPassword"
                    placeholder="请输入网络密钥"
                    :maxlength="20"
                    show-count
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>
                <n-form-item label="本地IP">
                  <n-input
                    v-model:value="joinLocalIp"
                    placeholder="请输入本地IP地址"
                    :disabled="!!currentNetwork"
                  />
                </n-form-item>

                <n-button
                  type="primary"
                  @click="joinNetwork"
                  :loading="joining"
                  :disabled="
                    !joinNetworkName || !joinNetworkPassword || !!currentNetwork
                  "
                  block
                  size="large"
                >
                  {{
                    joining
                      ? "加入中..."
                      : currentNetwork
                        ? "已连接网络"
                        : "加入网络"
                  }}
                </n-button>
              </n-space>
            </n-card>
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
      </n-tabs>
    </div>

    <!-- 帮助信息 -->
    <n-card title="使用说明" class="help-card" style="margin-top: 20px">
      <div class="help-content">
        <div class="help-item">
          <h4>🔗 加入虚拟网络</h4>
          <p>1. 切换到"加入网络"标签页</p>
          <p>2. 输入要加入的网络名称（从网络创建者处获取）</p>
          <p>3. 输入网络密钥（从网络创建者处获取）</p>
          <p>4. 设置本地IP地址（默认 10.114.114.2）</p>
          <p>5. 点击"加入网络"按钮连接到该网络</p>
        </div>
        <div class="help-item">
          <h4>🌐 创建虚拟网络</h4>
          <p>1. 切换到"创建网络"标签页</p>
          <p>2. 输入网络名称（必填）</p>
          <p>3. 设置网络密钥（必填）</p>
          <p>4. 设置本地IP地址（默认 10.114.114.1）</p>
          <p>5. 可选择添加备注信息（可选）</p>
          <p>6. 点击"创建网络"按钮完成创建</p>
        </div>
        <div class="help-item">
          <h4>▶️ 启动虚拟网络</h4>
          <p>1. 切换到"我的通道"标签页</p>
          <p>2. 找到要启动的网络卡片</p>
          <p>3. 点击网络卡片，弹出启动确认对话框</p>
          <p>4. 点击"启动网络"按钮开始启动</p>
          <p>5. 启动成功后，切换到"当前通道"标签页查看状态</p>
        </div>
        <div class="help-item">
          <h4>📋 管理虚拟网络</h4>
          <p>在"我的通道"标签页中可以查看和管理所有已创建的虚拟网络。</p>
          <p>• 点击网络卡片可启动该网络</p>
          <p>• 点击复制图标可复制网络 ID</p>
          <p>• 点击删除图标可删除该网络（不可恢复）</p>
        </div>
        <div class="help-item">
          <h4>🔌 断开虚拟网络</h4>
          <p>1. 切换到"当前通道"标签页</p>
          <p>2. 点击"断开连接"按钮</p>
          <p>3. 确认断开后，虚拟网络将停止运行</p>
        </div>
        <div class="help-item">
          <h4>📊 查看运行日志</h4>
          <p>在侧边栏"运行日志"页面中可以查看所有操作记录和运行状态。</p>
          <p>
            • 支持按日志类型筛选：全部日志、系统日志、FRP 日志、虚拟网络日志
          </p>
          <p>• 选择 FRP 日志时，可按隧道进一步筛选</p>
          <p>• 支持自动滚动和日志清空功能</p>
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
import { ref, onMounted, onUnmounted } from "vue";
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
  CopyOutline,
  InformationCircleOutline,
  TrashOutline,
} from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import router from "../../router";
import { loadAppSystemInfo } from "../../utils/localInfo";
import { addVirtualNetworkLog } from "../../utils/log";

// 定义网络项类型
interface NetworkItem {
  id: string;
  name: string;
  password: string;
  createTime: string;
  localIp: string;
  remark?: string;
}

// 基础响应式数据
const message = useMessage();
const dialog = useDialog();

// 表单数据
const networkName = ref("");
const networkPassword = ref("");
const localIp = ref("10.114.114.1");
const networkRemark = ref("");

const systemInfo = await loadAppSystemInfo();
let system = systemInfo.split(" ")[0];
let arch = systemInfo.split(" ")[1];

// 获取 EasyTier 版本号
const EASY_TIER_VERSION = "2.4.5";

// 获取 EasyTier 平台标识 (用于URL和文件夹名)
const getEasyTierPlatform = (): { urlName: string; folderName: string } => {
  if (system === "windows") {
    if (arch === "x86_64") {
      return {
        urlName: "windows-x86_64",
        folderName: "easytier-windows-x86_64",
      };
    } else if (arch === "aarch64" || arch === "arm64") {
      return { urlName: "windows-arm64", folderName: "easytier-windows-arm64" };
    } else if (arch === "i686") {
      return { urlName: "windows-i686", folderName: "easytier-windows-i686" };
    }
  } else if (system === "linux") {
    if (arch === "x86_64") {
      return { urlName: "linux-x86_64", folderName: "easytier-linux-x86_64" };
    } else if (arch === "aarch64") {
      return { urlName: "linux-aarch64", folderName: "easytier-linux-aarch64" };
    } else if (arch === "arm") {
      return { urlName: "linux-arm", folderName: "easytier-linux-arm" };
    } else if (arch === "armhf") {
      return { urlName: "linux-armhf", folderName: "easytier-linux-armhf" };
    } else if (arch === "armv7") {
      return { urlName: "linux-armv7", folderName: "easytier-linux-armv7" };
    } else if (arch === "armv7hf") {
      return { urlName: "linux-armv7hf", folderName: "easytier-linux-armv7hf" };
    } else if (arch === "loongarch64") {
      return {
        urlName: "linux-loongarch64",
        folderName: "easytier-linux-loongarch64",
      };
    } else if (arch === "mips") {
      return { urlName: "linux-mips", folderName: "easytier-linux-mips" };
    } else if (arch === "mipsel") {
      return { urlName: "linux-mipsel", folderName: "easytier-linux-mipsel" };
    } else if (arch === "riscv64") {
      return { urlName: "linux-riscv64", folderName: "easytier-linux-riscv64" };
    }
  } else if (system === "macos" || system === "darwin") {
    if (arch === "x86_64") {
      return { urlName: "macos-x86_64", folderName: "easytier-macos-x86_64" };
    } else if (arch === "aarch64" || arch === "arm64") {
      return { urlName: "macos-aarch64", folderName: "easytier-macos-aarch64" };
    }
  }
  // 默认返回 windows x86_64
  return { urlName: "windows-x86_64", folderName: "easytier-windows-x86_64" };
};

// 获取 EasyTier 下载 URL
const getEasyTierDownloadUrl = (): string => {
  const baseUrl = `https://gh-proxy.org/https://github.com/EasyTier/EasyTier/releases/download/v${EASY_TIER_VERSION}`;
  const platform = getEasyTierPlatform();
  return `${baseUrl}/easytier-${platform.urlName}-v${EASY_TIER_VERSION}.zip`;
};

// 获取 EasyTier 解压后的内层文件夹名
const getEasyTierInnerFolder = (): string => {
  return getEasyTierPlatform().folderName;
};

// 状态
const creating = ref(false);
const leaving = ref(false);
const loadingMyNetworks = ref(false);
const joining = ref(false);
const activeTab = ref("create");

// 加入网络表单数据
const joinNetworkName = ref("");
const joinNetworkPassword = ref("");
const joinLocalIp = ref("10.114.114.2");

// 当前网络
const currentNetwork = ref<any>(null);

// 我的网络列表
const myNetworks = ref<NetworkItem[]>([]);

// 清理函数数组
const cleanupFunctions = ref<(() => void)[]>([]);

// 本地存储相关
const STORAGE_KEY = "my_networks";

// 保存网络到本地存储
const saveNetworksToLocal = (networks: NetworkItem[]) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(networks));
  } catch (error) {
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
    addVirtualNetworkLog("创建失败：网络名称不能为空", "warning");
    return;
  }

  creating.value = true;
  addVirtualNetworkLog(`开始创建虚拟网络: ${networkName.value.trim()}`, "info");

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
    addVirtualNetworkLog(`虚拟网络 "${newNetwork.name}" 创建成功`, "success");
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

    if (result === true) {
      addVirtualNetworkLog(
        `虚拟网络 "${currentNetwork.value.config.name}" 已断开`,
        "success",
      );
      message.success("网络连接已断开");

      currentNetwork.value = null;

      activeTab.value = "create";
    } else {
      throw new Error("停止失败");
    }
  } catch (error) {
    addVirtualNetworkLog(`断开虚拟网络失败: ${error}`, "error");
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

// 加入他人网络
const joinNetwork = async () => {
  if (!joinNetworkName.value.trim()) {
    message.warning("请输入网络名称");
    addVirtualNetworkLog("加入失败：网络名称不能为空", "warning");
    return;
  }

  if (!joinNetworkPassword.value.trim()) {
    message.warning("请输入网络密钥");
    addVirtualNetworkLog("加入失败：网络密钥不能为空", "warning");
    return;
  }

  joining.value = true;
  addVirtualNetworkLog(
    `开始加入虚拟网络: ${joinNetworkName.value.trim()}`,
    "info",
  );

  // 生成唯一的网络ID，确保前后端一致
  const joinNetworkId = "JOIN_" + Date.now().toString(36).toUpperCase();

  try {
    const result = await invoke("start_easytire", {
      name: joinNetworkName.value.trim(),
      password: joinNetworkPassword.value.trim(),
      id: joinNetworkId,
      localIp: joinLocalIp.value,
    });

    if (Array.isArray(result) && result[0] === "true") {
      addVirtualNetworkLog(
        `成功加入虚拟网络 "${joinNetworkName.value.trim()}"`,
        "success",
      );
      message.success(`成功加入虚拟网络 "${joinNetworkName.value.trim()}"`);

      // 创建加入的网络对象
      const joinedNetwork: NetworkItem = {
        id: joinNetworkId,
        name: joinNetworkName.value.trim(),
        password: joinNetworkPassword.value.trim(),
        createTime: new Date().toLocaleString("zh-CN"),
        localIp: joinLocalIp.value,
        remark: "加入的网络",
      };

      // 保存到加入的网络列表（单独存储）
      const joinedNetworksKey = "joined_networks";
      const existingJoined = JSON.parse(
        localStorage.getItem(joinedNetworksKey) || "[]",
      );
      existingJoined.unshift(joinedNetwork);
      localStorage.setItem(joinedNetworksKey, JSON.stringify(existingJoined));

      currentNetwork.value = {
        config: {
          name: joinedNetwork.name,
          networkId: joinedNetwork.id,
          localIp: joinedNetwork.localIp,
          create_time: joinedNetwork.createTime,
        },
        status: "Active",
        nodes: {},
      };

      // 清空表单
      joinNetworkName.value = "";
      joinNetworkPassword.value = "";
      joinLocalIp.value = "10.114.114.2";

      activeTab.value = "status";

      setTimeout(() => {
        dialog.info({
          title: "虚拟网络已连接",
          content: `已成功加入虚拟网络 "${currentNetwork.value.config.name}"\n本地地址：${currentNetwork.value.config.localIp}`,
          positiveText: "复制本地地址",
          negativeText: "关闭",
          onPositiveClick: () => {
            copyToClipboard(currentNetwork.value.config.localIp);
            addVirtualNetworkLog("本地地址已复制到剪贴板", "info");
          },
        });
      }, 500);
    } else {
      throw new Error("加入失败");
    }
  } catch (error) {
    addVirtualNetworkLog(`加入虚拟网络失败: ${error}`, "error");
    message.error(`加入失败: ${error}`);
  } finally {
    joining.value = false;
  }
};

const startNetwork = async (network: NetworkItem) => {
  try {
    const result = await invoke("start_easytire", {
      name: network.name,
      password: network.password,
      id: network.id,
      localIp: network.localIp,
    });

    if (Array.isArray(result) && result[0] === "true") {
      addVirtualNetworkLog(`虚拟网络 "${network.name}" 启动成功`, "success");
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
    addVirtualNetworkLog(
      `虚拟网络 "${network.name}" 启动失败: ${error}`,
      "error",
    );
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
        addVirtualNetworkLog("本地地址已复制到剪贴板", "info");
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

      addVirtualNetworkLog(
        `虚拟网络 "${network?.name || networkId}" 已删除`,
        "warning",
      );
      message.success(`虚拟网络 ${networkId} 已删除`);
    },
  });
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
        let fileName = "easytier.zip";
        downloading.value = true;
        try {
          await invoke("download_file", {
            url: getEasyTierDownloadUrl(),
            fileName,
            needExtract: true,
            extractInnerFolder: getEasyTierInnerFolder(), // 解压后的内层文件夹名
            extractRenameTo: "easytier", // 重命名为
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
const downloading = ref(false);
const downloadProgress = ref(0);
const downloadedBytes = ref(0);
const totalBytes = ref(0);

onMounted(async () => {
  initLocalNetworks();
  checkHasEasyTire();
  addVirtualNetworkLog("虚拟网络管理系统已启动", "info");

  try {
    const processRunning = (await invoke(
      "check_easytire_process_running",
    )) as boolean;

    const networkId = (await invoke("get_active_easytire")) as string | null;

    if (processRunning && networkId) {
      const network = myNetworks.value.find((n) => n.id === networkId);
      if (network) {
        // 找到已保存的网络配置（创建的网络）
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
        addVirtualNetworkLog(`已恢复虚拟网络 "${network.name}" 的状态`, "info");
      } else if (networkId.startsWith("JOIN_")) {
        // 加入的网络（从 joined_networks 本地存储中查找）
        const joinedNetworksKey = "joined_networks";
        const joinedNetworks: NetworkItem[] = JSON.parse(
          localStorage.getItem(joinedNetworksKey) || "[]",
        );
        const joinedNetwork = joinedNetworks.find((n) => n.id === networkId);

        if (joinedNetwork) {
          currentNetwork.value = {
            config: {
              name: joinedNetwork.name,
              networkId: joinedNetwork.id,
              localIp: joinedNetwork.localIp,
              create_time: joinedNetwork.createTime,
            },
            status: "Active",
            nodes: {},
          };
          addVirtualNetworkLog(
            `已恢复到加入的虚拟网络 "${joinedNetwork.name}"`,
            "info",
          );
        } else {
          currentNetwork.value = {
            config: {
              name: "已加入的网络",
              networkId: networkId,
              localIp: "未知",
              create_time: new Date().toLocaleString(),
            },
            status: "Active",
            nodes: {},
          };
          addVirtualNetworkLog("已恢复到加入的虚拟网络", "info");
        }
        activeTab.value = "status";
      } else {
        addVirtualNetworkLog(
          "检测到进程运行，但未找到对应的网络配置",
          "warning",
        );
      }
    } else if (processRunning && !networkId) {
      addVirtualNetworkLog(
        "检测到 easytier-core 进程正在运行，但没有记录的网络ID",
        "warning",
      );
    } else if (!processRunning && networkId) {
      await invoke("stop_easytire", { networkId });
      addVirtualNetworkLog(
        "检测到记录的网络ID，但进程未运行，已清除状态",
        "warning",
      );
    } else {
    }
  } catch (error) {
    addVirtualNetworkLog(`检查活动网络失败: ${error}`, "error");
  }

  try {
    const unlistenProgress = await listen(
      "download-progress-easytier",
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
    // 监听失败不处理
  }
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
}
</style>
