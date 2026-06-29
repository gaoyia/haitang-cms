# 分类

## 获取分类列表

**请求**

```
GET /api/categories?lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选 |

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `id` | 分类 ID |
| `name` | 当前语言名称 |
| `description` | 当前语言描述 |
| `sort` | 排序 |
| `route_path` | 当前语言 SEO 完整路径 |
| `list_template` | 列表模板：`default` / `gallery` |
| `detail_template` | 详情模板：`default` / `gallery` |

---

## 获取单个分类

**请求**

```
GET /api/categories/:id?lang=zh-cn
```

---

## 按 SEO 路径查询

**请求**

```
GET /api/categories/by-path?lang=zh-cn&path=gallery
```

`path` 可为 slug 或完整 `route_path`。

---

## 分类归档页（HTML）

```
GET /<lang>/categories/<key>
```

| 项 | 说明 |
|----|------|
| `<key>` | 分类数字 ID 或当前语言 SEO slug |
| 模板 | 按 `list_template` 渲染 `category-list`（default）或 `gallery-list`（gallery） |
| 数据 | 页面内请求 `GET /api/categories/:id` 与 `GET /api/posts?category_id=` |

数字 ID 访问且已配置 `route_path` 时，会 **301** 重定向到 canonical URL。slug 冲突时返回 **409**。
