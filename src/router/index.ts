import {createRouter, createWebHistory} from 'vue-router'
import { unauthorized} from "../net/base.js";
import { Window } from '../types'

// 声明window类型
declare const window: Window

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: '/login',
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/Login.vue'),
      meta: {
        title: '登录',
      }
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: () => import('../views/Dashboard.vue'),
      redirect: '/dashboard/home',
      meta: {
        requiresAuth: true,
      },
      children: [
        {
          path: 'home',
          name: 'dashboardIndex',
          component: () => import('../views/Dashboard/Index.vue'),
          meta: {
            title: '首页',
          }
        },
        {
          path: 'proxy/create',
          name: 'create-tunnel',
          component: () => import('../views/Dashboard/proxies/CreateTunnel.vue'),
          meta: {
            title: '创建隧道',
          }
        },
          {
              path: 'proxy/list',
              name: 'proxy-list',
              component: () => import('../views/Dashboard/proxies/ManagerTunnel.vue'),
              meta: {
                  title: '隧道列表',
              }
          },
        {
          path: 'user/my-profile',
          name: 'user-profile',
          component: () => import('../views/Dashboard/Profile.vue'),
          meta: {
              title: '用户信息',
          }
        },
        {
          path: 'logs',
          name: 'logs',
          component: () => import('../views/Dashboard/Log.vue'),
          meta: {
            title: '日志',
          }
        },
        {
          path: 'settings',
          name:'settings',
          component: () => import('../views/Dashboard/Settings.vue'),
          meta: {
              title: '设置',
          }
        },
      ]
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'NotFound',
        component: () => import('../views/NotFound.vue'),
        meta: {
            title: '404',
        }
    }
  ],
})

router.beforeEach((to, _from, next) => {
  //需要登录的路由校验
  if (to.matched.some(record => record.meta.requiresAuth)) {
    if (unauthorized()) {
      next({
        name: 'login',
        query: { redirect: to.fullPath } // 携带跳转路径参数
      })
    } else {
      next()
    }
  }
  // 已登录用户禁止访问登录/注册页
  else if ((to.name === 'login') && !unauthorized()) {
    next({ name: 'dashboard' })
  }
  // 其他情况直接放行
  else if (to.matched.length === 0) {
    next("/dashboard")
  }else {
    next()
  }
})

// 添加路由导航守卫
router.beforeEach(() => {
  window.$loadingBar?.start()
})

router.afterEach(() => {
  window.$loadingBar?.finish()
})

router.onError(() => {
  window.$loadingBar?.error()
})

export default router
