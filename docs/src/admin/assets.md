# 资源管理

上传资源统一入库 `assets` 表，文件写入可配置存储后端（默认本地 `static/uploads/`）。与文章、轮播图的关联分别通过 `post_assets`、`banner_assets` 表维护。

## 上传资源

```
POST /api/admin/assets?purpose=cover|content|banner|attachment
Content-Type: multipart/form-data
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `purpose` | query | `string` | 必填：`cover`（封面图，仅图片）、`content`（正文插图，仅图片）、`banner`（轮播图，仅图片）、`attachment`（附件，含图片、视频、文档、压缩包等） |
| `post_id` | query | `i64` | 可选；与 `role` 同时提供时上传后自动关联到文章 |
| `role` | query | `string` | 可选：`cover` 或 `attachment`（文章） |
| `banner_id` | query | `i64` | 可选；与 `banner_role` 同时提供时上传后自动关联到轮播图 |
| `banner_role` | query | `string` | 可选：`image` |
| `file` | body | file | 文件字段名须为 `file` |

上传时会根据 Content-Type 与文件魔数（[`infer`](https://crates.io/crates/infer)）解析 MIME，并推断扩展名。磁盘路径格式为 `{YYYY-MM-DD}/{user_id}/{uuid_v7}.{ext}`（UTC 日期目录 + 上传用户 ID + UUID v7 文件名）；`assets.original_name` 为存储文件名（与 `storage_key` 末段一致）；`assets.upload_name` 为用户上传时的原始文件名（去除路径与控制字符，保留中文等 Unicode，最长 255 字符，不用于磁盘路径）。

启动时除 `push_schema` 建表外，还会执行增量补丁（如为已有库 `ALTER TABLE` 补 `upload_name` 列）；无需手动删库。

**响应** `AssetView`：

```json
{
  "id": 1,
  "storage_key": "2026-06-27/1/019582a3f7a4786890a1b2c3d4e5f6789.png",
  "original_name": "019582a3f7a4786890a1b2c3d4e5f6789.png",
  "upload_name": "cover-photo.jpg",
  "mime_type": "image/jpeg",
  "size": 102400,
  "purpose": "cover",
  "url": "/static/uploads/2026-06-27/1/019582a3f7a4786890a1b2c3d4e5f6789.png",
  "created_at": 1719000000,
  "ref_count": 0
}
```

## 资源列表

```
GET /api/admin/assets?purpose=&keyword=&page=1&page_size=10
```

| 参数 | 说明 |
|------|------|
| `purpose` | 可选，筛选用途 |
| `keyword` | 可选，匹配原文件名或存储文件名 |
| `page` / `page_size` | 分页 |

## 资源详情

```
GET /api/admin/assets/:id
```

## 彻底删除资源

```
DELETE /api/admin/assets/:id
```

无任意 `post_assets` / `banner_assets` 引用时可删除 DB 记录与存储对象；仍被引用时返回 **400**。

## 文章资源

### 获取文章封面与附件

```
GET /api/admin/posts/:id/assets
```

响应：

```json
{
  "covers": [{ "...AssetView" }],
  "cover_max": 3,
  "attachments": [{ "...AssetView" }]
}
```

### 关联资源到文章

```
POST /api/admin/posts/:id/assets
```

```json
{
  "asset_id": 1,
  "role": "cover",
  "sort_order": 0
}
```

- `cover`：可多条，按 `sort_order` 排序；上限由字典 **`post_cover_max`** 控制（默认 `3`，全局值，非多语言）。
- `attachment`：可多条，按 `sort_order` 排序；支持图片、视频、压缩包、Office 文档与纯文本；`asset.purpose` 须与 `role` 匹配。

### 全量更新封面排序

```
PUT /api/admin/posts/:id/assets/covers/order
```

```json
{
  "asset_ids": [2, 1]
}
```

`asset_ids` 须包含当前全部已关联封面 ID，数组顺序即新的 `sort_order`（从 0 起）。

### 全量更新附件排序

```
PUT /api/admin/posts/:id/assets/attachments/order
```

```json
{
  "asset_ids": [3, 1, 2]
}
```

`asset_ids` 须包含当前全部已关联附件 ID，数组顺序即新的 `sort_order`（从 0 起）。

### 解除关联

```
DELETE /api/admin/posts/:id/assets/:asset_id?purge=false
```

| 参数 | 说明 |
|------|------|
| `purge=false`（默认） | 仅删除 `post_assets` 行，文件保留在资源库 |
| `purge=true` | 解除本文关联后，若该资源无其他引用，则彻底删除 |

## 轮播图资源

### 获取轮播图图片

```
GET /api/admin/banners/:id/assets
```

响应：

```json
{
  "image": { "...AssetView" }
}
```

### 关联资源到轮播图

```
POST /api/admin/banners/:id/assets
```

```json
{
  "asset_id": 1,
  "role": "image",
  "sort_order": 0
}
```

- 同一轮播图仅保留一条 `role=image`，新关联会替换旧关联。
- 关联成功后会同步更新 `banners.image_url`，公开页与 API 仍通过 `image_url` 输出。
- `banner_assets.enabled`：`1` 展示图片，`0` 停用（保留关联，`image_url` 同步为空）。

### 启用/停用图片

```
PUT /api/admin/banners/:id/assets/image-enabled
```

```json
{ "enabled": false }
```

### 解除关联

```
DELETE /api/admin/banners/:id/assets/:asset_id?purge=false
```

参数含义与文章资源相同。

## 种子资源

首次启动时，会自动：

1. 默认轮播图文件位于 `static/uploads/seed/{admin_user_id}/banner-1.png`（仓库内已提交 `seed/1/` 示例，对应首次创建的 admin 用户 ID）。
2. 在 `assets` 表写入一条 `purpose=banner` 的记录（`storage_key` 与文件路径一致）。
3. 为 `home_banner` 组下的默认轮播图条目建立 `banner_assets` 关联，并同步 `banners.image_url` 为 `/static/uploads/seed/1/banner-1.png` 等公开 URL。

若轮播图已存在但未关联资源，启动时会自动补关联。

相册种子（`gallery-1.jpg` … `gallery-6.jpg`，与 admin 用户 ID 同目录）：

1. 首次安装且 `post_metas` 为空时，写入两篇预制文章：新闻「站点上线公告 / Site Launch Announcement」（分类 `news`）与画廊「春日花园 / Spring Garden」（分类 `gallery`）。
2. 画廊文章：`gallery-1.jpg` 入库为 `purpose=cover` 并关联封面；`gallery-2` … `gallery-6` 入库为 `purpose=attachment` 并关联附件。

## 文件类型图标

无缩略图预览的附件（PDF、Office、压缩包等）在管理后台展示 SVG 图标。

| 项 | 说明 |
|----|------|
| **源目录（Git 维护）** | `admin-web/public/fileicon/*.svg` |
| **运行时目录（构建产物）** | `static/fileicon/*.svg` |
| **公开 URL** | `/static/fileicon/{slug}.svg`（如 `/static/fileicon/pdf.svg`） |
| **未知类型** | `unknow-file.svg` |

### 同步方式

`cargo build` / `cargo run` 时，仓库根目录 `build.rs` 会将 `admin-web/public/fileicon/` 复制到 `static/fileicon/`，与 Rocket `/static` 挂载一致。管理前端 `pnpm dev` / `pnpm build` 时，Vite 插件也会执行相同同步，便于仅跑前端开发时后端静态目录仍可用。

前后台应统一使用 `/static/fileicon/…` 引用，不要 duplicate 到其它路径。

### 常用 slug 与扩展名

| slug | 典型扩展名 |
|------|------------|
| `pdf` | pdf |
| `doc` | doc, docx |
| `xls` / `xlsx` | xls / xlsx |
| `ppt` | ppt, pptx |
| `zip` / `rar` / `7z` | zip / rar / 7z |
| `txt` | txt |
| `md` | md |
| `video` / `mov` / `flv` | mp4、webm 等 / mov / flv |
| `unknow-file` | 未匹配时的兜底 |

完整文件列表以 `admin-web/public/fileicon/` 目录为准；新增图标时文件名即为 slug（不含 `.svg`），并在前端 `assetFileIcon.ts` 的扩展名映射中补充（若需 MIME 兜底）。

## 环境变量

| 变量 | 默认 | 说明 |
|------|------|------|
| `STORAGE_BACKEND` | `local` | 存储后端；`aliyun` / `tencent` 预留 |
| `STORAGE_LOCAL_DIR` | `static/uploads` | 本地存储目录 |
| `STORAGE_PUBLIC_PREFIX` | `/static/uploads` | 公开访问 URL 前缀 |
| `STORAGE_MAX_BYTES` | `10485760` | 单文件大小上限（字节） |

## 权限

| 权限码 | 说明 |
|--------|------|
| `asset:read` | 查看资源 |
| `asset:create` | 上传 |
| `asset:delete` | 删除 |
