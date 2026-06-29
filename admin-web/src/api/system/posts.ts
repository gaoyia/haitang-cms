import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";
import type { AssetView } from "@/api/system/assets.ts";

export interface PostView {
  id: number;
  title: string;
  description: string;
  content: string;
  tags: string;
  category_id: number;
  category_name: string;
  route_path: string;
  status: number;
  lang: string;
  created_at: number;
  updated_at: number;
  published_at: number;
  publish_time: number;
  display_time: number;
  covers?: AssetView[];
  attachments?: AssetView[];
  list_template?: string;
  detail_template?: string;
  meta_json?: string;
}

export interface PostI18nPayload {
  title: string;
  description: string;
  content: string;
  route_path: string;
  tags: string;
}

export interface PostDetailView {
  id: number;
  category_id: number;
  status: number;
  created_at: number;
  updated_at: number;
  published_at: number;
  publish_time: number;
  display_time: number;
  translations: Record<string, PostI18nPayload>;
  covers?: AssetView[];
  attachments?: AssetView[];
  meta_json?: string;
}

export interface CreatePostInput {
  title: string;
  description?: string;
  content?: string;
  tags?: string;
  category_id?: number;
  route_path?: string;
  lang?: string;
  status?: number;
  display_time?: number;
  publish_time?: number;
  meta_json?: string;
}

export interface UpdatePostInput {
  title?: string;
  description?: string;
  content?: string;
  tags?: string;
  category_id?: number;
  route_path?: string;
  lang?: string;
  status?: number;
  display_time?: number;
  publish_time?: number;
  meta_json?: string;
}

export interface PostListParams extends PageParams {
  category_id?: number;
}

export function listPostsApi(
  lang?: string,
  page?: PostListParams,
): Promise<Result<PageResult<PostView>>> {
  const categoryId = page?.category_id;
  return axios.get("/api/admin/posts", {
    params: {
      lang,
      page: page?.page,
      page_size: page?.page_size,
      ...(categoryId && categoryId > 0 ? { category_id: categoryId } : {}),
    },
  });
}

export function getPostApi(id: number): Promise<Result<PostDetailView>> {
  return axios.get(`/api/admin/posts/${id}`);
}

export function createPostApi(input: CreatePostInput): Promise<Result<PostView>> {
  return axios.post("/api/admin/posts", input);
}

export function updatePostApi(id: number, input: UpdatePostInput): Promise<Result<PostView>> {
  return axios.put(`/api/admin/posts/${id}`, input);
}

export function deletePostApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/posts/${id}`);
}
