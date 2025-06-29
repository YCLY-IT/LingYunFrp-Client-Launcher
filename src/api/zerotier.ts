import { invoke } from "@tauri-apps/api/core"

export interface CreateNetworkRequest {
  name: string
  description: string
  is_public: boolean
  max_members: number
}

export interface NetworkInfo {
  network_id: string
  name: string
  description: string
  is_public: boolean
  max_members: number
  current_members: number
  created_at: string
}

export class ZeroTierAPI {
  // 创建网络
  static async createNetwork(request: CreateNetworkRequest): Promise<string> {
    return await invoke('create_network', { request })
  }

  // 获取网络列表
  static async getNetworks(): Promise<NetworkInfo[]> {
    return await invoke('get_networks')
  }

  // 加入网络
  static async joinNetwork(networkId: string, nodeName: string): Promise<void> {
    return await invoke('join_network', { networkId, nodeName })
  }

  // 离开网络
  static async leaveNetwork(networkId: string): Promise<void> {
    return await invoke('leave_network', { networkId })
  }

  // 获取本地网络
  static async getLocalNetworks(): Promise<string[]> {
    return await invoke('get_local_networks')
  }

  // 检查ZeroTier是否已安装
  static async isInstalled(): Promise<boolean> {
    return await invoke('check_zerotier_installed')
  }
} 