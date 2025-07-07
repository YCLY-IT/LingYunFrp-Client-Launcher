<template>
  <div class="network-container">
    <n-alert v-if="!tapDriverInstalled" type="error" style="margin-bottom: 16px;">
      未检测到虚拟网卡驱动，请先安装驱动后再使用本功能。
    </n-alert>
    <n-modal v-model:show="showInstallDialog" title="驱动安装" preset="dialog" :mask-closable="false" :closable="false">
      <template #default>
        <div v-if="!downloading">
          <p>未检测到虚拟网卡驱动</p>
        </div>
        <div v-else>
          <n-progress type="line" :percentage="downloadProgress" />
          <p>下载进度：{{ downloadProgress }}%</p>
        </div>
      </template>
      <template #action>
        <n-button v-if="!downloading" type="primary" @click="showInstallDialog = false">确定</n-button>
      </template>
    </n-modal>
    <n-modal v-model:show="showAdminDialog" title="需要管理员权限" preset="dialog" :mask-closable="false" :closable="false">
      <template #default>
        <p>本功能需要以管理员权限运行，是否以管理员权限重启？</p>
      </template>
      <template #action>
        <n-button type="primary" @click="handleAdminDialog(true)">以管理员权限重启</n-button>
        <n-button @click="handleAdminDialog(false)">返回上一页</n-button>
      </template>
    </n-modal>
    <!-- 主要功能区域 -->
    <div class="main-content">
      <n-tabs v-model:value="activeTab" type="line" animated class="network-tabs">
        <!-- 当前连接 -->
        <n-tab-pane v-if="currentNetwork" name="status" tab="当前连接">
          <div class="tab-content">
            <n-card title="当前网络状态" class="status-card">
              <div class="network-status">
                <div class="network-info">
                  <h3>{{ currentNetwork.config.name }}</h3>
                  <div class="network-details">
                    <n-tag type="success" size="small">网络ID: {{ currentNetwork.config.network_id }}</n-tag>
                    <n-tag v-if="currentNetwork.config.password" type="warning" size="small">需要密码</n-tag>
                    <n-tag :type="currentNetwork.status === 'Active' ? 'success' : 'error'" size="small">
                      {{ currentNetwork.status === 'Active' ? '活跃' : '非活跃' }}
                    </n-tag>
                  </div>
                </div>
                <div class="connection-info">
                  <h4>连接信息</h4>
                  <n-alert type="info" style="margin-bottom: 16px">
                    <template #header>
                      <n-icon><InformationCircleOutline /></n-icon>
                      连接地址
                    </template>
                    将以下网络ID分享给其他用户：
                  </n-alert>
                  <div class="connection-details">
                    <div class="connection-item">
                      <span class="label">网络ID：</span>
                      <n-input :value="currentNetwork.config.network_id" readonly size="small">
                        <template #suffix>
                          <n-button text size="tiny" @click="copyToClipboard(currentNetwork.config.network_id)">
                            <template #icon>
                              <n-icon><CopyOutline /></n-icon>
                            </template>
                          </n-button>
                        </template>
                      </n-input>
                    </div>
                    <div class="connection-item">
                      <span class="label">虚拟子网：</span>
                      <n-input :value="currentNetwork.config.virtual_subnet" readonly size="small">
                        <template #suffix>
                          <n-button text size="tiny" @click="copyToClipboard(currentNetwork.config.virtual_subnet)">
                            <template #icon>
                              <n-icon><CopyOutline /></n-icon>
                            </template>
                          </n-button>
                        </template>
                      </n-input>
                    </div>
                  </div>
                </div>
                <div class="connections-section">
                  <h4>在线用户 ({{ Object.keys(currentNetwork.nodes).length }}/{{ currentNetwork.config.max_players }})</h4>
                  <div class="connections-list">
                    <!-- 房主节点 -->
                    <div
                      v-for="node in Object.values(currentNetwork.nodes).filter(n => n.is_owner)"
                      :key="node.id"
                      class="connection-item"
                    >
                      <div class="connection-avatar">
                        <n-icon size="20"><PersonOutline /></n-icon>
                      </div>
                      <div class="connection-info">
                        <span class="connection-name">{{ node.nickname }}</span>
                        <div class="connection-meta">
                          <span class="connection-status" :class="node.is_online ? 'online' : 'offline'">
                            {{ node.is_online ? '在线' : '离线' }}
                          </span>
                          <span class="connection-owner">房主</span>
                          <span class="virtual-ip">IP: {{ node.virtual_ip }}</span>
                        </div>
                      </div>
                    </div>
                    <!-- 其他节点 -->
                    <div
                      v-for="node in Object.values(currentNetwork.nodes).filter(n => !n.is_owner)"
                      :key="node.id"
                      class="connection-item"
                    >
                      <div class="connection-avatar">
                        <n-icon size="20"><PersonOutline /></n-icon>
                      </div>
                      <div class="connection-info">
                        <span class="connection-name">{{ node.nickname }}</span>
                        <div class="connection-meta">
                          <span class="connection-status" :class="node.is_online ? 'online' : 'offline'">
                            {{ node.is_online ? '在线' : '离线' }}
                          </span>
                          <span v-if="node.is_owner" class="connection-owner">创建者</span>
                          <span class="virtual-ip">IP: {{ node.virtual_ip }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
                <div class="network-actions">
                  <n-space>
                    <n-button type="info" @click="loadCurrentNetworkWithReset" :disabled="!tapDriverInstalled">
                      刷新状态
                    </n-button>
                    <n-button type="error" @click="leaveNetwork" :loading="leaving" :disabled="!tapDriverInstalled">
                      {{ leaving ? '断开中...' : '断开连接' }}
                    </n-button>
                  </n-space>
                </div>
              </div>
            </n-card>
          </div>
        </n-tab-pane>
        <!-- 创建网络 -->
        <n-tab-pane name="create" tab="创建网络">
          <div class="tab-content">
            <n-card title="创建新网络" class="create-network-card">
              <n-space vertical size="large">
                <n-form-item label="网络名称">
                  <n-input v-model:value="networkName" placeholder="请输入网络名称" :maxlength="20" show-count :disabled="!!currentNetwork || !tapDriverInstalled" />
                </n-form-item>
                <n-form-item label="网络密码（可选）">
                  <n-input v-model:value="networkPassword" placeholder="设置网络密码（可选）" type="password" show-password-on="click" :disabled="!!currentNetwork || !tapDriverInstalled" />
                </n-form-item>
                <n-form-item label="网络状态">
                  <n-switch v-model:value="networkIsPublic" :checked-value="true" :unchecked-value="false" :disabled="!!currentNetwork || !tapDriverInstalled" />
                  <span style="margin-left: 8px; color: #666;">{{ networkIsPublic ? '公开网络' : '私密网络' }}</span>
                </n-form-item>
                <n-form-item label="最大连接数">
                  <n-input-number v-model:value="maxConnections" :min="1" :max="50" placeholder="最大连接数" :disabled="!!currentNetwork || !tapDriverInstalled" />
                </n-form-item>
                
                <!-- 高级配置折叠栏 -->
                <n-collapse>
                  <n-collapse-item title="高级配置" name="advanced">
                    <n-space vertical size="large">
                      <n-form-item label="虚拟网络端口">
                        <n-input-number 
                          v-model:value="virtualNetworkPort" 
                          :min="1024" 
                          :max="65535" 
                          placeholder="虚拟网络端口" 
                          :disabled="!!currentNetwork || !tapDriverInstalled"
                          style="width: 100%;"
                        />
                        <template #feedback>
                          <span style="color: #666; font-size: 12px;">端口范围：1024-65535，默认：10000</span>
                        </template>
                      </n-form-item>
                    </n-space>
                  </n-collapse-item>
                </n-collapse>
                
                <n-button type="primary" @click="createNetwork" :loading="creating" :disabled="!networkName || !!currentNetwork || !tapDriverInstalled" block size="large">
                  {{ creating ? '创建中...' : currentNetwork ? '已连接网络' : '创建网络' }}
                </n-button>
              </n-space>
            </n-card>
          </div>
        </n-tab-pane>
        <!-- 加入网络 -->
        <n-tab-pane name="join" tab="加入网络">
          <div class="tab-content">
            <div class="join-section">
              <n-card title="加入网络" class="join-network-card">
                <n-space vertical size="large">
                  <n-form-item label="网络ID">
                    <n-input v-model:value="joinNetworkId" placeholder="请输入网络ID" :maxlength="10" :disabled="!!currentNetwork || !tapDriverInstalled" />
                  </n-form-item>
                  <n-form-item label="网络密码">
                    <n-input v-model:value="joinPassword" placeholder="请输入网络密码（如果有）" type="password" show-password-on="click" :disabled="!!currentNetwork || !tapDriverInstalled" />
                  </n-form-item>
                  <n-button type="info" @click="joinNetwork" :loading="joining" :disabled="!joinNetworkId || !!currentNetwork || !tapDriverInstalled" block size="large">
                    {{ joining ? '连接中...' : currentNetwork ? '已连接网络' : '加入网络' }}
                  </n-button>
                </n-space>
              </n-card>
            </div>
          </div>
        </n-tab-pane>
        <!-- 网络列表 -->
        <n-tab-pane name="networks" tab="网络列表">
          <div class="tab-content">
            <n-card title="网络列表" class="networks-card">
              <n-tabs type="line" animated>
                <n-tab-pane name="recent" tab="最近网络">
                  <div v-if="recentNetworks.length === 0" class="empty-state">
                    <n-empty description="暂无最近网络" size="small" />
                  </div>
                  <div v-else class="networks-container">
                    <div class="network-list">
                      <div v-for="network in recentNetworks" :key="network.id" class="network-item" :class="{ 'disabled': !!currentNetwork || !tapDriverInstalled }" @click="!currentNetwork && tapDriverInstalled && quickJoin(network)">
                        <div class="network-info">
                          <span class="network-name">{{ network.name }}</span>
                          <div class="network-meta">
                            <span class="network-id">网络: {{ network.name }} ({{ network.id }})</span>
                            <span class="network-time">{{ network.lastJoinTime }}</span>
                          </div>
                        </div>
                        <n-button text size="tiny">
                          <template #icon>
                            <n-icon><ArrowForwardOutline /></n-icon>
                          </template>
                        </n-button>
                      </div>
                    </div>
                  </div>
                </n-tab-pane>
                <n-tab-pane name="public" tab="公开网络">
                  <div class="public-networks-header">
                    <n-button type="primary" size="small" @click="refreshPublicNetworks" :loading="loadingPublicNetworks" :disabled="!tapDriverInstalled">刷新</n-button>
                  </div>
                  <div v-if="publicNetworks.length === 0" class="empty-state">
                    <n-empty description="暂无公开网络" size="small" />
                  </div>
                  <div v-else class="networks-container">
                    <div class="network-list">
                      <div v-for="network in publicNetworks" :key="network.id" class="network-item" :class="{ 'disabled': !!currentNetwork || !tapDriverInstalled }" @click="!currentNetwork && tapDriverInstalled && quickJoin(network)">
                        <div class="network-info">
                          <span class="network-name">{{ network.name }}</span>
                          <div class="network-meta">
                            <span class="network-id">网络: {{ network.name }} ({{ network.id }})</span>
                            <span class="network-connections">连接: {{ network.currentPlayers }}/{{ network.maxPlayers }}</span>
                            <span class="network-owner">创建者: {{ network.owner?.name }}</span>
                          </div>
                        </div>
                        <n-button text size="tiny">
                          <template #icon>
                            <n-icon><ArrowForwardOutline /></n-icon>
                          </template>
                        </n-button>
                      </div>
                    </div>
                  </div>
                </n-tab-pane>
              </n-tabs>
            </n-card>
          </div>
        </n-tab-pane>
        <!-- 当前连接 - 没有连接时显示在最后 -->
        <n-tab-pane v-if="!currentNetwork" name="status" tab="当前连接">
          <div class="tab-content">
            <n-card title="当前网络状态" class="status-card">
              <div class="empty-state">
                <n-empty description="暂无活动网络">
                  <template #icon>
                    <n-icon size="48" color="#d9d9d9">
                      <WifiOutline />
                    </n-icon>
                  </template>
                </n-empty>
              </div>
            </n-card>
          </div>
        </n-tab-pane>
      </n-tabs>
    </div>

    <!-- 帮助信息 -->
    <n-card title="使用说明" class="help-card" style="margin-top: 20px;">
      <div class="help-content">
        <div class="help-item">
          <h4>🌐 如何创建网络？</h4>
          <p>1. 输入网络名称</p>
          <p>2. 可选择设置网络密码</p>
          <p>3. 点击"创建网络"按钮</p>
        </div>
        <div class="help-item">
          <h4>🔗 如何加入网络？</h4>
          <p>1. 输入网络创建者提供的网络ID</p>
          <p>2. 如果网络设置了密码，请输入密码</p>
          <p>3. 点击"加入网络"按钮</p>
        </div>
        <div class="help-item">
          <h4>📋 分享连接信息</h4>
          <p>创建网络后，将显示的网络ID分享给其他用户即可建立虚拟网络连接。</p>
        </div>
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
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
  useNotification,
  useDialog,
  NSwitch,
  NTabs,
  NTabPane,
  NInputNumber,
  NCollapse,
  NCollapseItem,
} from 'naive-ui'
import {
  WifiOutline,
  PersonOutline,
  CopyOutline,
  ArrowForwardOutline,
  InformationCircleOutline
} from '@vicons/ionicons5'
import { NetworkListItem, VirtualNetwork } from '../../types/virtualNetwork'
import { useVirtualNetworkManager } from '../../composables/useVirtualNetworkManager'
import { VirtualNetworkAPI } from '../../net/user/virtualNetwork'
import { invoke } from '@tauri-apps/api/core'
import { emitTo, listen } from '@tauri-apps/api/event'
import { useRouter } from 'vue-router'

