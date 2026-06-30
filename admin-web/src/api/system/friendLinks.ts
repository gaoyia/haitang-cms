import type { AxiosRequestConfig } from "axios";
import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

export interface FriendLink {
  id: number;
  title: string;
  url: string;
  image_url: string;
  sort: number;
  status: number;
}

export interface CreateFriendLinkInput {
  title: string;
  url: string;
  image_url: string;
  sort?: number;
  status?: number;
}

export interface UpdateFriendLinkInput {
  title?: string;
  url?: string;
  image_url?: string;
  sort?: number;
  status?: number;
}

export function listFriendLinksApi(params?: PageParams) {
  return axios.get<Result<PageResult<FriendLink>>>("/api/admin/friend-links", { params });
}

export function getFriendLinkApi(id: number) {
  return axios.get<Result<FriendLink>>(`/api/admin/friend-links/${id}`);
}

export function createFriendLinkApi(data: CreateFriendLinkInput) {
  return axios.post<Result<FriendLink>>("/api/admin/friend-links", data);
}

export function updateFriendLinkApi(
  id: number,
  data: UpdateFriendLinkInput,
  config?: AxiosRequestConfig,
) {
  return axios.put<Result<FriendLink>>(`/api/admin/friend-links/${id}`, data, config);
}

export function deleteFriendLinkApi(id: number) {
  return axios.delete<Result<null>>(`/api/admin/friend-links/${id}`);
}
