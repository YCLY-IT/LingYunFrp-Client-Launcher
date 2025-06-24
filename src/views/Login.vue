<template>
  <div class="login">
    <NCard class="auth-card">
      <div class="auth-header">
        <div class="title-with-icon">
          <NIcon size="32" :component="LogInOutline" />
          <h1>{{ packageData.title }}</h1>
          <span>后台管理系统</span>
        </div>
        <br>
        <hr>
      </div>
      <NForm ref="formRef" :model="formValue" :rules="rules">
        <NFormItem path="username" label="用户名/邮箱">
          <NInput v-model:value="formValue.username" placeholder="请输入用户名或邮箱" />
        </NFormItem>
        <NFormItem path="password" label="密码">
          <NInput v-model:value="formValue.password" type="password" placeholder="请输入密码"
                  show-password-on="click" />
        </NFormItem>
        <div class="checkbox-forgot">
          <!-- <router-link to="/forget" class="forgot-link" style="color: #1976D2">忘记密码？</router-link> -->
           <a href="#" class="forgot-link" @click.prevent="OpenBrowser(packageData.url + '/forget')">忘记密码？</a>
           <p></p>
        </div>
        <NButton :loading="loading" type="primary" block secondary strong @click="handleSubmit">
          登录
        </NButton>
        <div class="form-footer register-link">
          <span>还没有账号？</span>
          <a href="#" @click.prevent="OpenBrowser(packageData.url + '/login')">立即注册</a>
        </div>
      </NForm>
    </NCard>
  </div>
</template>

<style lang="scss" scoped>
.login {
  background-image: url('https://dailybing.com/api/v1');
  height: 100vh;
  overflow: hidden;
  display: flex;
  position: relative;
}
</style>


<script setup lang="ts">
import packageData from '../../package.json';
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { NForm, NFormItem, NInput, NButton, NCard, NIcon, type FormRules, useMessage, type FormInst } from 'naive-ui'
import { LogInOutline } from '@vicons/ionicons5'
import { userApi } from '../net'
import { OpenBrowser } from '../net/base'

const router = useRouter()
const message = useMessage()
const loading = ref(false)
const formRef = ref<FormInst | null>(null)
const formValue = ref({
  username: '',
  password: '',
  remember: true
})

const rules: FormRules = {
  username: {
    required: true,
    message: '请输入用户名/邮箱',
    trigger: 'blur'
  },
  password: {
    required: true,
    message: '请输入密码',
    trigger: 'blur'
  }
}

const handleSubmit = async () => {
if (!formValue.value.username) {
    message.error('请输入用户名/邮箱')
    return
  }
  if (!formValue.value.password) {
    message.error('请输入密码')
    return
  }
  loading.value = true;
  try{
  userApi.login(
      formValue.value.username,
      formValue.value.password,
      formValue.value.remember,
      (data: any) => {
        localStorage.setItem('username', data.data.username)
        localStorage.setItem('nickname', data.data.nickname)
        localStorage.setItem('avatar', data.data.avatar)
        localStorage.setItem('email', data.data.email)
        message.success(data.message)
        setTimeout(() => {
          router.push('/dashboard');
        }, 1200)
      },
      (data: any)=> {
        message.error(data)
      },
  )
  } catch (error) {
    message.error(error) 
  } finally {
    loading.value = false;
  }
}

</script>

<style lang="scss" scoped>
@use '../assets/styles/login.scss';
</style>