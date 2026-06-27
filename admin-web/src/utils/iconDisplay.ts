/** 图标字段的展示类型 */
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import { SVG_PREFIX } from "@/config/index.ts";
import { isLocalIconName } from "@/utils/localIcons.ts";

export type IconDisplayKind = "image" | "icon" | "emoji" | "empty";

export interface IconDisplayResult {
  kind: IconDisplayKind;
  /** trim 后的原始值 */
  value: string;
}

/**
 * 解析菜单/标签等 icon 字段的展示方式：
 * 1. 含 http:// 或 https:// → 图片 URL
 * 2. 以英文字母开头 → 图标名（koi-* 本地 SVG 或 Element Plus 组件名）
 * 3. 其余 → 按 emoji/文本直接展示
 */
export function parseIconDisplay(raw: string | undefined | null): IconDisplayResult {
  const value = raw?.trim() ?? "";
  if (!value) {
    return { kind: "empty", value: "" };
  }
  if (/https?:\/\//i.test(value)) {
    return { kind: "image", value };
  }
  if (/^[a-zA-Z]/.test(value)) {
    return { kind: "icon", value };
  }
  return { kind: "emoji", value };
}

function isEpIconName(name: string): boolean {
  return name in ElementPlusIconsVue;
}

/** 图标名是否可实际渲染（空值、不存在的本地 SVG、未注册的 EP 图标均视为不可展示） */
export function hasIconDisplay(raw: string | undefined | null): boolean {
  const parsed = parseIconDisplay(raw);
  switch (parsed.kind) {
    case "empty":
      return false;
    case "image":
    case "emoji":
      return true;
    case "icon":
      if (parsed.value.startsWith(SVG_PREFIX)) {
        return isLocalIconName(parsed.value);
      }
      return isEpIconName(parsed.value);
    default:
      return false;
  }
}
