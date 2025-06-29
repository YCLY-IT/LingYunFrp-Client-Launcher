import type {
  CreateNetworkRequest,
  CreateNetworkResponse,
  JoinNetworkRequest,
  JoinNetworkResponse,
  LeaveNetworkResponse,
  NetworkListItem,
  VirtualNetwork
} from '../types/virtualNetwork'

export const VirtualNetworkAPI = {
  async createNetwork(req: CreateNetworkRequest): Promise<CreateNetworkResponse> {
    // TODO: 替换为实际 API 调用
    return {
      code: 0,
      data: {
        config: {
          name: req.name,
          network_id: Math.random().toString(36).slice(2, 10),
          password: req.password,
          virtual_subnet: '10.0.0.0/24',
          max_players: req.max_players
        },
        status: 'Active',
        nodes: {}
      }
    }
  },
  async joinNetwork(req: JoinNetworkRequest): Promise<JoinNetworkResponse> {
    // 模拟房主节点和本地节点
    const ownerNode = {
      id: 'owner-1',
      name: '房主节点',
      is_online: true,
      is_owner: true,
      virtual_ip: '10.0.0.1'
    }
    const localNode = {
      id: 'local-1',
      name: '本地节点',
      is_online: true,
      is_owner: false,
      virtual_ip: '10.0.0.2'
    }
    return {
      code: 0,
      data: {
        config: {
          name: '示例网络',
          network_id: req.network_id,
          password: req.password,
          virtual_subnet: '10.0.0.0/24',
          max_players: 10
        },
        status: 'Active',
        nodes: {
          [ownerNode.id]: ownerNode,
          [localNode.id]: localNode
        }
      }
    }
  },
  async leaveNetwork(): Promise<LeaveNetworkResponse> {
    // TODO: 替换为实际 API 调用
    return { code: 0 }
  },
  async getRecentNetworks(): Promise<NetworkListItem[]> {
    // TODO: 替换为实际 API 调用
    return []
  },
  async getPublicNetworks(): Promise<NetworkListItem[]> {
    // TODO: 替换为实际 API 调用
    return []
  }
} 