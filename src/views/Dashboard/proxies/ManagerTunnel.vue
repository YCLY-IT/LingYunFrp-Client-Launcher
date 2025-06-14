<template>
  <div class="proxies">
    <NCard title="隧道管理" class="tunnel-manager-card">
      <div class="toolbar">
        <div class="search-box">
          <NInput v-model:value="searchText" placeholder="搜索隧道..." clearable size="medium">
            <template #prefix>
              <NIcon>
                <SearchOutline />
              </NIcon>
            </template>
          </NInput>
        </div>

        <div class="toolbar-right">
          <NButtonGroup>
            <NButton :type="viewMode === 'grid' ? 'primary' : 'default'" @click="viewMode = 'grid'" size="medium">
              <template #icon>
                <NIcon>
                  <GridOutline />
                </NIcon>
              </template>
              <span class="view-text">网格</span><span class="view-suffix">视图</span>
            </NButton>
            <NButton :type="viewMode === 'list' ? 'primary' : 'default'" @click="viewMode = 'list'" size="medium">
              <template #icon>
                <NIcon>
                  <ListOutline />
                </NIcon>
              </template>
              <span class="view-text">列表</span><span class="view-suffix">视图</span>
            </NButton>
          </NButtonGroup>

          <NButton secondary @click="handleRefresh" size="medium">
            <template #icon>
              <NIcon>
                <RefreshOutline />
              </NIcon>
            </template>刷新
          </NButton>
        </div>
      </div>

      <!-- 网格视图 -->
      <div v-if="viewMode === 'grid'" class="proxy-grid">
        <template v-if="filteredProxies.length">
          <NCard v-for="proxy in filteredProxies" :key="proxy.proxyId" class="tunnel-card" hoverable>
            <template #header>
              <div class="tunnel-header">
                <NText class="tunnel-title" strong>
                  隧道: {{ proxy.proxyName }}
                </NText>
                <NSpace size="small">
                  <NTag :type="proxy.isOnline ? 'success' : 'error'" size="small" round>
                    {{ proxy.isOnline ? '在线' : '离线' }}
                  </NTag>
                  <NTag v-if="!node.status" type="error" size="small" round>
                    节点离线
                  </NTag>
                  <NTag v-if="proxy.is_banned" type="error" size="small" round>
                    已封禁
                  </NTag>
                  <NTag v-if="proxy.isDisabled" type="warning" size="small" round>
                    已禁用
                  </NTag>
                </NSpace>
              </div>
            </template>
            
            <NSpace vertical size="medium">
              <NDescriptions :column="1" size="small" label-placement="left">
                <NDescriptionsItem label="ID">
                  <NTag type="info" size="small" round># {{ proxy.proxyId }}</NTag>
                </NDescriptionsItem>
                <NDescriptionsItem label="协议">
                  <NTag :type="proxy.proxyType === 'http' || proxy.proxyType === 'https' ? 'warning' : 'success'" size="small" round>
                    {{ proxy.proxyType.toUpperCase() }}
                  </NTag>
                </NDescriptionsItem>
                <NDescriptionsItem :label="proxy.proxyType === 'http' || proxy.proxyType === 'https' ? '绑定域名' : '远程端口'">
                  <div v-if="proxy.proxyType === 'http' || proxy.proxyType === 'https'">
                    <NSpace size="small" wrap>
                      <NTag 
                        v-for="domain in JSON.parse(proxy.domain || '[]')" 
                        :key="domain" 
                        type="info" 
                        size="small" 
                        round
                        style="cursor: pointer"
                        @click="() => openUrl(proxy.proxyType, domain)"
                      >
                        {{ domain }}
                      </NTag>
                    </NSpace>
                  </div>
                  <NText v-else>{{ proxy.remotePort }}</NText>
                </NDescriptionsItem>
                <NDescriptionsItem label="节点">
                  <NText depth="3">{{ getNodeLabel(proxy.nodeId) }}</NText>
                </NDescriptionsItem>
              </NDescriptions>
            </NSpace>

            <template #action>
              <NSpace justify="space-between" align="center">
                <NDropdown :options="dropdownOptions(proxy)" @select="key => handleSelect(key, proxy)" trigger="click">
                  <NButton secondary size="small">
                    <template #icon>
                      <NIcon>
                        <BuildOutline />
                      </NIcon>
                    </template>
                    更多
                  </NButton>
                </NDropdown>
                <NSwitch 
                  :disabled="!node.status" 
                  :loading 
                  :value="proxy.isOnline" 
                  @click="handleStarProxy(proxy)"
                  size="medium"
                />
              </NSpace>
            </template>
          </NCard>
        </template>
        <div v-else class="empty-state">
          <NEmpty description="暂无隧道" size="large">
            <template #extra>
              <NButton type="primary" @click="() => router.push('/dashboard/proxy/create')">
                <template #icon>
                  <NIcon>
                    <AddOutline />
                  </NIcon>
                </template>
                创建隧道
              </NButton>
            </template>
          </NEmpty>
        </div>
      </div>

      <!-- 列表视图 -->
      <template v-else>
        <NDataTable 
          v-if="filteredProxies.length" 
          :columns="columns" 
          :data="filteredProxies"
          :bordered="false"
          :single-line="false"
          size="medium"
          class="tunnel-table"
        />
        <div v-else class="empty-state">
          <NEmpty description="暂无隧道" size="large">
            <template #extra>
              <NButton type="primary" @click="() => router.push('/proxy/create')">
                <template #icon>
                  <NIcon>
                    <AddOutline />
                  </NIcon>
                </template>
                创建隧道
              </NButton>
            </template>
          </NEmpty>
        </div>
      </template>
    </NCard>

    <!-- 远程地址信息弹窗 -->
    <NModal v-model:show="showModal" preset="dialog" title="隧道详细信息" style="width: 800px; max-width: 90vw">
      <template #header>
        <div>隧道详细信息</div>
      </template>
      <div v-if="selectedProxy" style="padding: 16px 0" :class="{
        'proxy-detail-container': selectedProxy.proxyType === 'http' || selectedProxy.proxyType === 'https'
      }">
        <div class="proxy-detail-left">
          <NDescriptions :column="1" size="medium" label-placement="left" bordered>
            <NDescriptionsItem label="状态">
              <NSpace size="small">
                <NTag :type="selectedProxy.isOnline ? 'success' : 'error'" size="small" round>
                  {{ selectedProxy.isOnline ? '在线' : '离线' }}
                </NTag>
                <NTag v-if="selectedProxy.is_banned" type="error" size="small" round>
                  已封禁
                </NTag>
              </NSpace>
            </NDescriptionsItem>
            <NDescriptionsItem label="隧道名称">
              <NText>{{ selectedProxy.proxyName }}</NText>
            </NDescriptionsItem>
            <NDescriptionsItem label="协议类型">
              <NTag type="info" size="small" round>{{ selectedProxy.proxyType.toUpperCase() }}</NTag>
            </NDescriptionsItem>
            <NDescriptionsItem label="本地端口">
              <NText code>{{ selectedProxy.localPort }}</NText>
            </NDescriptionsItem>
            <NDescriptionsItem label="本地地址">
              <NText code>{{ selectedProxy.localIp }}</NText>
            </NDescriptionsItem>
            <NDescriptionsItem label="节点名称">
              <NText>{{ getNodeLabel(selectedProxy.nodeId).split(' - ')[1] }}</NText>
            </NDescriptionsItem>
            <NDescriptionsItem v-if="selectedProxy.proxyType === 'http' || selectedProxy.proxyType === 'https'" label="绑定域名">
              <NSpace size="small" wrap>
                <NTag 
                  size="small" 
                  v-for="domain in JSON.parse(selectedProxy.domain || '[]')" 
                  :key="domain" 
                  type="info"
                  round
                  style="cursor: pointer" 
                  @click="selectedProxy && openUrl(selectedProxy.proxyType, domain)"
                >
                  {{ domain }}
                </NTag>
              </NSpace>
            </NDescriptionsItem>
            <NDescriptionsItem v-else label="链接地址">
              <NText code>
                {{nodeOptions.find(node => node.value === selectedProxy?.nodeId)?.hostname}}:{{
                  selectedProxy.remotePort
                }}
              </NText>
            </NDescriptionsItem>
          </NDescriptions>
        </div>
        <template v-if="selectedProxy.proxyType === 'http' || selectedProxy.proxyType === 'https'">
          <div class="proxy-detail-right">
            <NCard title="域名解析配置" size="small">
              <NAlert type="info" style="margin-bottom: 16px">
                添加以下信息至您的域名解析配置后，服务才会生效。
              </NAlert>
              <NDataTable 
                size="small" 
                :single-line="false"
                :data="JSON.parse(selectedProxy.domain || '[]').map(domain => ({
                  domain,
                  rootDomain: splitDomain(domain).rootDomain,
                  host: splitDomain(domain).host,
                  recordType: isIPAddress(nodeOptions.find(n => n.value === selectedProxy?.nodeId)?.hostname || '') ? 'A' : 'CNAME',
                  recordValue: nodeOptions.find(n => n.value === selectedProxy?.nodeId || '')?.hostname
                }))"
                :columns="[
                  { title: '根域名', key: 'rootDomain' },
                  { title: '主机记录', key: 'host' },
                  { title: '记录类型', key: 'recordType' },
                  { title: '记录值', key: 'recordValue', render: (row) => h(NText, { type: 'primary', code: true }, { default: () => row.recordValue }) }
                ]"
                :bordered="false"
              />
            </NCard>
          </div>
        </template>
      </div>
    </NModal>

    <!-- 删除确认弹窗 -->
    <NModal v-model:show="showDeleteModal" preset="dialog" title="是否删除此隧道？" style="width: 400px">
      <template #header>
        <div>删除确认</div>
      </template>
      <NText>确定要删除此隧道吗？此操作不可恢复。</NText>
      <template #action>
        <NSpace>
          <NButton size="small" @click="showDeleteModal = false">取消</NButton>
          <NButton size="small" type="error" :loading="loading" @click="handleDeleteConfirm">删除</NButton>
        </NSpace>
      </template>
    </NModal>

    <!-- 编辑隧道弹窗 -->
    <NModal v-model:show="showEditModal" preset="dialog" title="编辑隧道" style="width: 800px; max-width: 90vw">
      <NForm ref="editFormRef" :model="editForm" :rules="rules" label-placement="left" label-width="120"
             require-mark-placement="right-hanging" size="medium" style="padding-top: 12px;">
        <NFormItem label="隧道名称" path="proxyName">
          <NInput v-model:value="editForm.proxyName" placeholder="请输入隧道名称" />
        </NFormItem>
        <NFormItem label="本地地址" path="localIp">
          <NInput v-model:value="editForm.localIp" placeholder="请输入本地地址" />
        </NFormItem>
        <NFormItem label="本地端口" path="localPort">
          <NInputNumber v-model:value="editForm.localPort" :min="1" :max="65535" placeholder="请输入本地端口" />
        </NFormItem>
        <NFormItem v-if="editForm.proxyType !== 'http' && editForm.proxyType !== 'https'" label="远程端口"
                   path="remotePort">
          <NSpace>
            <NInputNumber v-model:value="editForm.remotePort" :min="1" :max="65535" placeholder="请输入远程端口" />
            <NButton size="medium" :loading="gettingFreePort" @click="handleGetFreePortForEdit">
              获取空闲端口
            </NButton>
          </NSpace>
        </NFormItem>
        <NFormItem v-if="editForm.proxyType === 'http' || editForm.proxyType === 'https'" label="绑定域名" path="domain">
          <NDynamicTags v-model:value="domainTags" :render-tag="renderDomainTag" @update:value="handleDomainsUpdate" />
        </NFormItem>

        <NDivider>高级配置</NDivider>
        <NText depth="3" style="padding-bottom: 15px; display: block;">
          提示：仅推荐技术用户使用, 一般用户请勿随意填写。请确保您的配置正确, 否则隧道可能无法启动。
        </NText>

        <NFormItem label="访问密钥" path="accessKey">
          <NInput v-model:value="editForm.accessKey" placeholder="访问密钥已不再支持" :disabled="true"/>
        </NFormItem>
        <NFormItem label="Host Header Rewrite" path="hostHeaderRewrite">
          <NInput v-model:value="editForm.hostHeaderRewrite" placeholder="请输入 Host 请求头重写值" />
        </NFormItem>
        <NFormItem label="X-From-Where" path="headerXFromWhere">
          <NInput v-model:value="editForm.headerXFromWhere" placeholder="请输入 X-From-Where 请求头值" />
        </NFormItem>
        <NFormItem label="Proxy Protocol" path="proxyProtocolVersion">
          <NSelect v-model:value="editForm.proxy_protocol_version" :options="[
            { label: '不启用', value: '' },
            { label: 'v1', value: 'v1' },
            { label: 'v2', value: 'v2' }
          ]" placeholder="Proxy Protocol Version" />
        </NFormItem>
        <NFormItem label="其他选项">
          <NSpace>
            <NSwitch v-model:value="editForm.use_encryption" :rail-style="switchButtonRailStyle">
              <template #checked>启用加密</template>
              <template #unchecked>禁用加密</template>
            </NSwitch>
            <NSwitch v-model:value="editForm.use_compression" :rail-style="switchButtonRailStyle">
              <template #checked>启用压缩</template>
              <template #unchecked>禁用压缩</template>
            </NSwitch>
          </NSpace>
        </NFormItem>
      </NForm>
      <template #action>
        <NSpace>
          <NButton size="small" @click="showEditModal = false">取消</NButton>
          <NButton size="small" type="primary" :loading="loading" @click="handleEditSubmit">确定</NButton>
        </NSpace>
      </template>
    </NModal>

    <!-- 禁用/启用确认弹窗 -->
    <NModal v-model:show="showToggleModal" preset="dialog" style="width: 400px">
      <template #header>
        <div>{{ toggleModalTitle }}</div>
      </template>
      <div>{{ toggleModalContent }}</div>
      <template #action>
        <NSpace>
          <NButton size="small" @click="showToggleModal = false">取消</NButton>
          <NButton size="small" type="primary" :loading="loading" @click="handleToggleConfirm">确定</NButton>
        </NSpace>
      </template>
    </NModal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h, watch } from 'vue'
