// 全局默认配置项

/** 首页地址 */
export const HOME_URL: string = "/home";

/** 跳转子页面静态路由父级节点 */
export const STATIC_URL: string = "/system/static";

/** 登录页地址 */
export const LOGIN_URL: string = "/login";

/** pinia 仓库前缀 */
export const CACHE_PREFIX: string = "haitang-";

/** Svg 本地图片前缀（须与 assets/icons 下 koi-*.svg 文件名一致） */
export const SVG_PREFIX: string = "koi-";

/** 默认主题颜色（海棠红，DESIGN.md primary） */
export const DEFAULT_THEME: string = "#b7102a";

/** 路由白名单 */
export const ROUTER_WHITE_LIST: string[] = ["/500"];
