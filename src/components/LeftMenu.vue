<template>
  <NMenu :collapsed-width="64" :collapsed-icon-size="24" :options="menuOptions" :value="selectedKey" :icon-size="22"
    @update:value="handleMenuSelect" style="user-select: none" :default-expanded-keys="defaultExpandedKeys" />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NMenu, useMessage, useDialog } from 'naive-ui'
import { useRouter } from 'vue-router'
import { getMenuOptions, defaultExpandedKeys } from '../shared/menuOptions.ts'
import type { MenuOption } from '../types/menu'
import { invoke } from '@tauri-apps/api/core'
import { emitTo } from '@tauri-apps/api/event'

const emit = defineEmits(['select'])
const router = useRouter()
const message = useMessage()
const dialog = useDialog()
const menuOptions = getMenuOptions()

// 管理员权限检查函数
const checkAdminPermission = async (): Promise<boolean> => {
  try {
    const isAdmin = await invoke<boolean>('is_admin')
    return isAdmin
  } catch (e) {
    console.error('管理员权限检测失败:', e)
    return false
  }
}

// 处理虚拟网络菜单点击
const handleNetworkMenuClick = async (): Promise<boolean> => {
  const isAdmin = await checkAdminPermission()
  if (!isAdmin) {
    dialog.warning({
      title: '需要管理员权限',
      content: '虚拟网络功能需要以管理员权限运行，是否以管理员权限重启？',
      positiveText: '以管理员权限重启',
      negativeText: '取消',
      onPositiveClick: async () => {
        try {
          await emitTo('main', 'request_admin')
        } catch (e) {
          message.error('重启失败，请手动以管理员权限运行')
        }
      },
      onNegativeClick: () => {
        // 用户取消，不做任何操作
      }
    })
    return false // 阻止默认导航
  }
  
  // 有管理员权限，正常导航
  router.push('/dashboard/network')
  return true
}

const handleMenuSelect = async (key: string, _option: MenuOption) => {
  // 递归查找选中的菜单项
  function findOption(options: MenuOption[], key: string): MenuOption | undefined {
    for (const opt of options) {
      if (opt.key === key) return opt
      if (opt.children) {
        const found = findOption(opt.children as MenuOption[], key)
        if (found) return found
      }
    }
    return undefined
  }
  const opt = findOption(menuOptions, key)
  if (!opt) return

  if (opt.key === 'network') {
    // 这里调用真正的权限校验和跳转逻辑
    await handleNetworkMenuClick()
  } else if (opt.link) {
    router.push(opt.link)
  }
  selectedKey.value = key
  emit('select')
}

const selectedKey = ref('dashboardIndex')

</script>