const message = useMessage()
const notification = useNotification()
const dialog = useDialog()
const router = useRouter()

// 表单数据
const networkName = ref('')
const networkPassword = ref('')
const networkIsPublic = ref(true)
const maxConnections = ref(10)
const virtualNetworkPort = ref(19417)
const joinNetworkId = ref('')
const joinPassword = ref('')

// 状态
const creating = ref(false)
const joining = ref(false)
const leaving = ref(false)
const loadingPublicNetworks = ref(false)
const activeTab = ref('create')

// 当前网络
const currentNetwork = ref<VirtualNetwork | null>(null)

// 网络列表
const recentNetworks = ref<NetworkListItem[]>([])
const publicNetworks = ref<NetworkListItem[]>([])

// 网络管理器
const networkManager = useVirtualNetworkManager()

const showInstallDialog = ref(false)
const downloading = ref(false)
const downloadProgress = ref(0)
const tapDriverInstalled = ref(true)
const showAdminDialog = ref(false)

const pollingTimer = ref<number | null>(null)
// 新增：标记虚拟网卡启动失败弹窗是否已弹出
const virtualNetworkErrorDialogShown = ref(false)

listen<number>('tap-driver-download-progress', (event) => {
  downloadProgress.value = event.payload
})

// 创建网络
const createNetwork = async () => {
  if (!tapDriverInstalled.value) {
    message.error('请先安装虚拟网卡驱动')
    showInstallDialog.value = true
    return
  }
  if (!networkName.value) {
    message.warning('请填写完整的网络信息')
    return
  }
  creating.value = true
  try {
    const response = await VirtualNetworkAPI.createNetwork({
      name: networkName.value,
      password: networkPassword.value || undefined,
      is_public: networkIsPublic.value,
      max_players: maxConnections.value
    })
    if (response.code === 0) {
      currentNetwork.value = response.data
      // === 通过 user_id 匹配本机节点 ===
      let myUserId = response.data.config.owner_id
      let myNode = response.data.nodes.find((n: any) => n.user_id === myUserId)
      if (!myNode) myNode = response.data.nodes[0]
      let myPublicAddr = myNode?.public_addr || ''
      let myLocalPort = virtualNetworkPort.value
      if (myPublicAddr) {
        const parts = myPublicAddr.split(':')
        if (parts.length >= 2) {
          myLocalPort = Number(parts[1]) || virtualNetworkPort.value
        }
      }
      const myVirtualIp = myNode?.virtual_ip
      if (myVirtualIp) {
        try {
          await invoke('start_virtual_network', { virtualIp: myVirtualIp })
          message.success('虚拟网卡已启动')
          // 启动虚拟网卡成功后再自动连接对端节点
          await invoke('auto_connect_peers', {
            nodes: response.data.nodes,
            myPublicAddr: myPublicAddr,
            myLocalPort: myLocalPort
          })
        } catch (e) {
          dialog.error({
            title: '虚拟网卡启动失败',
            content: '虚拟网卡启动失败: ' + e,
            positiveText: '手动启动',
            negativeText: '取消',
            onPositiveClick: async () => {
              try {
                await invoke('start_virtual_network', { virtualIp: myVirtualIp })
                message.success('虚拟网卡已启动')
                // 启动虚拟网卡成功后再自动连接对端节点
                await invoke('auto_connect_peers', {
                  nodes: response.data.nodes,
                  myPublicAddr: myPublicAddr,
                  myLocalPort: myLocalPort
                })
              } catch (e) {
                message.error('手动启动虚拟网卡失败: ' + e)
              }
            }
          })
        }
      } else {
        message.error('未找到本机虚拟IP，无法启动虚拟网卡')
      }
      await navigator.clipboard.writeText(response.data.config.network_id)
      message.success('网络创建成功！网络ID已复制到剪贴板')
      notification.success({
        title: '网络创建成功',
        description: `网络ID: ${response.data.config.network_id} 已复制到剪贴板，请分享给其他用户`,
        duration: 5000
      })
      await nextTick()
      activeTab.value = 'status'
      networkName.value = ''
      maxConnections.value = 10
      loadRecentNetworks()
    } else {
      // 创建失败时显示对话框
      dialog.error({
        title: '创建网络失败',
        content: response.message || '创建网络失败，请检查网络连接或稍后重试',
        positiveText: '重试',
        negativeText: '取消',
        onPositiveClick: () => {
          createNetwork()
        }
      })
    }
  } catch (error) {
    // 网络错误时显示对话框
    dialog.error({
      title: '网络错误',
      content: '创建网络失败，可能是网络连接问题或服务器暂时不可用。请检查网络连接后重试。',
      positiveText: '重试',
      negativeText: '取消',
      onPositiveClick: () => {
        createNetwork()
      }
    })
    // eslint-disable-next-line no-console
    console.error('创建网络错误:', error)
  } finally {
    creating.value = false
  }
}

