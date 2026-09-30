<script setup lang="ts">
import { computed, ref, nextTick, watch } from "vue";
import { motion } from "motion-v";
import {
  NLog,
  NCard,
  NSpace,
  NSelect,
  NSwitch,
  NButton,
  useMessage,
} from "naive-ui";
import {
  logStore,
  clearLogs,
  consoleLogStore,
  clearConsoleLogs,
} from "../../utils/log.ts";
import { useSystemStore, type LogLevel } from "../../stores/system";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";

const message = useMessage();
const systemStore = useSystemStore();

const selectedCategory = ref("all");
const selectedTunnel = ref("all");
const autoScroll = ref(true);
const logInst = ref<any>(null);

const categoryOptions = [
  { label: "全部日志", value: "all" },
  { label: "系统日志", value: "system" },
  { label: "FRP 日志", value: "frp" },
  { label: "虚拟网络日志", value: "virtual_network" },
  { label: "控制台日志", value: "console" },
];

const tunnelOptions = computed(() => {
  const base = [{ label: "全部隧道", value: "all" }];
  return base.concat(
    Object.keys(logStore.tunnelLogs).map((id) => ({
      label: id.startsWith("link-") ? `快速隧道 ${id}` : `隧道 ${id}`,
      value: id,
    })),
  );
});

const logs = computed(() => {
  let src: string[] = [];

  if (selectedCategory.value === "all") {
    src = logStore.allLogs;
  } else if (selectedCategory.value === "system") {
    src = logStore.systemLogs;
  } else if (selectedCategory.value === "frp") {
    if (selectedTunnel.value === "all") {
      src = logStore.frpLogs;
    } else {
      src = logStore.tunnelLogs[selectedTunnel.value] || [];
    }
  } else if (selectedCategory.value === "virtual_network") {
    src = logStore.virtualNetworkLogs;
  } else if (selectedCategory.value === "console") {
    src = filterConsoleLogsByLevel(
      consoleLogStore.logs,
      systemStore.consoleLogLevel,
    );
  }

  return src.join("\n");
});

const LOG_LEVEL_PRIORITY: Record<string, number> = {
  DEBUG: 0,
  LOG: 1,
  INFO: 1,
  WARN: 2,
  ERROR: 3,
};

const LEVEL_PRIORITY: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

function filterConsoleLogsByLevel(logs: string[], level: LogLevel): string[] {
  const minPriority = LEVEL_PRIORITY[level];
  return logs.filter((log) => {
    const match = log.match(/\[(DEBUG|LOG|INFO|WARN|ERROR)\]/);
    if (match) {
      const logPriority = LOG_LEVEL_PRIORITY[match[1]] ?? 0;
      return logPriority >= minPriority;
    }
    return true;
  });
}

const showTunnelSelect = computed(() => selectedCategory.value === "frp");

const handleScroll = ({ scrollTop, scrollHeight, containerHeight }: any) => {
  autoScroll.value = scrollHeight - (scrollTop + containerHeight) <= 50;
};

const scrollBottom = () =>
  nextTick(() => logInst.value?.scrollTo({ position: "bottom", silent: true }));

const handleClearLogs = () => {
  if (selectedCategory.value === "console") {
    clearConsoleLogs();
  } else {
    clearLogs();
  }
};

const exportLogs = async () => {
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = logs.value;
  const plainText = tempDiv.innerText || tempDiv.textContent || "";

  if (!plainText.trim() && !logs.value.trim()) {
    message.warning("当前没有可导出的日志");
    return;
  }

  const categoryLabels: Record<string, string> = {
    all: "全部日志",
    system: "系统日志",
    frp:
      selectedTunnel.value === "all"
        ? "FRP日志"
        : `FRP日志_${selectedTunnel.value}`,
    virtual_network: "虚拟网络日志",
    console: "控制台日志",
  };

  const defaultFileName = `${categoryLabels[selectedCategory.value]}_${new Date().toISOString().split("T")[0]}`;

  try {
    const filePath = await save({
      filters: [
        {
          name: "纯文本文件",
          extensions: ["txt"],
        },
        {
          name: "HTML 文件",
          extensions: ["html"],
        },
        {
          name: "JSON 文件",
          extensions: ["json"],
        },
      ],
      defaultPath: defaultFileName,
    });

    if (filePath) {
      const ext = filePath.split(".").pop()?.toLowerCase();
      let content = "";
      let actualExt = ext;

      if (ext === "html") {
        content = `<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <title>${categoryLabels[selectedCategory.value]}</title>
  <style>
    body { background: #1a1a1a; color: #fff; font-family: 'Consolas', 'Monaco', monospace; padding: 20px; }
    .log-line { margin: 2px 0; }
  </style>
</head>
<body>
${logs.value
  .split("\n")
  .map((line) => `<div class="log-line">${line}</div>`)
  .join("\n")}
</body>
</html>`;
      } else if (ext === "json") {
        const logLines =
          selectedCategory.value === "console"
            ? consoleLogStore.logs
            : logStore.allLogs;
        content = JSON.stringify(
          {
            category: selectedCategory.value,
            exportTime: new Date().toISOString(),
            count: logLines.length,
            logs: logLines,
          },
          null,
          2,
        );
      } else {
        content = plainText;
        actualExt = "txt";
      }

      await writeTextFile(filePath, content);
      message.success(`日志导出成功 (${actualExt?.toUpperCase() || "TXT"})`);
    }
  } catch (error) {
    message.error("日志导出失败");
    console.error(error);
  }
};

watch(logs, () => autoScroll.value && scrollBottom(), { flush: "post" });
</script>

<template>
  <motion.div
    :initial="{ opacity: 0, y: 20 }"
    :animate="{ opacity: 1, y: 0 }"
    :transition="{ type: 'spring', stiffness: 220, damping: 24 }"
  >
    <n-space vertical>
    <n-card title="运行日志">
      <template #header-extra>
        <n-space>
          <n-select
            v-if="showTunnelSelect"
            v-model:value="selectedTunnel"
            :options="tunnelOptions"
            placeholder="选择隧道"
            class="w-[180px]"
          />
          <n-select
            v-model:value="selectedCategory"
            :options="categoryOptions"
            placeholder="选择日志类型"
            class="w-[150px]"
          />
          <n-switch
            class="mt-0.5"
            size="large"
            v-model:value="autoScroll"
          >
            <template #checked>自动滚动开启</template>
            <template #unchecked>自动滚动关闭</template>
          </n-switch>
          <n-button
            size="large"
            class="mt-[7px]"
            text
            type="info"
            @click="exportLogs"
            >导出日志</n-button
          >
          <n-button
            size="large"
            class="mt-[7px]"
            text
            type="primary"
            @click="handleClearLogs"
            >清除日志</n-button
          >
        </n-space>
      </template>
      <n-log
        ref="logInst"
        :log="logs"
        :rows="25"
        language="naive-log"
        trim
        @scroll="handleScroll"
      />
    </n-card>
    </n-space>
  </motion.div>
</template>
