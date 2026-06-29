// md-editor 未导出全局配置；config() 的 deepMerge 会 excludeKeys 跳过 instance 字段
// @ts-expect-error 内部模块路径，用于注入本地 screenfull 实例
import { g as mdEditorGlobalConfig } from "md-editor-v3/lib/es/chunks/config.mjs";
import screenfull from "screenfull";

/**
 * 用本地依赖替代 md-editor-v3 默认从 unpkg 加载的 screenfull，
 * 避免浏览器跟踪防护拦截第三方 CDN 脚本。
 *
 * 不可仅用 config({ editorExtensions: { screenfull: { instance } } })，
 * 因为 md-editor 合并配置时会排除 instance 键，CDN 仍会加载。
 */
mdEditorGlobalConfig.editorExtensions.screenfull.instance = screenfull;
