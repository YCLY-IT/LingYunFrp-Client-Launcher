<template>
  <Suspense>
    <NConfigProvider :theme="theme" :theme-overrides="themeOverrides">
      <NDialogProvider>
        <NMessageProvider>
          <NNotificationProvider>
            <NLoadingBarProvider>
              <AppContent />
            </NLoadingBarProvider>
          </NNotificationProvider>
        </NMessageProvider>
      </NDialogProvider>
    </NConfigProvider>
  </Suspense>
</template>

<script setup lang="ts">
import { ref, computed, provide, onMounted, watch } from 'vue'
import {
  NConfigProvider,
  NMessageProvider,
  NDialogProvider,
  NNotificationProvider,
  NLoadingBarProvider,
  darkTheme
} from 'naive-ui'
import { themeOverrides } from './constants/theme'
import AppContent from './components/AppContent.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { userApi } from './net'
import { accessHandle } from './net/base'
import { useRoute } from 'vue-router'

// 从localStorage读取主题状态，默认跟随系统
const isDarkMode = ref(localStorage.getItem('theme') === 'dark' || 
  window.matchMedia('(prefers-color-scheme: dark)').matches)

// 主题状态
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)')
const theme = computed(() => isDarkMode.value ? darkTheme : null)

// 监听系统主题变化
prefersDark.addEventListener('change', (e) => {
  isDarkMode.value = e.matches
})
const cleanupFunctions = ref<(() => void)[]>([]);

const route = useRoute()
const isPageRefresh = ref(true) // 添加标记，默认为true表示是页面刷新

// 更新检查相关
const updateCheckInProgress = ref(false)
const lastUpdateCheck = ref<number>(0)
const UPDATE_CHECK_INTERVAL = 3600000 // 1小时
const isAppReady = ref(false)
const hasShownUpdateNotification = ref(false)
const MAX_RETRY_COUNT = 50 // 最大重试次数
const RETRY_INTERVAL = 200 // 重试间隔（毫秒）

const checkForUpdates = async () => {
  // 如果正在检查更新，直接返回
  if (updateCheckInProgress.value) return
  
  // 如果距离上次检查时间不足，直接返回
  const now = Date.now()
  if (now - lastUpdateCheck.value < UPDATE_CHECK_INTERVAL) return
  
  try {
    updateCheckInProgress.value = true
    const clientVersion = await invoke<string>('get_client_version')
    const systemInfo = await invoke<string>('get_system_info');
    let system = systemInfo.split(' ')[0];
    let arch = systemInfo.split(' ')[1];
    console.log(`客户端版本: ${clientVersion}, 系统: ${system}, 架构: ${arch}`);
    userApi.get(`/frp/updates/latest?software=LingYunFrpClient&system=${system}&arch=${arch}&version=${clientVersion}`, accessHandle(), (data: any) => {
      if (data.data.latest_info.version !== clientVersion && !hasShownUpdateNotification.value) {
        // 只在页面刷新时显示通知
        if (performance.navigation.type === 1) {
          let retryCount = 0
          
          // 等待应用准备就绪
          const showNotification = () => {
            if (isAppReady.value && (window as any).$notification) {
              (window as any).$notification.info({
                title: `新版本 ${data.data.latest_info.version} 可用`,
                content: data.data.latest_info.release_notes,
                duration: 0, 
              })
              hasShownUpdateNotification.value = true
            } else if (retryCount < MAX_RETRY_COUNT) {
              // 如果还没准备好且未超过最大重试次数，继续重试
              retryCount++
              setTimeout(showNotification, RETRY_INTERVAL)
            } else {
              // 超过最大重试次数，记录错误
              console.error('显示更新通知失败：组件未就绪')
            }
          }
          
          // 延迟一段时间后开始尝试显示通知
          setTimeout(showNotification, 1000)
        }
      }
      
      // 记录日志
      setTimeout(async () => {
        await invoke('emit_event', {
          event: 'log',
          payload: {
            message: `新版本 ${data.data.latest_info.version} 可用`,
          }
        });
        appendLog(`新版本 ${data.data.latest_info.version} 可用`)
      }, 50)
      
      // 更新最后检查时间
      lastUpdateCheck.value = now
    }, (message: string) => {
      console.error(message)
    })
  } catch (error) {
    console.error('检查更新失败:', error);
  } finally {
    updateCheckInProgress.value = false
  }
}

// 监听路由变化
watch(() => route.path, () => {
  isPageRefresh.value = false // 路由变化时，将标记设为false
  checkForUpdates()
})

// 主题切换函数
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  localStorage.setItem('theme', isDarkMode.value ? 'dark' : 'light')
}

// 提供给全局使用
provide('theme', {
  isDarkMode,
  theme,
  toggleTheme
})

onMounted(async () => {
  // 设置主题
  const theme = localStorage.getItem('theme')
  if (theme === 'dark') {
    isDarkMode.value = true
  } else if (theme === 'light') {
    isDarkMode.value = false
  }

  // 初始化日志和检查frpc
  localStorage.setItem('frpcLogs', '');
  await checkFrpcHas();
  
  try {
    const globalLogUnlisten = await listen('log', (event: any) => {
      appendLog(event.payload.message);
    });
    cleanupFunctions.value.push(globalLogUnlisten);
  } catch(error) {
    console.error('监听日志失败:', error) 
  }

  // 标记应用已准备就绪
  isAppReady.value = true

  // 延迟检查更新，确保所有组件都已初始化
  setTimeout(() => {
    checkForUpdates()
  }, 2000)
})

const checkFrpcHas = async () => {
  try {
    const hasFrpc = await invoke<boolean>('check_frpc_exists')
    if (!hasFrpc) {
      (window as any).$notification?.error({
        title: 'frpc.exe不存在',
        content: '请到系统设置下载frpc.exe',
        duration: 0, 
      })
      setTimeout( async () => {
        await invoke('emit_event', {
          event: 'log',
          payload: {
            message: `frpc.exe不存在，请到系统设置下载frpc.exe`,
          }
        });
        appendLog('frpc.exe不存在，请到系统设置中下载frpc.exe')
      }, 500)
    }
  } catch (error) {
    console.error('检查frpc.exe失败:', error)
  }
}

const appendLog = (message: string) => {
  const timestamp = new Date().toLocaleTimeString();
  const logMessage = `[${timestamp}] [系统] ${message}\n`;
  const savedLogs = localStorage.getItem('frpcLogs') || '';
  localStorage.setItem('frpcLogs', savedLogs + '\n\n' + logMessage);
};
</script>

<style lang="scss">
@use "./assets/styles/transitions.scss";
input, textarea, select {
  font-size: 16px !important;
}

@media screen and (max-width: 768px) {
  input, textarea, select {
    font-size: 16px !important;
  }
}
</style>

<style>
.n-card {
  border-radius: 10px; /* 设置全局圆角大小 */
}
</style>