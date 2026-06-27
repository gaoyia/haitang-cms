import type { Plugin } from "vite";
/**
 * 开发模式下：访问公开站路径时 302 到后端（VITE_SERVER），避免 admin SPA 404。
 */
export declare function devPublicRedirectPlugin(serverUrl: string, localesRaw: string | undefined): Plugin;
