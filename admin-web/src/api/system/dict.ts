import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

/** 字典键值映射（code → value） */
export type DictMap = Record<string, string>;

/** 公开接口：一次性获取全部字典（按 lang 解析多语言 value） */
export function getDictMapApi(lang?: string): Promise<Result<DictMap>> {
  return axios.get("/api/dicts/map", {
    params: lang ? { lang } : undefined,
  });
}

export interface DictMetaView {
  code: string;
  label: string;
  description: string;
  translatable: boolean;
  sort: number;
}

/** 列表项：含当前值预览 */
export interface DictMetaListView extends DictMetaView {
  preview_value: string;
}

export interface DictDetailView extends DictMetaView {
  values: Record<string, string>;
}

export interface CreateDictInput {
  code: string;
  label: string;
  description?: string;
  translatable?: boolean;
  sort?: number;
  value?: string;
  lang?: string;
}

export interface UpdateDictInput {
  label?: string;
  description?: string;
  translatable?: boolean;
  sort?: number;
}

export function listDictsApi(
  lang?: string,
  page?: PageParams,
): Promise<Result<PageResult<DictMetaListView>>> {
  return axios.get("/api/admin/dicts", {
    params: { lang, page: page?.page, page_size: page?.page_size },
  });
}

export function getDictApi(code: string): Promise<Result<DictDetailView>> {
  return axios.get(`/api/admin/dicts/${encodeURIComponent(code)}`);
}

export function createDictApi(input: CreateDictInput): Promise<Result<DictDetailView>> {
  return axios.post("/api/admin/dicts", input);
}

export function updateDictApi(code: string, input: UpdateDictInput): Promise<Result<DictDetailView>> {
  return axios.put(`/api/admin/dicts/${encodeURIComponent(code)}`, input);
}

export function updateDictValuesApi(code: string, values: Record<string, string>): Promise<Result<DictDetailView>> {
  return axios.put(`/api/admin/dicts/${encodeURIComponent(code)}/values`, { values });
}

export function deleteDictApi(code: string): Promise<Result<null>> {
  return axios.delete(`/api/admin/dicts/${encodeURIComponent(code)}`);
}
