# 海棠 CMS 待办清单

本文件记录计划中的功能与改进项。状态标记与 [工程原则](.cursor/rules/工程哲学.mdc) 一致：

| 标记 | 含义 |
|------|------|
| ✅ | 已完成 |
| ⚠️ | 部分实现 / 待验收 |
| 💤 | 暂缓 |
| ⏭️ | 延期 |
| ❌ | 未开始 |
| 🚫 | 已放弃 |

---

## 内容 · 文章

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ❌ | 文章置顶 | 文章支持置顶；分类/列表中置顶项排在非置顶之前 | `post_meta` 增加 `pinned` / `pin_sort`；排序：置顶 → `sort` / 发布时间；管理端开关；公开 API 与 Tera 同步 | 同分类内置顶始终最前；取消置顶后恢复默认顺序 |
| ✅ | 文章 Meta + I18n 多语言 | 同一逻辑 ID 下按语言维护标题、摘要、正文、SEO 路径、标签 | `post_meta` + `post_i18n`；管理端 `PostFormDrawer` 语言 Tab；公开 `/<lang>/posts/<key>` 与 `?lang=` API；`pick_i18n_row` fallback | 中英文可分别编辑与访问；文档 P2 ✅（见 `docs/src/i18n-data-model.md`） |
| ❌ | 文章 HTML 编辑器 | 除 Markdown 外支持富文本 / HTML 正文 | `content_format`（`markdown` \| `html`）；管理端接入 HTML 编辑器；公开站 DOMPurify 消毒渲染 | 后台可保存 HTML 文章；公开页正确渲染且无 XSS |
| ✅ | 文章 `meta_json` 迁入多语言 | 招聘/关于模板字段（薪资、联系方式等）按语言独立存储 | `post_i18n.meta_json`；`db_patch` 从 `post_metas` 回填；管理端各语言 Tab 独立 `recruitmentMeta`/`aboutMeta`；公开 API 按 `lang` 返回 | 各语言 Tab 可独立维护模板扩展字段；公开 API 按 `lang` 返回对应 `meta_json` |

---

## 轮播 · Banner

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ✅ | Hero 多语言文案 | 轮播图 `meta_json` 按语言码存储角标、标题、描述、标签与按钮；公开首页随语言切换 Hero | `banner_meta.rs` 解析与 fallback；管理端 `BannerItemDrawer` 按语言 Tab 编辑；公开 API / Tera 轮播切换；种子与文档 | 中英文 Hero 可分别配置；切换语言后首页文案正确；格式 `让…如<em>「…」</em>…` 见管理端提示 |

---

## SEO

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ❌ | 网站地图（Sitemap） | 生成标准站点地图供搜索引擎抓取 | 路由 `/sitemap.xml` 或按语言分片；收录首页、分类、文章、静态页；`hreflang` / sitemap index | XML 符合协议；主要公开 URL 均可列出 |
| ❌ | 站点级 SEO 字典 | 全站默认 SEO 元信息 | 字典项如 `site_seo_description`、`site_seo_keywords`；多语言 value；`base.html.tera` 注入 `<meta>` | 未单独配置的页面 fallback 全局字典；中英文可分别维护 |
| ❌ | 页面级 SEO | 文章、分类、关于等可覆盖站点默认 SEO | 按语言存描述、关键词；可选 OG / Twitter Card；优先级：页面 > 分类 > 站点全局 | 单篇可设独立 description/keywords；公开页 `<head>` 输出正确 |

---

## 路由 · URL

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ❌ | 路由转发表 | 短路径或别名 URL 映射到实际内容地址 | 数据表维护「源路径 → 目标路径」（按语言）；例：`/zh-cn/about` → `/zh-cn/posts/about-haitang-cms`；管理端 CRUD；Rocket 在页面路由前匹配（301 重定向或内部改写待定）；与菜单 / 分类 / 文章 `route_path` 冲突检测；Sitemap 输出规范 URL | 配置后访问源路径可到达目标内容；改删映射即时生效；无与现有路由冲突 |
| ❌ | 删除写死的 `/about` 页面 | 移除 `GET /<lang>/about` 与 `templates/about.html.tera`；「关于我们」改由文章 `/posts/about-haitang-cms`（about 模板）承载 | **依赖路由转发表完成后实施**，以便旧链接 `/zh-cn/about` 等先通过转发表映射；同步更新首页 Hero、轮播种子、菜单示例与文档中的 `/about` 引用；保留 about 分类/详情模板（`about-list`、`about-detail`） | `/about` 路由与静态模板已删除；转发表可访问旧 URL；关于内容仅来自 CMS 文章 |

> **路由 · URL 实施顺序**：先完成 **路由转发表**，再执行 **删除写死的 `/about` 页面**（避免外链与菜单在过渡期 404）。

---

## AI · Agent SDK

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ❌ | Agent SDK 接入与配置 | 对接外部 Agent API，服务端统一代理 | 字典配置 API Base URL、API Key（仅服务端）；Rocket 转发（鉴权、限流、日志、脱敏） | 管理端可配置；请求经 `/api/admin/...` 转发成功，前端不暴露密钥 |
| ❌ | AI Agent · 问答 | 基于站点或选中上下文的内容问答 | 依赖 SDK 接入；管理端或公开端场景待定 | 可发起问答并返回可用结果 |
| ❌ | AI Agent · 翻译 | 文章或多语言字段一键翻译 | 源语言 Tab → 目标语言 Tab | 翻译结果写入对应语言字段 |
| ❌ | AI Agent · 续写 | 在光标处或文末续写正文 | 与 Markdown / HTML 编辑器集成 | 续写内容可插入编辑器 |
| ❌ | AI Agent · 选中段落编辑 | 选中文字后改写、扩写、润色 | 与编辑器选区 API 集成 | 选中段落后可提交编辑指令并替换内容 |

> 上表 AI 能力（问答 / 翻译 / 续写 / 段落编辑）均依赖 **Agent SDK 接入与配置** 先行完成。

---

## 微信

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ❌ | 微信网页分享 SDK | 微信内分享自定义标题、描述、缩略图 | 服务端 JSSDK 签名接口；AppID / AppSecret 存服务端；前端 `updateAppMessageShareData` 等；文案可联动页面 SEO | 微信内置浏览器分享卡片信息符合预期 |

---

## 站点 · 其他

| 状态 | 功能 | 目标 | 待定点 | 验收 |
|------|------|------|--------|------|
| ✅ | 友情链接 | 公开站页脚展示友链，管理端 CRUD | `FriendLink` 模型；管理 API `/api/admin/friend-links`；公开 API `/api/friend-links`；页脚 `base.html.tera`；管理页 `friend-links/index`（含拖拽排序、资源库选图）；development 种子 8 条 | 后台可维护；公开页页脚展示启用友链；文档见 `docs/src/admin/friend-links.md` |

---

## 维护说明

| 操作 | 说明 |
|------|------|
| 交付完成 | 将对应行「状态」改为 ✅，并在提交或 PR 中注明 |
| 放弃范围 | 改为 🚫 并简短说明原因 |
| 大型条目 | 实施前先在 Issue 或设计文档补充数据模型与 API 草案 |
