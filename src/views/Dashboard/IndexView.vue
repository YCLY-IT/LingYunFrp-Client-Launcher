<template>
  <div class="home">
    <!-- 欢迎横幅 -->
    <div class="welcome-banner">欢迎回来, {{ nickname }}</div>

    <!-- 用户卡片 -->
    <div class="content-info">
      <n-card :loading="loading" class="user-card">
        <n-space>
          <div
            class="user-card-avatar"
            :style="{
              backgroundImage: `url(${avatar})`,
              backgroundSize: 'cover',
              backgroundPosition: 'center',
              width: '60px',
              height: '60px',
              borderRadius: '25%',
              marginTop: '1px',
              transform: 'scale(1.2)',
            }"
          ></div>
          <div style="margin-left: 16px; text-align: left; margin-top: 5px">
            <h3 style="margin: 0px">
              {{ forTime }}，{{ nickname }}，{{ currentDate }} 😊
            </h3>
            <n-skeleton
              style="margin: 8px 0px 0px; width: 500px"
              v-if="loading"
            />
            <p style="margin: 5px 0px 0px">{{ textHitokoto }}</p>
          </div>
        </n-space>
      </n-card>
    </div>

    <!-- 统计卡片 -->
    <div class="statistic-container">
      <Statistic
        :signRemainder="userInfoRef?.userInfo.signRemainder"
        ref="statisticRef"
      />
    </div>

    <!-- 内容面板 -->
    <div style="margin-top: 20px" class="content-grid">
      <div class="left-column">
        <!-- 用户信息卡片 -->
        <NCard title="用户信息" class="info-card">
          <NAlert
            v-if="!IsRealname"
            type="warning"
            title="未实名认证"
            style="margin-bottom: 16px"
          >
            您的账户尚未完成实名认证, 请尽快完成实名认证。
            <br />
            <NButton text type="primary" @click="goToRealname"
              >立即前往</NButton
            >
          </NAlert>
          <UserInfo ref="userInfoRef" @update="handleUserUpdate" />
        </NCard>
      </div>

      <div class="right-column">
        <div class="notice-and-welcome">
          <div class="welcome-card-container">
            <WelcomeCard />
          </div>
          <NCard title="通知内容" class="notice-card">
            <template #default>
              <div class="notice-scroll">
                <NScrollbar :vertical-rail-style="{ right: '-15px' }">
                  <div v-if="notices.length > 0" class="notice-list">
                    <div
                      v-for="(notice, _index) in notices"
                      :key="notice.id"
                      class="notice-item"
                      @click="openNoticeModal(notice)"
                    >
                      <div class="notice-item-content">
                        <span class="notice-item-title">{{
                          notice.title
                        }}</span>
                      </div>
                      <div class="notice-item-meta">
                        <n-tag
                          v-if="notice.type === 'danger'"
                          type="error"
                          size="small"
                          >紧急</n-tag
                        >
                        <n-tag
                          v-else-if="notice.type === 'warning'"
                          type="warning"
                          size="small"
                          >重要</n-tag
                        >
                        <n-tag
                          v-else-if="notice.type === 'info'"
                          type="info"
                          size="small"
                          >通知</n-tag
                        >
                        <span
                          class="notice-time"
                          :style="{ color: themeStore.$state.primaryColor }"
                          >{{ formatTime(notice.created_at) }}</span
                        >
                      </div>
                    </div>
                  </div>
                  <div v-else class="no-notice">暂无通知</div>
                </NScrollbar>
              </div>
            </template>
          </NCard>

          <!-- 通知详情模态框 -->
          <n-modal
            v-model:show="noticeModalVisible"
            preset="card"
            :title="selectedNotice?.title"
            style="width: 800px; max-width: 100vw; height: 90vh"
          >
            <n-scrollbar
              v-if="selectedNotice"
              style="height: calc(80vh - 120px)"
            >
              <div class="notice-modal-content">
                <div class="notice-modal-meta">
                  <n-tag
                    v-if="selectedNotice.type === 'danger'"
                    type="error"
                    size="small"
                    >紧急</n-tag
                  >
                  <n-tag
                    v-else-if="selectedNotice.type === 'warning'"
                    type="warning"
                    size="small"
                    >重要</n-tag
                  >
                  <n-tag
                    v-else-if="selectedNotice.type === 'info'"
                    type="info"
                    size="small"
                    >通知</n-tag
                  >
                  <span
                    class="notice-time"
                    :style="{ color: themeStore.$state.primaryColor }"
                    >{{ formatTime(selectedNotice.created_at) }}</span
                  >
                </div>
                <n-divider />
                <div
                  class="notice-content"
                  v-html="renderNoticeContent(selectedNotice.message)"
                />
              </div>
            </n-scrollbar>
          </n-modal>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  NCard,
  NAlert,
  NButton,
  NScrollbar,
  NTag,
  NModal,
  NDivider,
} from "naive-ui";
import { ref, onMounted, computed } from "vue";
import { marked } from "marked";
import DOMPurify from "dompurify";
import { useRouter } from "vue-router";
import { userApi } from "../../net";
import { accessHandle } from "../../net/base";
import UserInfo from "../../components/UserInfo.vue";
import WelcomeCard from "../../components/WelcomeCard.vue";
import { Broadcast, Traffic } from "../../types/User";
import { useThemeStore } from "../../stores/theme";

const router = useRouter();
const notices = ref<Broadcast[]>([]);
const nickname = localStorage.getItem("nickname") || "";
const themeStore = useThemeStore();

