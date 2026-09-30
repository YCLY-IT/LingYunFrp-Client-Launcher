<template>
  <div class="statistic">
    <n-grid
      class="mt-[15px]"
      cols="1 s:2 m:4"
      responsive="screen"
      :x-gap="15"
      :y-gap="20"
    >
      <n-gi v-for="(card, index) in cards" :key="index">
        <motion.div
          class="h-full"
          :initial="{ opacity: 0, y: 28, scale: 0.96 }"
          :animate="{ opacity: 1, y: 0, scale: 1 }"
          :transition="{
            delay: index * 0.09,
            type: 'spring',
            stiffness: 220,
            damping: 22,
          }"
          :while-hover="{ y: -6, scale: 1.02 }"
        >
          <n-card
            :title="card.title"
            size="small"
            class="h-full transition-shadow duration-300 hover:shadow-lg"
          >
            <n-flex justify="space-between">
              <span class="inline-flex text-primary">
                <n-icon class="mt-[5px]" size="32">
                  <component :is="card.icon" />
                </n-icon>
              </span>
              <n-statistic tabular-nums>
                <template #default>
                  <n-number-animation
                    :from="0"
                    :to="card.value"
                    :precision="card.precision"
                    show-separator
                  />
                </template>
                <template v-if="card.unit" #suffix>
                  {{ card.unit }}
                </template>
              </n-statistic>
            </n-flex>
          </n-card>
        </motion.div>
      </n-gi>
    </n-grid>
  </div>
</template>

<script setup lang="ts">
import { motion } from "motion-v";
import { userApi } from "../net";
import { accessHandle } from "../net/base";
import { TrafficType } from "../types";
import {
  ArrowDownCircleOutline,
  BarChartOutline,
  CalendarOutline,
  GlobeOutline,
} from "@vicons/ionicons5";
import { useMessage } from "naive-ui";
import { computed, markRaw, onMounted, ref } from "vue";

const traffic = ref<TrafficType>({} as TrafficType);

const message = useMessage();
// 格式化流量
const formatTraffic = (traffic: number) => {
  if (isNaN(traffic)) return { value: 0, unit: "MB" };
  if (traffic >= 1024) {
    return { value: +(traffic / 1024).toFixed(2), unit: "GB" };
  }
  return { value: +traffic.toFixed(2), unit: "MB" };
};

// 添加 props 定义
const props = defineProps<{
  signRemainder?: number;
}>();

// 卡片数据

const cards = computed(() => [
  {
    title: "总流量（非剩余）",
    icon: markRaw(GlobeOutline),
    ...formatTraffic(traffic.value.allTraffic || 0),
    precision: 2,
  },
  {
    title: "总使用",
    icon: markRaw(ArrowDownCircleOutline),
    ...formatTraffic(traffic.value.allUsedTraffic || 0),
    precision: 2,
  },
  {
    title: "今日使用",
    icon: markRaw(BarChartOutline),
    ...formatTraffic(traffic.value.todayUsedTraffic || 0),
    precision: 2,
  },
  {
    title: "签到次数",
    icon: markRaw(CalendarOutline),
    value: props.signRemainder ? props.signRemainder : 0,
    unit: "次",
    precision: 0,
  },
]);

// 获取用户流量
const getUserTraffic = async (): Promise<void> => {
  userApi.get(
    "/user/traffic",
    accessHandle(),
    (data) => {
      traffic.value = data.data;
    },
    (messageText) => {
      message.error(messageText);
    },
  );
};

defineExpose({
  getUserTraffic,
});

onMounted(() => {
  getUserTraffic();
});
</script>