// 加入网络
const joinNetwork = async () => {
  if (!joinNetworkId.value) {
    message.warning('请输入网络ID')
    return
  }
  joining.value = true
  try {
    const response = await VirtualNetworkAPI.joinNetwork({
      network_id: joinNetworkId.value,
      password: joinPassword.value || undefined
    })
    if (response.code === 0 && response.data) {
      currentNetwork.value = response.data
      // === 通过 user_id 匹配本机节点 ===
      let myUserId = response.data.config.owner_id
      let myNode = response.data.nodes.find((n: any) => n.user_id === myUserId)
      if (!myNode) myNode = response.data.nodes[0]
      let myPublicAddr = myNode?.public_addr || ''
      let myLocalPort = virtualNetworkPort.value
      if (myPublicAddr) {
        const parts = myPublicAddr.split(':')
        if (parts.length >= 2) {
          myLocalPort = Number(parts[1]) || virtualNetworkPort.value
        }
      }
      const myVirtualIp = myNode?.virtual_ip
      if (myVirtualIp) {
        try {
          await invoke('start_virtual_network', { virtual_ip: myVirtualIp })
          message.success('虚拟网卡已启动')
          // 启动虚拟网卡成功后再自动连接对端节点
          await invoke('auto_connect_peers', {
            nodes: response.data.nodes,
            my_public_addr: myPublicAddr,
            my_local_port: myLocalPort
          })
        } catch (e) {
          message.error('虚拟网卡启动失败')
        }
      } else {
        message.error('未找到本机虚拟IP，无法启动虚拟网卡')
      }
      message.success('成功加入网络！')
      await nextTick()
      activeTab.value = 'status'
      joinNetworkId.value = ''
      joinPassword.value = ''
      loadRecentNetworks()
    } else {
      // 加入失败时显示对话框
      dialog.error({
        title: '加入网络失败',
        content: response.message || '加入网络失败，请检查网络ID和密码是否正确',
        positiveText: '重试',
        negativeText: '取消',
        onPositiveClick: () => {
          joinNetwork()
        }
      })
    }
  } catch (error) {
    // 网络错误时显示对话框
    dialog.error({
      title: '网络错误',
      content: '加入网络失败，可能是网络连接问题或服务器暂时不可用。请检查网络连接后重试。',
      positiveText: '重试',
      negativeText: '取消',
      onPositiveClick: () => {
        joinNetwork()
      }
    })
    // eslint-disable-next-line no-console
    console.error('加入网络错误:', error)
  } finally {
    joining.value = false
  }
}