import {
  NCard, NButton, NButtonGroup, NTag, NDataTable, NTable, NSpace, NIcon,
  NModal, NInput, NDropdown, NForm, NFormItem, NSelect, NInputNumber,
  useMessage, type FormInst, type FormRules, NDivider, NSwitch, NText,
  NEmpty, NAlert, NDynamicTags, NDescriptions, NDescriptionsItem
} from 'naive-ui'
import { GridOutline, ListOutline, BuildOutline, RefreshOutline, SearchOutline, InformationCircleOutline, CreateOutline, TrashOutline, PowerOutline, AddOutline, CopyOutline, DocumentOutline, EllipsisHorizontalCircleOutline } from '@vicons/ionicons5'
import type { Node, Proxy} from '../../../types'
import { switchButtonRailStyle } from '../../../constants/theme'
import { useRouter } from 'vue-router'
import {userApi} from "../../../net";
import { accessHandle } from "../../../net/base.ts";
import { invoke } from '@tauri-apps/api/core'

const isIPAddress = (hostname: string) => {
  const ipRegex = /^(\d{1,3}\.){3}\d{1,3}$/
  return ipRegex.test(hostname)
}

const splitDomain = (domain: string) => {
  const parts = domain.split('.')
  if (parts.length <= 2) {
    return {
      host: '@',
      rootDomain: domain
    }
  }
  return {
    host: parts[0],
    rootDomain: parts.slice(1).join('.')
  }
}

