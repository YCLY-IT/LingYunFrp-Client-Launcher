// log.ts
import { listen } from "@tauri-apps/api/event";
import { reactive } from "vue";
import AnsiToHtml from "ansi-to-html";

const PALETTE = {
  error: "color:#f87171",
  warning: "color:#fb923c",
  info: "color:#60a5fa",
  debug: "color:#9ca3af",
  success: "color:#4ade80",
  time: "color:#9ca3af",
  system: "color:#60a5fa",
  tunnel: "color:#a855f7",
  console: "color:#a78bfa",
};

const ansi = new AnsiToHtml({
  fg: "#fff",
  bg: "#000",
  newline: true,
  escapeXML: false,
  stream: false,
  colors: {
    1: "#FB9FB1",
    2: "#ACC267",
    3: "#DDB26F",
    4: "#6FC2EF",
    5: "#E1A3EE",
    6: "#12CFC0",
    7: "#D0D0D0",
    9: "#F6363F",
  },
});

export const logStore = reactive<{
  allLogs: string[];
  tunnelLogs: Record<string, string[]>;
  systemLogs: string[];
  frpLogs: string[];
  virtualNetworkLogs: string[];
}>({
  allLogs: [],
  tunnelLogs: {},
  systemLogs: [],
  frpLogs: [],
  virtualNetworkLogs: [],
});
const MAX_LOG_COUNT = 10000;

function addLog(
  raw: string,
  category: "system" | "frp" | "virtual_network" = "system",
) {
  if (logStore.allLogs.at(-1) === raw) return;
  logStore.allLogs.push(raw);
  if (logStore.allLogs.length > MAX_LOG_COUNT) logStore.allLogs.splice(0, 1);

  switch (category) {
    case "system":
      logStore.systemLogs.push(raw);
      if (logStore.systemLogs.length > MAX_LOG_COUNT)
        logStore.systemLogs.splice(0, 1);
      break;
    case "frp":
      logStore.frpLogs.push(raw);
      if (logStore.frpLogs.length > MAX_LOG_COUNT)
        logStore.frpLogs.splice(0, 1);
      break;
    case "virtual_network":
      logStore.virtualNetworkLogs.push(raw);
      if (logStore.virtualNetworkLogs.length > MAX_LOG_COUNT)
        logStore.virtualNetworkLogs.splice(0, 1);
      break;
  }

  const m = raw.match(/隧道\s+(\d+|link-[A-Za-z0-9\-_]+)/);
  const tid = m?.[1];
  if (tid) {
    (logStore.tunnelLogs[tid] ||= []).push(raw);
    if (logStore.tunnelLogs[tid].length > MAX_LOG_COUNT)
      logStore.tunnelLogs[tid].splice(0, 1);
  }
  localStorage.setItem("frpcLogs", logStore.allLogs.join("\n"));
}

export const clearLogs = () => {
  logStore.allLogs = [];
  logStore.tunnelLogs = {};
  logStore.systemLogs = [];
  logStore.frpLogs = [];
  logStore.virtualNetworkLogs = [];
  localStorage.removeItem("frpcLogs");
};

let inited = false;

// 控制台日志存储
export const consoleLogStore = reactive<{
  logs: string[];
}>({
  logs: [],
});

const MAX_CONSOLE_LOG_COUNT = 5000;

const CONSOLE_LEVEL_STYLES: Record<string, string> = {
  debug: "color:#9ca3af",
  log: "color:#e5e5e5",
  info: "color:#60a5fa",
  warn: "color:#fb923c",
  error: "color:#f87171",
};

const IGNORED_CONSOLE_PATTERNS = [
  /\[naive\/code\]: hljs is not set/,
  /<Suspense> is an experimental feature/,
];

function getCallerInfo(): string {
  const stack = new Error().stack || "";
  const lines = stack.split("\n");
  for (const line of lines) {
    if (line.includes("log.ts") || line.includes("console.")) continue;
    const match = line.match(/(?:at\s+)?(?:.*?\s+\()?(.+?):(\d+):(\d+)\)?/);
    if (match) {
      const [, path, lineNum] = match;
      const fileName = path.split("/").pop()?.split("\\").pop() || path;
      return `${fileName}:${lineNum}`;
    }
  }
  return "";
}

// 添加控制台日志
function addConsoleLog(level: string, args: any[]) {
  const t = new Date().toLocaleString();
  const messages = args
    .map((arg) => {
      if (typeof arg === "object") {
        try {
          return JSON.stringify(arg);
        } catch {
          return String(arg);
        }
      }
      return String(arg);
    })
    .join(" ");

  // 过滤忽略的日志
  if (IGNORED_CONSOLE_PATTERNS.some((pattern) => pattern.test(messages))) {
    return;
  }

  const callerInfo = getCallerInfo();
  const location = callerInfo
    ? `<span style="color:#a855f7">[${callerInfo}]</span> `
    : "";

  const levelStyle = CONSOLE_LEVEL_STYLES[level] || CONSOLE_LEVEL_STYLES.log;
  const html = ansi.toHtml(messages);
  const logEntry = `<span style="${PALETTE.time}">[${t}]</span> <span style="${PALETTE.console}">[控制台]</span> <span style="${levelStyle}">[${level.toUpperCase()}]</span> ${location}<span style="${levelStyle}">${html}</span>`;

  consoleLogStore.logs.push(logEntry);
  if (consoleLogStore.logs.length > MAX_CONSOLE_LOG_COUNT) {
    consoleLogStore.logs.splice(0, 1);
  }

  // 保存到 localStorage
  localStorage.setItem("consoleLogs", consoleLogStore.logs.join("\n"));
}

