<template>
  <div class="login">
    <NCard class="auth-card">
      <div class="auth-header">
        <div class="title-with-icon">
          <NIcon size="32" :component="LogInOutline" />
          <h1>{{ packageData.title }}</h1>
          <span>后台管理系统</span>
        </div>
        <br />
        <hr />
      </div>
      <NForm ref="formRef" :model="formValue" :rules="rules">
        <NFormItem path="username" label="用户名/邮箱">
          <NInput
            v-model:value="formValue.username"
            placeholder="请输入用户名或邮箱"
          />
        </NFormItem>
        <NFormItem path="password" label="密码">
          <NInput
            v-model:value="formValue.password"
            type="password"
            placeholder="请输入密码"
            show-password-on="click"
          />
        </NFormItem>
        <div class="checkbox-forgot">
          <a
            href="#"
            class="forgot-link"
            @click.prevent="OpenBrowser(packageData.url + '/forget')"
            >忘记密码？</a
          >
          <p></p>
        </div>
        <NButton
          :loading="loading"
          type="primary"
          block
          secondary
          strong
          @click="handleSubmit"
        >
          登录
        </NButton>
        <div style="display: flex; align-items: center; gap: 8px">
          <div class="register-link">
            <span>还没有账号？</span>
            <a href="#" @click.prevent="OpenBrowser(packageData.url + '/login')"
              >立即注册</a
            >
          </div>
          <div class="register-link" style="position: absolute; right: 25px">
            <a href="#" @click.prevent="clientLogin">网页端登录</a>
          </div>
        </div>
      </NForm>
    </NCard>
  </div>
</template>

<script setup lang="ts">
import packageData from "../../package.json";
import { onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";
import {
  NForm,
  NFormItem,
  NInput,
  NButton,
  NCard,
  NIcon,
  type FormRules,
  type FormInst,
  useMessage,
  useDialog,
} from "naive-ui";
import { LogInOutline } from "@vicons/ionicons5";
import { userApi } from "../net";
import { OpenBrowser, storeToken } from "../net/base";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const router = useRouter();
const message = useMessage();
const dialog = useDialog();
const loading = ref(false);
const formRef = ref<FormInst | null>(null);
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
    dialog.success({
      title: "登录成功",
      content: "即将跳转",
      positiveText: "确定",
      onPositiveClick: () => {
        router.push("/dashboard");
      },
    });
  });

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

<style lang="scss" scoped>
@use "../assets/styles/login.scss";
.login {
  height: 100vh;
  overflow: hidden;
  display: flex;
  position: relative;
}
</style>
