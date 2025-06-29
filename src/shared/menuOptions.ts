import { h, Component, ref } from 'vue'
import { HomeOutline, AddCircleOutline, AppsOutline, IdCardOutline, SettingsOutline, ListOutline, WifiOutline } from '@vicons/ionicons5'
import { NIcon, type MenuOption } from 'naive-ui'
import { SquareTerminal } from 'lucide-vue-next';

const baseMenuOptions: MenuOption[] = [
  {
    label: '面板首页',
    icon: renderIcon(HomeOutline),
    key: 'dashboardIndex',
    link: '/dashboard/home',
  },
  {
    label: '隧道管理',
    icon: renderIcon(AppsOutline),
    key: 'tunnel-section',
    children: [
        {
          label: '创建隧道',
          icon: renderIcon(AddCircleOutline),
          key: 'create-tunnel',
          link: '/dashboard/proxy/create',
        },
        {
          label: '隧道列表',
          icon: renderIcon(ListOutline),
          key: 'proxy-list',
          link: '/dashboard/proxy/list',
        }
    ]
  },
  {
    label: '虚拟网络',
    icon: renderIcon(WifiOutline),
    disabled: true, // TODO: 暂时禁用
    key: 'network',
    link: '/dashboard/network',
  },
  {
    label: '用户中心',
    icon: renderIcon(IdCardOutline),
    key: 'user-profile',
    link: '/dashboard/user/my-profile',
  },
  {
    label: '日志',
    icon: renderIcon(SquareTerminal),
    key: 'logs',
    link: '/dashboard/logs',
  },
  {
    label: '系统设置',
    icon: renderIcon(SettingsOutline),
    key:'settings',
    link: '/dashboard/settings',
  }
]
export function getMenuOptions(): MenuOption[] {
  const options = [...baseMenuOptions]
  return options
}

export function renderIcon(icon: Component) {
  return () => h(NIcon, {
    component: icon,
    size: 22
  })
}

export const defaultExpandedKeys = ref<string[]>(['more'])