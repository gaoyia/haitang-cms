// 定义权限小仓库[选择式Api写法]
import { defineStore } from "pinia";
import { staticRouter } from "@/routers/modules/staticRouter";
import authMenu from "@/assets/json/authMenu.json";
import { generateRoutes, generateFlattenRoutes } from "@/utils/filterRoute.ts";
import { getShowStaticAndDynamicMenuList, getAllBreadcrumbList } from "@/utils/index.ts";
import { getMeApi } from "@/api/system/auth.ts";

// 权限数据，不进行持久化。否则刷新浏览器无法获取新的数据。
const authStore = defineStore("auth", {
  state: (): any => {
    return {
      menuList: [],
      recursiveMenuList: [],
      breadcrumbList: [],
      roleList: [],
      buttonList: [],
      loginUser: {
        userId: "",
        loginName: "",
        sex: "",
        avatar: "",
      },
    };
  },
  actions: {
    /** 获取菜单数据（首期使用静态 JSON，后续对接 /api/admin/nav） */
    async listRouters() {
      this.menuList = generateFlattenRoutes(authMenu.data);
      this.recursiveMenuList = getShowStaticAndDynamicMenuList(staticRouter).concat(
        generateRoutes(getShowStaticAndDynamicMenuList(authMenu.data), 0),
      );
      this.breadcrumbList = staticRouter.concat(generateRoutes(authMenu.data, 0));
    },
    /** 从后端获取当前用户信息 */
    async getLoginUserInfo() {
      try {
        const res: any = await getMeApi();
        const user = res?.data;
        if (!user) return;
        this.roleList = user.roles ?? [];
        this.buttonList = user.permissions ?? [];
        this.loginUser = {
          userId: String(user.id),
          loginName: user.username,
          sex: "",
          avatar: user.nickname || user.username,
        };
      } catch (error) {
        console.warn("[auth] 获取用户信息失败", error);
      }
    },
  },
  getters: {
    getButtonList: (state) => state.buttonList,
    getMenuList: (state) => state.menuList,
    showMenuList: (state) => state.recursiveMenuList,
    getBreadcrumbList: (state) => getAllBreadcrumbList(state.breadcrumbList),
  },
});

export default authStore;
