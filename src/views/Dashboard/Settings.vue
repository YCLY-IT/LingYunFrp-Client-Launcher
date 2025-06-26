<script setup lang="ts">
import packageData from '../../../package.json'
import { ref, onMounted } from 'vue'
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
  NCollapse,
  NCollapseItem,
  NPopconfirm,
  NText,
  NScrollbar,
  NDrawer,
  NDrawerContent,

} from 'naive-ui'
import { onBeforeRouteLeave } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { userApi } from '../../net'
import { accessHandle } from '../../net/base'


const message = useMessage();
const dialog = useDialog();
const downloading = ref(false);
const logs = ref('');

const currentVersion = ref('获取中...');
const checking = ref(false);
const autoStart = ref(false);
const autoRestoreTunnels = ref(true);
const deepLinkEnabled = ref(false);
const activeNames = ref<string[]>(['2']);


const getCurrentVersion = async () => {
  try {
    const version = await invoke('get_cpl_version') as string;
    currentVersion.value = version;
  } catch (e) {
    currentVersion.value = '获取失败';
    console.error('获取版本失败:', e);
  }
};

// 版本号比较函数，返回1表示a>b，0表示相等，-1表示a<b
function compareVersion(a: string, b: string): number {
  const aParts = a.split('.').map(Number)
  const bParts = b.split('.').map(Number)
  const len = Math.max(aParts.length, bParts.length)
  for (let i = 0; i < len; i++) {
    const aNum = aParts[i] || 0
    const bNum = bParts[i] || 0
    if (aNum > bNum) return 1
    if (aNum < bNum) return -1
  }
  return 0
}

const checkUpdate = async () => {
  checking.value = true;
  try {
    const clientVersion = await invoke<string>('get_client_version')
    const systemInfo = await invoke<string>('get_system_info');
    let system = systemInfo.split(' ')[0];
    let arch = systemInfo.split(' ')[1];
    userApi.get(`/frp/updates/latest?software=LingYunFrpClient&system=${system}&arch=${arch}&version=${clientVersion}`, accessHandle(), (data: any) => {
      const latestVersion = data.data.latest_info.version;
      // 检查 currentVersion 是否为合法版本号
      if (!/^\d+\.\d+\.\d+/.test(currentVersion.value)) {
        message.warning('当前版本号异常，无法比较');
        return;
      }
      if (compareVersion(latestVersion, currentVersion.value) === 1) {
        (window as any).$notification?.success({
          title: '更新提示',
          description: data.data.latest_info.release_notes,
          duration: 3000,
        });
      } else {
        message.success('当前已是最新版本');
      }
    }, () => {
      message.warning("当前没有新版本");
    }, (messageText) => {
      message.error(messageText);
    });
  } catch (e) {
    message.error('检查更新失败，请稍后重试');
  } finally {
    checking.value = false;
  }
};

const toggleAutoStart = async () => {
  try {
    await invoke('toggle_auto_start', { enable: autoStart.value });
    message.success(`${autoStart.value ? '启用' : '禁用'}开机自启动成功`);
    if (autoStart.value && !autoRestoreTunnels.value) {
      setTimeout(() => {
        message.info('提示：如需开机自动启动隧道，请同时开启"开机时恢复上次运行的隧道"选项');
      }, 500);
    }
    if (autoStart.value && deepLinkEnabled.value) {
      setTimeout(() => {
        message.warning('注意，通过"快速启动"功能启动的隧道无法开机自启动');
      }, 200);
    }
  } catch (e) {
    message.error(`设置开机自启动失败: ${e}`);
  }
};

const toggleAutoRestoreTunnels = (value: boolean) => {
  autoRestoreTunnels.value = value;
  message.success(`${value ? '启用' : '禁用'}开机恢复隧道成功`);
  if (!value && autoStart.value) {
    setTimeout(() => {
      message.warning('已禁用开机恢复隧道，程序将在开机时启动但不会自动启动隧道');
    }, 500);
  }
};


onBeforeRouteLeave((_to, _from, next) => {
  if (downloading.value) {
    dialog.warning({
      title: '提示',
      content: '正在下载 frpc，离开页面将中断下载。确定要离开吗？',
      positiveText: '继续下载',
      negativeText: '离开',
      onPositiveClick: () => { next(false) },
      onNegativeClick: () => { next() }
    })
  } else {
    next();
  }
});

