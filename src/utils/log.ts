import { listen } from "@tauri-apps/api/event";
import { reactive } from "vue";
import ansiToHtml from "ansi-to-html";

const convert = new ansiToHtml({
  fg: "#FFF",
  bg: "#000",
  newline: true,
  escapeXML: false,
  stream: false,
  colors: {
    "1": "#FB9FB1",
    "2": "#ACC267",
    "3": "#DDB26F",
    "4": "#6FC2EF",
    "5": "#E1A3EE",
    "6": "#12CFC0",
    "7": "#D0D0D0",
    "9": "#F6363F",
  },
});

export const logStore = reactive<{
  allLogs: string[];
  tunnelLogs: Record<string, string[]>;
}>({
  allLogs: [],
  tunnelLogs: {},
});

const MAX_LOG_COUNT = 10000;

/** 解析并归类日志 */
const addLog = (raw: string) => {
  // 简单去重：完全相同的行直接跳过
  if (logStore.allLogs[logStore.allLogs.length - 1] === raw) return;

  logStore.allLogs.push(raw);
  if (logStore.allLogs.length > MAX_LOG_COUNT) {
    logStore.allLogs.splice(0, logStore.allLogs.length - MAX_LOG_COUNT);
  }

  // 提取隧道 ID
  const m = raw.match(/隧道\s+(\d+|link-[A-Za-z0-9\-_]+)/);
  const tid = m?.[1];
  if (tid) {
    if (!logStore.tunnelLogs[tid]) logStore.tunnelLogs[tid] = [];
    logStore.tunnelLogs[tid].push(raw);
    if (logStore.tunnelLogs[tid].length > MAX_LOG_COUNT) {
      logStore.tunnelLogs[tid].splice(
        0,
        logStore.tunnelLogs[tid].length - MAX_LOG_COUNT,
      );
    }
  }

  // 持久化
  localStorage.setItem("frpcLogs", logStore.allLogs.join("\n"));
};

export const clearLogs = () => {
  logStore.allLogs = [];
  logStore.tunnelLogs = {};
  localStorage.removeItem("frpcLogs");
};

/** 全局初始化：只运行一次 */
let inited = false;
export const initLogService = async () => {
  if (inited) return;
  inited = true;

  // 恢复历史
  const saved = localStorage.getItem("frpcLogs");
  if (saved) {
    saved.split("\n").forEach(addLog);
  }

  // 系统日志
  await listen("log", (e: any) =>
    addLog(`[${new Date().toLocaleTimeString()}] [系统] ${e.payload.message}`),
  );

  // 隧道日志
  await listen("tunnel-event", (e: any) => {
    const { tunnelId, message } = e.payload;
    const colored = convert.toHtml(message);
    addLog(
      `[${new Date().toLocaleTimeString()}] [隧道 ${tunnelId}] ${colored}`,
    );
  });
};
