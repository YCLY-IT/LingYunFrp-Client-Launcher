<template>
  <div class="profile-container">
    <!-- 欢迎横幅 -->
    <div class="welcome-banner">个人资料设置</div>

    <div class="content-grid">
      <!-- 左侧设置区域 -->
      <div class="left-column">
        <n-card title="账户设置" class="settings-card">
          <div class="settings-grid">
            <!-- 修改用户名 -->
            <div class="setting-item" @click="showModal('changeUsername')">
              <div class="setting-icon">
                <UserIcon />
              </div>
              <div class="setting-content">
                <h3 class="setting-title">修改用户名</h3>
                <p class="setting-desc">点击这里可以修改您的用户名</p>
              </div>
              <div class="setting-arrow">
                <n-icon><ChevronRightIcon /></n-icon>
              </div>
            </div>

            <!-- 更换昵称 -->
            <div class="setting-item" @click="showModal('changeNickname')">
              <div class="setting-icon">
                <UserIcon />
              </div>
              <div class="setting-content">
                <h3 class="setting-title">更换昵称</h3>
                <p class="setting-desc">点击这里可以修改您的昵称</p>
              </div>
              <div class="setting-arrow">
                <n-icon><ChevronRightIcon /></n-icon>
              </div>
            </div>

            <!-- 更改头像 -->
            <div class="setting-item" @click="showModal('changeAvatar')">
              <div class="setting-icon">
                <ImageUpIcon />
              </div>
              <div class="setting-content">
                <h3 class="setting-title">更改头像</h3>
                <p class="setting-desc">点击这里上传图片，可以更换您的头像</p>
              </div>
              <div class="setting-arrow">
                <n-icon><ChevronRightIcon /></n-icon>
              </div>
            </div>

            <!-- 修改密码 -->
            <div class="setting-item" @click="showModal('changePassword')">
              <div class="setting-icon">
                <LockIcon />
              </div>
              <div class="setting-content">
                <h3 class="setting-title">修改密码</h3>
                <p class="setting-desc">点击这里可以修改您的登录密码</p>
              </div>
              <div class="setting-arrow">
                <n-icon><ChevronRightIcon /></n-icon>
              </div>
            </div>

            <!-- 实人认证 -->
            <div
              v-if="!UserInfo.isRealname"
              class="setting-item setting-item-warning"
              @click="openRealnameVerification"
            >
              <div class="setting-icon">
                <BadgeCheckIcon />
              </div>
              <div class="setting-content">
                <h3 class="setting-title">实人认证</h3>
                <p class="setting-desc">点击这里可以实人认证哦</p>
              </div>
              <div class="setting-arrow">
                <n-icon><ChevronRightIcon /></n-icon>
              </div>
            </div>
            <!-- 重置Token -->
            <div class="setting-item" @click="showModal('changeResetToken')">
              <div class="setting-icon">
                <KeyIcon />
              </div>
              <div class="setting-content">
                <h3 class="setting-title">重置Token</h3>
                <p class="setting-desc">点击这里可以重置您的Token (退出登录)</p>
              </div>
              <div class="setting-arrow">
                <n-icon><ChevronRightIcon /></n-icon>
              </div>
            </div>
          </div>
        </n-card>
      </div>

      <!-- 右侧账户详情区域 -->
      <div class="right-column">
        <n-card title="账户详情" class="account-card">
          <div class="user-profile-all" style="margin-left: 10px">
            <div class="user-profile">
              <div class="user-avatar">
                <div
                  :style="{
                    backgroundImage: `url(${UserInfo.avatar})`,
                    backgroundSize: 'cover',
                    backgroundPosition: 'center',
                    width: '80px',
                    height: '80px',
                    borderRadius: '64px',
                  }"
                  alt="User Avatar"
                />
              </div>
              <div class="user-info">
                <h3 class="user-greeting">Hi, {{ UserInfo.nickname }}</h3>
                <span style="display: flex; font-size: 17px"
                  >今天过的还好吗</span
                >
                <p class="user-email">{{ UserInfo.email }}</p>
              </div>
            </div>

            <div class="account-info-grid">
              <userInfo ref="userInfoRef" />
            </div>
          </div>
        </n-card>
      </div>
    </div>

    <!-- 模态窗口 -->
    <!-- 修改用户名模态窗口 -->
    <n-modal
      v-model:show="modals.changeUsername"
      preset="card"
      title="修改用户名"
      style="width: 500px"
    >
      <n-form
        ref="usernameFormRef"
        :model="forms.username"
        :rules="rules.username"
      >
        <n-form-item label="当前用户名" path="currentUsername">
          <n-input v-model:value="forms.username.currentUsername" disabled />
        </n-form-item>
        <n-form-item label="新用户名" path="newUsername">
          <n-input
            v-model:value="forms.username.newUsername"
            placeholder="请输入新用户名"
          />
        </n-form-item>
        <n-form-item label="邮箱" path="email">
          <n-input
            v-model:value="forms.username.emailCode"
            placeholder="请输入邮箱"
          />
        </n-form-item>
        <n-form-item label="验证码" path="emailCode">
          <div style="display: flex; gap: 8px">
            <n-input
              v-model:value="forms.username.emailCode"
              placeholder="请输入验证码"
            />
            <n-button
              :disabled="emailCodeSending"
              @click="
                sendEmailVerificationCode(
                  'UpdateUsername',
                  forms.username.emailCode,
                )
              "
            >
              {{ emailCodeButtonText }}
            </n-button>
          </div>
        </n-form-item>
      </n-form>
      <div class="modal-actions">
        <n-button
          style="margin-right: 16px"
          @click="modals.changeUsername = false"
          >取消</n-button
        >
        <n-button
          :loading="loading"
          type="primary"
          @click="handleChangeUsername"
          >确认修改</n-button
        >
      </div>
    </n-modal>

    <!-- 修改昵称模态 -->
    <n-modal
      v-model:show="modals.changeNickname"
      preset="card"
      title="更换昵称"
      style="width: 500px"
    >
      <n-form
        :model="forms.nickname"
        :rules="rules.nickname"
        ref="forms.nickname"
        label-placement="left"
        label-width="auto"
        :show-feedback="false"
      >
        <n-alert type="info" title="提示">
          在我们称呼你时将会使用该昵称。
        </n-alert>
        <n-form-item label="当前昵称" style="margin-top: 20px">
          <n-input
            v-model:value="UserInfo.nickname"
            readonly
            style="cursor: default"
          />
        </n-form-item>
        <n-form-item label="新的昵称" style="margin-top: 20px">
          <n-input
            v-model:value="forms.nickname.newNickname"
            placeholder="请输入新的昵称"
          />
        </n-form-item>
        <br />
      </n-form>
      <div class="modal-actions">
        <n-button
          style="margin-right: 16px"
          @click="modals.changeNickname = false"
          >取消</n-button
        >
        <n-button
          :loading="loading"
          type="primary"
          @click="handleUpdateNickname"
          >确定</n-button
        >
      </div>
    </n-modal>

    <!-- 更改头像模态窗口 -->
    <n-modal
      v-model:show="modals.changeAvatar"
      preset="card"
      title="更改头像"
      style="width: 500px"
    >
      <n-form ref="avatarFormRef" :model="forms.avatar">
        <n-tabs v-model:value="forms.avatar.avatarMode">
          <n-tab-pane name="upload" tab="上传图片">
            <n-form-item label="上传头像" path="avatarUrl">
              <n-upload
                @before-upload="handleBeforeUpload"
                v-model:file-list="forms.avatar.avatarFile"
                accept="image/*"
                list-type="image-card"
                :max="1"
              />
            </n-form-item>
          </n-tab-pane>
          <n-tab-pane name="qq" tab="QQ 头像">
            <n-form-item label="QQ号">
              <n-input
                v-model:value="forms.avatar.qqNumber"
                placeholder="请输入QQ号"
              />
            </n-form-item>
          </n-tab-pane>
          <n-tab-pane name="cravatar" tab="Cravatar">
            <n-form-item label="邮箱">
              <n-input :value="UserInfo.email" disabled />
            </n-form-item>
          </n-tab-pane>
        </n-tabs>
        <n-form-item label="预览">
          <div class="avatar-preview">
            <template v-if="forms.avatar.avatarMode === 'upload'">
              <div
                :style="{
                  backgroundImage: `url(${forms.avatar.avatarUrl})`,
                  borderRadius: '50%',
                  width: '100px',
                  height: '100px',
                  backgroundSize: 'cover',
                  backgroundPosition: 'center',
                }"
                alt="Avatar Preview"
              />
            </template>
            <template
              v-else-if="
                forms.avatar.avatarMode === 'qq' && forms.avatar.qqNumber
              "
            >
              <img
                :src="`https://q1.qlogo.cn/g?b=qq&nk=${forms.avatar.qqNumber}&s=640`"
                style="
                  width: 100px;
                  height: 100px;
                  border-radius: 50%;
                  object-fit: cover;
                "
              />
            </template>
            <template v-else-if="forms.avatar.avatarMode === 'cravatar'">
              <img
                :src="`https://cravatar.cn/avatar/${md5(UserInfo.email)}?s=100`"
                style="
                  width: 100px;
                  height: 100px;
                  border-radius: 50%;
                  object-fit: cover;
                "
              />
            </template>
            <template v-else>
              <div
                :style="{
                  backgroundImage: `url(${UserInfo.avatar})`,
                  borderRadius: '50%',
                  width: '100px',
                  height: '100px',
                  backgroundSize: 'cover',
                  backgroundPosition: 'center',
                }"
                alt="当前头像"
              />
            </template>
          </div>
        </n-form-item>
      </n-form>
      <div class="modal-actions">
        <n-button @click="modals.changeAvatar = false">取消</n-button>
        <n-button :loading="loading" type="primary" @click="handleChangeAvatar"
          >确认修改</n-button
        >
      </div>
    </n-modal>

    <!-- 修改密码模态窗口 -->
    <n-modal
      v-model:show="modals.changePassword"
      preset="card"
      title="修改密码"
      style="width: 500px"
    >
      <n-form
        ref="passwordFormRef"
        :model="forms.password"
        :rules="rules.password"
      >
        <n-form-item label="当前密码" path="currentPassword">
          <n-input
            v-model:value="forms.password.currentPassword"
            type="password"
            placeholder="请输入当前密码"
          />
        </n-form-item>
        <n-form-item label="新密码" path="newPassword">
          <n-input
            v-model:value="forms.password.newPassword"
            type="password"
            placeholder="请输入新密码"
          />
        </n-form-item>
        <n-form-item label="确认新密码" path="confirmPassword">
          <n-input
            v-model:value="forms.password.confirmPassword"
            type="password"
            placeholder="请再次输入新密码"
          />
        </n-form-item>
      </n-form>
      <div class="modal-actions">
        <n-button
          style="margin-right: 16px"
          @click="modals.changePassword = false"
          >取消</n-button
        >
        <n-button
          :loading="loading"
          type="primary"
          @click="handleChangePassword"
          >确认修改</n-button
        >
      </div>
    </n-modal>

    <!-- 裁剪头像模态窗口 -->
    <n-modal
      v-model:show="cropperVisible"
      preset="card"
      title="裁剪头像"
      style="width: 500px"
    >
      <div class="cropper-container" style="height: 360px">
        <Cropper
          ref="cropperRef"
          :src="cropperImg"
          :stencil-props="{
            aspectRatio: 1,
            minWidth: '80%',
            minHeight: '80%',
          }"
          :resize-image="{
            touch: true,
            wheel: true,
          }"
          :stencil-component="CircleStencil"
          :auto-zoom="true"
          :canvas="{
            width: 300,
            height: 300,
          }"
          :default-visibility="true"
          :class-names="{
            default: 'vue-advanced-cropper',
          }"
          style="height: 300px"
        />
      </div>
      <div class="modal-actions">
        <n-button @click="cropperVisible = false">取消</n-button>
        <n-button type="primary" @click="handleCropConfirm">确认</n-button>
      </div>
    </n-modal>
    <n-modal
      v-model:show="modals.changeResetToken"
      type="warning"
      preset="dialog"
      title="重置密钥"
      style="width: 500px"
    >
      <div style="margin-bottom: 24px">
        此操作将重置您的Token（密钥），这将可能导致你的服务全部中断，并会导致当前账号强制退出登录。请确认是否继续？
      </div>
      <div class="modal-actions">
        <n-button @click="modals.changeResetToken = false">取消</n-button>
        <n-button type="error" @click="handleResetToken" :loading="loading"
          >确认重置</n-button
        >
      </div>
    </n-modal>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import {
  NModal,
  NButton,
  NForm,
  NFormItem,
  NInput,
  NIcon,
  NCard,
  NUpload,
  useMessage,
  NTabs,
  NTabPane,
  useDialog,
} from "naive-ui";
import {
  UserIcon,
  ImageUpIcon,
  LockIcon,
  BadgeCheckIcon,
  ChevronRightIcon,
  KeyIcon, // 新增
  // 或 RefreshCcwIcon,
} from "lucide-vue-next";
import userInfo from "../../components/UserInfo.vue";
import { UploadFileInfo } from "naive-ui";
import { userApi } from "../../net";
import { accessHandle, OpenBrowser, removeToken } from "../../net/base";
import { Cropper, CircleStencil } from "vue-advanced-cropper";
import "vue-advanced-cropper/dist/style.css";
import md5 from "blueimp-md5";
import router from "../../router";

