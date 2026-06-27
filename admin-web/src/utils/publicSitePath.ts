/** 默认公开站语言（与后端 site_locales 种子一致，可用 VITE_PUBLIC_LOCALES 覆盖） */
const DEFAULT_PUBLIC_LOCALES = ["zh-cn", "en-us"];

function parseLocales(raw: string | undefined): string[] {
  if (!raw?.trim()) {
    return DEFAULT_PUBLIC_LOCALES;
  }
  return raw
    .split(",")
    .map((s) => s.trim().toLowerCase())
    .filter(Boolean);
}

function escapeRegex(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function buildLangPrefixPattern(locales: string[]): RegExp {
  const part = locales.map(escapeRegex).join("|");
  return new RegExp(`^/(${part})(/|$)`, "i");
}

/** 开发服务器上应跳过、交给 Vite 自身处理的路径 */
export function shouldSkipPublicRedirect(pathname: string): boolean {
  if (!pathname || pathname === "/") {
    return true;
  }
  if (
    pathname.startsWith("/@")
    || pathname.startsWith("/__")
    || pathname.startsWith("/node_modules")
    || pathname.startsWith("/src")
    || pathname.startsWith("/dev-api")
  ) {
    return true;
  }
  // 带扩展名的静态资源（admin 构建产物或 public 文件）
  if (/\.[a-zA-Z0-9]+$/.test(pathname)) {
    return true;
  }
  return false;
}

/**
 * 是否为公开站（Rocket Tera）路径，而非 admin SPA 路由。
 * 例如 /zh-cn/posts/1、/en-us/about、/posts
 */
export function isPublicSitePath(
  pathname: string,
  localesRaw?: string,
): boolean {
  const path = pathname.split("?")[0] || "/";
  if (shouldSkipPublicRedirect(path)) {
    return false;
  }

  const locales = parseLocales(localesRaw ?? import.meta.env.VITE_PUBLIC_LOCALES);
  if (buildLangPrefixPattern(locales).test(path)) {
    return true;
  }
  if (/^\/(posts|about)(\/|$)/.test(path)) {
    return true;
  }
  if (path === "/static" || path.startsWith("/static/")) {
    return true;
  }
  return false;
}

/** 开发环境：跳转到后端公开站同一地址 */
export function redirectToPublicServer(fullPath: string): void {
  const server = (import.meta.env.VITE_SERVER as string | undefined)?.replace(/\/$/, "")
    ?? "http://127.0.0.1:9000";
  window.location.replace(`${server}${fullPath}`);
}

/** 供 vite.config 使用（无 import.meta.env） */
export function isPublicSitePathForDev(
  pathname: string,
  localesRaw: string | undefined,
): boolean {
  const path = pathname.split("?")[0] || "/";
  if (shouldSkipPublicRedirect(path)) {
    return false;
  }
  const locales = parseLocales(localesRaw);
  if (buildLangPrefixPattern(locales).test(path)) {
    return true;
  }
  if (/^\/(posts|about)(\/|$)/.test(path)) {
    return true;
  }
  if (path === "/static" || path.startsWith("/static/")) {
    return true;
  }
  return false;
}
