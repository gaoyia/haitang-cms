import type { CategoryDetailTemplate, CategoryListTemplate } from "@/api/system/categories.ts";

/** Element Plus Tag 外观配置；不设 type 时使用默认色 */
export interface CategoryTemplateTagStyle {
  type?: "primary" | "success" | "info" | "warning" | "danger";
  effect: "light" | "dark" | "plain";
}

type CategoryTemplateTagKey = CategoryListTemplate;

/**
 * 分类模板与标签颜色映射（新增模板时在此扩展）
 */
export const CATEGORY_TEMPLATE_TAG: Record<CategoryTemplateTagKey, CategoryTemplateTagStyle> = {
  none: { type: "info", effect: "plain" },
  default: { type: "info", effect: "plain" },
  gallery: { type: "warning", effect: "plain" },
  recruitment: { type: "success", effect: "plain" },
  about: { effect: "dark" },
};

const FALLBACK_TAG: CategoryTemplateTagStyle = { type: "info", effect: "plain" };

/** 获取模板对应的 Tag 样式 */
export function getCategoryTemplateTagStyle(tpl: CategoryListTemplate): CategoryTemplateTagStyle {
  return CATEGORY_TEMPLATE_TAG[tpl] ?? FALLBACK_TAG;
}

/** 列表模板可选值（含 none） */
export const CATEGORY_LIST_TEMPLATE_VALUES: readonly CategoryListTemplate[] = [
  "none",
  "default",
  "gallery",
  "recruitment",
  "about",
];

/** 详情模板可选值（不含 none） */
export const CATEGORY_DETAIL_TEMPLATE_VALUES: readonly CategoryDetailTemplate[] = [
  "default",
  "gallery",
  "recruitment",
  "about",
];

/** @deprecated 使用 CATEGORY_LIST_TEMPLATE_VALUES / CATEGORY_DETAIL_TEMPLATE_VALUES */
export const CATEGORY_TEMPLATE_VALUES = CATEGORY_DETAIL_TEMPLATE_VALUES;