const userInfoRef = ref<InstanceType<typeof userInfo>>();
// 消息提示
const message = useMessage();
const dialog = useDialog();
// 用户信息
const UserInfo = reactive({
  username: localStorage.getItem("username") || "",
  nickname: localStorage.getItem("nickname") || "",
  email: localStorage.getItem("email") || "",
  avatar: localStorage.getItem("avatar") || "",
  isRealname: computed(() => userInfoRef.value?.userInfo.isRealname || false),
});
// 模态窗口状态
const modals = reactive({
  changeUsername: false,
  changeAvatar: false,
  changePassword: false,
  changeEmail: false,
  changeNickname: false,
  changeResetToken: false,
});

// 表单数据
const forms = reactive({
  username: {
    currentUsername: UserInfo.username,
    newUsername: "",
    emailCode: "",
  },
  avatar: {
    avatarMode: "upload", // 'upload' | 'qq' | 'cravatar'
    avatarUrl: UserInfo.avatar || "",
    avatarFile: [] as UploadFileInfo[],
    qqNumber: "",
  },
  password: {
    currentPassword: "",
    newPassword: "",
    confirmPassword: "",
  },
  email: {
    currentEmail: "",
    newEmail: "",
    verificationCode: "",
  },
  nickname: {
    newNickname: "",
  },
});

