import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

export interface UserView {
  id: number;
  username: string;
  nickname: string;
  email: string;
  status: number;
  role_ids: number[];
}

export interface CreateUserInput {
  username: string;
  password: string;
  nickname?: string;
  email?: string;
}

export interface UpdateUserInput {
  nickname?: string;
  email?: string;
  status?: number;
  password?: string;
}

export function listUsersApi(page?: PageParams): Promise<Result<PageResult<UserView>>> {
  return axios.get("/api/admin/users", {
    params: { page: page?.page, page_size: page?.page_size },
  });
}

export function getUserApi(id: number): Promise<Result<UserView>> {
  return axios.get(`/api/admin/users/${id}`);
}

export function createUserApi(input: CreateUserInput): Promise<Result<UserView>> {
  return axios.post("/api/admin/users", input);
}

export function updateUserApi(id: number, input: UpdateUserInput): Promise<Result<UserView>> {
  return axios.put(`/api/admin/users/${id}`, input);
}

export function deleteUserApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/users/${id}`);
}

export function assignUserRolesApi(id: number, roleIds: number[]): Promise<Result<null>> {
  return axios.put(`/api/admin/users/${id}/roles`, { role_ids: roleIds });
}
