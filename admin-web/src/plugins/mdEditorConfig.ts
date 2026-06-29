import { config } from "md-editor-v3";

/** 编辑器静态依赖目录（对应 public/md-editor-vendors/） */
function vendorUrl(relativePath: string): string {
  const base = import.meta.env.BASE_URL ?? "/";
  const normalized = base.endsWith("/") ? base : `${base}/`;
  return `${normalized}md-editor-vendors/${relativePath}`;
}

/**
 * 将 md-editor-v3 默认 unpkg CDN 改为同源静态资源，避免跟踪防护拦截。
 * 资源说明与同步方式见 public/md-editor-vendors/README.md
 */
config({
  editorExtensions: {
    highlight: {
      js: vendorUrl("highlight.min.js"),
      css: {
        github: {
          light: vendorUrl("styles/github.min.css"),
          dark: vendorUrl("styles/github-dark.min.css"),
        },
      },
    },
    katex: {
      js: vendorUrl("katex.min.js"),
      css: vendorUrl("katex.min.css"),
    },
    mermaid: {
      js: vendorUrl("mermaid.min.js"),
    },
    echarts: {
      js: vendorUrl("echarts.min.js"),
    },
    cropper: {
      js: vendorUrl("cropper.min.js"),
      css: vendorUrl("cropper.min.css"),
    },
    prettier: {
      standaloneJs: vendorUrl("prettier.standalone.js"),
      parserMarkdownJs: vendorUrl("prettier-markdown.js"),
    },
    screenfull: {
      js: vendorUrl("screenfull.js"),
    },
  },
});
