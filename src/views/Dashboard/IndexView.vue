<template>
  <div class="home">
    <!-- 欢迎横幅 -->
    <motion.div
      class="welcome-banner mb-2.5 select-none rounded px-0 py-2 text-[1.5em]"
      :initial="{ opacity: 0, x: -24 }"
      :animate="{ opacity: 1, x: 0 }"
      :transition="{ type: 'spring', stiffness: 200, damping: 22 }"
    >
      欢迎回来, {{ nickname }}
    </motion.div>

    <!-- 用户卡片 -->
    <motion.div
      class="content-info"
      :initial="{ opacity: 0, y: 24 }"
      :animate="{ opacity: 1, y: 0 }"
      :transition="{
        delay: 0.08,
        type: 'spring',
        stiffness: 220,
        damping: 24,
      }"
    >
      <n-card :loading="loading" class="user-card w-full">
        <n-space>
          <motion.div
            class="user-card-avatar h-[60px] w-[60px] rounded-[25%]"
            :style="{
              backgroundImage: `url(${avatar})`,
              backgroundSize: 'cover',
              backgroundPosition: 'center',
              marginTop: '1px',
            }"
            :initial="{ scale: 1.2, opacity: 0 }"
            :animate="{ scale: 1.2, opacity: 1 }"
            :transition="{ duration: 0.4, delay: 0.2, ease: 'easeOut' }"
            :while-hover="{ scale: 1.3, rotate: 3 }"
          ></motion.div>
          <div class="ml-4 mt-[5px] text-left">
            <h3 class="m-0">
              {{ forTime }}，{{ nickname }}，{{ currentDate }} 😊
            </h3>
            <n-skeleton
              class="mt-2 w-[500px]"
              :style="{ marginBottom: '0px' }"
              v-if="loading"
            />
            <p class="mt-[5px]">{{ textHitokoto }}</p>
          </div>
        </n-space>
      </n-card>
    </motion.div>

    <!-- 统计卡片 -->
    <motion.div
      class="statistic-container"
      :initial="{ opacity: 0, y: 24 }"
      :animate="{ opacity: 1, y: 0 }"
      :transition="{
        delay: 0.16,
        type: 'spring',
        stiffness: 220,
        damping: 24,
      }"
    >
      <Statistic
        :signRemainder="userInfoRef?.userInfo.signRemainder"
        ref="statisticRef"
      />
    </motion.div>

    <!-- 内容面板 -->
    <motion.div
      class="content-grid mt-5 grid grid-cols-1 items-stretch gap-5 md:grid-cols-2"
      :initial="{ opacity: 0, y: 28 }"
      :animate="{ opacity: 1, y: 0 }"
      :transition="{
        delay: 0.24,
        type: 'spring',
        stiffness: 200,
        damping: 24,
      }"
    >
      <div class="left-column flex flex-col gap-4">
        <!-- 用户信息卡片 -->
        <NCard title="用户信息" class="info-card w-full">
          <NAlert
            v-if="!IsRealname"
            type="warning"
            title="未实名认证"
            class="mb-4"
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

      <div class="right-column relative min-h-0">
        <div class="notice-and-welcome flex flex-col md:absolute md:inset-0">
          <div class="welcome-card-container w-full shrink-0">
            <WelcomeCard />
          </div>
          <NCard
            title="通知内容"
            class="notice-card mt-3 flex min-h-0 flex-1 flex-col max-md:max-h-[360px] max-md:min-h-[200px] [&_.n-card-content]:flex [&_.n-card-content]:min-h-0 [&_.n-card-content]:flex-1 [&_.n-card-content]:flex-col [&_.n-card-content]:overflow-hidden"
          >
            <template #default>
              <div
                class="notice-scroll -m-1 mt-2.5 flex max-h-full min-h-0 flex-1 flex-col p-1 [&_.n-scrollbar]:overflow-visible"
              >
                <NScrollbar :vertical-rail-style="{ right: '-15px' }">
                  <div v-if="notices.length > 0" class="notice-list flex flex-col gap-1">
                    <div
                      v-for="(notice, _index) in notices"
                      :key="notice.id"
                      class="notice-item flex cursor-pointer items-center justify-between gap-3 rounded-lg px-3 py-2.5 transition-all duration-200 hover:-translate-y-0.5 hover:bg-[var(--n-color-hover)]"
                      @click="openNoticeModal(notice)"
                    >
                      <div class="notice-item-content">
                        <span class="notice-item-title font-medium">{{
                          notice.title
                        }}</span>
                      </div>
                      <div class="notice-item-meta flex shrink-0 items-center gap-2">
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
                          class="notice-time text-xs opacity-80"
                          :style="{ color: themeStore.$state.primaryColor }"
                          >{{ formatTime(notice.created_at) }}</span
                        >
                      </div>
                    </div>
                  </div>
                  <div v-else class="no-notice py-8 text-center opacity-60">
                    暂无通知
                  </div>
                </NScrollbar>
              </div>
            </template>
          </NCard>

          <!-- 通知详情模态框 -->
          <n-modal
            v-model:show="noticeModalVisible"
            preset="card"
            :title="selectedNotice?.title"
            class="h-[80vh] max-w-[70vw]"
          >
            <n-scrollbar
              v-if="selectedNotice"
              class="h-[calc(80vh-120px)]"
            >
              <div class="notice-modal-content">
                <div class="notice-modal-meta flex items-center gap-2">
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
                    class="notice-time text-xs opacity-80"
                    :style="{ color: themeStore.$state.primaryColor }"
                    >{{ formatTime(selectedNotice.created_at) }}</span
                  >
                </div>
                <n-divider />
                <div
                  class="notice-content leading-relaxed [&_a]:text-primary [&_a]:no-underline [&_a:hover]:underline [&_blockquote]:border-l-4 [&_blockquote]:border-[var(--n-border-color)] [&_blockquote]:pl-3 [&_code]:rounded [&_code]:bg-black/5 [&_code]:px-1 [&_code]:py-0.5 [&_h1]:text-2xl [&_h2]:text-xl [&_h3]:text-lg [&_ol]:list-decimal [&_ol]:pl-6 [&_p]:my-2 [&_pre]:overflow-x-auto [&_pre]:rounded-lg [&_pre]:bg-black/5 [&_pre]:p-3 [&_ul]:list-disc [&_ul]:pl-6"
                  v-html="renderNoticeContent(selectedNotice.message)"
                />
              </div>
            </n-scrollbar>
          </n-modal>
        </div>
      </div>
    </motion.div>
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
import { motion } from "motion-v";
import { marked } from "marked";
import DOMPurify from "dompurify";
import { useRouter } from "vue-router";
import { userApi } from "../../net";
import { accessHandle } from "../../net/base";
import UserInfo from "../../components/UserInfo.vue";
import WelcomeCard from "../../components/WelcomeCard.vue";
import Statistic from "../../components/Statistic.vue";
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
