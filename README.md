# 海棠 CMS

基于 Rust 的内容管理系统，后端使用 Rocket + Toasty ORM + SQLite。

## 技术栈

| 层级 | 技术 |
|------|------|
| Web 框架 | [Rocket](https://rocket.rs) 0.6.0-dev |
| ORM | [Toasty](https://github.com/tokio-rs/toasty) 0.7.0 |
| 数据库 | SQLite |
| 模板引擎 | Tera (via rocket_dyn_templates) |
| 前端框架 | Vue 3 + Vite (admin-web) |
| UI 组件库 | Element Plus 2.x (admin-web，全量引入) |
| 前端库 | jQuery 4.0.0 (公开首页) |
| 文章正文 | Markdown（后台 md-editor-v3，公开站 marked + DOMPurify，见 [docs](docs/src/markdown.md)） |
| 认证 | JWT (jsonwebtoken) |

## 项目结构

```
haitang-cms/
├── build.rs                 # 构建时将 admin-web/public/fileicon 同步到 static/fileicon
├── src/
│   ├── main.rs              # 应用入口
│   ├── models/              # 数据模型（含 meta + i18n 双表）
│   │   ├── asset.rs         # assets + post_assets
│   ├── storage/             # 可插拔存储（local / OSS 占位）
│   │   ├── mod.rs           # 模块汇总 & re-export
│   │   ├── locale.rs        # 语言码规范与 fallback
│   │   ├── dict_meta.rs / dict_value.rs
│   │   ├── category.rs      # category_meta + category_i18n
│   │   ├── post.rs          # post_meta + post_i18n
│   │   ├── menu_item.rs     # menu_item_meta + menu_item_i18n
│   │   ├── auth.rs          # 认证模型 (JWT Claims)
│   │   └── response.rs      # 通用响应结构
│   ├── routes/              # 路由层
│   │   ├── mod.rs           # 路由汇总
│   │   ├── pages.rs         # 页面路由 (Tera 模板)
│   │   ├── api/             # 公开 API (/api/*)
│   │   │   ├── mod.rs
│   │   │   └── posts.rs
│   │   └── admin/           # 管理 API (/api/admin/*)
│   │       ├── mod.rs
│   │       ├── auth.rs      # 登录接口
│   │       └── posts.rs     # 文章 CRUD
│   └── guards/              # 请求守卫
│       ├── mod.rs
│       └── auth.rs          # AdminAuth JWT 守卫
├── templates/               # Tera 模板 (公开页面)
├── static/                  # 静态资源
│   ├── fileicon/            # 文件类型 SVG（构建自 admin-web/public/fileicon，勿手改）
│   ├── uploads/             # 上传目录；`seed/{admin_id}/` 含默认轮播图
│   └── resources/           # logo、样式、jQuery 等
│       ├── css/site.css     # 公开站点样式（DESIGN.md token）
│       ├── css/github-markdown.min.css
│       ├── logo.svg
│       └── js/
│           ├── markdown-render.js
│           ├── marked.min.js
│           └── purify.min.js
├── admin-web/               # 管理后台前端 (Vue 3)
├── docs/                    # API 文档 (mdbook)
├── vendor/                  # 本地资源（.gitignore 忽略，不进仓库）
│   ├── koi-ui/              # admin-web 参考的 KOI-UI 框架
│   ├── demo/                # 演示资源
│   └── jquery-*.js          # 公开首页 jQuery
├── db/                      # SQLite 数据库文件
├── Cargo.toml
└── Rocket.toml
```

## 路由约定

| 路径前缀 | 用途 | 授权 |
|---------|------|------|
| `/` | 重定向至 `/{默认语言}/` | 无需 |
| `/<lang>/` | 多语言公开首页 | 无需 |
| `/<lang>/rss`、`/<lang>/posts/<id>`（订阅 / 详情）、`/<lang>/about` | 多语言公开页 | 无需 |
| `/api/*` | 公开 API (JSON) | 无需 |
| `/api/admin/*` | 管理 API (JSON) | 需要 Bearer Token |
| `/static/*` | 静态资源 | 无需 |
| `/{ADMIN_WEB_PATH}/*` | 管理后台 SPA（默认 `/haitang-cms-admin/`） | 无需 |

### 公开首页

首页使用 Rocket 的 Tera 模板引擎渲染 HTML 页面，URL 带语言前缀（如 `/zh-cn/`、`/en-us/rss` 为 RSS 订阅）。访问 `/` 会重定向到字典项 `site_default_locale` 对应的首页。模板文件位于 `templates/` 目录。

### 管理后台

`admin-web/` 是基于 Element Plus 的后台管理框架（参考 [KOI-UI](https://gitee.com/KoiKite/koi-ui) MIT），默认海棠红主题，支持明暗模式、多种布局与 Tags-View，调用 `/api/admin/*` 接口。

**开发**（Vite dev server，端口 5174）：

```bash
cd admin-web && pnpm dev
```

开发时无需 `pnpm build`。`cargo run` 启动后端后，访问 `http://127.0.0.1:9000/haitang-cms-admin/`（或 `/haitang-cms-admin/dicts` 等）会由 Rocket **302 重定向**到 Vite dev server（默认 `http://127.0.0.1:5174`）。API 仍走 9000（Vite 将 `/dev-api` 代理到后端）。

**生产构建**（产物输出至 `static/haitang-cms-admin/`，由 Rocket 托管）：

```bash
cd admin-web && pnpm build
# 可选：自定义路径
# VITE_BASE=/my-admin/ VITE_BUILD_OUT_DIR=../static/my-admin pnpm build
```

构建后访问 `http://127.0.0.1:9000/haitang-cms-admin/`。后端路径由环境变量 `ADMIN_WEB_PATH` 控制，须与 `VITE_BASE` 的路径段一致。

## 开发

```bash
# 启动后端
cargo run

# 构建 API 文档
mdbook build docs

在线文档（main 分支 push 后自动部署）：https://gaoyia.github.io/haitang-cms/

# 启动管理后台前端
cd admin-web && pnpm dev
```

### 数据库重置（Schema 变更后）

多语言改造后表结构已变更。开发环境请删除旧库后重启：

```bash
# Windows PowerShell
Remove-Item db/haitang.sqlite -ErrorAction SilentlyContinue
cargo run
```

种子数据会自动写入默认字典、菜单、首页轮播图与用户（**development** 下每次启动幂等补全；**production** 仅在首次安装、库中尚无用户时写入一次）。多语言模型说明见 [docs/src/i18n-data-model.md](docs/src/i18n-data-model.md)。

## 环境变量

| 变量 | 默认值 | 说明 |
|------|-------|------|
| `HAITANG_ENV` | `development` | 运行环境：`development` / `production`；生产部署请显式设为 `production` |
| `JWT_SECRET` | `haitang-cms-dev-secret` | JWT 签名密钥 |
| `ADMIN_WEB_PATH` | `haitang-cms-admin` | 管理后台 SPA URL 路径段（不含斜杠） |
| `ADMIN_WEB_STATIC_DIR` | `static/{ADMIN_WEB_PATH}` | 管理后台构建产物目录（**production** 使用） |
| `ADMIN_WEB_DEV_URL` | `http://127.0.0.1:5174` | **development** 下后台 SPA 重定向目标（Vite dev） |
| `STORAGE_BACKEND` | `local` | 资源存储后端（`aliyun` / `tencent` 预留） |
| `STORAGE_LOCAL_DIR` | `static/uploads` | 本地存储目录 |
| `STORAGE_PUBLIC_PREFIX` | `/static/uploads` | 上传文件公开 URL 前缀 |
| `STORAGE_MAX_BYTES` | `10485760` | 单文件上传大小上限（字节） |

### admin-web 构建环境变量（`.env.production`）

| 变量 | 默认值 | 说明 |
|------|-------|------|
| `VITE_BASE` | `/haitang-cms-admin/` | 部署 base，须与 `ADMIN_WEB_PATH` 对应 |
| `VITE_BUILD_OUT_DIR` | `../static/haitang-cms-admin` | 构建输出目录 |
| `VITE_WEB_BASE_API` | （空） | 生产环境 API 前缀，空表示同源 `/api/...` |

## 默认管理员

| 用户名 | 密码 |
|-------|------|
| admin | admin123 |

> 生产环境请务必修改默认凭据和 JWT 密钥。
