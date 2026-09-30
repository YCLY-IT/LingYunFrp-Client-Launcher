<template>
  <div>
    <div
      class="user-info-scroll-wrapper max-[600px]:overflow-x-auto max-[600px]:[-webkit-overflow-scrolling:touch]"
    >
      <motion.div
        :key="loading ? 'loading' : 'loaded'"
        class="user-info-grid grid grid-cols-2 gap-5 max-[600px]:w-max"
        :initial="'hidden'"
        :animate="'visible'"
        :variants="gridVariants"
      >
        <template v-if="loading">
          <div v-for="i in 8" :key="i" class="user-info-item flex flex-col gap-2 p-0.5">
            <NSkeleton :sharp="false" size="medium" />
          </div>
        </template>
        <template v-else>
          <motion.div
            class="user-info-item relative flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              用户昵称
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ userInfo.nickname }}
            </div>
          </motion.div>

          <motion.div
            class="user-info-item-right relative ml-5 flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              实名认证
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              <NTag
                :type="userInfo.isRealname ? 'success' : 'default'"
                size="small"
              >
                {{ userInfo.isRealname ? "已实名" : "未实名" }}
              </NTag>
            </div>
          </motion.div>

          <motion.div
            class="user-info-item relative flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              用户组
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              <NTag type="info" size="small">
                {{ userInfo.friendlyGroup }}
              </NTag>
            </div>
          </motion.div>

          <motion.div
            class="user-info-item-right relative ml-5 flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              注册时间
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ formattedRegTime }}
            </div>
          </motion.div>

          <motion.div
            class="user-info-item relative flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              注册邮箱
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ userInfo.email }}
            </div>
          </motion.div>

          <motion.div
            class="user-info-item-right relative ml-5 flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              隧道数量
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ userInfo.usedProxies }} / {{ userInfo.maxProxies }}
            </div>
          </motion.div>
          <motion.div
            class="user-info-item relative flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              剩余流量
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ formattedTraffic }}
            </div>
          </motion.div>
          <motion.div
            class="user-info-item-right relative ml-5 flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              剩余积分
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ userInfo.point }} 分
            </div>
          </motion.div>
          <motion.div
            class="user-info-item relative flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              入站带宽
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ userInfo.inlimit / 128 }} Mbps
            </div>
          </motion.div>

          <motion.div
            class="user-info-item-right relative ml-5 flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div class="user-info-label text-sm text-[var(--n-text-color-2)]">
              出站带宽
            </div>
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              {{ userInfo.outlimit / 128 }} Mbps
            </div>
          </motion.div>
          <motion.div
            class="user-info-item relative flex flex-col gap-2 p-0.5"
            :variants="itemVariants"
          >
            <div
              class="user-info-value whitespace-nowrap text-sm text-[var(--n-text-color)]"
            >
              <NSpace class="token-section relative mt-[3px] flex justify-self-start">
                <NButton
                  text
                  type="primary"
                  size="small"
                  class="transition-transform duration-200 hover:scale-105"
                  @click="handleCopyToken"
                >
                  <template #icon>
                    <CopyPlusIcon />
                  </template>
                  <div class="text-sm">复制令牌</div>
                </NButton>
              </NSpace>
            </div>
          </motion.div>
        </template>
        <motion.div :variants="itemVariants">
          <NSpace vertical :size="4">
            <NButton
              text
              type="primary"
              :loading="signLoading"
              :disabled="!isSignAvailable"
              @click="handleSign"
            >
              <template #icon>
                <NIcon>
                  <CalendarOutline />
                </NIcon>
              </template>
              {{ signButtonText }}
            </NButton>
          </NSpace>
        </motion.div>
      </motion.div>
    </div>
    <br />
    <motion.div
      :initial="{ opacity: 0, y: 12 }"
      :animate="{ opacity: 1, y: 0 }"
      :transition="{ delay: 0.35, duration: 0.4, ease: [0.4, 0, 0.2, 1] }"
    >
      <NAlert class="user-info-item relative flex flex-col gap-2 p-0.5" type="info" show-icon>
        <NText depth="3" class="text-[13px]"
          >签到可以获得<NText type="primary"> 积分</NText> 和
        <NText type="primary">流量 </NText> 噢!(๑´ڡ`๑)
        </NText>
      </NAlert>
    </motion.div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { motion } from "motion-v";
import {
  NTag,
  NSkeleton,
  NButton,
  NIcon,
  NSpace,
  NText,
  NAlert,
  useMessage,
  useDialog,
} from "naive-ui";
import { CalendarOutline } from "@vicons/ionicons5";
import { userApi } from "../net";
import { accessHandle } from "../net/base.ts";
import { CopyPlusIcon } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
const emit = defineEmits<{
  (e: "update"): void;
}>();

const gridVariants = {
  hidden: {},
  visible: {
    transition: { staggerChildren: 0.045, delayChildren: 0.05 },
  },
};

const itemVariants = {
  hidden: { opacity: 0, y: 14 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { type: "spring" as const, stiffness: 260, damping: 24 },
  },
};

const message = useMessage();
const dialog = useDialog();
const loading = ref(true);
const signLoading = ref(false);
const isSignAvailable = ref(false);

const userInfo = ref({
  userId: 0,
  username: "",
  nickname: "",
  isRealname: false,
  group: "",
  friendlyGroup: "",
  usedProxies: 0,
  maxProxies: 0,
  regTime: "",
  traffic: 0,
  outlimit: 0,
  inlimit: 0,
  email: "",
  point: 0,
  status: 0,
  avatar: "",
  todaySigned: false,
  token: "",
  remainder: 0,
  signRemainder: 0,
});

const formatTime = (isoString: string) => {
  const date = new Date(isoString);
  return date.toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    hour12: false,
  });
};

const formatTraffic = (traffic: number) => {
  const value = traffic;
  if (isNaN(value)) return traffic;
  if (value >= 1024) {
    return `${(value / 1024).toFixed(2)} GB`;
  }
  return `${value.toFixed(2)} MB`;
};

const formattedRegTime = computed(() => formatTime(userInfo.value.regTime));
const formattedTraffic = computed(() => formatTraffic(userInfo.value.traffic));
const signButtonText = computed(() =>
  signLoading.value ? "签到中..." : isSignAvailable.value ? "签到" : "已签到",
);

// 执行签到
const handleSign = async () => {
  if (!isSignAvailable.value || signLoading.value) return;
  signLoading.value = true;
  userApi.post("/sign/", {}, accessHandle(), (data) => {
    if (data.code === 0) {
      dialog.success({
        title: "签到成功",
        content: `获得 ${data.data.point} 积分 和 ${data.data.traffic}MB 流量`,
        positiveText: "好的",
      });
      isSignAvailable.value = false;
      signLoading.value = false;
      emit("update");
      fetchUserInfo();
    } else {
      message.error(data.message || "签到失败");
      signLoading.value = false;
    }
  });
};

const handleCopyToken = async () => {
  try {
    await window.navigator.clipboard.writeText(userInfo.value.token);
    message.success("Token 已复制到剪贴板");
  } catch (err) {
    message.error("复制失败，请手动复制");
  }
};

const fetchUserInfo = async () => {
  loading.value = true;
  try {
    userApi.get("/user/info", accessHandle(), (data) => {
      if (data.code === 0) {
        userInfo.value = data.data;
        localStorage.setItem("group", userInfo.value.group);
        localStorage.setItem("token", userInfo.value.token);
        localStorage.setItem("username", userInfo.value.username);
        localStorage.setItem("nickname", userInfo.value.nickname);
        localStorage.setItem("token", userInfo.value.token);
        isSignAvailable.value = !data.data.sign;
      }
      loading.value = false;
      invoke<string>("get_image_base64", { url: data.data.avatar })
        .then((avatarBase64) => {
          localStorage.setItem(
            "avatar",
            "data:image/jpeg;base64," + avatarBase64,
          );
        })
        .catch(() => {
          console.log("获取头像失败");
        });
    });
  } catch (e) {
    console.log(e);
  }
};
onMounted(async () => {
  await fetchUserInfo();
});
defineExpose({
  userInfo,
});
</script>
