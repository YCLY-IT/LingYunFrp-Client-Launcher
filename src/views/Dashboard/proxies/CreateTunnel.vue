<template>
  <div class="content-grid flex flex-col justify-center gap-5">
    <!-- 实名认证提示弹窗 -->
    <NModal
      v-model:show="showRealnameModal"
      preset="dialog"
      title="未实名认证提示"
      :show-icon="false"
      style="width: 400px"
    >
      <div>您的账户尚未完成实名认证, 请尽快完成实名认证。<br /></div>
      <div style="margin-top: 12px; text-align: right">
        <NText depth="3">{{ countDown }}秒后自动关闭</NText>
      </div>
      <template #action>
        <NButton size="small" @click="closeModal('realname')">关闭</NButton>
        <NButton size="small" type="primary" @click="goToRealname"
          >立即前往</NButton
        >
      </template>
    </NModal>

    <!-- 搜索和区域筛选 -->
    <NCard title="筛选选项" class="filter-card mx-auto w-full max-w-[1200px] max-[1200px]:max-w-[900px] [&_.n-card-header]:border-b [&_.n-card-header]:border-b-[var(--n-border-color)] [&_.n-card__content]:p-4 max-md:[&_.n-grid]:grid-cols-1!">
      <NSpace vertical size="medium">
        <NInput
          style="margin-top: 10px"
          v-model:value="searchQuery"
          placeholder="搜索节点..."
          clearable
        >
          <template #prefix>
            <NIcon>
              <SearchOutline />
            </NIcon>
          </template>
        </NInput>

        <div class="filter-row flex w-full min-w-0 flex-nowrap items-center gap-[30px] max-md:w-full max-md:flex-col max-md:flex-wrap max-md:gap-2.5" style="margin-top: 5px">
          <div class="group-filter flex-none max-md:w-full">
            <NText>用户组筛选：</NText>
            <NSelect
              class="group-select w-[250px] max-w-full max-md:w-full!"
              style="width: 300px"
              v-model:value="selectedGroup"
              :options="[{ label: '全部', value: 'all' }, ...groupList]"
              clearable
              placeholder="请选择用户组"
            />
          </div>
          <div class="protocol-filter min-w-0 flex-1 max-md:w-full">
            <NText>协议筛选：</NText>
            <NSelect
              class="protocol-select box-border w-[340px] min-w-0 max-w-full max-md:w-full!"
              style="width: 100%"
              v-model:value="selectedProtocols"
              :options="protocolOptions"
              multiple
              clearable
              placeholder="请选择协议"
            />
          </div>
          <div class="latency-refresh flex flex-none items-end pt-[22px] max-md:w-full max-md:items-start max-md:pb-0">
            <NButton
              type="primary"
              size="medium"
              :loading="refreshingLatency"
              @click="handleRefreshLatency"
            >
              <template #icon>
                <NIcon>
                  <RefreshOutline />
                </NIcon>
              </template>
              刷新延迟
            </NButton>
          </div>
        </div>
      </NSpace>
    </NCard>

    <!-- 节点选择卡片 - 修改为折叠篮按地区分组 -->
    <NCard title="选择节点" class="node-card mx-auto w-full max-w-[1200px] max-[1200px]:max-w-[900px] [&_.n-card-header]:border-b [&_.n-card-header]:border-b-[var(--n-border-color)] [&_.n-card__content]:p-4 max-md:[&_.n-grid]:grid-cols-1!">
      <NSpin :show="nodeLoading" tip="节点加载中...">
        <NSpace vertical>
          <NCollapse v-model:expanded-names="expandedRegion">
            <NCollapseItem title="中国大陆" name="cn">
              <NGrid
                x-gap="8"
                y-gap="8"
                cols="3"
                responsive="screen"
                style="padding-top: 14px"
              >
                <NGridItem
                  v-for="node in filteredNodes.filter(
                    (n) => n.location === 'cn',
                  )"
                  :key="node.value"
                >
                  <NCard
                    hoverable
                    @click="handleNodeSelect(node)"
                    :class="[
                      {
                        'selected-node': selectedNodeId === node.value,
                        'node-offline': !node.isOnline,
                      },
                    ]"
                    class="node-item relative h-full animate-rise-in cursor-pointer border border-[var(--n-border-color)] transition-all duration-200 [&.selected-node]:border-[rgba(33,150,243,0.3)]! [&.selected-node]:bg-[rgba(33,150,243,0.02)] [&.selected-node]:shadow-[0_0_4px_rgba(33,150,243,0.15)] [&.selected-node:hover]:bg-[rgba(33,150,243,0.04)] [&.node-offline]:opacity-50 [&.node-offline]:grayscale-[0.4]"
                  >
                    <div class="node-header mb-2 flex items-start justify-between">
                      <div
                        class="node-title flex items-center gap-2"
                        style="
                          display: flex;
                          align-items: center;
                          overflow: hidden;
                        "
                      >
                        <NTag type="info" size="small" style="flex-shrink: 0"
                          ># {{ node.id }}</NTag
                        >
                        <NTooltip trigger="hover">
                          <template #trigger>
                            <NText
                              style="
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                margin-left: -3px;
                                margin-right: 4px;
                                flex: 1;
                                min-width: 0;
                                cursor: pointer;
                              "
                              >{{ node.name
                              }}<span
                                :style="{
                                  color: getLoadStatusColor(node.loadStatus),
                                }"
                                >({{
                                  getLoadStatusText(node.loadStatus)
                                }})</span
                              ></NText
                            >
                          </template>
                          {{ node.name }} ({{
                            getLoadStatusTooltip(node.loadStatus)
                          }})
                        </NTooltip>
                        <NTag
                          :type="getLatencyInfo(node.latency).type"
                          size="small"
                          style="flex-shrink: 0"
                        >
                          {{ getLatencyInfo(node.latency).text }}
                        </NTag>
                      </div>
                      <!-- <div class="node-tags">
                        <NTag
                          v-if="supportsUdp(node)"
                          type="success"
                          size="small"
                          >UDP</NTag
                        >
                        <NTag
                          v-if="supportsHttp(node) && supportsHttps(node)"
                          type="success"
                          size="small"
                          >HTTP(S)</NTag
                        >
                        <NTag
                          v-else-if="supportsHttp(node)"
                          type="success"
                          size="small"
                          >HTTP</NTag
                        >
                        <NTag
                          v-else-if="supportsHttps(node)"
                          type="success"
                          size="small"
                          >HTTPS</NTag
                        >
                      </div> -->
                    </div>
                    <NText depth="3" style="font-size: 13px; margin: 6px 0">{{
                      node.description
                    }}</NText>
                    <NSpace vertical style="margin-top: 4px">
                      <div class="info-item mb-0.5 flex items-start last:mb-0">
                        <NSpace wrap>
                          <NTag
                            v-for="group in node.allowGroups.filter(
                              (g) =>
                                !['admin', 'proxies', 'traffic'].includes(
                                  g.name.trim().toLowerCase(),
                                ),
                            )"
                            :key="group.name"
                            size="small"
                            type="info"
                          >
                            {{ group.friendlyName }}
                          </NTag>
                        </NSpace>
                      </div>
                      <div class="info-item mb-0.5 flex items-start last:mb-0" style="margin-top: -1px">
                        <NSpace wrap>
                          <NTag
                            v-if="supportsTcp(node)"
                            type="success"
                            size="small"
                            >TCP</NTag
                          >
                          <NTag
                            v-if="supportsUdp(node)"
                            type="success"
                            size="small"
                            >UDP</NTag
                          >
                          <NTag
                            v-if="supportsHttp(node) && supportsHttps(node)"
                            type="success"
                            size="small"
                            >HTTP(S)</NTag
                          >
                          <NTag
                            v-else-if="supportsHttp(node)"
                            type="success"
                            size="small"
                            >HTTP</NTag
                          >
                          <NTag
                            v-else-if="supportsHttps(node)"
                            type="success"
                            size="small"
                            >HTTPS</NTag
                          >
                          <NTag
                            v-if="supportsStcp(node)"
                            type="success"
                            size="small"
                            >STCP</NTag
                          >
                          <NTag
                            v-if="supportsXtcp(node)"
                            type="success"
                            size="small"
                            >XTCP</NTag
                          >
                        </NSpace>
                      </div>
                      <div class="info-item mb-0.5 flex items-start last:mb-0" style="margin-top: -1px">
                        <NSpace wrap>
                          <NTag type="warning" size="small">
                            {{ node.portRange.min }} - {{ node.portRange.max }}
                          </NTag>
                          <NTag type="info" size="small">
                            {{ node.bandWidth }} Mbps
                          </NTag>
                          <NTag
                            v-if="node.needRealname"
                            type="info"
                            size="small"
                          >
                            实名
                          </NTag>
                        </NSpace>
                      </div>
                    </NSpace>
                  </NCard>
                </NGridItem>
              </NGrid>
              <div
                v-if="
                  filteredNodes.filter((n) => n.location === 'cn').length === 0
                "
                class="no-results py-10 text-center"
              >
                <NEmpty description="没有找到符合条件的节点" />
              </div>
            </NCollapseItem>
            <NCollapseItem title="中国港澳台" name="cn-out">
              <NGrid
                x-gap="8"
                y-gap="8"
                cols="3"
                responsive="screen"
                style="padding-top: 14px"
              >
                <NGridItem
                  v-for="node in filteredNodes.filter(
                    (n) => n.location === 'cn-out',
                  )"
                  :key="node.value"
                >
                  <NCard
                    hoverable
                    @click="handleNodeSelect(node)"
                    :class="[
                      {
                        'selected-node': selectedNodeId === node.value,
                        'node-offline': !node.isOnline,
                      },
                    ]"
                    class="node-item relative h-full animate-rise-in cursor-pointer border border-[var(--n-border-color)] transition-all duration-200 [&.selected-node]:border-[rgba(33,150,243,0.3)]! [&.selected-node]:bg-[rgba(33,150,243,0.02)] [&.selected-node]:shadow-[0_0_4px_rgba(33,150,243,0.15)] [&.selected-node:hover]:bg-[rgba(33,150,243,0.04)] [&.node-offline]:opacity-50 [&.node-offline]:grayscale-[0.4]"
                  >
                    <div class="node-header mb-2 flex items-start justify-between">
                      <div
                        class="node-title flex items-center gap-2"
                        style="
                          display: flex;
                          align-items: center;
                          overflow: hidden;
                        "
                      >
                        <NTag type="info" size="small" style="flex-shrink: 0"
                          ># {{ node.id }}</NTag
                        >
                        <NTooltip trigger="hover">
                          <template #trigger>
                            <NText
                              style="
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                flex: 1;
                                min-width: 0;
                                cursor: pointer;
                              "
                              >{{ node.name
                              }}<span
                                :style="{
                                  color: getLoadStatusColor(node.loadStatus),
                                }"
                                >({{
                                  getLoadStatusText(node.loadStatus)
                                }})</span
                              ></NText
                            >
                          </template>
                          {{ node.name }} ({{
                            getLoadStatusTooltip(node.loadStatus)
                          }})
                        </NTooltip>
                        <NTag
                          :type="getLatencyInfo(node.latency).type"
                          size="small"
                          style="flex-shrink: 0"
                        >
                          {{ getLatencyInfo(node.latency).text }}
                        </NTag>
                      </div>
                      <!-- <div class="node-tags">
                        <NTag
                          v-if="supportsUdp(node)"
                          type="success"
                          size="small"
                          >UDP</NTag
                        >
                        <NTag
                          v-if="supportsHttp(node) && supportsHttps(node)"
                          type="success"
                          size="small"
                          >HTTP(S)</NTag
                        >
                        <NTag
                          v-else-if="supportsHttp(node)"
                          type="success"
                          size="small"
                          >HTTP</NTag
                        >
                        <NTag
                          v-else-if="supportsHttps(node)"
                          type="success"
                          size="small"
                          >HTTPS</NTag
                        >
                        <NTag
                          v-if="supportsStcp(node)"
                          type="success"
                          size="small"
                          >STCP</NTag
                        >
                        <NTag
                          v-if="supportsXtcp(node)"
                          type="success"
                          size="small"
                          >XTCP</NTag
                        >
                      </div> -->
                    </div>
                    <NText depth="3" style="font-size: 13px; margin: 6px 0">{{
                      node.description
                    }}</NText>
                    <NSpace vertical style="margin-top: 4px">
                      <div class="info-item mb-0.5 flex items-start last:mb-0">
                        <NSpace wrap>
                          <NTag
                            v-for="group in node.allowGroups.filter(
                              (g) =>
                                !['admin', 'proxies', 'traffic'].includes(
                                  g.name.trim().toLowerCase(),
                                ),
                            )"
                            :key="group.name"
                            size="small"
                            type="info"
                          >
                            {{ group.friendlyName }}
                          </NTag>
                        </NSpace>
                      </div>
                      <div class="info-item mb-0.5 flex items-start last:mb-0" style="margin-top: -1px">
                        <NSpace wrap>
                          <NTag
                            v-if="supportsTcp(node)"
                            type="success"
                            size="small"
                            >TCP</NTag
                          >
                          <NTag
                            v-if="supportsUdp(node)"
                            type="success"
                            size="small"
                            >UDP</NTag
                          >
                          <NTag
                            v-if="supportsHttp(node) && supportsHttps(node)"
                            type="success"
                            size="small"
                            >HTTP(S)</NTag
                          >
                          <NTag
                            v-else-if="supportsHttp(node)"
                            type="success"
                            size="small"
                            >HTTP</NTag
                          >
                          <NTag
                            v-else-if="supportsHttps(node)"
                            type="success"
                            size="small"
                            >HTTPS</NTag
                          >
                          <NTag
                            v-if="supportsStcp(node)"
                            type="success"
                            size="small"
                            >STCP</NTag
                          >
                          <NTag
                            v-if="supportsXtcp(node)"
                            type="success"
                            size="small"
                            >XTCP</NTag
                          >
                        </NSpace>
                      </div>
                      <div class="info-item mb-0.5 flex items-start last:mb-0" style="margin-top: -1px">
                        <NSpace wrap>
                          <NTag type="warning" size="small">
                            {{ node.portRange.min }} - {{ node.portRange.max }}
                          </NTag>
                          <NTag type="info" size="small">
                            {{ node.bandWidth }} Mbps
                          </NTag>
                          <NTag
                            v-if="node.needRealname"
                            type="info"
                            size="small"
                          >
                            实名
                          </NTag>
                        </NSpace>
                      </div>
                    </NSpace>
                  </NCard>
                </NGridItem>
              </NGrid>
              <div
                v-if="
                  filteredNodes.filter((n) => n.location === 'cn-out')
                    .length === 0
                "
                class="no-results py-10 text-center"
              >
                <NEmpty description="没有找到符合条件的节点" />
              </div>
            </NCollapseItem>
            <NCollapseItem title="海外地区" name="out">
              <NGrid
                x-gap="8"
                y-gap="8"
                cols="3"
                responsive="screen"
                style="padding-top: 14px"
              >
                <NGridItem
                  v-for="node in filteredNodes.filter(
                    (n) => n.location === 'out',
                  )"
                  :key="node.value"
                >
                  <NCard
                    hoverable
                    @click="handleNodeSelect(node)"
                    :class="[
                      {
                        'selected-node': selectedNodeId === node.value,
                        'node-offline': !node.isOnline,
                      },
                    ]"
                    class="node-item relative h-full animate-rise-in cursor-pointer border border-[var(--n-border-color)] transition-all duration-200 [&.selected-node]:border-[rgba(33,150,243,0.3)]! [&.selected-node]:bg-[rgba(33,150,243,0.02)] [&.selected-node]:shadow-[0_0_4px_rgba(33,150,243,0.15)] [&.selected-node:hover]:bg-[rgba(33,150,243,0.04)] [&.node-offline]:opacity-50 [&.node-offline]:grayscale-[0.4]"
                  >
                    <div class="node-header mb-2 flex items-start justify-between">
                      <div
                        class="node-title flex items-center gap-2"
                        style="
                          display: flex;
                          align-items: center;
                          overflow: hidden;
                        "
                      >
                        <NTag type="info" size="small" style="flex-shrink: 0"
                          ># {{ node.id }}</NTag
                        >
                        <NTooltip trigger="hover">
                          <template #trigger>
                            <NText
                              style="
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                margin-left: -3px;
                                margin-right: 4px;
                                flex: 1;
                                min-width: 0;
                                cursor: pointer;
                              "
                              >{{ node.name
                              }}<span
                                :style="{
                                  color: getLoadStatusColor(node.loadStatus),
                                }"
                                >({{
                                  getLoadStatusText(node.loadStatus)
                                }})</span
                              ></NText
                            >
                          </template>
                          {{ node.name }} ({{
                            getLoadStatusTooltip(node.loadStatus)
                          }})
                        </NTooltip>
                        <NTag
                          :type="getLatencyInfo(node.latency).type"
                          size="small"
                          style="flex-shrink: 0"
                        >
                          {{ getLatencyInfo(node.latency).text }}
                        </NTag>
                      </div>
                      <!-- <div class="node-tags">
                        <NTag
                          v-if="supportsUdp(node)"
                          type="success"
                          size="small"
                          >UDP</NTag
                        >
                        <NTag
                          v-if="supportsHttp(node) && supportsHttps(node)"
                          type="success"
                          size="small"
                          >HTTP(S)</NTag
                        >
                        <NTag
                          v-else-if="supportsHttp(node)"
                          type="success"
                          size="small"
                          >HTTP</NTag
                        >
                        <NTag
                          v-else-if="supportsHttps(node)"
                          type="success"
                          size="small"
                          >HTTPS</NTag
                        >
                        <NTag
                          v-if="supportsStcp(node)"
                          type="success"
                          size="small"
                          >STCP</NTag
                        >
                        <NTag
                          v-if="supportsXtcp(node)"
                          type="success"
                          size="small"
                          >XTCP</NTag
                        >
                      </div> -->
                    </div>
                    <NText depth="3" style="font-size: 13px; margin: 6px 0">{{
                      node.description
                    }}</NText>
                    <NSpace vertical style="margin-top: 4px">
                      <div class="info-item mb-0.5 flex items-start last:mb-0">
                        <NSpace wrap>
                          <NTag
                            v-for="group in node.allowGroups.filter(
                              (g) =>
                                !['admin', 'proxies', 'traffic'].includes(
                                  g.name.trim().toLowerCase(),
                                ),
                            )"
                            :key="group.name"
                            size="small"
                            type="info"
                          >
                            {{ group.friendlyName }}
                          </NTag>
                        </NSpace>
                      </div>
                      <div class="info-item mb-0.5 flex items-start last:mb-0" style="margin-top: -1px">
                        <NSpace wrap>
                          <NTag
                            v-if="supportsTcp(node)"
                            type="success"
                            size="small"
                            >TCP</NTag
                          >
                          <NTag
                            v-if="supportsUdp(node)"
                            type="success"
                            size="small"
                            >UDP</NTag
                          >
                          <NTag
                            v-if="supportsHttp(node) && supportsHttps(node)"
                            type="success"
                            size="small"
                            >HTTP(S)</NTag
                          >
                          <NTag
                            v-else-if="supportsHttp(node)"
                            type="success"
                            size="small"
                            >HTTP</NTag
                          >
                          <NTag
                            v-else-if="supportsHttps(node)"
                            type="success"
                            size="small"
                            >HTTPS</NTag
                          >
                          <NTag
                            v-if="supportsStcp(node)"
                            type="success"
                            size="small"
                            >STCP</NTag
                          >
                          <NTag
                            v-if="supportsXtcp(node)"
                            type="success"
                            size="small"
                            >XTCP</NTag
                          >
                        </NSpace>
                      </div>
                      <div class="info-item mb-0.5 flex items-start last:mb-0" style="margin-top: -1px">
                        <NSpace wrap>
                          <NTag type="warning" size="small">
                            {{ node.portRange.min }} - {{ node.portRange.max }}
                          </NTag>
                          <NTag type="info" size="small">
                            {{ node.bandWidth }} Mbps
                          </NTag>
                          <NTag
                            v-if="node.needRealname"
                            type="info"
                            size="small"
                          >
                            实名
                          </NTag>
                        </NSpace>
                      </div>
                    </NSpace>
                  </NCard>
                </NGridItem>
              </NGrid>
              <div
                v-if="
                  filteredNodes.filter((n) => n.location === 'out').length === 0
                "
                class="no-results py-10 text-center"
              >
                <NEmpty description="没有找到符合条件的节点" />
              </div>
            </NCollapseItem>
          </NCollapse>
        </NSpace>
      </NSpin>
    </NCard>

    <!-- 隧道配置弹窗 -->
    <NModal
      v-model:show="showConfigModal"
      preset="card"
      :title="modalTitle"
      style="width: 650px"
      :bordered="false"
      :segmented="{
        content: true,
        footer: 'soft',
      }"
    >
      <NForm
        ref="formRef"
        :model="formValue"
        :rules="rules"
        label-placement="left"
        label-width="120"
        require-mark-placement="right-hanging"
      >
        <NCollapse
          v-model:expanded-names="expandedAdvanced"
          :on-update:expanded-names="handleCreateFormCollapseUpdate"
        >
          <NCollapseItem name="basic" title="基本设置">
            <NFormItem label="隧道名称" path="name">
              <NInput
                v-model:value="formValue.name"
                placeholder="请输入隧道名称"
              />
            </NFormItem>

            <NFormItem label="本地地址" path="localAddr">
              <NInput
                v-model:value="formValue.localAddr"
                placeholder="请输入本地地址"
              />
            </NFormItem>

            <NFormItem label="本地端口" path="localPort">
              <NInputNumber
                v-model:value="formValue.localPort"
                :min="1"
                :max="65535"
                placeholder="请输入本地端口"
              />
            </NFormItem>

            <NFormItem label="协议类型" path="type">
              <NSelect
                v-model:value="formValue.type"
                :options="allowedProxyTypeOptions"
                placeholder="请选择协议类型"
              />
            </NFormItem>

            <NFormItem
              v-if="['stcp', 'xtcp'].includes(formValue.type)"
              label="访问密钥"
              path="accessKey"
            >
              <NInput
                v-model:value="formValue.accessKey"
                placeholder="请输入访问密钥"
              />
            </NFormItem>

            <NFormItem
              v-if="formValue.type === 'http' || formValue.type === 'https'"
              label="绑定域名"
              path="domain"
            >
              <NDynamicTags
                v-model:value="domainTags"
                :render-tag="renderDomainTag"
              />
            </NFormItem>

            <NFormItem
              v-if="['tcp', 'udp'].includes(formValue.type)"
              label="远程端口"
              path="remotePort"
            >
              <NSpace>
                <NInputNumber
                  v-model:value="formValue.remotePort"
                  :min="selectedNode?.portRange?.min || 1"
                  :max="selectedNode?.portRange?.max || 65535"
                  placeholder="请输入远程端口"
                />
                <NButton
                  size="medium"
                  :loading="gettingFreePort"
                  @click="handleGetFreePort"
                >
                  获取随机端口
                </NButton>
              </NSpace>
            </NFormItem>
          </NCollapseItem>

          <NCollapseItem title="高级配置" name="advanced">
            <template #header-extra>
              <NText depth="3" style="font-size: 12px; margin-left: 8px">
                仅推荐技术用户使用
              </NText>
            </template>

            <NFormItem label="Proxy Protocol" path="proxyProtocolVersion">
              <NSelect
                v-model:value="formValue.proxyProtocolVersion"
                :options="[
                  { label: '不启用', value: '' },
                  { label: 'v1', value: 'v1' },
                  { label: 'v2', value: 'v2' },
                ]"
                placeholder="Proxy Protocol Version"
              />
            </NFormItem>

            <NFormItem label="每个IP最大下载速率" path="ipLimitIn">
              <div class="speed-input-group flex items-center gap-2">
                <NInputNumber
                  v-model:value="formValue.ipLimitIn"
                  :min="0"
                  placeholder="请输入最大下载速率"
                  style="flex: 1"
                />
                <NSelect
                  v-model:value="formValue.ipLimitInUnit"
                  :options="speedUnitOptions"
                  style="width: 100px"
                />
              </div>
            </NFormItem>
            <NFormItem label="每个IP最大上传速率" path="ipLimitOut">
              <div class="speed-input-group flex items-center gap-2">
                <NInputNumber
                  v-model:value="formValue.ipLimitOut"
                  :min="0"
                  placeholder="请输入最大上传速率"
                  style="flex: 1"
                />
                <NSelect
                  v-model:value="formValue.ipLimitOutUnit"
                  :options="speedUnitOptions"
                  style="width: 100px"
                />
              </div>
            </NFormItem>
            <NFormItem label="其他选项">
              <div style="display: flex; gap: 16px">
                <NSwitch
                  v-model:value="formValue.useEncryption"
                  :rail-style="switchButtonRailStyle"
                >
                  <template #checked>启用加密</template>
                  <template #unchecked>禁用加密</template>
                </NSwitch>
                <NSwitch
                  v-model:value="formValue.useCompression"
                  :rail-style="switchButtonRailStyle"
                >
                  <template #checked>启用压缩</template>
                  <template #unchecked>禁用压缩</template>
                </NSwitch>
              </div>
            </NFormItem>
          </NCollapseItem>
        </NCollapse>
      </NForm>
      <template #footer>
        <div style="display: flex; justify-content: flex-end">
          <NButton @click="closeModal('config')">取消</NButton>
          <NButton
            type="primary"
            :loading="loading"
            @click="showCreateModal"
            style="margin-left: 12px"
          >
            <template #icon>
              <NIcon>
                <CloudUploadOutline />
              </NIcon>
            </template>
            创建隧道
          </NButton>
        </div>
      </template>
    </NModal>

    <!-- 创建隧道确认弹窗 -->
    <NModal
      v-model:show="showCreateConfirmModal"
      preset="dialog"
      title="确认创建隧道"
      :show-icon="false"
      style="width: 500px"
    >
      <div>
        <p>您即将创建以下隧道配置：</p>
        <div class="tunnel-confirm-details my-3 rounded-lg bg-[rgba(33,150,243,0.05)] p-4">
          <div class="confirm-item mb-2 flex last:mb-0">
            <span class="confirm-label w-[100px] font-medium text-[var(--n-text-color-2)]">节点：</span>
            <span>{{ selectedNode?.name || "未选择" }}</span>
          </div>
          <div class="confirm-item mb-2 flex last:mb-0">
            <span class="confirm-label w-[100px] font-medium text-[var(--n-text-color-2)]">隧道名称：</span>
            <span>{{ formValue.name }}</span>
          </div>
          <div class="confirm-item mb-2 flex last:mb-0">
            <span class="confirm-label w-[100px] font-medium text-[var(--n-text-color-2)]">本地地址：</span>
            <span>{{ formValue.localAddr }}:{{ formValue.localPort }}</span>
          </div>
          <div class="confirm-item mb-2 flex last:mb-0">
            <span class="confirm-label w-[100px] font-medium text-[var(--n-text-color-2)]">协议类型：</span>
            <span>{{ formValue.type.toUpperCase() }}</span>
          </div>
          <div
            v-if="formValue.type === 'http' || formValue.type === 'https'"
            class="confirm-item mb-2 flex last:mb-0"
          >
            <span class="confirm-label w-[100px] font-medium text-[var(--n-text-color-2)]">绑定域名：</span>
            <span>{{ domainTags.join(", ") }}</span>
          </div>
          <div
            v-if="['tcp', 'udp'].includes(formValue.type)"
            class="confirm-item mb-2 flex last:mb-0"
          >
            <span class="confirm-label w-[100px] font-medium text-[var(--n-text-color-2)]">远程端口：</span>
            <span>{{ formValue.remotePort }}</span>
          </div>
        </div>
        <p class="confirm-warning mt-3 text-sm">
          请确认以上信息无误，点击确认后将创建隧道。
        </p>
      </div>
      <template #action>
        <NButton size="medium" @click="closeModal('createConfirm')"
          >取消</NButton
        >
        <NButton
          size="medium"
          type="primary"
          :loading="loading"
          @click="handleCreate"
          >确认创建</NButton
        >
      </template>
    </NModal>
  </div>
