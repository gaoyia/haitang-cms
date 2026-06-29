import axios, { type Result } from "@/utils/axios.ts";

/** 登录请求参数 */
export interface LoginInput {
  username: string;
  password: string;
}

/** 登录用户信息 */
export interface LoginUserInfo {
  id: number;
  username: string;
  nickname: string;
  roles: string[];
  permissions: string[];
}

/** 登录响应 */
export interface LoginResponse {
  token: string;
  user: LoginUserInfo;
}

/** 后台侧栏导航项（与后端 AdminNavMenuJsonItem / 原 authMenu.json 对齐） */
export interface AdminNavMenuJsonItem {
  menuId: number;
  menuName: string;
  parentId: number;
  menuType: string;
  path: string;
  name: string;
  component: string;
  icon: string;
  isVisible: string;
  linkUrl: string;
  isKeepAlive: string;
  isTag: string;
  isAffix: string;
  redirect: string;
}

/** 管理员登录 */
export function loginApi(input: LoginInput) {
  return axios.post<LoginResponse>("/api/admin/login", input);
}

/** 获取当前用户信息 */
export function getMeApi() {
  return axios.get<LoginUserInfo>("/api/admin/me");
}

/** 获取后台侧边栏导航（code=admin_sidebar） */
export function getNavApi(code = "admin_sidebar"): Promise<Result<AdminNavMenuJsonItem[]>> {
  return axios.get("/api/admin/nav", {
    params: { code },
  });
}