// 初始化控制台日志监听
function initConsoleLogCapture() {
  const originalLog = console.log;
  const originalError = console.error;
  const originalWarn = console.warn;
  const originalInfo = console.info;
  const originalDebug = console.debug;

  console.log = function (...args: any[]) {
    addConsoleLog("log", args);
    originalLog.apply(console, args);
  };

  console.error = function (...args: any[]) {
    addConsoleLog("error", args);
    originalError.apply(console, args);
  };

  console.warn = function (...args: any[]) {
    addConsoleLog("warn", args);
    originalWarn.apply(console, args);
  };

  console.info = function (...args: any[]) {
    addConsoleLog("info", args);
    originalInfo.apply(console, args);
  };

  console.debug = function (...args: any[]) {
    addConsoleLog("debug", args);
    originalDebug.apply(console, args);
  };

  // 监听未捕获的错误
  window.addEventListener("error", (event) => {
    addConsoleLog("error", [
      `[未捕获错误] ${event.message} at ${event.filename}:${event.lineno}:${event.colno}`,
    ]);
  });

  // 监听未处理的 Promise 拒绝
  window.addEventListener("unhandledrejection", (event) => {
    const reason = event.reason;
    if (reason instanceof Error) {
      addConsoleLog("error", [
        `[未处理的 Promise 拒绝] ${reason.message}\n${reason.stack}`,
      ]);
    } else {
      addConsoleLog("error", [`[未处理的 Promise 拒绝] ${String(reason)}`]);
    }
  });
}

// 清空控制台日志
export const clearConsoleLogs = () => {
  consoleLogStore.logs = [];
  localStorage.removeItem("consoleLogs");
};

// 导出控制台日志为文件
export const exportConsoleLogs = (): string => {
  return consoleLogStore.logs.join("\n");
};

export const initLogService = async () => {
  if (inited) return;
  inited = true;

  // 清空旧的控制台日志，不恢复
  localStorage.removeItem("consoleLogs");

  // 初始化控制台日志捕获
  initConsoleLogCapture();

  const saved = localStorage.getItem("frpcLogs");
  if (saved) saved.split("\n").forEach((log) => addLog(log, "system"));

  await listen("log", (e: any) => {
    const { level, message } = e.payload;
    const t = new Date().toLocaleString();
    const html = ansi.toHtml(message);
    const style = PALETTE[level as keyof typeof PALETTE] ?? "";
    addLog(
      `<span style="${PALETTE.time}">[${t}]</span> <span style="${PALETTE.system}">[系统]</span> <span style="${style}">${html}</span>`,
      "system",
    );
  });

  await listen("tunnel-event", (e: any) => {
    const { tunnelId, message } = e.payload;
    const t = new Date().toLocaleString();
    const html = ansi.toHtml(message);
    addLog(
      `<span style="${PALETTE.time}">[${t}]</span> <span style="${PALETTE.tunnel}">[隧道 ${tunnelId}]</span> ${html}>`,
      "frp",
    );
  });

  // 监听 EasyTier 日志事件
  await listen("easytier-log", (e: any) => {
    const { level, message } = e.payload;
    const t = new Date().toLocaleString();
    const html = ansi.toHtml(message);
    const levelStyle = level === "error" ? PALETTE.error : PALETTE.info;
    addLog(
      `<span style="${PALETTE.time}">[${t}]</span> <span style="color:#a855f7">[虚拟网络]</span> <span style="${levelStyle}">${html}</span>`,
      "virtual_network",
    );
  });

  // 监听 EasyTier 停止事件
  await listen("easytier-stopped", (e: any) => {
    const { network_id } = e.payload;
    const t = new Date().toLocaleString();
    addLog(
      `<span style="${PALETTE.time}">[${t}]</span> <span style="color:#a855f7">[虚拟网络]</span> <span style="${PALETTE.warning}">EasyTier 进程已停止 (Network: ${network_id})</span>`,
      "virtual_network",
    );
  });
};

export const addVirtualNetworkLog = (
  message: string,
  level: "info" | "success" | "warning" | "error" = "info",
) => {
  const t = new Date().toLocaleString();
  const style = PALETTE[level] ?? "";
  const html = ansi.toHtml(message);
  addLog(
    `<span style="${PALETTE.time}">[${t}]</span> <span style="color:#a855f7">[虚拟网络]</span> <span style="${style}">${html}</span>`,
    "virtual_network",
  );
};
