import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";

export type LogLevel = "debug" | "info" | "warn" | "error";

interface SystemState {
  autoStart: boolean;
  autoRestoreTunnels: boolean;
  saveToTray: boolean;
  skipSystemProxy: boolean;
  consoleLogLevel: LogLevel;
}

const BOOT_SETTINGS_KEY = "boot_settings";

export const useSystemStore = defineStore("system", {
  state: (): SystemState => {
    const stored = localStorage.getItem(BOOT_SETTINGS_KEY);
    if (stored) {
      try {
        const settings = JSON.parse(stored);
        return {
          autoStart: Boolean(settings.autoStart),
          autoRestoreTunnels: Boolean(settings.autoRestoreTunnels),
          saveToTray: Boolean(settings.saveToTray),
          skipSystemProxy: Boolean(settings.skipSystemProxy),
          consoleLogLevel: settings.consoleLogLevel || "info",
        };
      } catch {
        // 解析失败，使用默认值
      }
    }
    return {
      autoStart: false,
      autoRestoreTunnels: true,
      saveToTray: false,
      skipSystemProxy: true,
      consoleLogLevel: "info",
    };
  },
  actions: {
    async loadAutoStartStatus() {
      try {
        this.autoStart = await invoke("check_auto_start_status");
        this.saveSettings();
      } catch (e) {
        console.error("加载自启动状态失败:", e);
      }
    },
    async toggleAutoStart() {
      try {
        const newStatus = !this.autoStart;
        await invoke("toggle_auto_start", { enable: newStatus });
        this.autoStart = newStatus;
        this.saveSettings();
        this.notifyAutoStartChange();
        this.notifySettingsChange();
      } catch (e) {
        console.error("切换自启失败:", e);
      }
    },
    setAutoRestoreTunnels(value: boolean) {
      this.autoRestoreTunnels = value;
      this.saveSettings();
      this.notifySettingsChange();
    },
    setSaveToTray(value: boolean) {
      this.saveToTray = value;
      this.saveSettings();
      this.notifySettingsChange();
    },
    setSkipSystemProxy(value: boolean) {
      this.skipSystemProxy = value;
      this.saveSettings();
      this.notifySettingsChange();
    },
    setConsoleLogLevel(value: LogLevel) {
      this.consoleLogLevel = value;
      this.saveSettings();
      this.notifySettingsChange();
    },
    saveSettings() {
      localStorage.setItem(
        BOOT_SETTINGS_KEY,
        JSON.stringify({
          autoStart: this.autoStart,
          autoRestoreTunnels: this.autoRestoreTunnels,
          saveToTray: this.saveToTray,
          skipSystemProxy: this.skipSystemProxy,
          consoleLogLevel: this.consoleLogLevel,
        }),
      );
    },
    notifyAutoStartChange() {
      emit("system-auto-start-changed", { autoStart: this.autoStart });
    },
    notifySettingsChange() {
      emit("system-settings-changed", {
        autoStart: this.autoStart,
        autoRestoreTunnels: this.autoRestoreTunnels,
        saveToTray: this.saveToTray,
        skipSystemProxy: this.skipSystemProxy,
        consoleLogLevel: this.consoleLogLevel,
      });
    },
  },
});
