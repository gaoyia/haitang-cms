# 公开接口

公开接口无需授权，用于前端页面数据展示。多语言约定见 [概述](./overview.md#多语言) 与 [多语言数据模型](./i18n-data-model.md)。

---

## 获取字典映射

**请求**

```
GET /api/dicts/map?lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，语言码 |

---

## 获取文章列表

**请求**

```
GET /api/posts?lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，语言码 |

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `id` | 文章逻辑 ID |
| `title` | 当前语言标题 |
| `content` | 当前语言正文 |
| `route_path` | 当前语言完整路径 |
| `lang` | 实际使用的语言码 |

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

---

## 获取分类列表

**请求**

```
GET /api/categories?lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选 |

---

## 获取轮播图列表

按轮播图组 `code` 返回已启用的条目，按 `sort` 升序。公开首页（`/<lang>/`）仅将 `home_banner` 组的图片用作 Hero 背景；无数据时使用默认渐变背景。

**请求**

```
GET /api/banners?code=home_banner
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `code` | query | `string` | 轮播图组 code，如 `home_banner` |

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `title` | 标题 |
| `image_url` | 图片地址 |
| `link_url` | 跳转链接，可为空 |
| `description` | 描述文案 |
| `sort` | 排序 |
| `status` | `1` 启用 |
