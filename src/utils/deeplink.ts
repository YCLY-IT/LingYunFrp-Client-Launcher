import { onMounted, onUnmounted } from "vue";
import { storeToken } from "../net/base";
import { listen } from "@tauri-apps/api/event";
import { useDialog } from "naive-ui";
import router from "../router";

const dialog = useDialog();

onMounted(async () => {
  const unlisten = await listen<string[]>("deep-link", (e) => {
    const raw = e.payload[0];
    const url = new URL(raw);

    /* ========== 登录深链 ========== */
    if (url.pathname === "/login-callback" && url.searchParams.has("token")) {
      const token = url.searchParams.get("token")!.replace("Bearer ", "");
      storeToken(token, true, new Date(Date.now() + 1000 * 60 * 60 * 24 * 7));

      dialog.success({
        title: "登录成功",
        content: "即将跳转",
        positiveText: "确定",
        onPositiveClick: () => router.push("/dashboard"),
      });
      return;
    }

    onUnmounted(() => unlisten()); // 组件卸载时取消监听
  });
});
