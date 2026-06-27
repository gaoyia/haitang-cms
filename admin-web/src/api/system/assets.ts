import axios, { type Result } from "@/utils/axios.ts";
import type { PageParams, PageResult } from "@/types/page.ts";

export type AssetPurpose = "cover" | "content" | "banner" | "attachment";
export type PostAssetRole = "cover" | "attachment";
export type BannerAssetRole = "image";

export interface AssetView {
  id: number;
  storage_key: string;
  original_name: string;
  upload_name: string;
  mime_type: string;
  size: number;
  purpose: AssetPurpose;
  url: string;
  created_at: number;
  ref_count: number;
}

export interface PostAssetsView {
  covers: AssetView[];
  cover_max: number;
  attachments: AssetView[];
}

export interface BannerAssetsView {
  image: AssetView | null;
  image_enabled: boolean;
}

export interface LinkPostAssetInput {
  asset_id: number;
  role: PostAssetRole;
  sort_order?: number;
}

export interface LinkBannerAssetInput {
  asset_id: number;
  role: BannerAssetRole;
  sort_order?: number;
  enabled?: boolean;
}

export function uploadAssetApi(
  file: File,
  purpose: AssetPurpose,
  options?: {
    postId?: number | null;
    role?: PostAssetRole;
    bannerId?: number | null;
    bannerRole?: BannerAssetRole;
  },
  onProgress?: (percent: number) => void,
): Promise<Result<AssetView>> {
  const formData = new FormData();
  formData.append("file", file);
  const params = new URLSearchParams({ purpose });
  if (options?.postId != null) {
    params.set("post_id", String(options.postId));
  }
  if (options?.role) {
    params.set("role", options.role);
  }
  if (options?.bannerId != null) {
    params.set("banner_id", String(options.bannerId));
  }
  if (options?.bannerRole) {
    params.set("banner_role", options.bannerRole);
  }
  return axios.upload(`/api/admin/assets?${params.toString()}`, formData, {
    onUploadProgress: (e) => {
      if (!onProgress) return;
      const total = e.total ?? 0;
      const pct = total > 0 ? Math.min(100, Math.round((e.loaded * 100) / total)) : 0;
      onProgress(pct);
    },
  });
}

export function listAssetsApi(
  params?: PageParams & { purpose?: AssetPurpose; keyword?: string },
): Promise<Result<PageResult<AssetView>>> {
  return axios.get("/api/admin/assets", {
    params: {
      page: params?.page,
      page_size: params?.page_size,
      purpose: params?.purpose,
      keyword: params?.keyword,
    },
  });
}

export function getAssetApi(id: number): Promise<Result<AssetView>> {
  return axios.get(`/api/admin/assets/${id}`);
}

export function deleteAssetApi(id: number): Promise<Result<null>> {
  return axios.delete(`/api/admin/assets/${id}`);
}

export function listPostAssetsApi(postId: number): Promise<Result<PostAssetsView>> {
  return axios.get(`/api/admin/posts/${postId}/assets`);
}

export function linkPostAssetApi(
  postId: number,
  input: LinkPostAssetInput,
): Promise<Result<null>> {
  return axios.post(`/api/admin/posts/${postId}/assets`, input);
}

export function unlinkPostAssetApi(
  postId: number,
  assetId: number,
  purge = false,
): Promise<Result<null>> {
  const q = purge ? "?purge=true" : "";
  return axios.delete(`/api/admin/posts/${postId}/assets/${assetId}${q}`);
}

export function listBannerAssetsApi(bannerId: number): Promise<Result<BannerAssetsView>> {
  return axios.get(`/api/admin/banners/${bannerId}/assets`);
}

export function linkBannerAssetApi(
  bannerId: number,
  input: LinkBannerAssetInput,
): Promise<Result<null>> {
  return axios.post(`/api/admin/banners/${bannerId}/assets`, input);
}

export function unlinkBannerAssetApi(
  bannerId: number,
  assetId: number,
  purge = false,
): Promise<Result<null>> {
  const q = purge ? "?purge=true" : "";
  return axios.delete(`/api/admin/banners/${bannerId}/assets/${assetId}${q}`);
}

export function setBannerImageEnabledApi(
  bannerId: number,
  enabled: boolean,
): Promise<Result<null>> {
  return axios.put(`/api/admin/banners/${bannerId}/assets/image-enabled`, { enabled });
}

/** 展示用文件名：优先用户上传原文件名 */
export function assetDisplayName(asset: Pick<AssetView, "upload_name" | "original_name">): string {
  const name = asset.upload_name?.trim();
  return name || asset.original_name;
}

/** 格式化字节大小 */
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
