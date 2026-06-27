import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

export interface RoleView {
  id: number;
  name: string;
  description: string;
  permissions: string[];
}

export interface PermissionGroup {
  group: string;
  permissions: { code: string; label: string }[];
}

export interface CreateRoleInput {
  name: string;
  description?: string;
  permissions?: string[];
}

export interface UpdateRoleInput {
  name?: string;
  description?: string;
  permissions?: string[];
}

export function listRolesApi(page?: PageParams): Promise<Result<PageResult<RoleView>>> {
  return axios.get("/api/admin/roles", {
    params: { page: page?.page, page_size: page?.page_size },
  });
}

export function getRoleApi(id: number): Promise<Result<RoleView>> {
  return axios.get(`/api/admin/roles/${id}`);
}

export function createRoleApi(input: CreateRoleInput): Promise<Result<RoleView>> {
  return axios.post("/api/admin/roles", input);
}

export function updateRoleApi(id: number, input: UpdateRoleInput): Promise<Result<RoleView>> {
  return axios.put(`/api/admin/roles/${id}`, input);
}

export function deleteRoleApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/roles/${id}`);
}

export function listPermissionsApi(): Promise<Result<PermissionGroup[]>> {
  return axios.get("/api/admin/permissions");
}
