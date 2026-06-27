# 用户

## 获取用户列表

```
GET /api/admin/users?page=1&page_size=10
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `page` | query | `i64` | 可选，页码 |
| `page_size` | query | `i64` | 可选，每页条数 |

返回 [分页响应](../overview.md#分页响应)，`list` 元素含 `id`、`username`、`nickname`、`email`、`status`、`role_ids`。

## 获取用户详情

```
GET /api/admin/users/:id
```

## 创建用户

```
POST /api/admin/users
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `username` | `string` | 用户名 |
| `password` | `string` | 明文密码（后端 bcrypt 存储） |
| `nickname` | `string` | 可选，默认与 username 相同 |
| `email` | `string` | 可选 |

## 更新用户

```
PUT /api/admin/users/:id
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `nickname` | `string` | 可选 |
| `email` | `string` | 可选 |
| `status` | `i64` | 可选，`0` 禁用，`1` 启用 |
| `password` | `string` | 可选，重置密码 |

## 分配角色

```
PUT /api/admin/users/:id/roles
```

**请求体**

```json
{
    "role_ids": [1, 2]
}
```

覆盖式分配：先清除旧关联，再写入 `role_ids`。

## 删除用户

```
DELETE /api/admin/users/:id
```

同时删除用户与角色的关联记录。
