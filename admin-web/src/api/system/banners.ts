import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

export interface BannerGroup {
  id: number;
  name: string;
  code: string;
  description: string;
  sort: number;
  status: number;
}

export interface Banner {
  id: number;
  group_id: number;
  group_name: string;
  title: string;
  image_url: string;
  link_url: string;
  description: string;
  sort: number;
  status: number;
}

export interface CreateBannerGroupInput {
  name: string;
  code: string;
  description?: string;
  sort?: number;
  status?: number;
}

export interface UpdateBannerGroupInput {
  name?: string;
  code?: string;
  description?: string;
  sort?: number;
  status?: number;
}

export interface CreateBannerInput {
  group_id: number;
  title: string;
  image_url?: string;
  link_url?: string;
  description?: string;
  sort?: number;
  status?: number;
}

export interface UpdateBannerInput {
  group_id?: number;
  title?: string;
  image_url?: string;
  link_url?: string;
  description?: string;
  sort?: number;
  status?: number;
}

export function listBannerGroupsApi(): Promise<Result<BannerGroup[]>> {
  return axios.get("/api/admin/banner-groups");
}

export function getBannerGroupApi(id: number): Promise<Result<BannerGroup>> {
  return axios.get(`/api/admin/banner-groups/${id}`);
}

export function createBannerGroupApi(input: CreateBannerGroupInput): Promise<Result<BannerGroup>> {
  return axios.post("/api/admin/banner-groups", input);
}

export function updateBannerGroupApi(id: number, input: UpdateBannerGroupInput): Promise<Result<BannerGroup>> {
  return axios.put(`/api/admin/banner-groups/${id}`, input);
}

export function deleteBannerGroupApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/banner-groups/${id}`);
}

export function listBannersApi(groupId?: number, page?: PageParams): Promise<Result<PageResult<Banner>>> {
  return axios.get("/api/admin/banners", {
    params: {
      group_id: groupId,
      page: page?.page,
      page_size: page?.page_size,
    },
  });
}

export function getBannerApi(id: number): Promise<Result<Banner>> {
  return axios.get(`/api/admin/banners/item/${id}`);
}

export function createBannerApi(input: CreateBannerInput): Promise<Result<Banner>> {
  return axios.post("/api/admin/banners", input);
}

export function updateBannerApi(id: number, input: UpdateBannerInput): Promise<Result<Banner>> {
  return axios.put(`/api/admin/banners/${id}`, input);
}

export function deleteBannerApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/banners/${id}`);
}
