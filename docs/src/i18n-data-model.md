# 多语言数据模型

海棠 CMS 采用 **Meta + I18n** 双表模式：与语言无关的结构/配置存 `*_meta`，文案与 SEO 路径存 `*_i18n`（联合主键 `(entity_id, lang)` 或 `(code, lang)`）。

## 约定

| 项 | 说明 |
|----|------|
| 语言码 | 小写 BCP 47，如 `zh-cn`、`en-us` |
| 全局字典 | `dict_value.lang = ""`（空串哨兵） |
| 默认语言 | 字典项 `site_default_locale`，缺翻译时 fallback |
| URL | `route_path` 存完整路径，如 `/zh-cn/posts/hello`；**同一 `(lang, route_path)` 非空时全局唯一**（管理端写入校验） |
| 开发库重置 | 删除 `db/haitang.sqlite` 后 `cargo run` 自动建表；**development** 下每次启动会幂等写入种子（字典、菜单、轮播图、admin 等）；**production** 仅在库中尚无用户时写入一次初始数据。启动时 `db_patch` 在任何环境下都会为已有库补列（如 `assets.upload_name`、`post_i18ns.meta_json` 等），与种子无关。首次安装且尚无文章时会写入预制内容：新闻「站点上线公告」、画廊「春日花园」、招聘「Rust 后端工程师」（加入我们分类，`recruitment` 模板）、关于我们「关于海棠 CMS」（`about` 模板） |

## 表结构

### 字典

| 表 | 主键 | 说明 |
|----|------|------|
| `dict_meta` | `code` | `label`、`translatable`、`sort` |
| `dict_value` | `(code, lang)` | `value` |

### 分类

| 表 | 主键 | 说明 |
|----|------|------|
| `category_meta` | `id` | `sort`、`list_template`、`detail_template` |
| `category_i18n` | `(category_id, lang)` | `name`、`description`、`route_path` |

| 字段 | 说明 |
|------|------|
| `list_template` | 分类归档页模板：`none`（无列表，归档 URL 返回 404）、`default`、`gallery`、`recruitment` 或 `about` |
| `detail_template` | 该分类下文章详情模板：`default`、`gallery`、`recruitment` 或 `about` |
| `route_path` | 分类归档 SEO 完整路径，如 `/zh-cn/categories/gallery`；空串时公开 URL 使用数字 ID |

公开归档页：`GET /<lang>/categories/<key>`，`<key>` 为分类 ID 或 slug，按 `list_template` 渲染 `category-list`、`gallery-list`、`recruitment-list` 或 `about-list`。

### 文章

| 表 | 主键 | 说明 |
|----|------|------|
| `post_meta` | `id` | `category_id`、`status`、`created_at`、`updated_at`、`published_at`、`publish_time`、`display_time`、`pinned` |
| `post_i18n` | `(post_id, lang)` | `title`、`description`、`content`（Markdown 源码）、`route_path`、`tags`、`meta_json` |

时间字段（均在 `post_meta`，Unix 秒）：

| 字段 | 说明 |
|------|------|
| `created_at` | 创建时间，写入后不变 |
| `updated_at` | 除「仅改状态」外的任意内容变更时刷新（分类、翻译、`display_time`、`publish_time` 等） |
| `published_at` | 首次实际公开时间；未到计划发布时间或未发布过为 0 |
| `publish_time` | 计划发布时间；`status = 1` 且留空时等于保存时刻；到达该时间后访客可见 |
| `display_time` | 前台展示时间，可手动编辑；留空保存时使用服务端当前时间；列表按置顶优先、同组内按此字段降序 |

| `pinned` | 是否置顶：`0` 否，`1` 是；列表排序等价于 `ORDER BY pinned DESC, display_time DESC` |

`post_i18n.meta_json`：JSON 对象字符串，默认 `{}`；招聘模板可存 `salary`、`location`、`employment_type`、`department`；关于我们模板可存 `highlight`、`founded`、`location`、`contact`（详情页「联系我们」区块展示地址，并预留地图空位）。公开 API 与 `PostView` 按请求语言返回对应 i18n 行的 `meta_json`。

> 历史版本曾将 `meta_json` 存于 `post_meta`；`db_patch` 启动时会将其回填至各语言 `post_i18n` 行并清空 `post_meta.meta_json`。

正文 Markdown 的编辑与公开渲染选型见 [Markdown 内容选型](./markdown.md)。

### 资源

| 表 | 主键 | 说明 |
|----|------|------|
| `assets` | `id` | `storage_key`、`original_name`、`upload_name`、`mime_type`、`size`、`purpose`（`cover` \| `content` \| `banner` \| `friend_link` \| `attachment`）、`created_at` |
| `post_assets` | `(post_id, asset_id)` | `role`（`cover` \| `attachment`）、`sort_order` |
| `banner_assets` | `(banner_id, asset_id)` | `role`（`image`）、`sort_order` |

封面全语言共用，每篇最多 `post_cover_max` 条（字典，默认 3）`role=cover`；附件可多条；轮播图每条最多一张 `role=image`，并同步 `banners.image_url`。详见 [资源管理 API](./admin/assets.md)。

### 菜单

| 表 | 主键 | 说明 |
|----|------|------|
| `menu_item_meta` | `id` | `group_id`、`parent_id`、`icon`、`permission`、`sort`、`status` |
| `menu_item_i18n` | `(menu_item_id, lang)` | `title`、`route_path` |

首次安装种子会创建 **4 个默认分类**：新闻（`news`）、画廊（`gallery`，gallery 模板）、加入我们（`join`）、关于我们（`about`）；后两者暂用 default 模板。`site_header` / `site_footer` 菜单组链到首页与上述分类 SEO 路径；页脚另含管理后台链接。

> 后台侧栏（`admin_sidebar`）仍为内置 + 前端 i18n 键，不走上述 i18n 表。

## 公开页面 URL

| 路径 | 说明 |
|------|------|
| `/` | 302 重定向至 `/{site_default_locale}/` |
| `/<lang>/` | 首页 |
| `/<lang>/rss` | 全站文章 RSS 2.0 订阅（`application/rss+xml`） |
| `/<lang>/posts` | 302 重定向至 `/<lang>/`（保留旧链接兼容） |
| `/<lang>/posts/<key>` | 文章详情（`<key>` 为 ID 或 SEO slug，Markdown 渲染） |
| `/<lang>/categories/<key>` | 分类归档（default / gallery 模板） |
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
