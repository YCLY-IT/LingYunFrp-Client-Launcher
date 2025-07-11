<template>
  <div
    v-if="visible"
    :style="menuStyle"
    class="custom-context-menu"
    @contextmenu.prevent
  >
    <ul>
      <li
        v-for="(item, index) in menu"
        :key="index"
        @mouseenter="showSubMenu(index)"
        @mouseleave="hideSubMenu(index)"
        :class="{ disabled: item.disabled }"
      >
        <span
          @click="!item.disabled && handleClick(item)"
          :style="item.disabled ? 'color:#aaa;cursor:not-allowed;' : ''"
          >{{ item.label }}</span
        >
        <span v-if="item.children" class="arrow">▶</span>
        <div
          v-if="item.children && subMenuIndex === index"
          class="submenu"
          :style="subMenuStyle"
        >
          <ul>
            <li
              v-for="(sub, subIndex) in item.children"
              :key="subIndex"
              @click="handleClick(sub)"
            >
              {{ sub.label }}
            </li>
          </ul>
        </div>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import {
  ref,
  reactive,
  onMounted,
  onBeforeUnmount,
  computed,
  inject,
} from "vue";

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
const subMenuStyle = reactive({ left: "100%", top: "0px" });
const subMenuIndex = ref<number | null>(null);
const themeContext = inject("theme") as { isDarkMode: { value: boolean } };
const theme = computed(() =>
  themeContext?.isDarkMode.value ? "dark" : "light",
);

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

<style lang="scss">
.custom-context-menu {
  position: fixed;
  z-index: 9999;
  background: v-bind('theme === "dark" ? "#232323" : "#fff"');
  border: 1px solid v-bind('theme === "dark" ? "#444" : "#e0e0e0"');
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.18);
  min-width: 140px;
  user-select: none;
  border-radius: 12px;
  padding: 4px 0;
  transition:
    box-shadow 0.2s,
    background 0.2s;
}
.custom-context-menu ul {
  list-style: none;
  margin: 0;
  padding: 0;
}
.custom-context-menu li {
  padding: 8px 24px 8px 18px;
  cursor: pointer;
  white-space: nowrap;
  position: relative;
  border-radius: 8px;
  margin: 2px 8px;
  transition: background 0.15s;
  color: v-bind('theme === "dark" ? "#eee" : "#222"');
}
.custom-context-menu li:hover {
  background: v-bind('theme === "dark" ? "#333" : "#f5f5f5"');
}
.arrow {
  float: right;
}
.submenu {
  position: absolute;
  top: 0;
  left: 100%;
  min-width: 120px;
  background: v-bind('theme === "dark" ? "#232323" : "#fff"');
  border: 1px solid v-bind('theme === "dark" ? "#444" : "#e0e0e0"');
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.18);
  border-radius: 10px;
  padding: 4px 0;
}
.custom-context-menu li.disabled > span {
  color: v-bind('theme === "dark" ? "#666" : "#aaa"');
  cursor: not-allowed;
}
</style>
