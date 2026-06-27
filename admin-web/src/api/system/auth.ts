import axios from "@/utils/axios.ts";

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

/** 管理员登录 */
export function loginApi(input: LoginInput) {
  return axios.post<LoginResponse>("/api/admin/login", input);
}

/** 获取当前用户信息 */
export function getMeApi() {
  return axios.get<LoginUserInfo>("/api/admin/me");
}

/** 获取后台侧边栏导航（code=admin_sidebar） */
export function getNavApi(code = "admin_sidebar") {
  return axios.get<NavMenuItem[]>(`/api/admin/nav?code=${encodeURIComponent(code)}`);
}

/** 导航菜单项（与后端 MenuView 对齐） */
export interface NavMenuItem {
  id: number;
  group_id?: number;
  parent_id: number;
  title: string;
  path: string;
  icon: string;
  permission: string;
  sort: number;
  status?: number;
  children: NavMenuItem[];
}
