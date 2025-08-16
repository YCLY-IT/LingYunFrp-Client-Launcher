import { userApi } from "../net";
import { accessHandle } from "../net/base";

export const checkUpdate = (
  software: string,
  system: string,
  arch: string,
  version: string,
  // 注意这个版本在后端并没有实际使用所以说可以随便传但为什么要弄,我也不清楚代码是以前写的
  currentVersion: string,
): Promise<{
  success: boolean;
  message: string;
  url: string;
  version: string;
}> => {
  return new Promise((resolve, _reject) => {
    userApi.get(
      `/frp/updates/latest?software=${software}&system=${system}&arch=${arch}&version=${version}`,
      accessHandle(),
      (data: any) => {
        const latestVersion = data.data.latest_info.version;

        if (!/^\d+\.\d+\.\d+/.test(currentVersion)) {
          resolve({
            success: false,
            message: "当前版本号异常，无法比较",
            url: "",
            version: "",
          });
          return;
        }

        if (compareVersion(latestVersion, currentVersion) !== 0) {
          resolve({
            success: true,
            message: data.data.latest_info.release_notes,
            url: data.data.latest_info.download_url,
            version: latestVersion,
          });
        } else {
          resolve({
            success: false,
            message: "当前已是最新版本",
            url: "",
            version: "",
          });
        }
      },
      () =>
        resolve({
          success: false,
          message: "当前没有新版本",
          url: "",
          version: "",
        }),
      (msg: string) =>
        resolve({ success: false, message: msg, url: "", version: "" }),
    );
  });
};

// 版本号比较函数，返回1表示a>b，0表示相等，-1表示a<b
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
