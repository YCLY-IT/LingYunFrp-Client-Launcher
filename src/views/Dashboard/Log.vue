<script setup lang="ts">
import { computed, ref, nextTick, watch } from "vue";
import {
  NLog,
  NCard,
  NSpace,
  NSelect,
  NSwitch,
  NButton,
  useMessage,
} from "naive-ui";
import { logStore, clearLogs } from "../../utils/log.ts";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";

const message = useMessage();

const selectedCategory = ref("all");
const selectedTunnel = ref("all");
const autoScroll = ref(true);
const logInst = ref<any>(null);

const categoryOptions = [
  { label: "全部日志", value: "all" },
  { label: "系统日志", value: "system" },
  { label: "FRP 日志", value: "frp" },
  { label: "虚拟网络日志", value: "virtual_network" },
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
  }

  return src.join("\n");
});

const showTunnelSelect = computed(() => selectedCategory.value === "frp");

const handleScroll = ({ scrollTop, scrollHeight, containerHeight }: any) => {
  autoScroll.value = scrollHeight - (scrollTop + containerHeight) <= 50;
};

const scrollBottom = () =>
  nextTick(() => logInst.value?.scrollTo({ position: "bottom", silent: true }));

const exportLogs = async () => {
  const tempDiv = document.createElement("div");
  tempDiv.innerHTML = logs.value;
  const plainText = tempDiv.innerText || tempDiv.textContent || "";

  if (!plainText.trim()) {
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
  };

  const defaultFileName = `${categoryLabels[selectedCategory.value]}_${new Date().toISOString().split("T")[0]}.txt`;

  try {
    const filePath = await save({
      filters: [
        {
          name: "文本文件",
          extensions: ["txt"],
        },
      ],
      defaultPath: defaultFileName,
    });

    if (filePath) {
      await writeTextFile(filePath, plainText);
      message.success("日志导出成功");
    }
  } catch (error) {
    message.error("日志导出失败");
    console.error(error);
  }
};

watch(logs, () => autoScroll.value && scrollBottom(), { flush: "post" });
</script>

<template>
  <n-space vertical>
    <n-card title="运行日志">
      <template #header-extra>
        <n-space>
          <n-select
            v-if="showTunnelSelect"
            v-model:value="selectedTunnel"
            :options="tunnelOptions"
            placeholder="选择隧道"
            style="width: 180px"
          />
          <n-select
            v-model:value="selectedCategory"
            :options="categoryOptions"
            placeholder="选择日志类型"
            style="width: 150px"
          />
          <n-switch
            style="margin-top: 2px"
            size="large"
            v-model:value="autoScroll"
          >
            <template #checked>自动滚动开启</template>
            <template #unchecked>自动滚动关闭</template>
          </n-switch>
          <n-button
            size="large"
            style="margin-top: 7px"
            text
            type="info"
            @click="exportLogs"
            >导出日志</n-button
          >
          <n-button
            size="large"
            style="margin-top: 7px"
            text
            type="primary"
            @click="clearLogs"
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
</template>
