import type { AssetPurpose } from "@/api/system/assets.ts";

/** 各用途的上传 accept 与是否仅图片 */
export function assetPurposeAccept(purpose: AssetPurpose): string {
  switch (purpose) {
    case "cover":
    case "content":
    case "banner":
      return "image/jpeg,image/png,image/webp,image/gif";
    case "attachment":
      return [
        "image/jpeg",
        "image/png",
        "image/webp",
        "image/gif",
        "video/mp4",
        "video/webm",
        "video/quicktime",
        ".zip",
        ".rar",
        ".7z",
        ".pdf",
        ".doc",
        ".docx",
        ".xls",
        ".xlsx",
        ".ppt",
        ".pptx",
        ".txt",
        ".md",
        ".mp4",
        ".webm",
        ".mov",
        ".mkv",
        ".avi",
      ].join(",");
    default:
      return "";
  }
}

export function assetPurposeIsImage(purpose: AssetPurpose): boolean {
  return purpose === "cover" || purpose === "content" || purpose === "banner";
}
