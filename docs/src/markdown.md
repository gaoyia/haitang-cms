# Markdown 内容选型

文章正文（`post_i18n.content`）以 **Markdown 源码** 存储，不在数据库中保存 HTML。后台负责编辑，公开站负责只读渲染；两端选用不同技术栈，但预览/阅读样式统一为 **GitHub 风格**。

## 总体原则

| 原则 | 说明 |
|------|------|
| 存 Markdown | API 与数据库字段类型不变，仍为 `string` |
| 样式对齐 | 后台 `preview-theme="github"`，公开站使用 `github-markdown-css` 的 `.markdown-body` |
| 安全渲染 | 公开站 HTML 输出必须经 DOMPurify 消毒，禁止直接 `innerHTML` 原始 Markdown 解析结果 |
| 分栈实现 | 管理端 Vue 3；公开站 jQuery + Tera，不在公开页引入 md-editor 或整包 Vue |

## 管理端：编辑

| 项 | 选型 |
|----|------|
| 库 | [md-editor-v3](https://github.com/imzbf/md-editor-v3) |
| 封装 | `admin-web/src/components/KoiMarkdownEditor/Index.vue` |
| 使用场景 | 文章抽屉 `PostFormDrawer` 的正文字段 |
| 预览主题 | `preview-theme="github"`、`code-theme="github"` |
| 暗色模式 | 跟随 `globalStore.isDark`，`theme="dark" \| "light"` |
| 语言 | 工具栏 `language="zh-CN"` |

### 选型理由

- Vue 3 原生，与 `admin-web` 技术栈一致，维护活跃。
- 内置工具栏、实时预览、目录、代码块、表格、页内全屏/全屏，接入成本低（`v-model` 绑定字符串即可）。
- 未选用 Vditor（体积与集成偏重）、Milkdown（插件体系复杂）、Toast UI Editor（维护节奏一般）。

### 当前能力

- 工具栏已启用 **图片上传**（`purpose=content`），上传后插入 Markdown 图片语法；接口见 [资源管理 API](../admin/assets.md)。
- 多语言 Tab 下每个语言 Tab 各有一个编辑器实例，需传入唯一 `editor-id`（如 `post-content-zh-cn`）。

### 相关代码（管理端）

| 路径 | 说明 |
|------|------|
| `admin-web/package.json` | 依赖 `md-editor-v3` |
| `admin-web/src/components/KoiMarkdownEditor/Index.vue` | 全局组件封装 |
| `admin-web/src/views/content/posts/components/PostFormDrawer.vue` | 文章表单 |

## 公开站：渲染

| 项 | 选型 |
|----|------|
| 解析 | [marked](https://marked.js.org/) |
| 安全 | [DOMPurify](https://github.com/cure53/DOMPurify) |
| 样式 | [github-markdown-css](https://github.com/sindresorhus/github-markdown-css)（`.markdown-body`） |
| 集成方式 | 静态资源放入 `static/resources/`，封装 `markdown-render.js` 供 Tera 模板调用 |

### 相关代码（公开站）

| 路径 | 说明 |
|------|------|
| `static/resources/js/markdown-render.js` | `renderMarkdown`、`stripMarkdown` |
| `static/resources/js/marked.min.js` | Markdown 解析 |
| `static/resources/js/purify.min.js` | HTML 消毒 |
| `static/resources/css/github-markdown.min.css` | 阅读样式 |
| `templates/post-detail.html.tera` | 文章详情页 |
| `templates/posts.html.tera` | 列表页摘要与链接 |
| `src/routes/pages.rs` | `GET /<lang>/posts/<key>`（数字 ID 或 SEO slug） |
| `static/resources/css/site.css` | `.markdown-body`、`.post-detail-*` 样式 |

### 数据流

```
post_i18n.content (Markdown)
  → GET /api/posts/:id?lang=
  → marked.parse (GFM)
  → DOMPurify.sanitize
  → 注入 .markdown-body 容器
```

### 推荐落地步骤

1. **静态资源**（与 jQuery 一样本地 vendoring，避免公开页依赖外网 CDN）：

   ```
   static/resources/css/github-markdown.min.css
   static/resources/js/marked.min.js
   static/resources/js/purify.min.js
   static/resources/js/markdown-render.js
   ```

2. **封装 `markdown-render.js`**（示意）：

   ```javascript
   window.renderMarkdown = function (markdown, container) {
     if (!markdown || !container || typeof marked === "undefined" || typeof DOMPurify === "undefined") {
       return;
     }
     container.classList.add("markdown-body");
     container.innerHTML = DOMPurify.sanitize(
       marked.parse(markdown, { gfm: true, breaks: true })
     );
   };
   ```

3. **文章详情页**：`GET /<lang>/posts/<key>`，`<key>` 可为文章 ID 或该语言 SEO slug（如 `/zh-cn/posts/测试测试`）；模板 `templates/post-detail.html.tera`，调用 `GET /api/posts/:id?lang=` 并执行 `renderMarkdown`（仅展示 `status = 1` 的已发布文章）。

4. **列表页摘要**（`templates/posts.html.tera`）：优先使用 `description`；若无摘要，通过 `stripMarkdown(content)` 去标记后截断；列表项链接至详情页。

5. **样式微调**：`static/resources/css/site.css` 中 `.markdown-body` 与 `.post-detail-*` 与站点 token 对齐。

### 不推荐方案

| 方案 | 原因 |
|------|------|
| 公开页使用 md-editor-v3 的 `MdPreview` | 需引入 Vue，与 jQuery 公开站架构冲突 |
| 服务端 Rust `pulldown-cmark` 输出 HTML | SEO 更好，但样式仍需单独维护，与后台预览对齐成本高；可作为后续优化项 |
| 列表/详情直接输出未消毒 HTML | XSS 风险 |

## 依赖版本（升级时对照）

升级主版本或更换库时，须同步更新本文档，并手测「后台编辑 → 保存 → 公开阅读」链路。

| 用途 | 包名 | 当前版本 | 所在位置 |
|------|------|----------|----------|
| 后台编辑 | `md-editor-v3` | ^6.5.3 | `admin-web/package.json` |
| 公开解析 | `marked` | 15.0.7 | `static/resources/js/marked.min.js` |
| 公开消毒 | `dompurify` | 3.2.4 | `static/resources/js/purify.min.js` |
| 公开样式 | `github-markdown-css` | 5.8.1 | `static/resources/css/github-markdown.min.css` |
| 公开封装 | — | — | `static/resources/js/markdown-render.js` |

## 阶段状态

| 阶段 | 内容 | 状态 |
|------|------|------|
| P0 | 后台 `KoiMarkdownEditor` + 文章抽屉接入 | ✅ |
| P1 | 公开站静态资源 + `markdown-render.js` | ✅ |
| P2 | 文章详情页 + Markdown 渲染 | ✅ |
| P3 | 列表摘要去 MD 标记 | ✅ |
| P4 | 图片上传与编辑器 `image` 工具栏 | ❌ |

## 相关文档

- [管理后台 UI 约定](./admin-ui.md)
- [多语言数据模型 · 文章](./i18n-data-model.md#文章)
- [管理接口 · 文章](./admin/posts.md)
- [公开接口 · 文章](./public/posts.md)
