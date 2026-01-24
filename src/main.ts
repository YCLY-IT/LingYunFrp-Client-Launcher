import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";

import { invoke } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
async function getIsDebug() {
  return await invoke<boolean>("get_now_mode");
}
const isDebug = await getIsDebug();
if (!isDebug) {
  document.addEventListener("keydown", (e) => {
    if (
      (e.ctrlKey && e.shiftKey && e.key === "I") || // Ctrl+Shift+I
      (e.ctrlKey && e.shiftKey && e.key === "J") || // Ctrl+Shift+J
      (e.ctrlKey && e.key === "U") || // Ctrl+U
      e.key === "F12"
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

await onOpenUrl((event) => {
  console.log(event);
});

/* 哥哥太厉害了>w<, 以后每天都要跟咱问声好哦, 嘿嘿（★＞U＜★） 2025/09/06 16:15:06 */
