import { invoke } from "@tauri-apps/api/core";

export function useSystemChecker() {
  const checkFrpcHas = async () => {
    try {
      const hasFrpc = await invoke<boolean>("check_frpc_exists");
      if (!hasFrpc) {
        (window as any).$notification?.error({
          title: "frpc.exe不存在",
          content: "请到系统设置下载frpc.exe",
          duration: 0,
        });
        setTimeout(async () => {
          await invoke("emit_event", {
            event: "log",
            payload: {
              level: "warning",
              message: `frpc.exe不存在，请到系统设置下载frpc.exe`,
            },
          });
        }, 500);
      }
    } catch (error) {
      console.error("检查frpc.exe失败:", error);
    }
  };

  return {
    checkFrpcHas,
  };
}