// 离开网络
const leaveNetwork = async () => {
  if (!currentNetwork.value) return
  // 检查是否为房主
  const myUserId = currentNetwork.value.config.owner_id
  const myNode = currentNetwork.value.nodes.find((n: any) => n.user_id === myUserId)
  if (myNode && myNode.is_owner) {
    dialog.warning({
      title: '房主离开提示',
      content: '您是该房间的房主，离开后房间将解散，所有成员都将断开连接。是否确认离开？',
      positiveText: '确认离开',
      negativeText: '取消',
      onPositiveClick: async () => {
        await doLeaveNetwork()
      }
    })
    return
  }
  await doLeaveNetwork()
}

// 真正执行离开网络的函数
const doLeaveNetwork = async () => {
  leaving.value = true
  try {
    const response = await VirtualNetworkAPI.leaveNetwork(currentNetwork.value.config.network_id)
    if (response.code === 0) {
      currentNetwork.value = null
      message.success('已断开网络连接')
      await nextTick()
      activeTab.value = 'create'
    } else {
      // 断开连接失败时显示对话框
      dialog.error({
        title: '断开连接失败',
        content: response.message || '断开网络连接失败，请稍后重试',
        positiveText: '重试',
        negativeText: '取消',
        onPositiveClick: () => {
          leaveNetwork()
        }
      })
    }
  } catch (error) {
    // 网络错误时显示对话框
    dialog.error({
      title: '网络错误',
      content: '断开网络连接失败，可能是网络连接问题。请检查网络连接后重试。',
      positiveText: '重试',
      negativeText: '取消',
      onPositiveClick: () => {
        leaveNetwork()
      }
    })
    // eslint-disable-next-line no-console
    console.error('断开连接错误:', error)
  } finally {
    leaving.value = false
  }
}

