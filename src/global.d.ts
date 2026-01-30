import { DialogApi, MessageApi, NotificationApi } from "naive-ui";

declare global {
  const __BUILD_TIME__: string;
  const __BUILD_FINGERPRINT__: string;

  export interface Window {
    $loadingBar?: {
      start: () => void;
      finish: () => void;
      error: () => void;
    };
    $message?: MessageApi;
    $dialog?: DialogApi;
    $notification?: NotificationApi;
  }
}
