import { invoke } from "@tauri-apps/api/core";
import { accessHandle } from "../base";
import type {
  CreateNetworkRequest,
  JoinNetworkRequest,
  LeaveNetworkResponse,
  NetworkListItem,
} from "../../types/virtualNetwork";

export const VirtualNetworkAPI = {
  async createNetwork(req: CreateNetworkRequest): Promise<any> {
    try {
      const resp = await invoke<any>("create_virtual_network", {
        name: req.name,
        password: req.password,
        isPublic: req.is_public,
        maxPlayers: req.max_players,
        headers: accessHandle(),
      });
      return resp;
    } catch (e: any) {
      return { code: 1, message: e?.toString() || "创建网络失败" };
    }
  },
  async joinNetwork(req: JoinNetworkRequest): Promise<any> {
    try {
      const data = await invoke<any>("join_virtual_network", {
        networkId: req.network_id,
        password: req.password,
        headers: accessHandle(),
      });
      return data;
    } catch (e: any) {
      return { code: 1, message: e?.toString() || "加入网络失败" };
    }
  },
  async leaveNetwork(networkId: string): Promise<LeaveNetworkResponse> {
    try {
      const data = await invoke<any>("leave_virtual_network", {
        networkId: networkId,
        headers: accessHandle(),
      });
      if (data.code !== 0) {
        throw new Error(data.message);
      }
      return { code: 0 };
    } catch (e: any) {
      return { code: 1, message: e?.toString() || "断开网络失败" };
    }
  },
  async getCurrentNetwork(): Promise<any> {
    try {
      const data = await invoke<any>("get_current_virtual_network", {
        headers: accessHandle(),
      });
      return data;
    } catch (e: any) {
      return { code: 1, message: e?.toString() || "获取当前网络失败" };
    }
  },
  async getRecentNetworks(): Promise<NetworkListItem[]> {
    // TODO: 后端支持后实现
    return [];
  },
  async getPublicNetworks(): Promise<NetworkListItem[]> {
    // TODO: 后端支持后实现
    return [];
  },
};
