/** 字典项 code：站点名称 */
export const DICT_SITE_NAME = "site_name";

/** 字典项 code：站点 Logo */
export const DICT_SITE_LOGO = "site_logo";

/** 字典项 code：备案号 */
export const DICT_SITE_ICP = "site_icp";

const DEFAULT_SITE_NAME = import.meta.env.VITE_WEB_TITLE || "海棠 CMS";

/** 默认 Logo（admin 静态资源） */
export const DEFAULT_SITE_LOGO = "/logo.svg";

/**
 * 将字典中的 Logo 地址解析为可请求的 URL。
 * - 完整 URL 原样返回
 * - `/static/*` 走后端静态资源（开发环境拼接 VITE_SERVER）
 * - 其余以 `/` 开头的路径视为当前站点资源
 */
export function resolveSiteLogoUrl(raw: string | undefined | null): string {
  const value = raw?.trim();
  if (!value) {
    return DEFAULT_SITE_LOGO;
  }
  if (/^https?:\/\//i.test(value)) {
    return value;
  }
  if (value.startsWith("/static/")) {
    const server = (import.meta.env.VITE_SERVER as string | undefined)?.replace(/\/$/, "") ?? "";
    return server ? `${server}${value}` : value;
  }
  return value;
}

/** 解析站点名称，空值时使用环境变量或内置默认 */
export function resolveSiteName(raw: string | undefined | null): string {
  const value = raw?.trim();
  return value || DEFAULT_SITE_NAME;
}

/** 更新浏览器 favicon */
export function updateFavicon(href: string): void {
  if (!href) return;
  let link = document.querySelector<HTMLLinkElement>("link[rel*='icon']");
  if (!link) {
    link = document.createElement("link");
    link.rel = "icon";
    document.head.appendChild(link);
  }
  if (link.href !== href) {
    link.href = href;
  }
}

/** 组合页面标题：「页面名 - 站点名」 */
export function buildPageTitle(pageTitle: string | undefined, siteName: string): string {
  const page = pageTitle?.trim();
  const site = siteName.trim() || DEFAULT_SITE_NAME;
  return page ? `${page} - ${site}` : site;
}