// 表单规则
const rules = {
  username: {
    newUsername: [
      { required: true, message: "请输入新用户名", trigger: "blur" },
      {
        min: 3,
        max: 20,
        message: "用户名长度应在3-20个字符之间",
        trigger: "blur",
      },
    ],
    password: [{ required: true, message: "请输入密码", trigger: "blur" }],
  },
  nickname: {
    newNickname: [
      { required: true, message: "请输入新的昵称", trigger: "blur" },
      {
        min: 2,
        max: 20,
        message: "昵称长度应在2-20个字符之间",
        trigger: "blur",
      },
    ],
  },
  password: {
    currentPassword: [
      { required: true, message: "请输入当前密码", trigger: "blur" },
    ],
    newPassword: [
      { required: true, message: "请输入新密码", trigger: "blur" },
      { min: 6, message: "密码长度不能少于6个字符", trigger: "blur" },
    ],
    confirmPassword: [
      { required: true, message: "请确认新密码", trigger: "blur" },
      {
        validator: (value) => {
          return value === forms.password.newPassword;
        },
        message: "两次输入的密码不一致",
        trigger: "blur",
      },
    ],
  },
  email: {
    newEmail: [
      { required: true, message: "请输入新邮箱", trigger: "blur" },
      {
        pattern: /^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/,
        message: "请输入有效的邮箱地址",
        trigger: "blur",
      },
    ],
    verificationCode: [
      { required: true, message: "请输入验证码", trigger: "blur" },
      { len: 6, message: "验证码长度应为6位", trigger: "blur" },
    ],
  },
};

