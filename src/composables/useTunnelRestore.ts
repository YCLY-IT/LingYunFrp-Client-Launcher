import { invoke } from "@tauri-apps/api/core";
import { h } from "vue";
import { useTunnelStore } from "../stores/tunnel";
import { get, accessHandle } from "../net/base";

const RUNNING_TUNNELS_KEY = "running_tunnel_ids";
const BOOT_SETTINGS_KEY = "boot_settings";

interface Proxy {
  proxyId: number;
  nodeId: number;
  isOnline: boolean;
  isDisabled: boolean;
  isBanned: boolean;
  proxyName: string;
  remotePort?: number;
}

interface Node {
  nodeId: number;
  isDisabled: boolean;
  status: boolean;
  hostname?: string;
}

function getPromise(
  url: string,
  headers: Record<string, string>,
): Promise<any> {
  return new Promise((resolve, reject) => {
    get(
      url,
      headers,
      (data: any) => resolve(data),
      (err: string) => reject(new Error(err)),
      (err: any) => reject(err),
    );
  });
}

export async function restoreTunnels() {
  const tunnelStore = useTunnelStore();

  const configStr = localStorage.getItem(BOOT_SETTINGS_KEY);
  if (!configStr) return;

  let config: any;
  try {
    config = JSON.parse(configStr);
  } catch {
    return;
  }

  if (!config.autoRestoreTunnels) return;

  const str = localStorage.getItem(RUNNING_TUNNELS_KEY);
  if (!str) return;

  let ids: number[] = [];
  try {
    ids = JSON.parse(str);
  } catch {
    return;
  }
  if (!Array.isArray(ids) || ids.length === 0) return;

  const token = localStorage.getItem("token") || "";
  if (!token) return;

  let proxies: Proxy[] = [];
  let nodes: Node[] = [];

  try {
    const [proxiesRes, nodesRes] = await Promise.all([
      getPromise("/proxies/", accessHandle()),
      getPromise("/proxies/nodes", accessHandle()),
    ]);

    if (proxiesRes.code === 0) {
      proxies = proxiesRes.data || [];
    }
    if (nodesRes.code === 0) {
      nodes = nodesRes.data || [];
    }
  } catch {
    return;
  }

  for (const id of ids) {
    const proxy = proxies.find((p) => p.proxyId === id);
    if (!proxy) {
      removeRunningId(id);
      continue;
    }

    const node = nodes.find((n) => n.nodeId === proxy.nodeId);
    if (!node || node.isDisabled || node.status === false) continue;
    if (proxy.isDisabled || proxy.isBanned) continue;
    if (proxy.isOnline) continue;
    if (tunnelStore.isStarting(id)) continue;

    try {
      await invoke<number>("start_proxy", {
        proxyId: id,
        token: token,
      });

      tunnelStore.addStartingProxy(id);
      checkTunnelStatus(id, proxy, node);
    } catch {
      removeRunningId(id);
    }
  }
}

async function checkTunnelStatus(
  proxyId: number,
  proxy: Proxy,
  node: Node | undefined,
) {
  const tunnelStore = useTunnelStore();

  try {
    const startSuccess = await invoke<boolean>("wait_for_tunnel_start", {
      proxyId: proxyId,
    });

    tunnelStore.removeStartingProxy(proxyId);

    if (startSuccess) {
      // 构建访问地址
      const hostname = node?.hostname || "";
      const port = proxy.remotePort || "";
      const address = hostname && port ? `${hostname}:${port}` : "";

      // 启动成功，显示通知
      (window as any).$notification?.success({
        title: "隧道启动成功",
        content: `隧道 "${proxy.proxyName}" 已启动`,
        meta: address
          ? () =>
              h(
                "a",
                {
                  href: "#",
                  style:
                    "color: #18a058; text-decoration: underline; cursor: pointer;",
                  onClick: (e: Event) => {
                    e.preventDefault();
                    e.stopPropagation();
                    navigator.clipboard.writeText(address);
                    (window as any).$message?.success("连接地址已复制到剪贴板");
                  },
                },
                address,
              )
          : undefined,
        duration: 5000,
      });
    } else {
      removeRunningId(proxyId);
    }
  } catch {
    tunnelStore.removeStartingProxy(proxyId);
    removeRunningId(proxyId);
  }
}

function removeRunningId(id: number) {
  const raw = localStorage.getItem(RUNNING_TUNNELS_KEY);
  let ids: number[] = [];
  try {
    ids = JSON.parse(raw || "[]");
  } catch {
    return;
  }
  if (!Array.isArray(ids)) return;

  const next = ids.filter((tid) => tid !== id);
  localStorage.setItem(RUNNING_TUNNELS_KEY, JSON.stringify(next));
}
