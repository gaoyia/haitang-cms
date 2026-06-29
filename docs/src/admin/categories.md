# 分类

## 获取分类列表

```
GET /api/admin/categories?lang=zh-cn&page=1&page_size=10
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，合并指定语言文案 |
| `page` | query | `i64` | 可选，页码 |
| `page_size` | query | `i64` | 可选，每页条数 |

返回 [分页响应](../overview.md#分页响应)，`list` 元素为 `CategoryView`（含 `route_path`、`list_template`、`detail_template`）。

## 获取分类详情

```
GET /api/admin/categories/:id
```

返回 `CategoryDetailView`，含 `translations` 对象（各语言 `name`、`description`、`route_path`）及 `list_template`、`detail_template`。

## 创建分类

```
POST /api/admin/categories
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `string` | 分类名称 |
| `description` | `string` | 可选，描述 |
| `sort` | `i64` | 可选，排序 |
| `lang` | `string` | 可选，写入的语言行 |
| `list_template` | `string` | 可选，`default` 或 `gallery`，默认 `default` |
| `detail_template` | `string` | 可选，`default` 或 `gallery`，默认 `default` |
| `route_path` | `string` | 可选，SEO 完整路径或 slug，须匹配 `/{lang}/categories/{slug}` |

## 更新分类

```
PUT /api/admin/categories/:id
```

字段与创建相同，均为可选。若该分类下仍有文章，删除返回 400。

## 删除分类

```
DELETE /api/admin/categories/:id
```

删除 meta 及全部 i18n 行。

## 相册栏目配置

详见 [相册栏目操作说明](./category-gallery.md)。