// 快速加入
const quickJoin = (network: NetworkListItem) => {
  if (currentNetwork.value) {
    message.warning('请先断开当前网络连接')
    return
  }
  joinNetworkId.value = network.id
  joinNetwork()
}

// 刷新公开网络列表
const refreshPublicNetworks = async () => {
  loadingPublicNetworks.value = true
  try {
    publicNetworks.value = await VirtualNetworkAPI.getPublicNetworks()
  } catch (error) {
    // 获取公开网络失败时显示对话框
    dialog.error({
      title: '获取网络列表失败',
      content: '获取公开网络列表失败，可能是网络连接问题。请检查网络连接后重试。',
      positiveText: '重试',
      negativeText: '取消',
      onPositiveClick: () => {
        refreshPublicNetworks()
      }
    })
    // eslint-disable-next-line no-console
    console.error('获取公开网络错误:', error)
  } finally {
    loadingPublicNetworks.value = false
  }
}

// 加载最近网络
const loadRecentNetworks = async () => {
  try {
    recentNetworks.value = await VirtualNetworkAPI.getRecentNetworks()
  } catch (error) {
    // eslint-disable-next-line no-console
    console.error('获取最近网络错误:', error)
  }
}

const isOkForStartNetwork = ref(false)
// 获取当前网络状态
const loadCurrentNetwork = async () => {
  try {
    const response = await VirtualNetworkAPI.getCurrentNetwork()
    console.log('当前网络状态完整响应:', JSON.stringify(response, null, 2))
    
    if (response.code === 0 && response.data) {
      currentNetwork.value = response.data
      console.log('当前网络状态:', response.data)
      // === 自动启动虚拟网卡和连接节点 ===
      let myUserId = response.data.config.owner_id
      let myNode = response.data.nodes.find((n: any) => n.user_id === myUserId)
      if (!myNode) myNode = response.data.nodes[0]
      let myPublicAddr = myNode?.public_addr || ''
      let myLocalPort = virtualNetworkPort.value
      if (myPublicAddr) {
        const parts = myPublicAddr.split(':')
        if (parts.length >= 2) {
          myLocalPort = Number(parts[1]) || virtualNetworkPort.value
        }
      }
      const myVirtualIp = myNode?.virtual_ip
      if (myVirtualIp && !isOkForStartNetwork.value) {
        try {
          await invoke('auto_connect_peers', {
            nodes: response.data.nodes,
            myPublicAddr: myPublicAddr,
            myLocalPort: myLocalPort
          })
          await invoke('start_virtual_network', { virtualIp: myVirtualIp })
          isOkForStartNetwork.value = true
          // 启动成功，重置弹窗标记
          virtualNetworkErrorDialogShown.value = false
        } catch (e) {
          // 只弹一次
          if (!virtualNetworkErrorDialogShown.value) {
            virtualNetworkErrorDialogShown.value = true
            if(e != "虚拟网卡已启动"){
              dialog.error({
                title: '虚拟网卡启动失败',
                content: '启动错误:' + e,
                positiveText: '重试',
                negativeText: '取消',
                onPositiveClick: () => {
                  virtualNetworkErrorDialogShown.value = false
                  loadCurrentNetwork()
                }
              })
            }
          }
        }
      }
      setTimeout(() => {
        if (activeTab.value !== 'status') {
          activeTab.value = 'status'
        }
      }, 50)
    } else {
      currentNetwork.value = null
      console.log('当前没有连接任何网络，响应:', response)
    }
  } catch (error) {
    // eslint-disable-next-line no-console
    console.error('获取当前网络状态错误:', error)
    message.error('获取网络状态失败')
  }
}