// 表单引用
const usernameFormRef = ref(null);
const avatarFormRef = ref(null);
const passwordFormRef = ref(null);
const loading = ref(false);

// 邮箱验证码
const emailCodeSending = ref(false);
const emailCodeCountdown = ref(0);
const emailCodeButtonText = computed(() => {
  if (emailCodeSending.value) return "发送中...";
  if (emailCodeCountdown.value > 0) return `${emailCodeCountdown.value}s后重试`;
  return "获取验证码";
});

// 裁剪相关
const cropperImg = ref("");
const cropperVisible = ref(false);
const cropperRef = ref();
const croppedAvatarBase64 = ref("");

// 显示模态窗口
const showModal = (modalName) => {
  modals[modalName] = true;
};

const openRealnameVerification = () => {
  message.info("正在打开实名认证页面...");
  OpenBrowser("https://www.lyfrp.cn/dashboard/profile");
};

// 处理修改用户名
const handleChangeUsername = async () => {
  if (!forms.username.newUsername) {
    message.error("请输入新用户名");
    return;
  }
  if (!forms.username.emailCode) {
    message.error("请输入邮箱验证码");
    return;
  }
  if (forms.username.newUsername === UserInfo.username) {
    message.error("新用户名与当前用户名相同");
    return;
  }
  loading.value = true;
  try {
    userApi.post(
      "/user/update/username",
      {
        newUsername: forms.username.newUsername,
        emailCode: forms.username.emailCode,
      },
      accessHandle(),
      (data) => {
        UserInfo.username = forms.username.newUsername;
        message.success(data.message);
        forms.username.newUsername = "";
        forms.username.emailCode = "";
        modals.changeUsername = false;
      },
      (errors) => {
        message.error(errors || "用户名修改失败");
      },
    );
  } catch (errors) {
    // 表单验证失败
    message.error("用户名修改失败");
  }
  loading.value = false;
};
const handleResetToken = async () => {
  loading.value = true;
  userApi.post("/user/reset/token", {}, accessHandle(), (data) => {
    if (data.code === 0) {
      loading.value = false;
      modals.changeResetToken = false;
      dialog.success({
        title: "重置成功",
        content: "令牌重置成功，请重新登录",
        positiveText: "确定",
        onPositiveClick: () => {
          removeToken();
          router.push("/login");
        },
        closable: false,
      });
    }
  });
};
const sendEmailVerificationCode = async (model: string, email: string) => {
  if (emailCodeSending.value) return;
  if (!email) {
    message.error("请输入邮箱");
    return;
  }
  emailCodeSending.value = true;
  emailCodeCountdown.value = 60; // 初始化倒计时为60秒
  loading.value = true;

  try {
    userApi.post(
      `/user/code/${model}`,
      { email: email },
      accessHandle(),
      (data) => {
        if (data.code !== 0) {
          const timer = setInterval(() => {
            if (emailCodeCountdown.value > 0) {
              emailCodeCountdown.value--;
            } else {
              clearInterval(timer); // 倒计时结束后清除定时器
            }
          }, 1000);
          message.success("验证码发送成功");
        }
      },
      (messageText) => {
        message.error(messageText || "验证码发送失败");
      },
    );
  } catch (error) {
    message.error("验证码发送失败");
  } finally {
    emailCodeSending.value = false; // 无论成功/失败都关闭发送中状态
    loading.value = false;
  }
};
const handleUpdateNickname = async () => {
  if (!forms.nickname.newNickname) {
    message.error("昵称不能为空");
    return;
  }
  loading.value = true;
  try {
    userApi.post(
      `/user/update/nickname/${forms.nickname.newNickname}`,
      { nickname: forms.nickname.newNickname },
      accessHandle(),
      (data) => {
        localStorage.setItem("nickname", forms.nickname.newNickname);
        message.success(data.message);
        forms.nickname.newNickname = "";
        setTimeout(() => {
          window.location.reload();
        }, 1500);
      },
      (messageText) => {
        message.error(messageText || "昵称修改失败");
      },
    );
  } catch (error) {
    message.error("昵称修改失败");
  } finally {
    modals.changeNickname = false;
    loading.value = false;
  }
};

