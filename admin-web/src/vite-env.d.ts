/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_ENV: string;
  readonly VITE_WEB_TITLE: string;
  readonly VITE_WEB_EN_TITLE: string;
  readonly VITE_LOGIN_TITLE: string;
  readonly VITE_LOGIN_EN_TITLE: string;
  readonly VITE_WEB_BASE_API: string;
  readonly VITE_SERVER: string;
  readonly VITE_ROUTER_MODE: string;
  readonly VITE_DROP_CONSOLE: string;
  /** 部署 base path，如 /haitang-cms-admin/ */
  readonly VITE_BASE: string;
  /** 构建输出目录，相对 admin-web 根目录 */
  readonly VITE_BUILD_OUT_DIR: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
