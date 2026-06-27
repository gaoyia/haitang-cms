# 登录授权

## 登录

获取 JWT Token 及当前用户信息。

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

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `token` | JWT 字符串，有效期 24 小时 |
| `user.id` | 用户 ID |
| `user.username` | 用户名 |
| `user.nickname` | 昵称 |
| `user.roles` | 角色名称列表 |
| `user.permissions` | 权限 code 列表 |

> Token 过期后需重新登录。后续管理接口请在请求头携带 `Authorization: Bearer <token>`。

---

## 获取当前用户

根据 JWT 解析当前用户，角色与权限从数据库实时读取（非 Token 内缓存值）。

**请求**

```
GET /api/admin/me
Authorization: Bearer <token>
```

**响应**：与登录接口中 `user` 字段结构相同。
