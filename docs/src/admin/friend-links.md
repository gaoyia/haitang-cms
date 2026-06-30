# 友情链接 API

管理端路径前缀：`/api/admin/friend-links`（需 Bearer Token）。

公开接口：`GET /api/friend-links`，返回已启用的友链列表。

## 数据模型

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | number | 主键 |
| `title` | string | 展示名称（图片 alt） |
| `url` | string | 跳转链接，须 `http://` 或 `https://` |
| `image_url` | string | 友链图片公开 URL |
| `sort` | number | 排序，升序 |
| `status` | number | `0` 禁用，`1` 启用 |

友链图片须使用资源库 `purpose=friend_link` 上传的图片，或引用其公开 URL。

## 管理端接口

### 列表

```
GET /api/admin/friend-links?page=1&page_size=10
```

### 详情

```
GET /api/admin/friend-links/{id}
```

### 创建

```
POST /api/admin/friend-links
Content-Type: application/json

{
  "title": "Rust",
  "url": "https://www.rust-lang.org/",
  "image_url": "/static/uploads/seed/1/friend-link-rust.jpg",
  "sort": 0,
  "status": 1
}
```

### 更新

```
PUT /api/admin/friend-links/{id}
```

### 删除

```
DELETE /api/admin/friend-links/{id}
```

## 公开站渲染

`site_page_context` 注入 `friend_links`，页脚模板 `base.html.tera` 在版权栏上方展示友链图片网格。

## 种子数据

development 启动时会幂等写入 8 条示例友链（对应 `static/uploads/seed/1/friend-link-*.jpg`），并将图片入库为 `purpose=friend_link` 资源。
