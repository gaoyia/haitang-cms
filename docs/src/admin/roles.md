# 角色与权限

## 获取角色列表

```
GET /api/admin/roles?page=1&page_size=10
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `page` | query | `i64` | 可选，页码 |
| `page_size` | query | `i64` | 可选，每页条数 |

返回 [分页响应](../overview.md#分页响应)，`list` 元素含 `id`、`name`、`description`、`permissions`（字符串数组）。

## 获取角色详情

```
GET /api/admin/roles/:id
```

## 创建角色

```
POST /api/admin/roles
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `string` | 角色名称 |
| `description` | `string` | 可选，描述 |
| `permissions` | `string[]` | 可选，权限 code 列表 |

## 更新角色

```
PUT /api/admin/roles/:id
```

字段与创建相同，均为可选。

## 删除角色

```
DELETE /api/admin/roles/:id
```

## 获取系统权限列表

```
GET /api/admin/permissions
```

返回按分组聚合的全部可用权限，供角色编辑页勾选：

```json
[
    {
        "group": "文章",
        "permissions": [
            { "code": "post:read", "label": "查看文章" }
        ]
    }
]
```
