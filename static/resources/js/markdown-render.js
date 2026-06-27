/**
 * 公开站 Markdown 渲染（marked + DOMPurify + github-markdown-css）
 * 选型说明见 docs/src/markdown.md
 */
(function (global) {
  "use strict";

  /** 去除 Markdown 标记，供列表摘要使用 */
  function stripMarkdown(text) {
    if (!text) return "";
    return String(text)
      .replace(/```[\s\S]*?```/g, " ")
      .replace(/`[^`]+`/g, " ")
      .replace(/!\[[^\]]*\]\([^)]*\)/g, " ")
      .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
      .replace(/^#{1,6}\s+/gm, "")
      .replace(/^\s*[-*+]\s+/gm, "")
      .replace(/^\s*\d+\.\s+/gm, "")
      .replace(/[*_~>#|]/g, " ")
      .replace(/\s+/g, " ")
      .trim();
  }

  /** 将 Markdown 安全渲染到容器，容器会追加 markdown-body 类 */
  function renderMarkdown(markdown, container) {
    if (!container) return false;
    if (!markdown) {
      container.classList.add("markdown-body");
      container.innerHTML = "";
      return true;
    }
    if (typeof global.marked === "undefined" || typeof global.DOMPurify === "undefined") {
      return false;
    }
    container.classList.add("markdown-body");
    var html = global.marked.parse(String(markdown), { gfm: true, breaks: true });
    container.innerHTML = global.DOMPurify.sanitize(html);
    return true;
  }

  global.stripMarkdown = stripMarkdown;
  global.renderMarkdown = renderMarkdown;
})(typeof window !== "undefined" ? window : this);