// 刷新状态按钮，主动刷新时重置弹窗标记
const loadCurrentNetworkWithReset = async () => {
  virtualNetworkErrorDialogShown.value = false
  await loadCurrentNetwork()
}

// 复制到剪贴板
const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    message.success('已复制到剪贴板')
  } catch (error) {
    message.error('复制失败')
  }
}

// 初始化网络管理器
const initNetworkManager = async () => {
  try {
    await networkManager.init()
    networkManager.onStatusChange((status) => {
      // eslint-disable-next-line no-console
      console.log('网络状态变化:', status)
    })
    networkManager.onMessage((msg) => {
      // eslint-disable-next-line no-console
      console.log('收到网络消息:', msg)
    })
  } catch (error) {
    // eslint-disable-next-line no-console
    console.error('初始化网络管理器失败:', error)
  }
}

const handleAdminDialog = async (agree: boolean) => {
  showAdminDialog.value = false
  if (agree) {
    // 以管理员权限重启
    await emitTo('main', 'request_admin')
  } else {
    // 返回上一页
    router.back()
  }
}

onMounted(async () => {
  // 检查管理员权限
  setTimeout(async () => {
    try {
      const isAdmin = await invoke<boolean>('is_admin')
      if (!isAdmin) {
        showAdminDialog.value = true
        return
      }
    } catch (e) {
      message.error('管理员权限检测失败')
    }
  }, 50)
  // 1. 检查驱动
  try {
    const installed = await invoke<boolean>('is_tap_driver_installed')
    tapDriverInstalled.value = installed
    if (!installed) {
      showInstallDialog.value = true
      message.error('未检测到虚拟网卡驱动，请先安装驱动后再使用')
      return // 关键：未安装直接 return，后续不执行
    }
  } catch (e) {
    tapDriverInstalled.value = false
    message.error('驱动检测失败，请检查环境')
    return // 检测失败也 return
  }
  // 2. 只有驱动检测通过才执行后续
  await initNetworkManager()
  await Promise.all([
    loadCurrentNetwork(),
    loadRecentNetworks(),
    refreshPublicNetworks()
  ])
  pollingTimer.value = window.setInterval(() => {
    loadCurrentNetwork()
  }, 10000)
})

