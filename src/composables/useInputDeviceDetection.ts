import { ref, provide } from "vue";

export function useInputDeviceDetection() {
  const isTouchDevice = ref(false);

  provide("isTouchDevice", isTouchDevice);

  const detectInputMethod = (event: PointerEvent) => {
    if (event.pointerType === "touch") {
      isTouchDevice.value = true;
    } else if (event.pointerType === "mouse") {
      isTouchDevice.value = false;
    }
  };

  const initialize = () => {
    window.addEventListener("pointerdown", detectInputMethod);
  };

  const cleanup = () => {
    window.removeEventListener("pointerdown", detectInputMethod);
  };

  return {
    isTouchDevice,
    initialize,
    cleanup,
  };
}