const message = useMessage()
const loading = ref(false)
const proxies = ref<Proxy[]>([])
const viewMode = ref<'grid' | 'list'>('grid')
const searchText = ref('')
const nodeOptions = ref<{ label: string; value: number; hostname: string }[]>([])
const node = ref<Node | null>(null)
const showModal = ref(false)
const selectedProxy = ref<Proxy | null>(null)
const showEditModal = ref(false)
const editFormRef = ref<FormInst | null>(null)

const editForm = ref({
  proxyId: 0,
  proxyName: '',
  localIp: '',
  localPort: 0,
  remotePort: 0,
  domain: '',
  location: '',
  accessKey: '',
  hostHeaderRewrite: '',
  headerXFromWhere: '',
  use_encryption: false,
  use_compression: false,
  proxy_protocol_version: '',
  proxyType: '',
  nodeId: 0
})
const router = useRouter()
const gettingFreePort = ref(false)
const token = ref('')
const domainTags = ref<string[]>([])


const rules: FormRules = {
  proxyName: {
    required: true,
    message: '请输入隧道名称',
    trigger: ['blur', 'input']
  },
  localIp: {
    required: true,
    message: '请输入本地地址',
    trigger: ['blur', 'input']
  },
  localPort: {
    required: true,
    type: 'number',
    message: '请输入本地端口',
    trigger: ['blur', 'input'],
    validator: (_rule: any, value: number) => {
      if (value < 1 || value > 65535) {
        return new Error('端口范围必须在 1-65535 之间')
      }
      return true
    }
  },
  remotePort: {
    required: true,
    message: '请输入远程端口',
    trigger: ['blur', 'input'],
    validator: (_rule: any, value: number) => {
      if (editForm.value.proxyType === 'http' || editForm.value.proxyType === 'https') {
        return true
      }
      if (value < 1 || value > 65535) {
        return new Error('端口范围必须在 1-65535 之间')
      }
      return true
    }
  },
  domain: {
    validator: (_rule: any, _value: string) => {
      if (editForm.value.proxyType === 'http' || editForm.value.proxyType === 'https') {
        if (!domainTags.value.length) {
          return new Error('请至少添加一个域名')
        }
      }
      return true
    },
    trigger: ['blur', 'change', 'input']
  }
}

