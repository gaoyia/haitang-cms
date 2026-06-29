# 文章

`content` 为 Markdown 源码；公开页渲染方案见 [Markdown 内容选型](../markdown.md)。

## 获取文章列表

**请求**

```
GET /api/posts?lang=zh-cn&category_id=1
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，语言码 |
| `category_id` | query | `i64` | 可选，按分类过滤 |

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `id` | 文章逻辑 ID |
| `title` | 当前语言标题 |
| `content` | 当前语言正文 |
| `route_path` | 当前语言完整路径 |
| `lang` | 实际使用的语言码 |
| `covers` | 封面资源列表（含 `url`） |
| `attachments` | 附件资源列表 |
| `list_template` | 所属分类列表模板 |
| `detail_template` | 所属分类详情模板 |
| `publish_time` | 计划发布时间；仅 `status = 1` 且 `publish_time <= 当前时间` 的文章会出现在列表与详情 API 中 |

---

## 获取文章详情

**请求**

```
GET /api/posts/:id?lang=en-us
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `id` | path | `i64` | 文章 ID |
| `lang` | query | `string` | 可选 |

公开 HTML 详情页：`GET /<lang>/posts/<key>`，`<key>` 为文章 ID 或当前语言 SEO slug（对应 `route_path` 最后一段）；按文章所属分类的 `detail_template` 渲染 `post-detail`（default）或 `gallery-detail`（gallery）。仅展示 `status = 1` 且已到达 `publish_time` 的文章。若用数字 ID 访问且已配置 SEO 路径，会重定向到 canonical URL。若同一语言下 slug 重复（数据异常），公开页返回 **409 Conflict**；管理端创建/更新时会拒绝重复 slug。
