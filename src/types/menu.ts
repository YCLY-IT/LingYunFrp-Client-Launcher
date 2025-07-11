import type { MenuOption as NaiveMenuOption } from "naive-ui";

// 扩展 MenuOption 类型以支持自定义点击处理
export interface MenuOption extends Omit<NaiveMenuOption, "onClick"> {
  onClick?: (() => Promise<boolean | void>) | string;
  link?: string;
}