// 过滤隧道列表
const filteredProxies = computed(() => {
  const search = searchText.value.toLowerCase()
  return proxies.value.filter(proxy =>
      proxy.proxyName.toLowerCase().includes(search) ||
      proxy.proxyType.toLowerCase().includes(search) ||
      (proxy.domain || '').toLowerCase().includes(search) ||
      getNodeLabel(proxy.nodeId).toLowerCase().includes(search)
  )
})

const handleRefresh = async () => {
  loading.value = true
  try {
    userApi.get("/proxy/list", accessHandle(), (data) => {
      if (data.code === 0) {
        proxies.value = data.data
      } else {
        message.error(data.message || '获取隧道列表失败')
      }
    }, (messageText) => {
      message.error(messageText || '获取隧道列表失败')
    }, (error) => {
      message.error(error.message || '获取隧道列表失败')
      loading.value = false
    })
  } catch (error: any) {
    message.error(error?.response?.data?.message || '获取隧道列表失败')
  } finally {
    loading.value = false
  }
}


// 获取节点列表
const fetchNodes = async () => {
  try {
    userApi.get("/proxy/node/list",  accessHandle(), (data) => {
      if (data.code === 0) {
        nodeOptions.value = (data.data || []).map((node: any) => ({
          label: node.name,
          value: node.nodeId,
          hostname: node.hostname
        }))
        node.value = data.data
      } else {
        message.error('获取节点列表失败')
      }
    }, () => {
    })
  } catch (error: any) {
    message.error('获取节点列表失败')
  }
}

