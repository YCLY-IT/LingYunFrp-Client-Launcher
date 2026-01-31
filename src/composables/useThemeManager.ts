import { computed, watch } from "vue";
import { darkTheme, lightTheme } from "naive-ui";
import { useThemeStore } from "../stores/theme";

export function useThemeManager() {
  const themeStore = useThemeStore();
  let animationFrameId: number | null = null;
  let isRGBRunning = false;

  const theme = computed(() =>
    themeStore.theme === "dark" ? darkTheme : lightTheme,
  );

  const toggleTheme = () => {
    themeStore.theme = themeStore.theme === "dark" ? "light" : "dark";
  };

  const themeOverrides = computed(() => {
    const commonColors = {
      primaryColor: themeStore.primaryColor,
      primaryColorHover: themeStore.primaryColor,
      primaryColorPressed: themeStore.primaryColor,
      primaryColorSuppl: themeStore.primaryColor,
    };

    const hasBackgroundImage = !!themeStore.backgroundImage;
    const bodyColor = hasBackgroundImage
      ? "transparent"
      : themeStore.theme === "light"
        ? "#f5f5f5"
        : undefined;

    const lightThemeOverrides =
      themeStore.theme === "light"
        ? {
            bodyColor: bodyColor || "#f5f5f5",
          }
        : {};

    return {
      common: {
        ...commonColors,
        ...lightThemeOverrides,
        ...(hasBackgroundImage ? { bodyColor: "transparent" } : {}),
      },
      Button: {
        textColorPrimary: "#fff",
        textColorHoverPrimary: "#fff",
        textColorPressedPrimary: "#fff",
        textColorFocusPrimary: "#fff",
        textColorDisabledPrimary: "#fff",
        colorPrimary: themeStore.primaryColor,
        colorHoverPrimary: themeStore.primaryColor,
        colorPressedPrimary: themeStore.primaryColor,
        colorFocusPrimary: themeStore.primaryColor,
        colorDisabledPrimary: themeStore.primaryColor,
      },
    };
  });

  const animatePrimaryColor = () => {
    if (isRGBRunning) return;
    isRGBRunning = true;

    let r = 255,
      g = 0,
      b = 0;
    let dr = -5,
      dg = 5,
      db = 0;

    const step = () => {
      if (!themeStore.isRGBMode) {
        isRGBRunning = false;
        if (animationFrameId) {
          cancelAnimationFrame(animationFrameId);
          animationFrameId = null;
        }
        return;
      }
      if (r <= 0 && g >= 255) {
        dr = 0;
        dg = -5;
        db = 5;
      }
      if (g <= 0 && b >= 255) {
        dr = 5;
        dg = 0;
        db = -5;
      }
      if (b <= 0 && r >= 255) {
        dr = -5;
        dg = 5;
        db = 0;
      }
      r += dr;
      g += dg;
      b += db;

      themeStore.primaryColor = `rgb(${r}, ${g}, ${b})`;
      animationFrameId = requestAnimationFrame(step);
    };

    step();
  };

  watch(
    () => themeStore.isRGBMode,
    (newVal) => {
      if (newVal) {
        animatePrimaryColor();
      } else {
        if (animationFrameId) {
          cancelAnimationFrame(animationFrameId);
          animationFrameId = null;
        }
        isRGBRunning = false;

        const defaultColor =
          localStorage.getItem("app-primary-color") || "#2080F0FF";
        themeStore.setPrimaryColor(defaultColor);
      }
    },
  );

  watch(
    () => themeStore.backgroundImage,
    () => {},
  );

  const initializeTheme = () => {
    if (themeStore.isRGBMode) {
      animatePrimaryColor();
    }

    if (themeStore.isDialogBoxHairGlass) {
      document.documentElement.style.setProperty("--modal-filter", "10px");
    } else {
      document.documentElement.style.setProperty("--modal-filter", "0px");
    }

    if (themeStore.backgroundImage) {
      const opacity = Math.max(20, themeStore.backgroundOpacity || 100);
      document.documentElement.style.setProperty(
        "--background-image",
        `url(${themeStore.backgroundImage})`,
      );
      document.documentElement.style.setProperty(
        "--background-blur",
        `${themeStore.backgroundBlur}px`,
      );
      document.documentElement.style.setProperty(
        "--background-opacity",
        `${opacity / 100}`,
      );
    } else {
      document.documentElement.style.removeProperty("--background-image");
      document.documentElement.style.removeProperty("--background-blur");
      document.documentElement.style.removeProperty("--background-opacity");
    }

    if (themeStore.colorBlindMode) {
      document.documentElement.classList.add("color-blind-mode");
      document.documentElement.style.setProperty(
        "--color-blind-filter",
        "url(#colorblind)",
      );
    }

    if (themeStore.highContrastMode) {
      document.documentElement.classList.add("high-contrast-mode");
    }

    if (themeStore.frostedGlassMode && themeStore.backgroundImage) {
      document.documentElement.classList.add("frosted-glass-mode");
      document.documentElement.style.setProperty(
        "--frosted-glass-blur",
        `${themeStore.frostedGlassIntensity}px`,
      );
      document.documentElement.style.setProperty(
        "--frosted-glass-transition",
        "all 0.3s ease",
      );
    }
  };

  const cleanup = () => {
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
  };

  return {
    theme,
    toggleTheme,
    themeOverrides,
    initializeTheme,
    cleanup,
  };
}
