# 管理接口

管理接口需要 JWT Token 授权。请在请求头中携带：

```
Authorization: Bearer <token>
```

多语言数据模型见 [多语言数据模型](./i18n-data-model.md)。读接口支持 `?lang=`；写接口可通过 body 中 `lang` 指定更新的语言行。

---

## 登录授权

获取 JWT Token。

**请求**

```
POST /api/admin/login
Content-Type: application/json
```

**请求体**

```json
{
    "username": "admin",
    "password": "admin123"
}
```

> Token 有效期为 24 小时，过期后需重新登录。

---

## 字典

### 获取字典 meta 列表

```
GET /api/admin/dicts
```

### 获取字典详情（meta + values）

```
GET /api/admin/dicts/:code
```

路径参数为字典 **code**（非数字 id）。

### 创建字典

```
POST /api/admin/dicts
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | `string` | 唯一标识 |
| `label` | `string` | 后台显示名 |
| `translatable` | `bool` | 是否多语言 |
| `value` | `string` | 初始 value |
| `lang` | `string` | 可选，多语言项的语言码 |

### 更新字典 meta

```
PUT /api/admin/dicts/:code
```

### 更新字典 values

```
PUT /api/admin/dicts/:code/values
```

**请求体**

```json
{
    "values": {
        "zh-cn": "海棠 CMS",
        "en-us": "Haitang CMS"
    }
}
```

### 删除字典

```
DELETE /api/admin/dicts/:code
```

---

## 文章

### 创建文章

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

### 更新文章

```
PUT /api/admin/posts/:id
```

可同时更新 `post_meta`（`category_id`、`tags`、`status`）与指定 `lang` 的 i18n 字段。

### 获取文章详情（管理端）

```
GET /api/admin/posts/:id
```

返回 `PostDetailView`，含 `translations` 对象（各语言文案）。

### 删除文章

```
DELETE /api/admin/posts/:id
```

删除 meta 及全部 i18n 行。

---

## 分类

```
GET /api/admin/categories
POST /api/admin/categories
GET /api/admin/categories/:id
PUT /api/admin/categories/:id
DELETE /api/admin/categories/:id
```

创建时写入 `category_meta` 与默认语言 `category_i18n`；详情含 `translations`。

---

## 菜单

```
GET /api/admin/menu-groups
POST /api/admin/menu-groups
PUT /api/admin/menu-groups/:id
DELETE /api/admin/menu-groups/:id

GET /api/admin/menus/overview?lang=zh-cn
GET /api/admin/menus?group_id=1&lang=zh-cn
GET /api/admin/menus/item/:id?lang=zh-cn
POST /api/admin/menus
PUT /api/admin/menus/:id
DELETE /api/admin/menus/:id
```

创建/更新 body 支持 `lang` 字段指定文案语言；结构字段（`parent_id`、`sort`、`icon` 等）存于 meta。多语言 `title` / `path` 须分语言多次 PUT，每次同时传 `title` 与 `path` 避免空路径覆盖。

`admin_sidebar` 菜单组（id=0）为内置只读；公开站菜单组如 `site_header`、`site_footer` 可编辑。
