import { DialogApi, MessageApi, NotificationApi } from "naive-ui";

declare global {
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
