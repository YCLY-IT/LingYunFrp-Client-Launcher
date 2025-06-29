import { ref } from 'vue'

type StatusChangeCallback = (status: string) => void
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type MessageCallback = (msg: any) => void

export function useVirtualNetworkManager() {
  const statusChangeCallbacks: StatusChangeCallback[] = []
  const messageCallbacks: MessageCallback[] = []
  const initialized = ref(false)

  async function init() {
    // TODO: 初始化网络管理器
    initialized.value = true
  }

  function destroy() {
    // TODO: 清理资源
    initialized.value = false
    statusChangeCallbacks.length = 0
    messageCallbacks.length = 0
  }

  function onStatusChange(cb: StatusChangeCallback) {
    statusChangeCallbacks.push(cb)
  }

  function onMessage(cb: MessageCallback) {
    messageCallbacks.push(cb)
  }

  // 模拟触发
  function triggerStatusChange(status: string) {
    statusChangeCallbacks.forEach(cb => cb(status))
  }
  function triggerMessage(msg: any) {
    messageCallbacks.forEach(cb => cb(msg))
  }

  return {
    init,
    destroy,
    onStatusChange,
    onMessage,
    triggerStatusChange,
    triggerMessage,
    initialized
  }
} 