// 上传前处理（本地上传）
const handleBeforeUpload = async (options: { file: UploadFileInfo }) => {
  const { file } = options;
  if (!file.type?.startsWith("image/")) {
    message.error("只能上传图片文件");
    return false;
  }
  if (file.file?.size && file.file.size > 2 * 1024 * 1024) {
    message.error("图片大小不能超过2MB");
    return false;
  }
  const reader = new FileReader();
  reader.onload = (e) => {
    cropperImg.value = e.target?.result as string;
    cropperVisible.value = true;
  };
  if (file.file) {
    reader.readAsDataURL(file.file);
  }
  return false;
};

// 裁剪确认
const handleCropConfirm = () => {
  const { canvas } = cropperRef.value.getResult();
  if (canvas) {
    croppedAvatarBase64.value = canvas.toDataURL("image/png", 1);
    forms.avatar.avatarUrl = croppedAvatarBase64.value;
    // 生成 UploadFileInfo 结构
    const arr = croppedAvatarBase64.value.split(",");
    const bstr = atob(arr[1]);
    let n = bstr.length;
    const u8arr = new Uint8Array(n);
    while (n--) {
      u8arr[n] = bstr.charCodeAt(n);
    }
    const file = new File([u8arr], "avatar.png", { type: "image/png" });
    forms.avatar.avatarFile = [
      {
        id: Date.now().toString(),
        name: "avatar.png",
        status: "finished",
        percentage: 100,
        file: file,
        url: croppedAvatarBase64.value,
        type: "image/png",
        batchId: null,
        thumbnailUrl: "",
        fullPath: "",
      },
    ];
    cropperVisible.value = false;
  }
};