onUnmounted(() => {
  networkManager.destroy()
  if (pollingTimer.value) {
    clearInterval(pollingTimer.value)
  }
})
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

.create-network-card, .join-network-card, .networks-card, .status-card {
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

.public-networks-header {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 16px;
}

.network-status {
  .network-info {
    margin-bottom: 20px;
    
    h3 {
      margin: 0 0 12px 0;
      font-size: 20px;
      font-weight: 600;
      color: var(--text-color-1);
    }
    
    .network-details {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }
  }
  
  .connection-info {
    margin-bottom: 20px;
    
    h4 {
      margin: 0 0 12px 0;
      font-size: 16px;
      font-weight: 500;
      color: var(--text-color-1);
    }
    
    .connection-details {
      .connection-item {
        display: flex;
        align-items: center;
        margin-bottom: 12px;
        
        .label {
          min-width: 100px;
          font-weight: 500;
          color: var(--text-color-2);
        }
        
        .n-input {
          flex: 1;
        }
      }
    }
  }
  
  .connections-section {
    margin-bottom: 20px;
    
    h4 {
      margin: 0 0 12px 0;
      font-size: 16px;
      font-weight: 500;
      color: var(--text-color-1);
    }
    
    .connections-list {
      .connection-item {
        display: flex;
        align-items: center;
        padding: 8px 0;
        border-bottom: 1px solid var(--border-color);
        
        &:last-child {
          border-bottom: none;
        }
        
        .connection-avatar {
          margin-right: 12px;
          color: var(--text-color-3);
        }
        
        .connection-info {
          flex: 1;
          display: flex;
          justify-content: space-between;
          align-items: center;
          
          .connection-name {
            font-weight: 500;
            color: var(--text-color-1);
          }
          
          .connection-meta {
            display: flex;
            align-items: center;
            gap: 8px;
          }
          
          .connection-status {
            font-size: 12px;
            padding: 2px 8px;
            border-radius: 10px;
            
            &.online {
              background-color: var(--success-color-suppl);
              color: var(--success-color);
            }
            
            &.connecting {
              background-color: var(--warning-color-suppl);
              color: var(--warning-color);
            }
            
            &.offline {
              background-color: var(--error-color-suppl);
              color: var(--error-color);
            }
          }
          
          .connection-owner {
            font-size: 12px;
            padding: 2px 8px;
            border-radius: 10px;
            background-color: var(--primary-color-suppl);
            color: var(--primary-color);
          }
          
          .virtual-ip {
            font-size: 12px;
            color: var(--text-color-3);
          }
        }
      }
    }
  }
  
  .network-actions {
    margin-top: 20px;
  }
}

.networks-container {
  max-height: 400px;
  overflow-y: auto;
}

.network-list {
  .network-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.2s;
    
    &:hover {
      background-color: var(--hover-color);
    }
    
    &.disabled {
      opacity: 0.6;
      cursor: not-allowed;
      
      &:hover {
        background-color: transparent;
      }
    }
    
    .network-info {
      flex: 1;
      
      .network-name {
        display: block;
        font-weight: 500;
        margin-bottom: 4px;
        color: var(--text-color-1);
      }
      
      .network-meta {
        display: flex;
        gap: 8px;
        flex-wrap: wrap;
      }
      
      .network-id {
        font-size: 12px;
        color: var(--text-color-3);
        margin-right: 8px;
      }
      
      .network-time {
        font-size: 12px;
        color: var(--text-color-4);
      }
      
      .network-connections {
        font-size: 12px;
        color: var(--primary-color);
      }
      
      .network-owner {
        font-size: 12px;
        color: var(--text-color-3);
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
}
</style> 