const getNodeLabel = (nodeId: number) => {
  const node = nodeOptions.value?.find(node => node.value === nodeId)
  return node ? `#${nodeId} - ${node.label}` : `#${nodeId}`
}

// 初始化数据
const fetchNodesAndRefresh = async () => {
  await fetchNodes()
  await handleRefresh()
}

fetchNodesAndRefresh()

function renderIcon(icon: any) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const showToggleModal = ref(false)
const showKickModal = ref(false)
const proxyToOperate = ref<Proxy | null>(null)

const toggleModalTitle = computed(() => {
  if (!proxyToOperate.value) return ''
  return proxyToOperate.value.isDisabled ? '启用确认' : '禁用确认'
})

const toggleModalContent = computed(() => {
  if (!proxyToOperate.value) return ''
  return proxyToOperate.value.isDisabled ? '确认要启用此隧道吗？' : '确认要禁用此隧道吗？'
})

const handleToken = async () => {
  try {
    userApi.get("/user/info/token", accessHandle(), (data) => {
      if (data.code === 0) {
        token.value = data.data.token.token
      } else {
        message.error(data.message || '获取Token失败')
      }
    }, () => {
      message.error('获取Token失败')
    })
  } catch (error: any) {
    message.error('获取Token失败')
  }
}
handleToken()

