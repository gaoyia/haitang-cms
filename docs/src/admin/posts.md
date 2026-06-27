# 文章

## 创建文章

**请求**

```
POST /api/admin/posts
```

**请求体**

```json
{
    "title": "新文章标题",
    "content": "文章内容",
    "lang": "zh-cn",
    "route_path": "/zh-cn/posts/hello"
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `title` | `string` | 是 | 指定语言的标题 |
| `lang` | `string` | 否 | 默认 `site_default_locale` |
| `route_path` | `string` | 否 | 该语言的完整 URL 路径 |

## 更新文章

```
PUT /api/admin/posts/:id
```

可同时更新 `post_meta`（`category_id`、`tags`、`status`）与指定 `lang` 的 i18n 字段。

## 获取文章详情（管理端）

```
GET /api/admin/posts/:id
```

返回 `PostDetailView`，含 `translations` 对象（各语言文案）。

## 删除文章

```
DELETE /api/admin/posts/:id
```

删除 meta 及全部 i18n 行。
