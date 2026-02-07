import { ref, provide } from "vue";
import { useThemeManager } from "./useThemeManager";
import { useUpdateManager } from "./useUpdateManager";
import { useSystemChecker } from "./useSystemChecker";
import { useInputDeviceDetection } from "./useInputDeviceDetection";
import { restoreTunnels } from "./useTunnelRestore";

export function useAppInitialization() {
  const isAppReady = ref(false);
  const contextMenuRef = ref();

  const {
    theme,
    toggleTheme,
    themeOverrides,
    initializeTheme: initTheme,
    cleanup: cleanupTheme,
  } = useThemeManager();
  const {
    updateModalVisible,
    updateProgress,
    updateStatus,
    updateDownloaded,
    updateTotal,
    checkForUpdates,
    performAutoUpdate,
  } = useUpdateManager(isAppReady);
  const { checkFrpcHas } = useSystemChecker();
  const { initialize: initInputDetection, cleanup: cleanupInputDetection } =
    useInputDeviceDetection();

  provide("theme", {
    theme,
    toggleTheme,
  });

  const initializeApp = async () => {
    initTheme();
    initInputDetection();

    localStorage.setItem("frpcLogs", "");
    await checkFrpcHas();

    isAppReady.value = true;

    setTimeout(() => {
      checkForUpdates();
    }, 1000);

    // 应用启动后恢复上次未关闭的隧道
    setTimeout(() => {
      restoreTunnels();
    }, 500);

    document.addEventListener("contextmenu", (e) => {
      const target = e.target as HTMLElement;
      const selection = window.getSelection()?.toString() || "";

      // 检查是否是输入框、文本域或可编辑元素
      const isInputElement =
        target instanceof HTMLInputElement ||
        target instanceof HTMLTextAreaElement ||
        target.isContentEditable;

      // 检查是否有选中的文字
      const hasSelectedText = selection.length > 0;

      // 只有在输入框或可编辑元素上，或有选中文字时才显示自定义右键菜单
      if (isInputElement || hasSelectedText) {
        e.preventDefault();
        contextMenuRef.value?.showMenu(e.clientX, e.clientY);
      } else {
        // 非允许区域直接禁用右键
        e.preventDefault();
      }
    });
  };

  const cleanup = () => {
    cleanupTheme();
    cleanupInputDetection();
  };

  return {
    isAppReady,
    contextMenuRef,
    theme,
    toggleTheme,
    themeOverrides,
    updateModalVisible,
    updateProgress,
    updateStatus,
    updateDownloaded,
    updateTotal,
    checkForUpdates,
    performAutoUpdate,
    initializeApp,
    cleanup,
  };
}