onMounted(async () => {
  getCurrentVersion();
});

const downloadFrpc = async () => {
  activeNames.value = ['1'];
  downloading.value = true;
  try {
    await invoke('download_frpc');
    message.success('下载成功');
  } catch (e) {
    message.error(`下载失败: ${e}`);
  } finally {
    downloading.value = false;
  }
};

const getFrpcVersion = async () => {
  try {
    const result = await invoke('get_frpc_cli_version');
    let frpcInfo;
    try {
      frpcInfo = JSON.parse(result as string);
    } catch (e) {
      console.error('解析失败，原始数据:', result); // 记录原始响应
      throw new Error(`数据解析失败: ${e instanceof Error ? e.message : String(e)}`);
    }
    
    // 添加字段验证
    if (!frpcInfo?.version || typeof frpcInfo.version !== 'string') {
      throw new Error('无效的版本信息格式');
    }
    if (frpcInfo.version === "未知") {
      logs.value += `${new Date().toLocaleTimeString()} [系统] 未找到Frpc\n`;
      message.warning('Frpc可执行文件不存在，请配置或下载');
      return;
    }
    logs.value += `${new Date().toLocaleTimeString()} [系统] 检测到Frpc版本: ${frpcInfo.version}\n`;
    message.success(`当前版本: ${frpcInfo.version}`);
  } catch (e) {
    logs.value += `${new Date().toLocaleTimeString()} [系统] 获取Frpc版本失败: ${e}\n`;
    message.error(`获取版本失败: ${e}`);
  }
};

const killAllProcesses = async () => {
  try {
    await invoke('kill_all_processes');
    message.success('已终止所有 frpc 进程');
  } catch (e) {
    message.error(`操作失败: ${e}`);
  }
};

const openAppDataDir = async () => {
  try {
    await invoke('open_app_data_dir');
    message.success('已打开数据目录');
  } catch (e) {
    console.error('打开数据目录失败:', e);
    message.error(`打开数据目录失败: ${e}`);
  }
};

const manualModeVisible = ref(false);
const showManualMode = () => {
  manualModeVisible.value = true;
};

const appDataDir = ref('');
const expectedFrpcFilename = ref('');
const getExpectedFrpcInfo = async () => {
  try {
    const frpcVersion = await invoke('get_frpc_cli_version') as {
      version: string;
      path: string;
      filename: string;
    };
    if (frpcVersion && frpcVersion.filename) {
      expectedFrpcFilename.value = frpcVersion.filename;
      return;
    }
  } catch (e) {
    console.error('获取frpc信息失败:', e);
  }
  const isWindows = navigator.platform.toLowerCase().includes('win');
  if (isWindows) {
    expectedFrpcFilename.value = 'frpc.exe';
  } else {
    const isMac = navigator.platform.toLowerCase().includes('mac');
    if (isMac) {
      const isArm = /arm|aarch/i.test(navigator.platform) ||
        (/Mac/.test(navigator.userAgent) && navigator.maxTouchPoints > 1);
      expectedFrpcFilename.value = isArm ? 'frpc_darwin_arm64' : 'frpc_darwin_amd64';
    } else {
      const isLinuxArm = /arm|aarch/i.test(navigator.platform);
      expectedFrpcFilename.value = isLinuxArm ? 'frpc_linux_arm64' : 'frpc_linux_amd64';
    }
  }
};

onMounted(async () => {
  try {
    appDataDir.value = await invoke('get_app_data_dir') as string;
    await getExpectedFrpcInfo();
  } catch (e) {
    console.error('获取应用数据目录失败:', e);
    message.error(`获取应用数据目录失败: ${e}`);
  }
});

const restoreUpdateNotification = () => {
  localStorage.removeItem('suppressUpdateNotification');
  message.success('已恢复更新提示');
};

const disableUpdateNotification = () => {
  localStorage.setItem('suppressUpdateNotification', 'true');
  message.success('已禁用更新提示');
};
</script>

