// 虚拟网络节点
export interface VirtualNetworkNode {
  id: string;
  name: string;
  is_online: boolean;
  is_owner: boolean;
  virtual_ip: string;
}

// 虚拟网络配置
export interface VirtualNetworkConfig {
  name: string;
  network_id: string;
  password?: string;
  virtual_subnet: string;
  max_players: number;
}

// 虚拟网络
export interface VirtualNetwork {
  config: VirtualNetworkConfig;
  status: 'Active' | 'Inactive';
  nodes: Record<string, VirtualNetworkNode>;
}

// 创建网络请求
export interface CreateNetworkRequest {
  name: string;
  password?: string;
  is_public: boolean;
  max_players: number;
}

// 创建网络响应
export interface CreateNetworkResponse {
  code: number;
  message?: string;
  data?: VirtualNetwork;
}

// 加入网络请求
export interface JoinNetworkRequest {
  network_id: string;
  password?: string;
}

// 加入网络响应
export interface JoinNetworkResponse {
  code: number;
  message?: string;
  data?: VirtualNetwork;
}

// 离开网络响应
export interface LeaveNetworkResponse {
  code: number;
  message?: string;
}

// 网络列表项
export interface NetworkListItem {
  id: string;
  name: string;
  lastJoinTime?: string;
  currentPlayers?: number;
  maxPlayers?: number;
  owner?: { name: string };
} 