</template>

<script setup lang="ts">
import { ref, h, computed, onMounted, watch, nextTick } from "vue";
import {
  NCard,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NSelect,
  NButton,
  NIcon,
  useMessage,
  type FormRules,
  type FormInst,
  NSwitch,
  NTag,
  NSpace,
  NText,
  NGrid,
  NGridItem,
  NDynamicTags,
  NModal,
  NEmpty,
  NSpin,
  NCollapse,
  NCollapseItem,
} from "naive-ui";
import {
  CloudUploadOutline,
  SearchOutline,
  RefreshOutline,
} from "@vicons/ionicons5";
import { switchButtonRailStyle } from "../../../constants/theme.ts";
import { useRouter } from "vue-router";
import { userApi } from "../../../net";
import { accessHandle } from "../../../net/base.ts";
import { invoke } from "@tauri-apps/api/core";

const router = useRouter();
const message = useMessage();
const formRef = ref<FormInst | null>(null);
const loading = ref(false);
const nodeLoading = ref(false);
const refreshingLatency = ref(false);

// ========== 弹窗互斥逻辑 ========== //
const modalStack = ref<string[]>([]);

const showRealnameModal = ref(false);
const showConfigModal = ref(false);
const showCreateConfirmModal = ref(false);

function setModalVisible(name: string, visible: boolean) {
  if (name === "realname") showRealnameModal.value = visible;
  if (name === "config") showConfigModal.value = visible;
  if (name === "createConfirm") showCreateConfirmModal.value = visible;
}