<template>
    <div class="settings">
        <n-scrollbar>
            <n-space vertical>

            
            <n-card title="设置">
                <n-space vertical>
                <n-collapse v-model:expanded-names="activeNames" accordion>
                    <n-collapse-item title="版本信息" name="2">
                    <n-space vertical>
                        <n-text>当前版本：Beta v{{ currentVersion }}</n-text>
                        <n-space>
                        <n-button @click="checkUpdate" :loading="checking">
                            {{ checking ? '检查中...' : '检查更新' }}
                        </n-button>
                        <n-button @click="openAppDataDir">
                            打开软件数据目录
                        </n-button>
                        <n-button tertiary type="warning" @click="disableUpdateNotification">
                          禁用更新提示
                        </n-button>
                        <n-button tertiary type="info" @click="restoreUpdateNotification">
                          恢复更新提示
                        </n-button>
                        </n-space>
                    </n-space>
                    </n-collapse-item>
                    <n-collapse-item title="Frpc 管理" name="1">
                    <template #header-extra>
                        首次使用请在这里下载或配置 Frpc
                    </template>
                    <n-space>
                        <n-button @click="downloadFrpc" :loading="downloading" :disabled="downloading">
                        {{ downloading ? '正在进行操作...' : '自动下载/更新 Frpc' }}
                        </n-button>
                        <n-button @click="getFrpcVersion" :disabled="downloading">获取本地 Frpc 版本</n-button>
                        <n-button @click="showManualMode" :disabled="downloading">
                        手动配置 Frpc 可执行文件
                        </n-button>
                        <n-popconfirm 
                          @positive-click="killAllProcesses" 
                          :disabled="downloading"
                          positive-text="终止"
                          negative-text="取消">
                          <template #trigger>
                            <n-button type="warning" :disabled="downloading">终止所有 Frpc 进程</n-button>
                          </template>
                          确认终止所有 Frpc 进程？这将会断开所有连接
                        </n-popconfirm>
                    </n-space>
                    <br />
                    <n-card title="运行日志" class="mt-4">
                        <n-log :rows="10" :log="logs" :loading="false" trim />
                    </n-card>
                    </n-collapse-item>
                    <n-collapse-item title="启动设置" name="3">
                    <n-space vertical>
                        <n-space style="display: flex; margin-bottom: 10px; margin-top: 5px;">
                        <n-switch v-model:value="autoStart" @update:value="toggleAutoStart" />
                        <span>开机自启动</span>
                        </n-space>
                        <n-space style="display: flex;" v-if="autoStart">
                        <n-switch v-model:value="autoRestoreTunnels" @update:value="toggleAutoRestoreTunnels" />
                        <span>开机时恢复上次运行的隧道</span>
                        </n-space>
                    </n-space>
                    </n-collapse-item>
                </n-collapse>
                </n-space>
            </n-card>
            </n-space>

            <n-drawer v-model:show="manualModeVisible" :width="600" placement="right">
            <n-drawer-content title="手动配置 Frpc 可执行文件" closable>
                <n-space vertical>
                <n-alert type="info">
                    如果自动下载失败，您可以手动下载 Frpc 可执行文件并放置到程序数据目录
                </n-alert>
                <n-alert type="warning" title="注意">
                    请在 {{ packageData.title }} 管理面板 - 下载中心 下载对应操作系统和对应平台的 Frpc 可执行文件。 <br />
                    <br />
                    1. Windows 系统请下载 Windows 64 位版本的 Frpc 可执行文件 <br />
                    2. Mac 系统请下载 Mac 64 位版本的 Frpc 可执行文件 <br />
                    3. Linux 系统请下载 Linux 64 位版本的 Frpc 可执行文件 <br />
                    <br />
                    4. 下载完成后，将 Frpc 可执行文件放置到应用数据目录 <br />
                    5. 打开应用数据目录，找到 Frpc 可执行文件并将其重命名为 frpc.exe <br />

                </n-alert>
                <n-form-item label="应用数据目录">
                    <n-input v-model:value="appDataDir" readonly />
                    <n-button @click="openAppDataDir">
                    打开数据目录
                    </n-button>
                </n-form-item>
                <n-form-item label="Frpc 可执行文件名称">
                    <n-input v-model:value="expectedFrpcFilename" readonly />
                </n-form-item>
                </n-space>
                <template #footer><n-space justify="end">
                    <n-button @click="manualModeVisible = false">
                    关闭
                    </n-button>
                    <n-button type="primary" @click="getFrpcVersion(); manualModeVisible = false">
                    完成并检查
                    </n-button>
                </n-space></template>
            </n-drawer-content>
            </n-drawer>
        </n-scrollbar>
    </div>
</template>