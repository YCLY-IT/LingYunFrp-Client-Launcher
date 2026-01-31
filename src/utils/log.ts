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
export const initLogService = async () => {
  if (inited) return;
  inited = true;

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
      `<span style="${PALETTE.time}">[${t}]</span> <span style="${PALETTE.tunnel}">[隧道 ${tunnelId}]</span> ${html}`,
      "frp",
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