const dropdownOptions = (proxy: Proxy) => [
  {
    label: '查看详情',
    key: 'view',
    icon: renderIcon(InformationCircleOutline)
  },
  {
    type: 'divider',
    key: 'd1'
  },
  {
    label: '编辑',
    key: 'edit',
    icon: renderIcon(CreateOutline)
  },
  {
    type: 'divider',
    key: 'd2'
  },
  {
    label: proxy.isDisabled ? '启用' : '禁用',
    key: 'toggle',
    icon: renderIcon(PowerOutline)
  },
  {
    label: '删除',
    key: 'delete',
    icon: renderIcon(TrashOutline)
  }
]

const handleToggleClick = (proxy: Proxy) => {
  proxyToOperate.value = proxy
  showToggleModal.value = true
}

const handleToggleConfirm = async () => {
  if (!proxyToOperate.value) return
  try {
    loading.value = true
    userApi.post("/proxy/toggle", {
      proxyId: proxyToOperate.value.proxyId,
      isDisabled: !proxyToOperate.value.isDisabled
    }, accessHandle(), (data) => {
      if (data.code === 0) {
        message.success('操作成功')
        showToggleModal.value = false
        handleRefresh()
      } else {
        message.error(data.message || '操作失败')
      }
    }, (messageText) => {
      message.error(messageText || '操作失败')
    })
  } catch (error) {
    message.error(error)
  } finally {
    loading.value = false
    showToggleModal.value = false
  }
}

const handleKickClick = (proxy: Proxy) => {
  proxyToOperate.value = proxy
  showKickModal.value = true
}

const handleEdit = (proxy: Proxy) => {
  editForm.value = {
    proxyId: proxy.proxyId,
    proxyName: proxy.proxyName,
    localIp: proxy.localIp,
    localPort: proxy.localPort,
    remotePort: proxy.remotePort,
    domain: proxy.domain || '',
    location: proxy.location || '',
    accessKey: '',
    hostHeaderRewrite: proxy.hostHeaderRewrite || '',
    headerXFromWhere: proxy.headerXFromWhere || '',
    use_encryption: proxy.useEncryption || false,
    use_compression: proxy.useCompression || false,
    proxy_protocol_version: proxy.proxyProtocolVersion || '',
    proxyType: proxy.proxyType,
    nodeId: proxy.nodeId
  }
  // 处理域名数组
  try {
    domainTags.value = proxy.domain ? JSON.parse(proxy.domain) : []
  } catch {
    domainTags.value = proxy.domain ? [proxy.domain] : []
  }
  showEditModal.value = true
}

const handleEditSubmit = () => {
  editFormRef.value?.validate(async (errors) => {
    if (!errors) {
      loading.value = true
      try {
        userApi.post("/proxy/update", editForm.value, accessHandle(), (data) => {
          if (data.code === 0) {
            message.success('更新隧道成功')
            showEditModal.value = false
            handleRefresh()
          } else {
            message.error(data.message || '更新隧道失败')
          }
        }, (messageText) => {
          message.error("更新隧道失败:" + messageText || '更新隧道失败')
        }, () => {
          loading.value = false
        })
      } catch (error: any) {
        message.error(error?.response?.data?.message || '更新隧道失败')
      } finally {
        loading.value = false
      }
    }
  })
}

// 监听协议类型变化
watch(() => editForm.value.proxyType, (newType) => {
  if (newType !== 'http' && newType !== 'https') {
    domainTags.value = []
    editForm.value.domain = ''
  }
})

const showDeleteModal = ref(false)
const proxyToDelete = ref<Proxy | null>(null)

const handleDeleteClick = (proxy: Proxy) => {
  proxyToDelete.value = proxy
  showDeleteModal.value = true
}