function getCurrentOpenModal(): string | null {
  if (showRealnameModal.value) return "realname";
  if (showConfigModal.value) return "config";
  if (showCreateConfirmModal.value) return "createConfirm";
  return null;
}

function openModal(modalName: string) {
  const currentModal = getCurrentOpenModal();
  if (currentModal && currentModal !== modalName) {
    modalStack.value.push(currentModal);
    setModalVisible(currentModal, false);
  }
  setModalVisible(modalName, true);
}

function closeModal(modalName: string) {
  setModalVisible(modalName, false);
  nextTick(() => {
    if (modalStack.value.length > 0) {
      const prevModal = modalStack.value.pop();
      if (prevModal) setModalVisible(prevModal, true);
    }
  });
}
// ========== 弹窗互斥逻辑 END ========== //

const searchQuery = ref("");
const selectedGroup = ref("all");
const selectedProtocols = ref<string[]>([]);
const selectedNodeId = ref<number | null>(null);

const formValue = ref({
  nodeId: null as number | null,
  localAddr: "",
  localPort: null as number | null,
  remotePort: null as number | null,
  type: "",
  domain: "",
  name: "",
  accessKey: "",
  proxyProtocolVersion: "",
  useEncryption: false,
  useCompression: false,
  ipLimitIn: null as number | null,
  ipLimitInUnit: "MB",
  ipLimitOut: null as number | null,
  ipLimitOutUnit: "MB",
});

