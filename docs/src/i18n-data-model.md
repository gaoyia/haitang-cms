# 多语言数据模型

海棠 CMS 采用 **Meta + I18n** 双表模式：与语言无关的结构/配置存 `*_meta`，文案与 SEO 路径存 `*_i18n`（联合主键 `(entity_id, lang)` 或 `(code, lang)`）。

## 约定

| 项 | 说明 |
|----|------|
| 语言码 | 小写 BCP 47，如 `zh-cn`、`en-us` |
| 全局字典 | `dict_value.lang = ""`（空串哨兵） |
| 默认语言 | 字典项 `site_default_locale`，缺翻译时 fallback |
| URL | `route_path` 存完整路径，如 `/zh-cn/posts/hello`；**同一 `(lang, route_path)` 非空时全局唯一**（管理端写入校验） |
| 开发库重置 | 删除 `db/haitang.sqlite` 后 `cargo run` 自动建表并种子；默认轮播图位于 `static/uploads/seed/1/banner-1.png`，入库 `storage_key = seed/1/banner-1.png` |

## 表结构

### 字典

| 表 | 主键 | 说明 |
|----|------|------|
| `dict_meta` | `code` | `label`、`translatable`、`sort` |
| `dict_value` | `(code, lang)` | `value` |

### 分类

| 表 | 主键 | 说明 |
|----|------|------|
| `category_meta` | `id` | `sort` |
| `category_i18n` | `(category_id, lang)` | `name`、`description` |

### 文章

| 表 | 主键 | 说明 |
|----|------|------|
| `post_meta` | `id` | `category_id`、`status` |
| `post_i18n` | `(post_id, lang)` | `title`、`description`、`content`（Markdown 源码）、`route_path`、`tags` |

正文 Markdown 的编辑与公开渲染选型见 [Markdown 内容选型](./markdown.md)。

### 资源

| 表 | 主键 | 说明 |
|----|------|------|
| `assets` | `id` | `storage_key`、`original_name`、`upload_name`、`mime_type`、`size`、`purpose`（`cover` \| `content` \| `banner` \| `attachment`）、`created_at` |
| `post_assets` | `(post_id, asset_id)` | `role`（`cover` \| `attachment`）、`sort_order` |
| `banner_assets` | `(banner_id, asset_id)` | `role`（`image`）、`sort_order` |

封面全语言共用，每篇最多 `post_cover_max` 条（字典，默认 3）`role=cover`；附件可多条；轮播图每条最多一张 `role=image`，并同步 `banners.image_url`。详见 [资源管理 API](./admin/assets.md)。

### 菜单

| 表 | 主键 | 说明 |
|----|------|------|
| `menu_item_meta` | `id` | `group_id`、`parent_id`、`icon`、`permission`、`sort`、`status` |
| `menu_item_i18n` | `(menu_item_id, lang)` | `title`、`route_path` |

> 后台侧栏（`admin_sidebar`）仍为内置 + 前端 i18n 键，不走上述 i18n 表。

## 公开页面 URL

| 路径 | 说明 |
|------|------|
| `/` | 302 重定向至 `/{site_default_locale}/` |
| `/<lang>/` | 首页 |
| `/<lang>/posts` | 文章列表 |
| `/<lang>/posts/<key>` | 文章详情（`<key>` 为 ID 或 SEO slug，Markdown 渲染） |
| `/<lang>/about` | 关于页 |

`lang` 须为字典项 `site_locales` 中的语言码（如 `zh-cn`、`en-us`）；无效语言会重定向到默认语言首页。

---

## API 语言参数

公开与管理读接口支持查询参数 `lang`（如 `GET /api/dicts/map?lang=en-us`）。未传时使用 `site_default_locale`。

## 实现位置

| 模块 | 路径 |
|------|------|
| 语言工具 | `src/models/locale.rs` |
| 字典服务 | `src/models/dict.rs` |
| 分类 | `src/models/category.rs` |
| 文章 | `src/models/post.rs` |
| 菜单 | `src/models/menu_item.rs`、`src/models/menu.rs` |

## 阶段状态

| 阶段 | 内容 | 状态 |
|------|------|------|
| P0 | 字典 meta + value | ✅ |
| P1 | 分类 meta + i18n | ✅ |
| P2 | 文章 meta + i18n | ✅ |
| P3 | 菜单 meta + i18n | ✅ |
| 公开站 `/<lang>/...` 路由 | ✅ |
| 后续 | hreflang 与 SEO 元数据 | ⚠️ 待办 |