const handleDeleteConfirm = async () => {
  if (!proxyToDelete.value) return
  try {
    userApi.post("/proxy/delete", {
      proxyId: proxyToDelete.value.proxyId
    }, accessHandle(), (data) => {
      console.log(data)
      if (data.code === 0) {
        message.success('删除隧道成功')
        handleRefresh()
      } else {
        message.error('删除隧道失败')
      }
    }, (messageText) => {
      loading.value = false
      message.error(messageText || '删除隧道失败')
    })
    showDeleteModal.value = false
  } catch (error: any) {
    message.error(error?.response?.data?.message || '删除隧道失败')
  }
}

const handleSelect = (key: string, proxy: Proxy) => {
  switch (key) {
    case 'view':
      selectedProxy.value = proxy
      showModal.value = true
      break
    case 'edit':
      handleEdit(proxy)
      break
    case 'kickProxy':
      handleKickClick(proxy)
      break
    case 'toggle':
      handleToggleClick(proxy)
      break
    case 'delete':
      handleDeleteClick(proxy)
      break
  }
}

const handleGetFreePortForEdit = async () => {
  try {
    gettingFreePort.value = true
    userApi.post("/proxy/freePort", {
      nodeId: editForm.value.nodeId,
      protocol: editForm.value.proxyType === 'udp' ? 'udp' : 'tcp'
      }, accessHandle(), (data) => {
      if (data.code === 0) {
        editForm.value.remotePort = data.data
      } else {
        message.error(data.message || '获取空闲端口失败')
      }
    })
  } catch (error: any) {
    message.error(error?.response?.data?.message || '获取空闲端口失败')
  } finally {
    gettingFreePort.value = false
  }
}
const openUrl = (protocol: string, domain: string) => {
  window.open(`${protocol}://${domain}`, '_blank')
}

const handleDomainsUpdate = (tags: string[]) => {
  domainTags.value = tags
  editForm.value.domain = JSON.stringify(tags)
}

const checkFrpcHas = async () => {
  try {
    const hasFrpc = await invoke<boolean>('check_frpc_exists')
    if (!hasFrpc) {
      message.warning('未检测到frpc.exe，请确保已正确安装客户端')
    }
  } catch (error) {
    console.error('检查frpc.exe失败:', error)
  }
}

checkFrpcHas()
const renderDomainTag = (tag: string) => {
  return h(
      NTag,
      {
        round: true,
        closable: true,
        style: 'cursor: pointer',
        onClose: () => {
          const index = domainTags.value.indexOf(tag)
          if (index !== -1) {
            const newTags = [...domainTags.value]
            newTags.splice(index, 1)
            handleDomainsUpdate(newTags)
          }
        },
        onDblclick: (e: { target: HTMLElement }) => {
          const tagEl = e.target as HTMLElement
          const input = document.createElement('input')
          input.style.width = '100px'
          input.value = tag
          input.onkeydown = (e) => {
            if (e.key === 'Enter') {
              const newValue = input.value.trim()
              if (newValue && newValue !== tag) {
                const index = domainTags.value.indexOf(tag)
                if (index !== -1) {
                  const newTags = [...domainTags.value]
                  newTags[index] = newValue
                  handleDomainsUpdate(newTags)
                }
              }
              input.remove()
            }
            if (e.key === 'Escape') {
              input.remove()
            }
          }
          input.onblur = () => {
            const newValue = input.value.trim()
            if (newValue && newValue !== tag) {
              const index = domainTags.value.indexOf(tag)
              if (index !== -1) {
                const newTags = [...domainTags.value]
                newTags[index] = newValue
                handleDomainsUpdate(newTags)
              }
            }
            input.remove()
          }
          tagEl.appendChild(input)
          input.focus()
        }
      },
      { default: () => tag }
  )
}