// 通知模态框
const noticeModalVisible = ref(false);
const selectedNotice = ref<Broadcast | null>(null);

// 打开通知详情
const openNoticeModal = (notice: Broadcast) => {
  selectedNotice.value = notice;
  noticeModalVisible.value = true;
};

// 用户信息引用
const userInfoRef = ref<{
  userInfo: { isRealname: boolean; avatar: string; signRemainder: number };
} | null>(null);
const statisticRef = ref();

// 是否实名认证
const IsRealname = computed(
  () => userInfoRef.value?.userInfo.isRealname || true,
);

// 一言和流量数据
const textHitokoto = ref("");
const traffic = ref<Traffic>({} as Traffic);
const loading = ref(false);

// 现在几点
const forTime = computed(() => {
  const date = new Date();
  const hours = date.getHours();
  if (hours < 6) {
    return "凌晨好";
  } else if (hours < 12) {
    return "早上好";
  } else if (hours < 18) {
    return "下午好";
  } else {
    return "晚上好";
  }
});

// 用户头像
const avatar =
  userInfoRef.value?.userInfo.avatar || localStorage.getItem("avatar");

// 配置 marked
marked.setOptions({
  gfm: true,
  breaks: true,
});

// 前往实名认证
const goToRealname = () => {
  router.push("/dashboard/profile");
};

// 渲染通知
const renderNoticeContent = (message: string) => {
  try {
    const html = marked.parse(message) as string;
    return DOMPurify.sanitize(html);
  } catch {
    return message;
  }
};

// 格式化时间
const formatTime = (timeStr: string) => {
  const date = new Date(timeStr);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));
  const hours = Math.floor(diff / (1000 * 60 * 60));
  const minutes = Math.floor(diff / (1000 * 60));

  if (days > 0) return `${days}天前`;
  if (hours > 0) return `${hours}小时前`;
  if (minutes > 0) return `${minutes}分钟前`;
  return "刚刚";
};

const handleUserUpdate = () => {
  getUserTraffic();
  statisticRef.value?.getUserTraffic();
};

// 获取通知
const fetchNotice = async (): Promise<void> => {
  userApi.get("/info/broadcasts", accessHandle(), (data) => {
    if (data.code === 0) {
      // 按类型优先级排序: danger > warning > info
      const typePriority: Record<string, number> = {
        danger: 0,
        warning: 1,
        info: 2,
      };
      const broadcasts = data.data || [];
      notices.value = broadcasts.sort((a: Broadcast, b: Broadcast) => {
        return (typePriority[a.type] || 2) - (typePriority[b.type] || 2);
      });
    }
  });
};

// 获取一言
const getHitokoto = async (): Promise<void> => {
  loading.value = true;
  userApi.getHitokoto({}, (data) => {
    textHitokoto.value = data.hitokoto;
    loading.value = false;
  });
};
// 获取用户流量
const getUserTraffic = async (): Promise<void> => {
  userApi.get("/user/traffic", accessHandle(), (data) => {
    traffic.value = data.data;
  });
};

const currentDate = computed(() => {
  const date = new Date();
  return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
});

// 页面挂载后执行
onMounted(() => {
  fetchNotice();
  getHitokoto();
  getUserTraffic();
});
</script>

<style lang="scss" scoped>
@use "../../assets/styles/home.scss";
.left-column,
.right-column {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.right-column {
  height: 100%;
}

@media (max-width: 768px) {
  .right-column {
    height: auto;
    min-height: 568px;
  }

  .notice-and-welcome {
    height: auto;
    min-height: 400px;
  }

  .notice-card {
    max-height: 360px;
  }

  .notice-scroll {
    max-height: 400px;
  }
}

.notice-and-welcome {
  display: flex;
  flex-direction: column;
  height: 100%;
  flex: 1 1 0;
  min-height: 0;
}

.notice-card {
  flex: 1 1 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
  margin-top: 12px;
}

.notice-card :deep(.n-card__content) {
  flex: 1 1 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.notice-scroll {
  padding: 4px;
  margin: -4px;
}

.notice-list {
  padding: 4px;
}

.notice-scroll {
  flex: 1 1 0;
  min-height: 0;
  max-height: 100%;
  margin-top: 10px;
}

.notice-scroll :deep(.n-scrollbar) {
  overflow: visible;
}

.notice-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.notice-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--n-card-color);
  border: 1px solid var(--n-border-color);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.notice-item:hover {
  background-color: var(--n-color-hover);
  box-shadow: 0 4px 16px var(--n-color-hover-shadow);
  transform: scale(1.02);
}

.notice-item-content {
  flex: 1;
  display: flex;
  align-items: center;
  overflow: hidden;
}

.notice-item-title {
  font-size: 15px;
  font-weight: 500;
  color: var(--n-text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.notice-item-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  margin-left: 12px;
}

.notice-time {
  font-size: 12px;
  font-weight: normal;
}

.notice-modal-content {
  padding: 8px 0;
}

.notice-modal-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.notice-content {
  line-height: 1.8;
  color: var(--n-text-color);
  font-size: 14px;
  word-break: break-word;
}

.notice-content :deep(p) {
  margin: 8px 0;
}

.no-notice {
  text-align: center;
  color: var(--n-text-color-3);
  padding: 40px 0;
  font-size: 14px;
}

.welcome-card-container {
  width: 100%;
  margin: 0;
  padding: 0;
}

.card-container {
  width: 100% !important;
  max-width: none !important;
  min-width: 0 !important;
}
</style>