// 头像上传主逻辑
const handleChangeAvatar = () => {
  let params: any = { mode: forms.avatar.avatarMode };
  if (forms.avatar.avatarMode === "upload") {
    if (!forms.avatar.avatarFile || forms.avatar.avatarFile.length === 0) {
      message.error("请先上传头像");
      return;
    }
    // base64去头部
    const base64Data = forms.avatar.avatarUrl.split(",")[1];
    params = { mode: "upload", file: base64Data };
  } else if (forms.avatar.avatarMode === "qq") {
    if (!forms.avatar.qqNumber) {
      message.error("请输入QQ号");
      return;
    }
    params = { mode: "qq", qq: forms.avatar.qqNumber };
  } else if (forms.avatar.avatarMode === "cravatar") {
    if (!UserInfo.email) {
      message.error("未获取到邮箱，无法使用Cravatar");
      return;
    }
    params = { mode: "cravatar", cravatar: md5(UserInfo.email) };
  }
  loading.value = true;
  message.loading("正在上传头像...");
  userApi.post(
    "/user/update/avatar",
    params,
    undefined,
    (data) => {
      if (data.code === 0) {
        localStorage.setItem("avatar", data.data);
        message.success("头像上传成功");
        modals.changeAvatar = false;
        setTimeout(() => {
          window.location.reload();
        }, 1000);
      } else {
        message.error(data.message || "头像上传失败");
      }
      loading.value = false;
    },
    (error) => {
      message.error(typeof error === "string" ? error : "头像上传失败");
      loading.value = false;
    },
  );
};

// 处理修改密码
const handleChangePassword = async () => {
  if (!forms.password.currentPassword) {
    message.error("请输入当前密码");
    return;
  }

  if (forms.password.newPassword !== forms.password.confirmPassword) {
    message.error("两次输入的密码不一致");
    return;
  }
  loading.value = true;
  try {
    userApi.post(
      "/user/update/password",
      {
        oldPassword: forms.password.currentPassword,
        newPassword: forms.password.newPassword,
        confirmPassword: forms.password.confirmPassword,
      },
      accessHandle(),
      (data) => {
        forms.password.currentPassword = "";
        forms.password.newPassword = "";
        forms.password.confirmPassword = "";
        modals.changePassword = false;
        message.success(data.message);
        removeToken();
        setTimeout(() => {
          window.location.href = "/login";
        }, 2000);
      },
    );
  } catch (errors) {
    message.error("密码修改失败");
  } finally {
    loading.value = false;
  }
};
</script>

<style lang="scss" scoped>
@use "../../assets/styles/variables" as *;
.n-button {
  margin-right: 1em;
}

