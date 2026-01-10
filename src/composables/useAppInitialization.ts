import { ref, provide } from "vue";
import { useThemeManager } from "./useThemeManager";
import { useUpdateManager } from "./useUpdateManager";
import { useSystemChecker } from "./useSystemChecker";
import { useInputDeviceDetection } from "./useInputDeviceDetection";

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
    }, 2000);

    document.addEventListener("contextmenu", (e) => {
      e.preventDefault();
      contextMenuRef.value?.showMenu(e.clientX, e.clientY);
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