const protocolOptions = [
  { label: "TCP", value: "tcp" },
  { label: "UDP", value: "udp" },
  { label: "HTTP", value: "http" },
  { label: "HTTPS", value: "https" },
  { label: "STCP", value: "stcp" },
  { label: "XTCP", value: "xtcp" },
];

const speedUnitOptions = [
  { label: "KB/s", value: "KB" },
  { label: "MB/s", value: "MB" },
  { label: "Mbps", value: "Mbps" },
];

const nodeOptions = ref<
  {
    label: string;
    value: number;
    id: number;
    name: string;
    hostname: string;
    description: string;
    isOnline: boolean;
    isDisabled: boolean;
    bandWidth: number;
    location: string;
    servicePort: number;
    allowedProtocols: string[];
    allowGroups: { name: string; friendlyName: string }[];
    needRealname: boolean;
    portRange: {
      min: number;
      max: number;
    };
    latency?: number | null;
    loadStatus?: string;
  }[]
>([]);
// 添加过滤节点的计算属性
const filteredNodes = computed(() => {
  return nodeOptions.value
    .filter((node) => {
      // 隐藏超载节点
      if (node.loadStatus === "overload") {
        return false;
      }
      // 用户组多选筛选
      if (!selectedGroup.value.includes("all")) {
        const groupNames = node.allowGroups.map((g) => g.name);
        if (!groupNames.some((name) => selectedGroup.value.includes(name))) {
          return false;
        }
      }
      // 协议多选筛选
      if (selectedProtocols.value.length > 0) {
        if (
          !selectedProtocols.value.every((protocol) =>
            node.allowedProtocols.includes(protocol),
          )
        ) {
          return false;
        }
      }
      // 搜索筛选
      if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        return (
          node.name.toLowerCase().includes(query) ||
          node.description.toLowerCase().includes(query) ||
          node.id.toString().includes(query)
        );
      }
      return true;
    })
    .sort((a, b) => {
      // 在线优先，ID升序
      if (a.isOnline !== b.isOnline) {
        return a.isOnline ? -1 : 1;
      }
      return a.id - b.id;
    });
});

