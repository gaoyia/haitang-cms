import { resolveAssetUrl } from "@/utils/siteAsset.ts";

/**
 * 文件类型图标（SVG）
 *
 * - 源目录：`admin-web/public/fileicon/`（维护、提交 Git）
 * - 运行时 URL：`/static/fileicon/{slug}.svg`（由 build.rs / Vite 同步到 `static/fileicon/`）
 * - 未知类型：`unknow-file.svg`
 *
 * 完整清单见 docs/src/admin/assets.md「文件类型图标」。
 */

/** 静态资源 URL 前缀（相对站点根，经 Rocket `/static` 挂载） */
export const FILE_ICON_PUBLIC_PREFIX = "/static/fileicon";

/** 未知类型图标文件名（不含路径） */
export const FILE_ICON_UNKNOWN = "unknow-file.svg";

/** 扩展名 → 图标 slug（与 public/fileicon 下 `{slug}.svg` 对应） */
const EXT_ICON: Record<string, string> = {
  pdf: "pdf",
  doc: "doc",
  docx: "doc",
  xls: "xls",
  xlsx: "xlsx",
  ppt: "ppt",
  pptx: "ppt",
  zip: "zip",
  rar: "rar",
  "7z": "7z",
  txt: "txt",
  md: "md",
  csv: "csv",
  html: "html",
  htm: "html",
  mp3: "mp3",
  wav: "audio",
  flac: "audio",
  mov: "mov",
  mp4: "video",
  webm: "video",
  mkv: "video",
  avi: "video",
  flv: "flv",
  png: "image",
  jpg: "image",
  jpeg: "image",
  gif: "image",
  webp: "image",
  svg: "svg",
  psd: "psd",
  ai: "ai",
  eps: "eps",
  apk: "apk",
  exe: "exe",
  sh: "shell",
  bash: "shell",
};

const MIME_ICON: [string, string][] = [
  ["application/pdf", "pdf"],
  ["application/msword", "doc"],
  [
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "doc",
  ],
  ["application/vnd.ms-excel", "xls"],
  [
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlsx",
  ],
  ["application/vnd.ms-powerpoint", "ppt"],
  [
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "ppt",
  ],
  ["application/zip", "zip"],
  ["application/x-zip-compressed", "zip"],
  ["application/x-rar-compressed", "rar"],
  ["application/vnd.rar", "rar"],
  ["application/x-7z-compressed", "7z"],
  ["text/plain", "txt"],
  ["text/markdown", "md"],
  ["text/csv", "csv"],
  ["text/html", "html"],
  ["audio/", "audio"],
  ["video/", "video"],
  ["image/", "image"],
];

function fileExtension(filename: string): string {
  const base = filename.trim().split(/[/\\]/).pop() ?? filename;
  const idx = base.lastIndexOf(".");
  if (idx <= 0) return "";
  return base.slice(idx + 1).toLowerCase();
}

/** 是否可直接缩略图预览（图片 / 视频） */
export function isAssetPreviewable(mimeType: string): boolean {
  const mime = mimeType.trim().toLowerCase();
  return mime.startsWith("image/") || mime.startsWith("video/");
}

/** 解析图标 slug（不含 `.svg`） */
export function resolveAssetFileIconSlug(
  mimeType: string,
  filename: string,
): string {
  const mime = mimeType.trim().toLowerCase();
  const ext = fileExtension(filename);

  if (ext && EXT_ICON[ext]) {
    return EXT_ICON[ext];
  }

  for (const [pattern, slug] of MIME_ICON) {
    if (pattern.endsWith("/")) {
      if (mime.startsWith(pattern)) return slug;
    } else if (mime === pattern || mime.startsWith(`${pattern};`)) {
      return slug;
    }
  }

  if (mime.includes("pdf")) return "pdf";
  if (mime.includes("word") || mime.includes("document")) return "doc";
  if (mime.includes("spreadsheet") || mime.includes("excel")) {
    return ext === "xls" ? "xls" : "xlsx";
  }
  if (mime.includes("presentation") || mime.includes("powerpoint")) return "ppt";
  if (mime.includes("zip")) return "zip";
  if (mime.includes("rar")) return "rar";
  if (mime.includes("7z")) return "7z";
  if (mime.startsWith("text/")) return "txt";

  return FILE_ICON_UNKNOWN.replace(/\.svg$/, "");
}

/** 相对站点根的图标路径，如 `/static/fileicon/pdf.svg` */
export function assetFileIconPath(mimeType: string, filename: string): string {
  const slug = resolveAssetFileIconSlug(mimeType, filename);
  return `${FILE_ICON_PUBLIC_PREFIX}/${slug}.svg`;
}

/** 可请求的完整 URL（开发环境拼接 VITE_SERVER） */
export function assetFileIconUrl(mimeType: string, filename: string): string {
  return resolveAssetUrl(assetFileIconPath(mimeType, filename));
}
