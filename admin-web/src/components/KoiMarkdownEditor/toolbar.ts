import type { Component } from "vue";
import type { ToolbarNames } from "md-editor-v3";
import {
  ChatLineSquare,
  Document,
  DocumentChecked,
  DocumentCopy,
  FullScreen,
  Grid,
  Link,
  List,
  Picture,
  Tickets,
} from "@element-plus/icons-vue";

/** 窄屏菜单：编辑器命令前缀；全屏等特殊动作见 COMPACT_PAGE_FULLSCREEN 等 */
export const COMPACT_PAGE_FULLSCREEN = "@pageFullscreen";
export const COMPACT_FULLSCREEN = "@fullscreen";

/** 窄屏工具菜单项 */
export type CompactToolItem = {
  label: string;
  command: string;
  /** Element Plus 图标组件 */
  icon?: Component;
  /** 无合适图标时用字母字形（如 B、I） */
  glyph?: string;
  glyphClass?: string;
};

/** 窄屏下拉菜单分区 */
export type CompactMenuSections = {
  /** 顶栏全屏（各占一列） */
  fullscreen: CompactToolItem[];
  /** 下方双列工具 */
  tools: CompactToolItem[];
};

/** 与 Element / admin-ui 一致：宽度 < 768px 启用紧凑工具栏 */
export const MD_EDITOR_COMPACT_MAX_WIDTH = 768;

export const fullToolbars: ToolbarNames[] = [
  "bold",
  "underline",
  "italic",
  "-",
  "title",
  "strikeThrough",
  "quote",
  "unorderedList",
  "orderedList",
  "task",
  "-",
  "codeRow",
  "code",
  "link",
  "table",
  "-",
  "revoke",
  "next",
  "=",
  "preview",
  "catalog",
  "pageFullscreen",
  "fullscreen",
];

/** 窄屏工具菜单在 toolbars 数组中的占位索引（对应 defToolbars 第一项） */
export const COMPACT_TOOL_MENU = 0;

export const compactToolbars: ToolbarNames[] = [
  COMPACT_TOOL_MENU,
  "-",
  "revoke",
  "next",
  "=",
  "preview",
  "catalog",
];

/** 按需插入图片按钮（位于 link 之后） */
export function withImageUpload(items: ToolbarNames[]): ToolbarNames[] {
  const next = [...items];
  const linkIdx = next.indexOf("link");
  if (linkIdx >= 0) {
    next.splice(linkIdx + 1, 0, "image");
  } else {
    next.unshift("image");
  }
  return next;
}

/** 窄屏下拉菜单（顶栏全屏 + 双列工具） */
export function buildCompactMenuSections(enableImageUpload: boolean): CompactMenuSections {
  const tools: CompactToolItem[] = [
    { label: "加粗", command: "bold", glyph: "B", glyphClass: "bold" },
    { label: "下划线", command: "underline", glyph: "U", glyphClass: "underline" },
    { label: "斜体", command: "italic", glyph: "I", glyphClass: "italic" },
    { label: "删除线", command: "strikeThrough", glyph: "S", glyphClass: "strike" },
    { label: "标题", command: "h2", icon: Document },
    { label: "引用", command: "quote", icon: ChatLineSquare },
    { label: "无序列表", command: "unorderedList", icon: List },
    { label: "有序列表", command: "orderedList", icon: Tickets },
    { label: "任务列表", command: "task", icon: DocumentChecked },
    { label: "行内代码", command: "codeRow", glyph: "</>", glyphClass: "mono" },
    { label: "代码块", command: "code", icon: DocumentCopy },
    { label: "链接", command: "link", icon: Link },
    { label: "表格", command: "table", icon: Grid },
  ];
  if (enableImageUpload) {
    tools.push({ label: "图片", command: "image", icon: Picture });
  }
  return {
    fullscreen: [
      { label: "页面全屏", command: COMPACT_PAGE_FULLSCREEN, icon: FullScreen },
      { label: "浏览器全屏", command: COMPACT_FULLSCREEN, icon: FullScreen },
    ],
    tools,
  };
}
