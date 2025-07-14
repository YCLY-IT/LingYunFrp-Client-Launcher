import { Window } from "../types";
import { invoke } from "@tauri-apps/api/core";
async function getClientVersion() {
  return await invoke<string>("get_client_version");
}

class DialogDeduplicator {
  private static instance: DialogDeduplicator;
  private lastShowTime: number = 0;
  private readonly DEBOUNCE_TIME = 4000;

  static getInstance(): DialogDeduplicator {
    if (!DialogDeduplicator.instance) {
      DialogDeduplicator.instance = new DialogDeduplicator();
    }
    return DialogDeduplicator.instance;
  }

  showDialog(options: any): void {
    const now = Date.now();
    if (now - this.lastShowTime < this.DEBOUNCE_TIME) {
      return; // 4秒内只弹一次
    }
    this.lastShowTime = now;
    window.$dialog?.error(options);
  }
}

const dialogDeduplicator = DialogDeduplicator.getInstance();
// 消息去重机制
class MessageDeduplicator {
  private static instance: MessageDeduplicator;
  private lastShowTime: number = 0;
  private readonly DEBOUNCE_TIME = 4000;

  static getInstance(): MessageDeduplicator {
    if (!MessageDeduplicator.instance) {
      MessageDeduplicator.instance = new MessageDeduplicator();
    }
    return MessageDeduplicator.instance;
  }

  showMessage(
    message: string,
    type: "error" | "warning" | "success" | "info" = "error",
  ): void {
    const now = Date.now();
    if (now - this.lastShowTime < this.DEBOUNCE_TIME) {
      return; // 4秒内只显示一条消息
    }
    this.lastShowTime = now;

    switch (type) {
      case "error":
        messageDeduplicator.showMessage(message, "error");
        break;
      case "warning":
        messageDeduplicator.showMessage(message, "warning");
        break;
      case "success":
        messageDeduplicator.showMessage(message, "success");
        break;
      case "info":
        messageDeduplicator.showMessage(message, "info");
        break;
    }
  }
}

const messageDeduplicator = MessageDeduplicator.getInstance();

const defaultFailure = (messageText: string) => {
  //! TODO: only console warning, don't show message here
  messageDeduplicator.showMessage(messageText, "warning");
  window.$loadingBar?.error();
};

const defaultError = (err: any) => {
  //! TODO: only console error, don't show message here
  console.error(err);
  if (err.response) {
    if (err.response.data.code === 2) {
      messageDeduplicator.showMessage("登录信息已过期，请重新登录", "error");
    }
  }
  messageDeduplicator.showMessage("请求失败，网络可能存在问题", "error");
  window.$loadingBar?.error();
};

//! TODO: Specifies the params and return value type
function storeToken(Authorization: any, remember: boolean, expires: any) {
  const token = {
    Authorization: Authorization,
    remember: remember,
    expires: expires,
  };
  const tokenStr = JSON.stringify(token);
  if (remember) {
    localStorage.setItem("Authorization", tokenStr);
  } else {
    sessionStorage.setItem("Authorization", tokenStr);
  }
}

//! TODO: Specifies the return value type
function getToken() {
  const tokenStr =
    localStorage.getItem("Authorization") ||
    sessionStorage.getItem("Authorization");
  if (tokenStr) {
    const token = JSON.parse(tokenStr);
    if (token.expires && token.expires < new Date().getTime()) {
      removeToken();
      //! TODO: only return error, don't show message here
      messageDeduplicator.showMessage("登录信息已过期，请重新登录", "error");
      return null;
    }
    return token.Authorization;
  }
  return null;
}

function removeToken() {
  localStorage.removeItem("Authorization");
  sessionStorage.removeItem("Authorization");
}

//! TODO: why the return value has two type(string or Object)?
declare const window: Window;

function accessHandle() {
  return {
    Authorization: `Bearer ${getToken()}`,
  };
}

//! TODO: use promise instead of callback
async function post(
  url: string,
  data: any,
  headers: Record<string, string | number>,
  success: Function,
  failure = defaultFailure,
  error = defaultError,
) {
  window.$loadingBar?.start();
  const postHeaders = {
    ...headers,
    ClientVersion: await getClientVersion(),
    Client: "LingYunFRPClient",
  };
  // 通过 Tauri 后端转发请求
  invoke("forward_request", {
    url: url,
    method: "POST",
    data: data,
    headers: postHeaders,
  })
    .then((data: any) => {
      if (data.code === 0) {
        success(data);
        window.$loadingBar?.finish();
      } else if (data.code === 2) {
        dialogDeduplicator.showDialog({
          title: "提示",
          content: "登录信息已过期，请重新登录",
          positiveText: "确定",
          negativeText: "取消",
          onPositiveClick: () => {
            removeToken();
            window.location.href = "/login";
          },
        });

        failure(data.message);
        window.$loadingBar?.error();
      } else if (data.code === 1) {
        failure(data.message);
        window.$loadingBar?.error();
      }
    })
    .catch((err) => {
      error(err);
    });
}

//! TODO: use promise instead of callback
async function get(
  url: string,
  headers: Record<string, string>,
  success: Function,
  failure = defaultFailure,
  error = defaultError,
) {
  window.$loadingBar?.start();
  const getHeaders = {
    ...headers,
    ClientVersion: await getClientVersion(),
    Client: "LingYunFRPClient",
  };
  invoke("forward_request", {
    url: url,
    method: "GET",
    data: {},
    headers: getHeaders,
  })
    .then((data: any) => {
      // 检查是否是完整的URL（外部API）
      if (url.startsWith("http://") || url.startsWith("https://")) {
        window.$loadingBar?.finish();
        success(data);
      } else {
        if (data.code === 0) {
          window.$loadingBar?.finish();
          success(data);
        } else if (data.code === 2) {
          dialogDeduplicator.showDialog({
            title: "提示",
            content: "登录信息已过期，请重新登录",
            positiveText: "确定",
            negativeText: "取消",
            onPositiveClick: () => {
              removeToken();
              window.location.href = "/login";
            },
          });
          failure(data.message);
          window.$loadingBar?.error();
        } else if (data.code === 1) {
          window.$loadingBar?.error();
          failure(data.message);
        }
      }
    })
    .catch((err) => {
      error(err);
    });
}

function unauthorized() {
  return !getToken();
}

async function OpenBrowser(url: string) {
  // 外部浏览器打开
  await invoke("open_url", { url: url })
    .then(() => {
      console.log("打开浏览器成功");
    })
    .catch((err) => {
      console.error("打开浏览器失败:", err);
    });
}

export {
  defaultFailure,
  defaultError,
  storeToken,
  getToken,
  accessHandle,
  removeToken,
  post,
  get,
  unauthorized,
  OpenBrowser,
};
