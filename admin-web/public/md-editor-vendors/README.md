# Markdown 编辑器静态依赖（md-editor-v3）

本目录存放 **管理后台 Markdown 编辑器**（`md-editor-v3`）在浏览器端按需加载的第三方脚本与样式。

这些文件**不是**海棠 CMS 业务代码，而是编辑器插件的运行时依赖，与 `public/fileicon/` 等资源同属前端静态资产。

## 为何放在 public

`md-editor-v3` 默认从 unpkg CDN 动态注入上述文件。Edge 等浏览器的**跟踪防护**会拦截第三方 CDN，导致代码高亮、公式、图表等功能异常。改为同源静态路径可避免该问题，且无需在 `package.json` 中安装仅用于 UMD 注入的包。

## 文件清单

| 文件 | 用途 |
|------|------|
| `highlight.min.js` + `styles/github*.css` | 代码块语法高亮（与编辑器 `code-theme="github"` 一致） |
| `katex.min.js` / `katex.min.css` | 行内/块级公式 |
| `mermaid.min.js` | Mermaid 图 |
| `echarts.min.js` | 图表代码块 |
| `cropper.min.js` / `cropper.min.css` | 图片裁剪上传 |
| `prettier.standalone.js` / `prettier-markdown.js` | Markdown 格式化 |
| `screenfull.js` | 编辑器全屏（`screenfull@5.2.0` UMD） |

版本与 `md-editor-v3@6.5.3` 内置 unpkg 地址对齐。

## 更新方式

升级 `md-editor-v3` 后，在 `admin-web` 目录执行：

```bash
pnpm run sync:md-editor-vendors
```

然后检查 `scripts/syncMdEditorVendors.mjs` 中的版本号是否与新版 md-editor 默认 CDN 一致，并同步更新 `src/plugins/mdEditorConfig.ts` 中的路径映射。

## 配置入口

运行时 URL 重写：`admin-web/src/plugins/mdEditorConfig.ts`（在 `main.ts` 最早引入）。
