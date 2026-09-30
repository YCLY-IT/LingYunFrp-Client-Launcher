<template>
  <motion.div
    class="welcome-card flex w-full items-center p-0"
    :initial="{ opacity: 0, y: 28, scale: 0.97 }"
    :animate="{ opacity: 1, y: 0, scale: 1 }"
    :transition="{ type: 'spring', stiffness: 200, damping: 24 }"
  >
    <div
      class="card-container relative m-0 min-h-[250px] w-full min-w-0 max-w-full overflow-hidden rounded-2xl bg-white shadow-[0_4px_16px_rgba(0,0,0,0.08)] transition-shadow duration-300 hover:shadow-[0_10px_30px_rgba(0,0,0,0.16)] max-[600px]:min-h-[180px] max-[600px]:rounded-[10px]"
      ref="cardRef"
    >
      <!-- 背景图片 -->
      <motion.img
        class="bg-img absolute inset-0 z-1 h-full w-full object-cover"
        src="/images/bg.png"
        alt="背景"
        :initial="{ scale: 1.12 }"
        :animate="{ scale: 1 }"
        :transition="{ duration: 1.2, ease: 'easeOut' }"
      />

      <!-- 内容层 -->
      <div
        class="content-layer relative z-2 flex h-full w-full flex-col justify-between p-[18px] pt-6 max-[600px]:gap-2 max-[600px]:px-3 max-[600px]:pb-2 max-[600px]:pt-2.5"
      >
        <!-- 顶部：欢迎语和天气 -->
        <div
          class="row top-row flex w-full items-start justify-between max-[350px]:flex-col max-[350px]:items-start max-[350px]:justify-start max-[350px]:gap-1 max-[600px]:gap-2"
        >
          <motion.div
            class="welcome-title text-xl font-bold tracking-[1px] text-[#00334e] [font-family:msyh,sans-serif] max-[600px]:mb-0.5 max-[600px]:text-[15px]"
            :initial="{ opacity: 0, x: -20 }"
            :animate="{ opacity: 1, x: 0 }"
            :transition="{ delay: 0.25, duration: 0.5, ease: [0.4, 0, 0.2, 1] }"
          >
            欢迎来到
            <span
              class="brand font-bold text-[#1976d2] max-[600px]:text-[15px]"
              >LingYunFrp</span
            >
          </motion.div>
          <div
            class="weather-box flex items-start gap-2.5 max-[600px]:mt-1 max-[600px]:gap-1.5"
          >
            <motion.img
              :src="weatherIconSrc"
              :alt="weatherInfo.weather"
              class="weather-icon mt-0.5 h-[38px] w-[38px] max-[600px]:mt-0 max-[600px]:h-7 max-[600px]:w-7"
              :animate="{ y: [0, -5, 0], rotate: [0, 6, 0] }"
              :transition="{
                duration: 4,
                repeat: Infinity,
                ease: 'easeInOut',
              }"
            />
            <div
              class="weather-info flex flex-col gap-0.5 text-[13px] text-[#00334e] max-[600px]:text-[11px]"
            >
              <div>{{ weatherInfo.weather }}</div>
              <div
                class="weather-detail flex gap-2.5 text-[11px] text-[#00334e] max-[600px]:gap-1.5 max-[600px]:text-[10px]"
              >
                <span>温度: {{ weatherInfo.temp }}℃</span>
                <span>湿度: {{ weatherInfo.humidity }}%RH</span>
              </div>
              <div
                class="weather-detail flex gap-2.5 text-[11px] text-[#00334e] max-[600px]:gap-1.5 max-[600px]:text-[10px]"
              >
                <span>风向: {{ weatherInfo.winddirection }}方</span>
                <span>风力: {{ weatherInfo.windpower }}级</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 中部：自定义文字 -->
        <div
          class="row custom-row mt-2.5 flex w-full justify-center max-[600px]:mt-1"
          v-if="customText"
        >
          <span
            class="custom-text text-lg font-bold tracking-[1px] text-[#1976d2] [font-family:msyh,sans-serif] max-[600px]:text-[13px]"
            >{{ customText }}</span
          >
        </div>

        <!-- 底部：访问信息 -->
        <div
          class="row bottom-row mt-[18px] flex w-full items-end justify-between max-[350px]:flex-col max-[350px]:items-start max-[350px]:justify-start max-[350px]:gap-1 max-[600px]:items-start max-[600px]:gap-2"
        >
          <div
            class="info-list flex flex-col gap-1.5 max-[600px]:gap-[3px]"
          >
            <div
              class="info-item flex animate-rise-in items-center gap-1.5 text-xs text-[#00334e] [animation-delay:300ms] [font-family:msyh,sans-serif] max-[600px]:gap-1 max-[600px]:text-[10px]"
            >
              <img src="/icon/ico/IP.png" class="info-icon inline-block h-4 w-4 align-middle max-[600px]:h-[13px] max-[600px]:w-[13px]" />
              <span>{{ visitorInfo.ip }}</span>
            </div>
            <div
              class="info-item flex animate-rise-in items-center gap-1.5 text-xs text-[#00334e] [animation-delay:360ms] [font-family:msyh,sans-serif] max-[600px]:gap-1 max-[600px]:text-[10px]"
            >
              <img src="/icon/ico/system.png" class="info-icon inline-block h-4 w-4 align-middle max-[600px]:h-[13px] max-[600px]:w-[13px]" />
              <span>{{ visitorInfo.os }}</span>
            </div>
            <div
              class="info-item flex animate-rise-in items-center gap-1.5 text-xs text-[#00334e] [animation-delay:420ms] [font-family:msyh,sans-serif] max-[600px]:gap-1 max-[600px]:text-[10px]"
            >
              <img src="/icon/ico/bro.png" class="info-icon inline-block h-4 w-4 align-middle max-[600px]:h-[13px] max-[600px]:w-[13px]" />
              <span>{{ visitorInfo.browser }}</span>
            </div>
            <div
              class="info-item flex animate-rise-in items-center gap-1.5 text-xs text-[#00334e] [animation-delay:480ms] [font-family:msyh,sans-serif] max-[600px]:gap-1 max-[600px]:text-[10px]"
            >
              <img src="/icon/ico/local.png" class="info-icon inline-block h-4 w-4 align-middle max-[600px]:h-[13px] max-[600px]:w-[13px]" />
              <span>{{ location }}</span>
            </div>
          </div>
          <div
            class="date-list flex flex-col gap-1.5 max-[600px]:gap-[3px]"
          >
            <div
              class="info-item flex animate-rise-in items-center gap-1.5 text-xs text-[#00334e] [animation-delay:540ms] [font-family:msyh,sans-serif] max-[600px]:gap-1 max-[600px]:text-[10px]"
            >
              <img src="/icon/ico/time.png" class="info-icon inline-block h-4 w-4 align-middle max-[600px]:h-[13px] max-[600px]:w-[13px]" />
              <span>{{ currentDate }}</span>
            </div>
            <div
              class="info-item flex animate-rise-in items-center gap-1.5 text-xs text-[#00334e] [animation-delay:600ms] [font-family:msyh,sans-serif] max-[600px]:gap-1 max-[600px]:text-[10px]"
            >
              <span>更新时间: {{ weatherInfo.reporttime }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </motion.div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { motion } from "motion-v";
import { useMessage } from "naive-ui";
import { weatherService } from "../net/user/weatherService";

const message = useMessage();

// 响应式数据
const cardRef = ref<HTMLElement>();
const customText = ref("");

// 访问者信息
const visitorInfo = ref({
  ip: "",
  os: "",
  browser: "",
  province: "",
  city: "",
});

// 天气信息
const weatherInfo = ref({
  weather: "未知",
  temp: "--",
  humidity: "--",
  winddirection: "--",
  windpower: "--",
  reporttime: "--",
});

// 计算属性
const currentDate = computed(() => {
  const now = new Date();
  const weekArray = ["日", "一", "二", "三", "四", "五", "六"];
  const year = now.getFullYear();
  const month = now.getMonth() + 1;
  const date = now.getDate();
  const week = weekArray[now.getDay()];
  return `${year}年${month}月${date}日 星期${week}`;
});

const location = computed(() => {
  return `${visitorInfo.value.province}-${visitorInfo.value.city}`;
});

const weatherIconSrc = computed(() => {
  const weather = weatherInfo.value.weather;
  console.log("当前天气描述:", weather); // 调试信息

  // 更完善的天气图标匹配逻辑
  if (
    weather.includes("晴") ||
    weather.includes("Sunny") ||
    weather.includes("Clear")
  ) {
    return "/icon/weather/sunny.png";
  }
  if (
    weather.includes("云") ||
    weather.includes("cloudy") ||
    weather.includes("Partly")
  ) {
    return "/icon/weather/dyun.png";
  }
  if (weather.includes("阴") || weather.includes("Overcast")) {
    return "/icon/weather/yin.png";
  }
  if (
    weather.includes("雾") ||
    weather.includes("Mist") ||
    weather.includes("Fog") ||
    weather.includes("Haze")
  ) {
    return "/icon/weather/wu.png";
  }
  if (
    weather.includes("雨") ||
    weather.includes("rain") ||
    weather.includes("Shower") ||
    weather.includes("Drizzle") ||
    weather.includes("Thunderstorm")
  ) {
    return "/icon/weather/rain.png";
  }
  if (weather.includes("雪") || weather.includes("snow")) {
    return "/icon/weather/snow.png";
  }
  if (
    weather.includes("风") ||
    weather.includes("wind") ||
    weather.includes("breeze") ||
    weather.includes("Gale") ||
    weather.includes("Storm") ||
    weather.includes("Typhoon")
  ) {
    return "/icon/weather/sha.png";
  }
  if (
    weather.includes("沙") ||
    weather.includes("Dust") ||
    weather.includes("Sand")
  ) {
    return "/icon/weather/sha.png";
  }

  // 如果没有匹配到任何天气类型，返回未知图标
  console.warn("未匹配到天气图标，使用默认图标:", weather);
  return "/icon/weather/unknow.png";
});

// 获取浏览器信息
const getBrowser = (): string => {
  const ua = navigator.userAgent;

  if (ua.includes("Chrome")) {
    const match = ua.match(/Chrome\/([0-9.]+)/);
    return `Chrome(${match?.[1] || "unknown"})`;
  }
  if (ua.includes("Firefox")) {
    const match = ua.match(/Firefox\/([0-9.]+)/);
    return `Firefox(${match?.[1] || "unknown"})`;
  }
  if (ua.includes("Safari") && !ua.includes("Chrome")) {
    const match = ua.match(/Version\/([0-9.]+)/);
    return `Safari(${match?.[1] || "unknown"})`;
  }
  if (ua.includes("Edge")) {
    const match = ua.match(/Edge\/([0-9.]+)/);
    return `Edge(${match?.[1] || "unknown"})`;
  }
  if (ua.includes("MSIE") || ua.includes("Trident")) {
    const match = ua.match(/MSIE ([0-9.]+)/) || ua.match(/rv:([0-9.]+)/);
    return `Internet Explorer(${match?.[1] || "unknown"})`;
  }

  return "未知浏览器";
};

// 获取操作系统信息
const getOS = (): string => {
  const ua = navigator.userAgent;

  if (ua.includes("Windows")) {
    if (ua.includes("Windows NT 10.0")) return "Windows 10";
    if (ua.includes("Windows NT 6.3")) return "Windows 8.1";
    if (ua.includes("Windows NT 6.2")) return "Windows 8";
    if (ua.includes("Windows NT 6.1")) return "Windows 7";
    if (ua.includes("Windows NT 6.0")) return "Windows Vista";
    if (ua.includes("Windows NT 5.1")) return "Windows XP";
    return "Windows";
  }
  if (ua.includes("Mac OS X")) {
    const match = ua.match(/Mac OS X ([0-9._]+)/);
    return `Mac OS X(${match?.[1]?.replace(/_/g, ".") || "unknown"})`;
  }
  if (ua.includes("Linux")) {
    if (ua.includes("Android")) {
      const match = ua.match(/Android ([0-9.]+)/);
      return `Android(${match?.[1] || "unknown"})`;
    }
    return "Linux";
  }
  if (ua.includes("iPhone")) {
    const match = ua.match(/OS ([0-9._]+)/);
    return `iOS(${match?.[1]?.replace(/_/g, ".") || "unknown"})`;
  }
  if (ua.includes("iPad")) {
    const match = ua.match(/OS ([0-9._]+)/);
    return `iPadOS(${match?.[1]?.replace(/_/g, ".") || "unknown"})`;
  }

  return "未知系统";
};

// 获取访问者信息
const getVisitorInfo = async () => {
  try {
    const { location, weather } = await weatherService.getVisitorInfo();

    visitorInfo.value = {
      ip: location.ip,
      os: visitorInfo.value.os, // 保持原有的系统信息
      browser: visitorInfo.value.browser, // 保持原有的浏览器信息
      province: location.province,
      city: location.city,
    };

    weatherInfo.value = weather;
  } catch (error) {
    console.error("获取访问者信息失败:", error);
    message.error("获取访问者信息失败");
  }
};

// 初始化访问者信息
const initVisitorInfo = () => {
  visitorInfo.value.browser = getBrowser();
  visitorInfo.value.os = getOS();
};

// 组件挂载时初始化
onMounted(() => {
  initVisitorInfo();
  getVisitorInfo();
});
</script>
