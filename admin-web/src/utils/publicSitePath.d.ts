/** 开发服务器上应跳过、交给 Vite 自身处理的路径 */
export declare function shouldSkipPublicRedirect(pathname: string): boolean;
/**
 * 是否为公开站（Rocket Tera）路径，而非 admin SPA 路由。
 * 例如 /zh-cn/posts/1、/en-us/about
 */
export declare function isPublicSitePath(pathname: string, localesRaw?: string): boolean;
/** 开发环境：跳转到后端公开站同一地址 */
export declare function redirectToPublicServer(fullPath: string): void;
/** 供 vite.config 使用（无 import.meta.env） */
export declare function isPublicSitePathForDev(pathname: string, localesRaw: string | undefined): boolean;
