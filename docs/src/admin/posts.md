# 文章

`content` 字段为 **Markdown 源码**，存储与渲染约定见 [Markdown 内容选型](../markdown.md)。

## 获取文章列表

```
GET /api/admin/posts?lang=zh-cn&page=1&page_size=10
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，合并指定语言文案 |
| `page` | query | `i64` | 可选，页码 |
| `page_size` | query | `i64` | 可选，每页条数 |

返回 [分页响应](../overview.md#分页响应)，`list` 元素为 `PostView`。

## 获取文章详情

```
GET /api/admin/posts/:id
```

返回 `PostDetailView`，含 `translations` 对象（各语言文案），以及 `covers`（数组）、`attachments`（数组）。资源关联见 [资源管理](./assets.md)。

## 文章资源

```
GET /api/admin/posts/:id/assets
POST /api/admin/posts/:id/assets
DELETE /api/admin/posts/:id/assets/:asset_id?purge=
```

详见 [资源管理](./assets.md)。

## 创建文章

```
POST /api/admin/posts
```

**请求体**

```json
{
    "title": "新文章标题",
    "content": "文章内容",
    "description": "摘要",
    "tags": "Rust, CMS",
    "category_id": 1,
    "lang": "zh-cn",
    "route_path": "/zh-cn/posts/hello",
    "status": 1
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `title` | `string` | 是 | 指定语言的标题 |
| `content` | `string` | 否 | 正文 |
| `description` | `string` | 否 | 摘要 |
| `tags` | `string` | 否 | 该语言下的标签，逗号分隔；保存时会规范化空白与分隔符 |
| `category_id` | `i64` | 否 | 分类 ID |
| `lang` | `string` | 否 | 默认 `site_default_locale` |
| `route_path` | `string` | 否 | 该语言的 SEO 完整路径或 slug；空串合法，非空时须匹配 `/{lang}/posts/{slug}`，且**同一语言下不可与其他文章重复** |
| `status` | `i64` | 否 | `0` 草稿，`1` 已发布 |

## 更新文章

```
PUT /api/admin/posts/:id
```

可同时更新 `post_meta`（`category_id`、`status`）与指定 `lang` 的 i18n 字段（含 `tags`）。字段与创建接口相同，均为可选。

## 删除文章

```
DELETE /api/admin/posts/:id
```

删除 meta 及全部 i18n 行。
