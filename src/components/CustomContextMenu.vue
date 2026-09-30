<template>
  <AnimatePresence>
    <motion.div
      v-if="visible"
      :style="menuStyle"
      :initial="{ opacity: 0, scale: 0.9, y: -8 }"
      :animate="{ opacity: 1, scale: 1, y: 0 }"
      :exit="{ opacity: 0, scale: 0.95 }"
      :transition="{ duration: 0.16, ease: 'easeOut' }"
      class="custom-context-menu fixed z-9999 min-w-[140px] origin-top-left select-none rounded-xl border py-1 shadow-[0_4px_16px_rgba(0,0,0,0.18)] transition-[box-shadow,background] duration-200"
      :class="isDark ? 'border-[#444] bg-[#232323]' : 'border-[#e0e0e0] bg-white'"
      @contextmenu.prevent
    >
      <ul class="m-0 list-none p-0">
        <li
          v-for="(item, index) in menu"
          :key="index"
          class="relative mx-2 my-0.5 cursor-pointer whitespace-nowrap rounded-lg py-2 pl-[18px] pr-6 transition-colors duration-150"
          :class="[
            isDark
              ? 'text-[#eee] hover:bg-[#333]'
              : 'text-[#222] hover:bg-[#f5f5f5]',
            item.disabled
              ? isDark
                ? 'cursor-not-allowed text-[#666]'
                : 'cursor-not-allowed text-[#aaa]'
              : '',
          ]"
          @mouseenter="showSubMenu(index)"
          @mouseleave="hideSubMenu(index)"
        >
          <span @click="!item.disabled && handleClick(item)">{{
            item.label
          }}</span>
          <span v-if="item.children" class="float-right">▶</span>
          <AnimatePresence>
            <motion.div
              v-if="item.children && subMenuIndex === index"
              :initial="{ opacity: 0, x: -8 }"
              :animate="{ opacity: 1, x: 0 }"
              :exit="{ opacity: 0, x: -8 }"
              :transition="{ duration: 0.15, ease: 'easeOut' }"
              class="absolute left-full top-0 min-w-[120px] rounded-[10px] border py-1 shadow-[0_4px_16px_rgba(0,0,0,0.18)]"
              :class="
                isDark ? 'border-[#444] bg-[#232323]' : 'border-[#e0e0e0] bg-white'
              "
            >
              <ul class="m-0 list-none p-0">
                <li
                  v-for="(sub, subIndex) in item.children"
                  :key="subIndex"
                  class="mx-2 my-0.5 cursor-pointer whitespace-nowrap rounded-lg py-2 pl-[18px] pr-6 transition-colors duration-150"
                  :class="
                    isDark
                      ? 'text-[#eee] hover:bg-[#333]'
                      : 'text-[#222] hover:bg-[#f5f5f5]'
                  "
                  @click="handleClick(sub)"
                >
                  {{ sub.label }}
                </li>
              </ul>
            </motion.div>
          </AnimatePresence>
        </li>
      </ul>
    </motion.div>
  </AnimatePresence>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onBeforeUnmount, computed } from "vue";
import { AnimatePresence, motion } from "motion-v";
import { useThemeStore } from "../stores/theme";

interface MenuItem {
  label: string;
  onClick?: () => void;
  children?: MenuItem[];
  disabled?: boolean;
}

const props = defineProps<{
  menu?: MenuItem[];
}>();

const clipboardHasData = ref(true);
const isInputActive = ref(true);

async function checkClipboard() {
  if (navigator.clipboard) {
    try {
      const text = await navigator.clipboard.readText();
      clipboardHasData.value = !!text;
    } catch {
      clipboardHasData.value = false;
    }
  } else {
    clipboardHasData.value = true; // 兼容性处理，无法检测时默认可用
  }
}

function checkInputActive() {
  const active = document.activeElement as HTMLElement | null;
  isInputActive.value =
    !!active &&
    (active instanceof HTMLInputElement ||
      active instanceof HTMLTextAreaElement ||
      active.isContentEditable);
}

const defaultMenu: MenuItem[] = [
  {
    label: "全选",
    onClick: () => {
      document.execCommand("selectAll");
    },
    get disabled() {
      return !isInputActive.value;
    },
  },
  {
    label: "复制",
    onClick: () => {
      document.execCommand("copy");
    },
  },
  {
    label: "粘贴",
    onClick: async () => {
      if (clipboardHasData.value) {
        if (
          navigator.clipboard &&
          window.document.activeElement instanceof HTMLElement
        ) {
          try {
            const text = await navigator.clipboard.readText();
            const active = window.document.activeElement as HTMLElement;
            if (
              active instanceof HTMLInputElement ||
              active instanceof HTMLTextAreaElement
            ) {
              const start = active.selectionStart || 0;
              const end = active.selectionEnd || 0;
              const value = active.value;
              active.value = value.slice(0, start) + text + value.slice(end);
              active.selectionStart = active.selectionEnd = start + text.length;
            } else if (active.isContentEditable) {
              document.execCommand("insertText", false, text);
            }
          } catch (e) {
            // 粘贴失败
          }
        } else {
          document.execCommand("paste");
        }
      }
    },
    get disabled() {
      return !clipboardHasData.value;
    },
  },
  {
    label: "剪切",
    onClick: () => {
      document.execCommand("cut");
    },
  },
];

const menu = computed(() => props.menu ?? defaultMenu);

const visible = ref(false);
const menuStyle = reactive({ left: "0px", top: "0px" });
const subMenuIndex = ref<number | null>(null);
const themeStore = useThemeStore();
const theme = computed(() => (themeStore.theme === "dark" ? "dark" : "light"));
const isDark = computed(() => theme.value === "dark");

function showMenu(x: number, y: number) {
  menuStyle.left = x + "px";
  menuStyle.top = y + "px";
  visible.value = true;
  checkClipboard();
  checkInputActive();
}
function hideMenu() {
  visible.value = false;
  subMenuIndex.value = null;
}
function showSubMenu(index: number) {
  subMenuIndex.value = index;
}
function hideSubMenu(index: number) {
  if (subMenuIndex.value === index) subMenuIndex.value = null;
}
function handleClick(item: MenuItem) {
  if ((item as any).disabled) return;
  if (item.onClick) item.onClick();
  hideMenu();
}

onMounted(() => {
  document.addEventListener("click", hideMenu);
});
onBeforeUnmount(() => {
  document.removeEventListener("click", hideMenu);
});

// 暴露方法给父组件
defineExpose({ showMenu, hideMenu });
</script>
