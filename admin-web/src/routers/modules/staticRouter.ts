import { RouteRecordRaw } from "vue-router";
import { HOME_URL, LOGIN_URL } from "@/config";
import Layout from "@/layouts/index.vue";

export const layoutRouter: RouteRecordRaw[] = [
  {
    path: LOGIN_URL,
    name: "login",
    component: () => import("@/views/login/index.vue"),
    meta: {
      title: "menu.login.auth",
    },
  },
];

/** 静态路由 */
export const staticRouter: RouteRecordRaw[] = [
  {
    path: "/",
    name: "layout",
    component: Layout,
    redirect: HOME_URL,
    meta: {
      menuId: "-1",
      title: "menu.home.auth",
      icon: "koi-home",
      isVisible: "1",
      linkUrl: "",
      isKeepAlive: "1",
      isTag: "0",
      isAffix: "1",
    },
    children: [
      {
        path: HOME_URL,
        name: "homePage",
        component: () => import("@/views/home/index.vue"),
        meta: {
          menuId: "-2",
          title: "menu.home.work.name",
          icon: "koi-work",
          isVisible: "1",
          linkUrl: "",
          isKeepAlive: "1",
          isTag: "1",
          isAffix: "1",
        },
      },
    ],
  },
];

/** 错误页面路由 */
export const errorRouter = [
  {
    path: "/403",
    name: "403",
    component: () => import("@/views/error/403.vue"),
    meta: {
      menuId: "-403",
      title: "menu.coding.403.name",
      icon: "QuestionFilled",
      isVisible: "1",
      linkUrl: "",
      isKeepAlive: "1",
      isTag: "0",
      isAffix: "0",
    },
  },
  {
    path: "/404",
    name: "404",
    component: () => import("@/views/error/404.vue"),
    meta: {
      menuId: "-404",
      title: "menu.coding.404.name",
      icon: "CircleCloseFilled",
      isVisible: "1",
      linkUrl: "",
      isKeepAlive: "1",
      isTag: "0",
      isAffix: "0",
    },
  },
  {
    path: "/500",
    name: "500",
    component: () => import("@/views/error/500.vue"),
    meta: {
      menuId: "-500",
      title: "menu.coding.500.name",
      icon: "WarningFilled",
      isVisible: "1",
      linkUrl: "",
      isKeepAlive: "1",
      isTag: "0",
      isAffix: "0",
    },
  },
  {
    path: "/:pathMatch(.*)*",
    component: () => import("@/views/error/404.vue"),
  },
];
