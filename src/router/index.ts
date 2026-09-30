import { createRouter, createWebHashHistory } from "vue-router";
import { unauthorized } from "../net/base.js";

// 声明window类型
declare const window: Window;

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      redirect: "/login",
    },
    {
      path: "/login",
      name: "login",
      component: () => import("../views/Login.vue"),
      meta: {
        title: "登录",
      },
    },
    {
      path: "/dashboard",
      name: "dashboard",
      component: () => import("../views/Dashboard.vue"),
      redirect: "/dashboard/home",
      meta: {
        requiresAuth: true,
      },
      children: [
        {
          path: "home",
          name: "dashboardIndex",
          component: () => import("../views/Dashboard/IndexView.vue"),
          meta: {
            title: "首页",
          },
        },
        {
          path: "proxy/create",
          name: "create-tunnel",
          component: () =>
            import("../views/Dashboard/proxies/CreateTunnel.vue"),
          meta: {
            title: "创建隧道",
          },
        },
        {
          path: "proxy/list",
          name: "proxy-list",
          component: () =>
            import("../views/Dashboard/proxies/ManagerTunnel.vue"),
          meta: {
            title: "隧道列表",
          },
        },
        {
          path: "user/my-profile",
          name: "user-profile",
          component: () => import("../views/Dashboard/Profile.vue"),
          meta: {
            title: "用户信息",
          },
        },
        {
          path: "logs",
          name: "logs",
          component: () => import("../views/Dashboard/Log.vue"),
          meta: {
            title: "日志",
          },
        },
        {
          path: "settings",
          name: "settings",
          component: () => import("../views/Dashboard/Settings.vue"),
          meta: {
            title: "设置",
          },
        },
        {
          path: "network",
          name: "network",
          component: () => import("../views/Dashboard/Network.vue"),
          meta: {
            title: "连接虚拟网络",
          },
        },
      ],
    },
    {
      path: "/tray-menu",
      name: "tray-menu",
      component: () => import("../views/TrayMenu.vue"),
      meta: {
        title: "托盘菜单",
      },
    },
    {
      path: "/:pathMatch(.*)*",
      name: "NotFound",
      component: () => import("../views/NotFound.vue"),
      meta: {
        title: "404",
      },
    },
  ],
});

router.beforeEach((to) => {
  //需要登录的路由校验
  if (to.matched.some((record) => record.meta.requiresAuth)) {
    if (unauthorized()) {
      return {
        name: "login",
        query: { redirect: to.fullPath }, // 携带跳转路径参数
      };
    }
    return true;
  }
  // 已登录用户禁止访问登录/注册页
  if (to.name === "login" && !unauthorized()) {
    return { name: "dashboard" };
  }
  // 其他情况直接放行
  if (to.matched.length === 0) {
    return "/dashboard";
  }
  return true;
});

export default router;
