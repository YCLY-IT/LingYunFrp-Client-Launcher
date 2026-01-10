import { ref, h, type Ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { loadAppVersion, loadAppSystemInfo } from "../utils/localInfo";
import { userApi } from "../net";
import { accessHandle } from "../net/base";

export function useUpdateManager(isAppReady: Ref<boolean>) {
  const updateCheckInProgress = ref(false);
  const lastUpdateCheck = ref<number>(0);
  const UPDATE_CHECK_INTERVAL = 3600000;
  const hasShownUpdateNotification = ref(false);
  const MAX_RETRY_COUNT = 50;
  const RETRY_INTERVAL = 200;

  const updateModalVisible = ref(false);
  const updateProgress = ref(0);
  const updateStatus = ref("");
  const updateDownloaded = ref(0);
  const updateTotal = ref(0);

  function compareVersion(a: string, b: string): number {
    const aParts = a.split(".").map(Number);
    const bParts = b.split(".").map(Number);
    const len = Math.max(aParts.length, bParts.length);
    for (let i = 0; i < len; i++) {
      const aNum = aParts[i] || 0;
      const bNum = bParts[i] || 0;
      if (aNum > bNum) return 1;
      if (aNum < bNum) return -1;
    }
    return 0;
  }

  const checkForUpdates = async () => {
    if (updateCheckInProgress.value) return;

    const now = Date.now();
    if (now - lastUpdateCheck.value < UPDATE_CHECK_INTERVAL) return;

    try {
      updateCheckInProgress.value = true;
      const clientVersion = await loadAppVersion();
      const systemInfo = await loadAppSystemInfo();
      let system = systemInfo.split(" ")[0];
      let arch = systemInfo.split(" ")[1];
      console.log(
        `客户端版本: ${clientVersion}, 系统: ${system}, 架构: ${arch}`,
      );
      userApi.get(
        `/frp/updates/latest?software=LingYunFrpClient&system=${system}&arch=${arch}&version=${clientVersion}`,
        accessHandle(),
        (data: any) => {
          if (
            compareVersion(data.data.latest_info.version, clientVersion) ===
              1 &&
            !hasShownUpdateNotification.value
          ) {
            let retryCount = 0;

            if (data.data.latest_info.force_update) {
              const downloadUrl = data.data.latest_info.download_url;
              const fileName = downloadUrl.split("/").pop() || "update.exe";

              (window as any).$dialog.warning({
                title: "需要更新",
                content: `当前版本 ${clientVersion} 已停止支持，请立即更新到最新版本 ${data.data.latest_info.version}`,
                positiveText: "立即更新",
                negativeText: "退出应用",
                onPositiveClick: async () => {
                  await performAutoUpdate(downloadUrl, fileName);
                },
                onNegativeClick: () => {
                  invoke("quit_window", { isKeep: false });
                },
                closable: false,
                closeOnEsc: false,
                maskClosable: false,
              });
              return;
            }

            if (localStorage.getItem("suppressUpdateNotification") === "true") {
              return;
            }

            const showNotification = () => {
              if (isAppReady.value && (window as any).$notification) {
                (window as any).$notification.info({
                  title: `新版本 ${data.data.latest_info.version} 可用`,
                  content: data.data.latest_info.release_notes,
                  duration: 0,
                  action: () =>
                    h(
                      "button",
                      {
                        style: `
                        margin-left: 16px;
                        color: #409eff;
                        background: none;
                        border: none;
                        cursor: pointer;
                        font-size: 14px;
                        padding: 0;
                        text-decoration: underline transparent;
                        transition: text-decoration-color 0.2s;
                      `,
                        onmouseenter: (e: MouseEvent) => {
                          (e.target as HTMLElement).style.textDecorationColor =
                            "#409eff";
                        },
                        onmouseleave: (e: MouseEvent) => {
                          (e.target as HTMLElement).style.textDecorationColor =
                            "transparent";
                        },
                        onclick: () => {
                          localStorage.setItem(
                            "suppressUpdateNotification",
                            "true",
                          );
                          (window as any).$notification.destroyAll();
                        },
                      },
                      "以后不再提示",
                    ),
                });
                hasShownUpdateNotification.value = true;
              } else if (retryCount < MAX_RETRY_COUNT) {
                retryCount++;
                setTimeout(showNotification, RETRY_INTERVAL);
              } else {
                console.error("显示更新通知失败：组件未就绪");
              }
            };

            setTimeout(showNotification, 1000);
          }

          setTimeout(async () => {
            await invoke("emit_event", {
              event: "log",
              payload: {
                type: "info",
                message: `新版本 ${data.data.latest_info.version} 可用`,
              },
            });
          }, 50);

          lastUpdateCheck.value = now;
        },
        (message: string) => {
          console.error(message);
        },
      );
    } catch (error) {
      console.error("检查更新失败:", error);
    } finally {
      updateCheckInProgress.value = false;
    }
  };

  const performAutoUpdate = async (downloadUrl: string, fileName: string) => {
    try {
      updateModalVisible.value = true;
      updateProgress.value = 0;
      updateStatus.value = "正在准备下载更新...";
      updateDownloaded.value = 0;
      updateTotal.value = 0;

      const unlisten = await listen("update-download-start", () => {
        updateStatus.value = "开始下载更新...";
      });

      const progressUnlisten = await listen(
        "update-download-progress",
        (data: any) => {
          updateProgress.value = data.payload.percentage || 0;
          updateDownloaded.value = data.payload.downloaded;
          updateTotal.value = data.payload.total;
          const downloaded = (data.payload.downloaded / 1024 / 1024).toFixed(2);
          const total = data.payload.total
            ? (data.payload.total / 1024 / 1024).toFixed(2)
            : "未知";
          updateStatus.value = `下载中... ${data.payload.percentage}% (${downloaded}MB / ${total}MB)`;
        },
      );

      const completeUnlisten = await listen(
        "update-download-complete",
        async (data: any) => {
          updateStatus.value = "下载完成，正在安装...";

          unlisten();
          progressUnlisten();
          completeUnlisten();

          const installUnlisten = await listen("update-install-start", () => {
            updateStatus.value = "正在安装更新，应用将自动重启...";
          });

          setTimeout(async () => {
            try {
              await invoke("install_and_restart", {
                installerPath: data.payload.file_path,
              });
              installUnlisten();
            } catch (error) {
              console.error("安装失败:", error);
              updateStatus.value = "安装失败";
              window.$notification?.error({
                title: "安装失败",
                content: "自动安装失败，请手动下载安装包进行更新",
                duration: 0,
              });
            }
          }, 1000);
        },
      );

      await invoke<string>("auto_update", {
        downloadUrl,
        fileName,
      });
    } catch (error) {
      console.error("自动更新失败:", error);
      updateStatus.value = "更新失败";
      window.$notification?.error({
        title: "更新失败",
        content: "自动更新失败，请手动下载安装包进行更新",
        duration: 0,
      });
    }
  };

  return {
    updateModalVisible,
    updateProgress,
    updateStatus,
    updateDownloaded,
    updateTotal,
    checkForUpdates,
    performAutoUpdate,
  };
}
