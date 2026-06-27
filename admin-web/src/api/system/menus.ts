import axios, { type Result } from "@/utils/axios.ts";

export interface MenuGroup {
  id: number;
  name: string;
  code: string;
  description: string;
  sort: number;
  status: number;
  readonly: boolean;
}

export interface MenuItem {
  id: number;
  group_id: number;
  parent_id: number;
  title: string;
  path: string;
  icon: string;
  permission: string;
  sort: number;
  status: number;
  children: MenuItem[];
}

export interface MenuGroupTree {
  group: MenuGroup;
  menus: MenuItem[];
}

export interface PermissionGroup {
  group: string;
  permissions: { code: string; label: string }[];
}

export interface CreateMenuGroupInput {
  name: string;
  code: string;
  description?: string;
  sort?: number;
  status?: number;
}

export interface UpdateMenuGroupInput {
  name?: string;
  code?: string;
  description?: string;
  sort?: number;
  status?: number;
}

export interface CreateMenuInput {
  group_id: number;
  parent_id?: number;
  title: string;
  path?: string;
  icon?: string;
  permission?: string;
  sort?: number;
  status?: number;
  lang?: string;
}

export interface UpdateMenuInput {
  group_id?: number;
  parent_id?: number;
  title?: string;
  path?: string;
  icon?: string;
  permission?: string;
  sort?: number;
  status?: number;
  lang?: string;
}

export function listMenuGroupsApi(): Promise<Result<MenuGroup[]>> {
  return axios.get("/api/admin/menu-groups");
}

export function getMenuGroupApi(id: number): Promise<Result<MenuGroup>> {
  return axios.get(`/api/admin/menu-groups/${id}`);
}

export function createMenuGroupApi(input: CreateMenuGroupInput): Promise<Result<MenuGroup>> {
  return axios.post("/api/admin/menu-groups", input);
}

export function updateMenuGroupApi(id: number, input: UpdateMenuGroupInput): Promise<Result<MenuGroup>> {
  return axios.put(`/api/admin/menu-groups/${id}`, input);
}

export function deleteMenuGroupApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/menu-groups/${id}`);
}

export function getMenusOverviewApi(lang?: string): Promise<Result<MenuGroupTree[]>> {
  return axios.get("/api/admin/menus/overview", { params: lang ? { lang } : undefined });
}

export function listMenusApi(groupId: number, lang?: string): Promise<Result<MenuItem[]>> {
  return axios.get("/api/admin/menus", {
    params: { group_id: groupId, ...(lang ? { lang } : {}) },
  });
}

export function getMenuItemApi(id: number, lang?: string): Promise<Result<MenuItem>> {
  return axios.get(`/api/admin/menus/item/${id}`, {
    params: lang ? { lang } : undefined,
  });
}

export function createMenuApi(input: CreateMenuInput): Promise<Result<MenuItem>> {
  return axios.post("/api/admin/menus", input);
}

export function updateMenuApi(id: number, input: UpdateMenuInput): Promise<Result<MenuItem>> {
  return axios.put(`/api/admin/menus/${id}`, input);
}

export function deleteMenuApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/menus/${id}`);
}

export function listPermissionsApi(): Promise<Result<PermissionGroup[]>> {
  return axios.get("/api/admin/permissions");
}