const columns = [
  {
    title: 'ID',
    key: 'proxyId',
    render(row) {
      return h(NTag, { type: 'info', size: 'medium', round: true }, { default: () => `# ${row.proxyId}` })
    }
  },
  {
    title: '名称',
    key: 'proxyName',
    render(row) {
      return h(NText, { style: 'white-space: nowrap; overflow: hidden; text-overflow: ellipsis;' }, { default: () => row.proxyName })
    }
  },
  {
    title: '类型',
    key: 'proxyType',
    render(row) {
      return h(NTag, { size: 'small', round: true }, { default: () => row.proxyType.toUpperCase() })
    }
  },
  {
    title: '远程端口',
    key: 'remotePort',
    render(row) {
      return h(NText, { code: true }, { default: () => row.remotePort })
    }
  },
  {
    title: '节点',
    key: 'nodeId',
    render(row) {
      return h(NText, { depth: 3, style: 'white-space: nowrap; overflow: hidden; text-overflow: ellipsis;' }, { default: () => getNodeLabel(row.nodeId) })
    }
  },
  {
    title: '状态',
    key: 'status',
    render(row) {
      return h(NSpace, { size: 4 }, {
        default: () => [
          h(NTag, {
            type: row.isOnline ? 'success' : 'error',
            size: 'small',
            round: true
          }, { default: () => row.isOnline ? '在线' : '离线' }),
          row.isBanned && h(NTag, {
            type: 'error',
            size: 'small',
            round: true
          }, { default: () => '已封禁' }),
          row.isDisabled && h(NTag, {
            type: 'warning',
            size: 'small',
            round: true
          }, { default: () => '已禁用' })
        ].filter(Boolean)
      })
    }
  },
  {
    title: '操作',
    key: 'actions',
    render(row) {
      return h(NDropdown, {
        trigger: 'click',
        options: dropdownOptions(row),
        onSelect: (key: string) => handleSelect(key, row),
        placement: 'bottom'
      }, {
        default: () => h(NButton, {
          text: true,
          style: 'display: flex; align-items: center;'
        }, {
          icon: () => h(NIcon, null, {
            default: () => h(EllipsisHorizontalCircleOutline)
          })
        })
      })
    }
  }
]
const handleStarProxy = async (proxy: Proxy) => {
  if (!node.value?.status) {
    message.error('节点离线，请先启用节点后再操作');
    return
  }
  if (proxy.isDisabled) {
    message.error('此隧道已被禁用，请先启用后再操作');
    return
  }
  try {
    loading.value = true;
    if (!proxy.isOnline) {
      const success = await invoke('start_proxy', {
        proxyId: proxy.proxyId,
        token: token.value
      });
      
      // 在隧道启动成功的回调处添加日志记录
      if (success) {
        message.info('正在尝试启动隧道');
      }
      setTimeout(() => {
        handleRefresh();
        if (proxy.isOnline) {
          message.success('隧道启动成功');
        } else {
          message.error('隧道启动失败，请检查配置或网络连接');
        }
      }, 200);
    } else {
      const success = await invoke('stop_proxy', { 
        proxyId: proxy.proxyId 
      });
      
      if (success) {
        proxy.isOnline = false;
        message.success('隧道停止成功');
      }
    }
  } catch (error) {
    message.error(`操作失败: ${error}`);
    console.error('隧道操作失败:', error);
  } finally {
    loading.value = false;
    setTimeout(() => {
      handleRefresh();
    }, 500);
  }
}
</script>

<style scoped>
.tunnel-manager-card {
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.toolbar {
  margin-bottom: 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.search-box {
  flex: 1;
  min-width: 200px;
}

.toolbar-right {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-shrink: 0;
}

.proxy-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
  min-height: 200px;
}

.tunnel-card {
  border-radius: 12px;
  transition: all 0.3s ease;
}

.tunnel-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}

.tunnel-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
}

.tunnel-title {
  font-size: 16px;
  font-weight: 600;
  flex: 1;
  word-break: break-word;
}

.empty-state {
  grid-column: 1 / -1;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 300px;
}

.tunnel-table {
  border-radius: 8px;
}

.proxy-detail-container {
  display: flex;
  gap: 24px;
}

.proxy-detail-left {
  flex: 1.2;
}

.proxy-detail-right {
  flex: 1.5;
  min-width: 300px;
}

@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .toolbar-right {
    justify-content: space-between;
  }

  .proxy-grid {
    grid-template-columns: 1fr;
  }

  .proxy-detail-container {
    flex-direction: column;
  }

  .proxy-detail-right {
    min-width: unset;
  }

  .view-suffix {
    display: none;
  }
}

@media (max-width: 480px) {
  .tunnel-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
}
</style>