import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
import "./assets/styles/tailwind.css";

import { invoke } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";

async function getIsDebug() {
  try {
    return await invoke<boolean>("get_now_mode");
  } catch {
    // 非 Tauri 环境（例如直接用浏览器打开 dev server）无法调用 IPC。
    // 这里必须兜底，否则顶层 await 会抛错，导致下面的 app.mount 永远不会执行，
    // 页面会一直停留在 index.html 的初始加载界面。
    return true;
  }
}
const isDebug = await getIsDebug();
if (!isDebug) {
  document.addEventListener("keydown", (e) => {
    if (
      (e.ctrlKey && e.shiftKey && e.key === "I") || // Ctrl+Shift+I
      (e.ctrlKey && e.shiftKey && e.key === "J") || // Ctrl+Shift+J
      (e.ctrlKey && e.key === "U") || // Ctrl+U
      e.key === "F12" ||
      e.key === "F5"
    ) {
      e.preventDefault();
    }
  });
}

const app = createApp(App);

app.use(createPinia());
app.use(router);

// 等待路由准备就绪后再挂载应用
router.isReady().then(() => {
  app.mount("#app");

  // 立即隐藏初始加载界面
  const initialLoading = document.getElementById("initial-loading");
  if (initialLoading) {
    initialLoading.classList.add("hidden");
    setTimeout(() => {
      initialLoading.remove();
    }, 300);
  }
});

try {
  await onOpenUrl((event) => {
    console.log(event);
  });
} catch (err) {
  // 同样是非 Tauri 环境下的兜底，避免顶层 await 抛错影响应用挂载
  console.warn("deep-link 插件不可用（非 Tauri 环境）:", err);
}
