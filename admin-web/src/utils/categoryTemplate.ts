import type { CategoryTemplate } from "@/api/system/categories.ts";

/** Element Plus Tag 外观配置 */
export interface CategoryTemplateTagStyle {
  type: "" | "success" | "info" | "warning" | "danger";
  effect: "light" | "dark" | "plain";
}

/**
 * 分类模板与标签颜色映射（新增模板时在此扩展）
 *
 * - default：信息蓝，常规文字列表 / Markdown 详情
 * - gallery：警告橙，相册卡片列表 / 图片为主详情
 * - recruitment：成功绿，招聘列表 / 岗位详情
 * - about：主色深，关于我们卡片列表 / 介绍详情
 */
export const CATEGORY_TEMPLATE_TAG: Record<CategoryTemplate, CategoryTemplateTagStyle> = {
  default: { type: "info", effect: "plain" },
  gallery: { type: "warning", effect: "plain" },
  recruitment: { type: "success", effect: "plain" },
  about: { type: "", effect: "dark" },
};

const FALLBACK_TAG: CategoryTemplateTagStyle = { type: "info", effect: "plain" };

/** 获取模板对应的 Tag 样式 */
export function getCategoryTemplateTagStyle(tpl: CategoryTemplate): CategoryTemplateTagStyle {
  return CATEGORY_TEMPLATE_TAG[tpl] ?? FALLBACK_TAG;
}

/** 当前系统支持的分类模板值（与后端 normalize 保持一致） */
export const CATEGORY_TEMPLATE_VALUES: readonly CategoryTemplate[] = [
  "default",
  "gallery",
  "recruitment",
  "about",
];