// 添加协议支持检查函数
const supportsTcp = (node: any) => {
  return node.allowedProtocols.includes("tcp");
};

const supportsUdp = (node: any) => {
  return node.allowedProtocols.includes("udp");
};

const supportsHttp = (node: any) => {
  return node.allowedProtocols.includes("http");
};

const supportsHttps = (node: any) => {
  return node.allowedProtocols.includes("https");
};

const supportsStcp = (node: any) => {
  return node.allowedProtocols.includes("stcp");
};

const supportsXtcp = (node: any) => {
  return node.allowedProtocols.includes("xtcp");
};

const rules: FormRules = {
  nodeId: {
    required: true,
    message: "请选择节点",
    trigger: "blur",
  },
  localAddr: {
    required: true,
    message: "请输入本地地址",
    trigger: "blur",
  },
  localPort: {
    required: true,
    type: "number",
    message: "请输入本地端口",
    trigger: "blur",
    validator: (_rule, value) => {
      if (typeof value !== "number" || value < 1 || value > 65535) {
        return new Error("端口范围必须在 1-65535 之间");
      }
      return true;
    },
  },
  remotePort: {
    required: true,
    type: "number",
    message: "请输入远程端口",
    trigger: "blur",
    validator: (_rule, value) => {
      if (
        ["http", "https", "stcp", "xtcp"].includes(formValue.value.type || "")
      ) {
        return true;
      }
      if (typeof value !== "number" || value < 1 || value > 65535) {
        return new Error("端口范围必须在 1-65535之间");
      }
      return true;
    },
  },
  type: {
    required: true,
    message: "请选择隧道类型",
    trigger: "blur",
  },
  name: {
    required: true,
    message: "请输入隧道名称",
    trigger: "blur",
  },
  accessKey: {
    validator: (_rule, value) => {
      if (["stcp", "xtcp"].includes(formValue.value.type) && !value) {
        return new Error("使用 STCP/XTCP 协议时，访问密钥为必填项");
      }
      return true;
    },
    trigger: ["blur", "change"],
  },
  domain: {
    validator: (_rule, _value) => {
      if (formValue.value.type === "http" || formValue.value.type === "https") {
        if (!domainTags.value.length) {
          return new Error("请至少添加一个域名");
        }
      }
      return true;
    },
    trigger: ["blur", "change"],
  },
};

