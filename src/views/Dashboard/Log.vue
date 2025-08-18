<script setup lang="ts">
import { computed, ref, nextTick, watch } from "vue";
import { NLog, NCard, NSpace, NSelect, NSwitch, NButton } from "naive-ui";
import { logStore, clearLogs } from "../../utils/log.ts";

const selectedTunnel = ref("all");
const autoScroll = ref(true);
const logInst = ref<any>(null);

const tunnelOptions = computed(() => {
  const base = [{ label: "全部日志", value: "all" }];
  return base.concat(
    Object.keys(logStore.tunnelLogs).map((id) => ({
      label: id.startsWith("link-") ? `快速隧道 ${id}` : `隧道 ${id}`,
      value: id,
    })),
  );
});

const logs = computed(() => {
  const src =
    selectedTunnel.value === "all"
      ? logStore.allLogs
      : logStore.tunnelLogs[selectedTunnel.value] || [];
  return src.join("\n");
});

const handleScroll = ({ scrollTop, scrollHeight, containerHeight }: any) => {
  autoScroll.value = scrollHeight - (scrollTop + containerHeight) <= 50;
};

const scrollBottom = () =>
  nextTick(() => logInst.value?.scrollTo({ position: "bottom", silent: true }));

watch(logs, () => autoScroll.value && scrollBottom(), { flush: "post" });
</script>

<template>
  <n-space vertical>
    <n-card title="运行日志">
      <template #header-extra>
        <n-space>
          <n-select
            v-model:value="selectedTunnel"
            :options="tunnelOptions"
            placeholder="选择隧道"
            style="width: 180px"
          />
          <n-switch v-model:value="autoScroll">
            <template #checked>自动滚动开启</template>
            <template #unchecked>自动滚动关闭</template>
          </n-switch>
          <n-button text type="primary" @click="clearLogs">清除日志</n-button>
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
