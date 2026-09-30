<template>
  <div
    class="login relative flex h-screen items-center justify-center overflow-hidden px-5"
  >
    <motion.div
      class="w-full max-w-[420px]"
      :initial="{ opacity: 0, y: 44, scale: 0.94 }"
      :animate="{ opacity: 1, y: 0, scale: 1 }"
      :transition="{ type: 'spring', stiffness: 210, damping: 22, delay: 0.04 }"
    >
      <NCard class="auth-card rounded-l-none! rounded-r-[15px]!">
        <div class="mb-6 text-center">
          <div class="flex flex-col items-center justify-center gap-3">
            <motion.div
              class="text-primary"
              :animate="{ y: [0, -6, 0] }"
              :transition="{
                duration: 3,
                repeat: Infinity,
                ease: 'easeInOut',
              }"
            >
              <NIcon size="32" :component="LogInOutline" />
            </motion.div>
            <h1
              class="m-0 bg-gradient-to-r from-primary to-primary-hover bg-clip-text text-2xl font-bold text-transparent"
            >
              {{ packageData.title }}
            </h1>
            <span class="text-[var(--n-text-color-2)]">后台管理系统</span>
          </div>
          <br />
          <hr />
        </div>
        <NForm ref="formRef" :model="formValue" :rules="rules">
          <NFormItem
            path="username"
            label="用户名/邮箱"
            class="animate-rise-in [animation-delay:80ms]"
          >
            <NInput
              v-model:value="formValue.username"
              placeholder="请输入用户名或邮箱"
            />
          </NFormItem>
          <NFormItem
            path="password"
            label="密码"
            class="animate-rise-in [animation-delay:160ms]"
          >
            <NInput
              v-model:value="formValue.password"
              type="password"
              placeholder="请输入密码"
              show-password-on="click"
            />
          </NFormItem>
          <div
            class="checkbox-forgot animate-rise-in -mt-2 mb-4 flex justify-end [animation-delay:220ms]"
          >
            <a
              href="#"
              class="text-sm text-[var(--n-text-color-2)] no-underline transition-colors duration-200 hover:text-primary"
              @click.prevent="OpenBrowser(packageData.url + '/forget')"
              >忘记密码？</a
            >
            <p></p>
          </div>
          <div class="animate-rise-in [animation-delay:280ms]">
            <NButton
              :loading="loading"
              type="primary"
              block
              secondary
              strong
              class="transition-transform duration-200 hover:scale-[1.02] active:scale-[0.98]"
              @click="handleSubmit"
            >
              登录
            </NButton>
          </div>
          <div
            class="animate-rise-in mt-4 flex items-center gap-2 [animation-delay:340ms]"
          >
            <div class="flex items-center justify-center gap-2">
              <span class="text-[var(--n-text-color-2)]">还没有账号？</span>
              <a
                href="#"
                class="font-medium text-primary no-underline transition-colors duration-200 hover:text-primary-pressed"
                @click.prevent="OpenBrowser(packageData.url + '/login')"
                >立即注册</a
              >
            </div>
            <div class="absolute right-[25px]">
              <a
                href="#"
                class="text-sm text-[var(--n-text-color-2)] no-underline transition-colors duration-200 hover:text-primary"
                @click.prevent="clientLogin"
                >网页端登录</a
              >
            </div>
          </div>
        </NForm>
      </NCard>
    </motion.div>
  </div>
</template>

<script setup lang="ts">
import packageData from "../../package.json";
import { onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";
import { motion } from "motion-v";
import {
  NForm,
  NFormItem,
  NInput,
  NButton,
  NCard,
  NIcon,
  type FormRules,
  useMessage,
  useDialog,
} from "naive-ui";
import { LogInOutline } from "@vicons/ionicons5";
import { userApi } from "../net";
import { OpenBrowser, storeToken } from "../net/base";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useThemeStore } from "../stores/theme";

const router = useRouter();
const message = useMessage();
const dialog = useDialog();
const themeStore = useThemeStore();
const loading = ref(false);
const formValue = ref({
  username: "",
  password: "",
  remember: true,
});

const rules: FormRules = {
  username: {
    required: true,
    message: "请输入用户名/邮箱",
    trigger: "blur",
  },
  password: {
    required: true,
    message: "请输入密码",
    trigger: "blur",
  },
};

const handleSubmit = async () => {
  if (!formValue.value.username) {
    message.error("请输入用户名/邮箱");
    return;
  }
  if (!formValue.value.password) {
    message.error("请输入密码");
    return;
  }
  loading.value = true;
  try {
    userApi.login(
      formValue.value.username,
      formValue.value.password,
      formValue.value.remember,
      async (data: any) => {
        localStorage.setItem("username", data.data.username);
        localStorage.setItem("nickname", data.data.nickname);
        localStorage.setItem("avatarURL", data.data.avatar);
        localStorage.setItem("email", data.data.email);

        message.success(data.message);

        // 立即跳转
        setTimeout(() => {
          router.push("/dashboard");
        }, 1000);

        // 异步处理 base64，不阻塞跳转
        invoke<string>("get_image_base64", { url: data.data.avatar })
          .then((avatarBase64) => {
            localStorage.setItem(
              "avatar",
              "data:image/jpeg;base64," + avatarBase64,
            );
          })
          .catch(() => {
            message.error("获取头像失败");
          });
      },
      (data: any) => {
        message.error(data);
      },
    );
  } catch (error) {
    message.error(error);
  } finally {
    loading.value = false;
  }
};
let unlisten: () => void = () => {};
onMounted(async () => {
  unlisten = await listen("deep-link", (event) => {
    const url = new URL(event.payload[0]);
    const token = url.searchParams.get("token").replace("Bearer ", "");
    storeToken(token, true, new Date(Date.now() + 1000 * 60 * 60 * 24 * 7));
    localStorage.setItem("isDeepLinkLogin", "true");
    dialog.success({
      title: "登录成功",
      content: "即将跳转",
      positiveText: "确定",
      onPositiveClick: () => {
        router.push("/dashboard");
      },
    });
  });

  if (themeStore.backgroundImage !== "") {
    return;
  }
  const loginEl = document.querySelector(".login") as HTMLElement;
  if (loginEl) {
    try {
      const base64 = await invoke<string>("get_image_base64", {
        url: "https://api.nxvav.cn/api/bing",
      });
      loginEl.style.backgroundImage = `url('data:image/jpeg;base64,${base64}')`;
      loginEl.style.backgroundSize = "cover";
      loginEl.style.backgroundPosition = "center";
    } catch (e) {
      // 失败时可设置默认背景
      loginEl.style.background = "#222";
    }
  }
});

const clientLogin = () => {
  const ts = new Date(Math.floor(Date.now() / 20_000) * 20_000)
    .toISOString()
    .slice(0, 19)
    .replace(/[-:T]/g, "");
  const cipher = [...ts].map((c, i) => (+c + 7 * i + 23) % 10).join("");
  console.log("明文", ts, "密文", cipher);
  OpenBrowser(packageData.url + "/dashboard/profile?login=" + cipher);
};
onUnmounted(() => {
  // 移除监听
  unlisten();
});
</script>