const groupNameMap = ref<Record<string, string>>({});
const groupList = ref<{ label: string; value: string }[]>([]);

const fetchUserGroups = async () => {
  return new Promise((resolve) => {
    userApi.get(
      "/info/groups",
      accessHandle(),
      (data) => {
        if (data.code === 0) {
          const groups =
            typeof data.data.groups === "string"
              ? JSON.parse(data.data.groups)
              : data.data.groups;

          groupNameMap.value = groups.reduce(
            (acc: Record<string, string>, group: any) => {
              acc[group.name] = group.friendlyName;
              return acc;
            },
            {} as Record<string, string>,
          );
          groupList.value = groups
            .filter(
              (group: any) =>
                !["proxies", "traffic", "admin"].includes(
                  group.name.trim().toLowerCase(),
                ),
            )
            .map((group: any) => ({
              label: group.friendlyName,
              value: group.name,
            }));
          resolve(true);
        } else {
          message.error(data.message || "获取用户组列表失败");
          resolve(false);
        }
      },
      (messageText) => {
        message.error(messageText || "获取用户组列表失败");
        resolve(false);
      },
    );
  });
};

const fetchNodes = async () => {
  nodeLoading.value = true;
  userApi.get(
    "/proxies/nodes",
    accessHandle(),
    (data) => {
      if (data.code === 0) {
        const nodes = Array.isArray(data.data) ? data.data : [];
        nodeOptions.value = nodes.map((node: any) => {
          const [minPort, maxPort] = (node.allowPort || "0-0")
            .split("-")
            .map(Number);
          const allowedProtocols = (node.allowType || "")
            .split(";")
            .map((type: string) => type.trim())
            .filter((p: string) => p);

          const allowGroups = (node.allowGroup || "")
            .split(";")
            .map((group: string) => group.trim())
            .filter((group: string) => group)
            .map((group: string) => ({
              name: group,
              friendlyName: groupNameMap.value[group] || group,
            }));

          return {
            label: `#${node.id} - ${node.name}`,
            value: node.id,
            id: node.id,
            name: node.name,
            hostname: node.hostname,
            description: node.description,
            isOnline: node.status,
            isDisabled: node.isDisabled,
            allowedProtocols,
            allowGroups,
            needRealname: node.needRealname,
            bandWidth: node.bandWidth,
            location: node.location,
            servicePort: node.servicePort || 7000,
            portRange: {
              min: minPort,
              max: maxPort,
            },
            latency: null,
            loadStatus: node.loadStatus,
          };
        });
        // 获取节点列表后开始检测延迟
        checkNodesLatency();
      } else {
        message.error(data.message || "获取节点列表失败");
      }
      nodeLoading.value = false;
    },
    (err) => {
      message.error(err || "获取节点列表失败");
      nodeLoading.value = false;
    },
  );
};

