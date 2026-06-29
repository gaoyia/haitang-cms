import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

/** 详情页模板 */
export type CategoryDetailTemplate = "default" | "gallery" | "recruitment" | "about";

/** 列表页模板（none 表示无列表，仅详情可访问） */
export type CategoryListTemplate = CategoryDetailTemplate | "none";

export interface CategoryView {
  id: number;
  name: string;
  description: string;
  sort: number;
  route_path: string;
  list_template: CategoryListTemplate;
  detail_template: CategoryDetailTemplate;
}

export interface CategoryI18nPayload {
  name: string;
  description: string;
  route_path: string;
}

export interface CategoryDetailView {
  id: number;
  sort: number;
  list_template: CategoryListTemplate;
  detail_template: CategoryDetailTemplate;
  translations: Record<string, CategoryI18nPayload>;
}

export interface CreateCategoryInput {
  name: string;
  description?: string;
  sort?: number;
  lang?: string;
  list_template?: CategoryListTemplate;
  detail_template?: CategoryDetailTemplate;
  route_path?: string;
}

export interface UpdateCategoryInput {
  name?: string;
  description?: string;
  sort?: number;
  lang?: string;
  list_template?: CategoryListTemplate;
  detail_template?: CategoryDetailTemplate;
  route_path?: string;
}

export function listCategoriesApi(lang?: string, page?: PageParams): Promise<Result<PageResult<CategoryView>>> {
  return axios.get("/api/admin/categories", {
    params: { lang, page: page?.page, page_size: page?.page_size },
  });
}

export function getCategoryApi(id: number): Promise<Result<CategoryDetailView>> {
  return axios.get(`/api/admin/categories/${id}`);
}

export function createCategoryApi(input: CreateCategoryInput): Promise<Result<CategoryView>> {
  return axios.post("/api/admin/categories", input);
}

export function updateCategoryApi(id: number, input: UpdateCategoryInput): Promise<Result<CategoryView>> {
  return axios.put(`/api/admin/categories/${id}`, input);
}

export function deleteCategoryApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/categories/${id}`);
}
