<template>
  <div class="home">
    <!-- 欢迎横幅 -->
    <div class="welcome-banner">
      欢迎回来, {{ nickname }}
    </div>

    <!-- 用户卡片 -->
    <div class="content-info">
      <n-card :loading="loading" class="user-card">
        <n-space>
          <div class="user-card-avatar" :style="{ backgroundImage: `url(${avatar})`, backgroundSize: 'cover', backgroundPosition: 'center', width: '64px', height: '64px', borderRadius: '64px', marginTop: '1px', transform: 'scale(1.2)' }"></div>
          <div style="margin-left: 16px; text-align: left; margin-top: 5px;">
            <h3 style="margin: 0px;">{{ forTime }}，{{ nickname }}，{{ currentDate }} 😊</h3>
            <n-skeleton style="margin: 8px 0px 0px; width: 500px;" v-if="loading" />
            <p style="margin: 5px 0px 0px;">{{ textHitokoto }}</p>
          </div>
        </n-space>
      </n-card>
    </div>

    <!-- 统计卡片 -->
    <div class="statistic-container">
      <Statistic :signRemainder="userInfoRef?.userInfo.signRemainder" ref="statisticRef" />
    </div>

    <!-- 内容面板 -->
    <div style="margin-top: 20px;" class="content-grid">
      <div class="left-column">
        <!-- 用户信息卡片 -->
        <NCard title="用户信息" class="info-card">
          <NAlert v-if="!IsRealname" type="warning" title="未实名认证" style="margin-bottom: 16px">
            您的账户尚未完成实名认证, 请尽快完成实名认证。
            <br>
            <NButton text type="primary" @click="goToRealname">立即前往</NButton>
          </NAlert>
          <UserInfo ref="userInfoRef" @update="handleUserUpdate" />
        </NCard>
      </div>

      <!-- 通知卡片 -->
      <NCard title="通知内容" class="notice-card">
        <div class="markdown-content" v-html="renderedNotice" />
      </NCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NCard, NAlert, NButton, useMessage } from 'naive-ui'
import { ref, onMounted, computed } from 'vue'
import { marked } from 'marked'
import DOMPurify from 'dompurify'
import { useRouter } from 'vue-router'
import { userApi } from "../../net"
import { accessHandle } from "../../net/base"
import UserInfo from "../../components/UserInfo.vue"
import { Traffic } from '../../types/User'

const router = useRouter()
const message = useMessage()
const notices = ref('')
const nickname = localStorage.getItem('nickname') || ''

// 用户信息引用
const userInfoRef = ref<{ userInfo: { isRealname: boolean; avatar: string; signRemainder: number; } } | null>(null)

// 是否实名认证
const IsRealname = computed(() => userInfoRef.value?.userInfo.isRealname || true)

// 一言和流量数据
const textHitokoto = ref('')
const traffic = ref<Traffic>({} as Traffic)
const loading = ref(false)

// 现在几点
const forTime = computed(() => {
  const date = new Date()
  const hours = date.getHours()
  if (hours < 6) {
    return '凌晨好'
  } else if (hours < 12) {
    return '早上好'
  } else if (hours < 18) {
    return '下午好'
  } else {
    return '晚上好'
  }
})

// 用户头像
const avatar = userInfoRef.value?.userInfo.avatar || localStorage.getItem('avatar');

// 配置 marked
marked.setOptions({
  gfm: true,
  breaks: true
})

// 前往实名认证
const goToRealname = () => {
  router.push('/dashboard/profile')
}

// 渲染通知
const renderedNotice = computed(() => {
  if (!notices.value) return ''
  try {
    const html = marked.parse(notices.value) as string
    return DOMPurify.sanitize(html)
  } catch {
    return ''
  }
})

const handleUserUpdate = () => {
  getUserTraffic()
}

// 获取通知
const fetchNotice = async (): Promise<void> => {
  userApi.get('/user/info/broadcast', accessHandle(), (data) => {
    if (data.code === 0) {
      notices.value = data.data[0].broadcast
    } else {
      message.error(data.message || '获取公告失败')
    }
  }, (messageText) => {
    message.error('获取公告失败:' + messageText)
  })
}

// 获取一言
const getHitokoto = async (): Promise<void> => {
  loading.value = true
  userApi.getHitokoto({}, (data) => {
    textHitokoto.value = data.hitokoto
    loading.value = false
  }, (messageText) => {
    message.error('获取一言失败:' + messageText)
  })
}
// 获取用户流量
const getUserTraffic = async (): Promise<void> => {
  userApi.get('/user/info/traffic', accessHandle(), (data) => {
    traffic.value = data.data
  }, (messageText) => {
    message.error('获取用户流量失败:' + messageText)
  })
}

const currentDate = computed(() => {
  const date = new Date()
  return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`
})

// 页面挂载后执行
onMounted(() => {
  fetchNotice()
  getHitokoto()
  getUserTraffic()
})
</script>

<style lang="scss" scoped>
@use '../../assets/styles/home.scss';
</style>