// 检测节点延迟
const checkNodesLatency = async () => {
  for (const node of nodeOptions.value) {
    if (!node.isOnline || node.isDisabled) {
      node.latency = -1; // 离线或禁用标记为 -1
      continue;
    }
    try {
      // 使用节点的 servicePort 进行检测
      const latency = await invoke("tcping", {
        host: node.hostname,
        port: node.servicePort,
        timeoutMs: 3000,
      });
      node.latency = latency as number;
    } catch {
      node.latency = -1; // 连接失败标记为 -1
    }
  }
};

// 刷新延迟
const handleRefreshLatency = async () => {
  if (refreshingLatency.value) return;
  refreshingLatency.value = true;
  // 将所有节点延迟重置为 null（显示检测中...）
  for (const node of nodeOptions.value) {
    node.latency = null;
  }
  await checkNodesLatency();
  refreshingLatency.value = false;
  message.success("延迟检测完成");
};

// 获取延迟显示文本和类型
const getLatencyInfo = (
  latency: number | null | undefined,
): {
  text: string;
  type: "default" | "error" | "success" | "warning" | "primary" | "info";
} => {
  if (latency === null || latency === undefined) {
    return { text: "检测中...", type: "default" };
  }
  if (latency === -1) {
    return { text: "超时", type: "error" };
  }
  if (latency < 50) {
    return { text: `${latency}ms`, type: "success" };
  }
  if (latency < 100) {
    return { text: `${latency}ms`, type: "warning" };
  }
  return { text: `${latency}ms`, type: "error" };
};

/**
 * loadStatus 取值说明：
 * - low: 低负载（综合评分 < 30%）
 * - normal: 正常（综合评分 30%~60%）
 * - high: 高负载（综合评分 60%~80%）
 * - overload: 超载（综合评分 >= 80%）
 * - offline: 节点离线
 * - disabled: 节点被禁用
 * - unknown: 状态未知
 */

// 获取负载状态颜色
const getLoadStatusColor = (loadStatus: string | undefined): string => {
  console.log(loadStatus);
  switch (loadStatus) {
    case "low":
      return "#18a058"; // 绿色
    case "normal":
      return "#2080f0"; // 蓝色
    case "high":
      return "#f0a020"; // 橙色
    case "overload":
      return "#d03050"; // 红色
    case "offline":
      return "#8c8c8c"; // 灰色
    case "disabled":
      return "#8c8c8c"; // 灰色
    case "unknown":
      return "#8c8c8c"; // 灰色
    default:
      return "#8c8c8c"; // 灰色
  }
};

