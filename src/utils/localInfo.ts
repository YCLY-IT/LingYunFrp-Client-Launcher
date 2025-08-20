import { invoke } from "@tauri-apps/api/core";

let versionPromise: Promise<string> | null = null;
let systemInfoPromise: Promise<string> | null = null;

export function loadAppVersion(): Promise<string> {
  if (!versionPromise) {
    versionPromise = invoke<string>("get_client_version").catch(
      () => "unknown",
    );
  }
  return versionPromise;
}

/**
 * @returns 系统信息[0]
 * @returns 系统架构[1]
 */
export function loadAppSystemInfo(): Promise<string> {
  if (!systemInfoPromise) {
    systemInfoPromise = invoke<string>("get_system_info").catch(
      () => "unknown",
    );
  }
  return systemInfoPromise;
}