.profile-container {
  .welcome-banner {
    user-select: none;
    font-size: 1.5em;
    margin-bottom: 20px;
    padding: 12px 16px;
    background-color: $bg-color-hover;
    border-radius: 8px;
    font-weight: 500;
    color: $text-color;
  }

  .content-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    align-items: start;
  }

  @media (max-width: 768px) {
    .content-grid {
      grid-template-columns: 1fr;
    }
  }

  .left-column {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .right-column {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .settings-card,
  .account-card {
    width: 100%;
    box-shadow: $box-shadow;
    transition: $transition-all;

    &:hover {
      box-shadow: $box-shadow-hover;
    }

    :deep(.n-card__content) {
      padding: 16px;
    }
  }

  .settings-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .setting-item {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px;
    border-radius: 8px;
    cursor: pointer;
    transition: $transition-all;
    border: 1px solid transparent;

    &:hover {
      background-color: $bg-color-hover;
      border-color: $border-color;
      transform: translateY(-1px);
    }

    &.setting-item-warning {
      border-left: 4px solid #faad14;
      background-color: rgba(250, 173, 20, 0.05);

      &:hover {
        background-color: rgba(250, 173, 20, 0.1);
      }
    }
  }

  .setting-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: $primary-suppl;
    border-radius: 12px;
    color: $primary-color;
    flex-shrink: 0;

    svg {
      width: 24px;
      height: 24px;
    }
  }

  .setting-content {
    flex: 1;
    min-width: 0;
  }

  .setting-title {
    font-size: 16px;
    font-weight: 500;
    margin: 0 0 4px 0;
    color: $text-color;
  }

  .setting-desc {
    font-size: 14px;
    color: $text-color-2;
    margin: 0;
    line-height: 1.4;
  }

  .setting-arrow {
    color: $text-color-3;
    transition: $transition-all;

    svg {
      width: 20px;
      height: 20px;
    }
  }

  .setting-item:hover .setting-arrow {
    color: $primary-color;
    transform: translateX(2px);
  }

  .user-profile {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 24px;
  }

  .user-avatar {
    width: 80px;
    height: 80px;
    border-radius: 8px;
    overflow: hidden;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }
  }

  .user-info {
    flex: 1;
  }

  .user-greeting {
    font-size: 18px;
    font-weight: 500;
    margin: 0 0 4px 0;

    .user-id {
      font-weight: normal;
      opacity: 0.7;
    }
  }

  .user-email {
    font-size: 14px;
    color: #666;
    margin: 0;
  }

  .account-info-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
    border-radius: 8px;
    padding: 16px;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .info-label {
    font-size: 14px;
    color: #666;
  }

  .info-value {
    font-size: 16px;
    font-weight: 500;
  }

  // 模态窗口样式
  .modal-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 24px;
    padding-top: 16px;
    border-top: 1px solid $border-color;

    .n-button {
      min-width: 80px;
      margin-left: 16px;

      &:first-child {
        margin-left: 0;
      }
    }
  }

  .avatar-preview {
    width: 100%;
    height: 120px;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  // 响应式设计
  @media (max-width: 768px) {
    .welcome-banner {
      font-size: 1.2em;
      padding: 10px 12px;
    }

    .user-profile {
      flex-direction: column;
      text-align: center;
      gap: 16px;
    }

    .setting-item {
      padding: 12px;
      gap: 12px;
    }

    .setting-icon {
      width: 40px;
      height: 40px;

      svg {
        width: 20px;
        height: 20px;
      }
    }

    .setting-title {
      font-size: 15px;
    }

    .setting-desc {
      font-size: 13px;
    }
  }

  @media (max-width: 480px) {
    .content-grid {
      gap: 16px;
    }

    .setting-item {
      padding: 10px;
    }

    .user-profile {
      padding: 16px;
    }

    .user-greeting {
      font-size: 18px;
    }

    .user-subtitle {
      font-size: 14px;
    }
  }
}

.profile-container .modal-actions {
  display: flex !important;
  justify-content: flex-end !important;
}

// 裁剪器容器样式
.cropper-container {
  width: 100%;
  height: 300px;
  position: relative;
  margin-bottom: 20px;

  :deep(.vue-advanced-cropper) {
    height: 300px !important;

    .vue-advanced-cropper__image {
      opacity: 1 !important;
    }

    .vue-circle-stencil {
      box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.4);
      max-width: 200px;
      max-height: 200px;
    }
  }
}

.preview-container {
  margin-top: 20px;

  .cropper-preview {
    border: 2px solid #eee;
  }
}

// 确保裁剪区域是圆形的
:deep(.cropper-view-box),
:deep(.cropper-face) {
  border-radius: 50%;
}

:deep(.cropper-view-box) {
  outline: 0;
}

// 确保所有模态窗口的按钮样式一致
:deep(.n-modal) {
  .modal-actions {
    .n-button + .n-button {
      margin-left: 16px;
    }
  }
}

:deep(.n-form-item) {
  .n-form-item-label {
    font-size: 16px;
  }
}
</style>