// 获取负载状态显示文本
const getLoadStatusText = (loadStatus: string | undefined): string => {
  switch (loadStatus) {
    case "low":
      return "低负载";
    case "normal":
      return "正常";
    case "high":
      return "高负载";
    case "overload":
      return "超载";
    case "offline":
      return "离线";
    case "disabled":
      return "禁用";
    case "unknown":
      return "未知";
    default:
      return "未知";
  }
};

// 获取负载状态提示信息
const getLoadStatusTooltip = (loadStatus: string | undefined): string => {
  switch (loadStatus) {
    case "low":
      return "低负载（综合评分 < 30%）";
    case "normal":
      return "正常（综合评分 30%~60%）";
    case "high":
      return "高负载（综合评分 60%~80%）";
    case "overload":
      return "超载（综合评分 >= 80%）";
    case "offline":
      return "节点离线";
    case "disabled":
      return "节点被禁用";
    case "unknown":
      return "状态未知";
    default:
      return "状态未知";
  }
};

const selectedNode = ref<{
  id: number;
  name: string;
  hostname: string;
  allowedProtocols: string[];
  allowGroups: { name: string; friendlyName: string }[];
  portRange: {
    min: number;
    max: number;
  };
  latency: number;
} | null>(null);

const allowedProxyTypeOptions = computed(() => {
  if (!selectedNode.value) return protocolOptions;
  return protocolOptions.filter((opt) =>
    selectedNode.value?.allowedProtocols.includes(opt.value),
  );
});

const modalTitle = computed(() => {
  if (!selectedNode.value) return "隧道配置";
  const latencyInfo = getLatencyInfo(selectedNode.value.latency);
  return `${selectedNode.value.name} [${latencyInfo.text}]`;
});

const domainTags = ref<string[]>([]);

const handleDomainTagsUpdate = (tags: string[]) => {
  domainTags.value = tags;
  formValue.value.domain = JSON.stringify(tags);
};

const renderDomainTag = (tag: string) => {
  return h(
    NTag,
    {
      round: false,
      closable: true,
      onClose: () => {
        const index = domainTags.value.indexOf(tag);
        if (index !== -1) {
          const newTags = [...domainTags.value];
          newTags.splice(index, 1);
          domainTags.value = newTags;
          handleDomainTagsUpdate(newTags);
        }
      },
    },
    { default: () => tag },
  );
};

const countDown = ref(10);
let timer: number | null = null;

const goToRealname = () => {
  router.push("/dashboard/profile");
};

const handleNodeSelect = (node: any) => {
  if (node.isDisabled) {
    message.error("该节点已被禁用，无法选择");
    return;
  }
  if (!node.isOnline) {
    message.error("该节点已离线，请选择其他节点");
    return;
  }
  selectedNodeId.value = node.value;
  selectedNode.value = {
    id: node.id,
    name: node.name,
    hostname: node.hostname,
    allowedProtocols: node.allowedProtocols,
    allowGroups: node.allowGroups,
    portRange: node.portRange,
    latency: node.latency,
  };
  // 设置表单默认值
  formValue.value.nodeId = node.value;
  formValue.value.type = node.allowedProtocols[0] || "";
  formValue.value.remotePort = null;
  openModal("config");
};

const showCreateModal = () => {
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      openModal("createConfirm");
    }
  });
};

const convertSpeedToKB = (value: number | null, unit: string): number => {
  if (!value || value <= 0) return 0;
  switch (unit) {
    case "KB":
      return value;
    case "MB":
      return value * 1024;
    case "Mbps":
      return value * 125; // 1 Mbps = 125 KB/s
    default:
      return value;
  }
};

const handleCreate = async () => {
  loading.value = true;
  try {
    const requestData = {
      nodeId: formValue.value.nodeId,
      proxyName: formValue.value.name,
      localIp: formValue.value.localAddr,
      localPort: formValue.value.localPort,
      remotePort: formValue.value.remotePort,
      domain: ["http", "https"].includes(formValue.value.type)
        ? JSON.stringify(domainTags.value)
        : "",
      proxyType: formValue.value.type,
      accessKey: formValue.value.accessKey,
      proxyProtocolVersion: formValue.value.proxyProtocolVersion,
      useEncryption: formValue.value.useEncryption,
      useCompression: formValue.value.useCompression,
      ipLimitIn: convertSpeedToKB(
        formValue.value.ipLimitIn,
        formValue.value.ipLimitInUnit,
      ),
      ipLimitOut: convertSpeedToKB(
        formValue.value.ipLimitOut,
        formValue.value.ipLimitOutUnit,
      ),
    };

    userApi.post(
      "/proxies/",
      requestData,
      accessHandle(),
      (data) => {
        if (data.code === 0) {
          message.success("隧道创建成功");
          formRef.value?.restoreValidation();
          closeModal("createConfirm");
          closeModal("config");
          selectedNodeId.value = null;
        } else {
          message.error(data.message || "创建失败");
        }
        loading.value = false;
      },
      (err) => {
        message.error(err || "创建失败");
        loading.value = false;
      },
    );
  } catch (error) {
    const errorMsg = error || "服务器连接异常";
    message.error(`创建失败: ${errorMsg}`);
    loading.value = false;
  }
};

const init = async () => {
  await fetchUserGroups();
  fetchNodes();
};

onMounted(() => {
  init();
});

const gettingFreePort = ref(false);

const handleGetFreePort = async () => {
  if (!selectedNode.value) return;

  const min = selectedNode.value.portRange.min || 1024;
  const max = selectedNode.value.portRange.max || 65535;
  formValue.value.remotePort =
    Math.floor(Math.random() * (max - min + 1)) + min;
};

watch(showRealnameModal, (newVal) => {
  if (!newVal && timer) {
    clearInterval(timer);
    timer = null;
  }
});

const expandedRegion = ref(["cn"]);
const expandedAdvanced = ref<string[]>(["basic"]);

const handleCreateFormCollapseUpdate = (names: string[]) => {
  if (names.length > 1) {
    expandedAdvanced.value = [names[names.length - 1]];
  } else {
    expandedAdvanced.value = names;
  }
};
</script>
