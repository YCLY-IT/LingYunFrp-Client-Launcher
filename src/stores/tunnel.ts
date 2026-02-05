import { defineStore } from "pinia";
import { ref } from "vue";

export const useTunnelStore = defineStore("tunnel", () => {
  // 存储正在启动中的隧道ID
  const startingProxies = ref<Set<number>>(new Set());

  // 添加正在启动的隧道
  const addStartingProxy = (proxyId: number) => {
    startingProxies.value.add(proxyId);
  };

  // 移除正在启动的隧道
  const removeStartingProxy = (proxyId: number) => {
    startingProxies.value.delete(proxyId);
  };

  // 检查隧道是否正在启动
  const isStarting = (proxyId: number): boolean => {
    return startingProxies.value.has(proxyId);
  };

  // 清空所有正在启动的隧道
  const clearStartingProxies = () => {
    startingProxies.value.clear();
  };

  return {
    startingProxies,
    addStartingProxy,
    removeStartingProxy,
    isStarting,
    clearStartingProxies,
  };
});
