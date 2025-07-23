import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  base: "./",
  build: {
    target: "es2022",
    rollupOptions: {
      output: {
        manualChunks(id: any) {
          if (id.includes("node_modules")) {
            if (id.includes("naive-ui")) return;
            if (id.includes("vue-router")) return "vue-router";
            if (id.includes("pinia")) return "pinia";
            if (id.includes("@vueuse")) return "vueuse";
            if (id.includes("axios")) return "axios";
            if (id.includes("highlight.js")) return "highlightjs";
            if (id.includes("dayjs")) return "dayjs";
            if (id.includes("numbro")) return "numbro";
            if (id.includes("@tauri-apps")) return "tauri";
            if (id.includes("/node_modules/vue")) return "vue-core";
            return "vendor";
          }
        },
      },
    },
  },
  plugins: [
    vue(),
    AutoImport({
      imports: [
        "vue",
        {
          "naive-ui": [
            "useDialog",
            "useMessage",
            "useNotification",
            "useLoadingBar",
          ],
        },
      ],
    }),
    Components({
      resolvers: [NaiveUiResolver()],
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: "0.0.0.0",
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
});
