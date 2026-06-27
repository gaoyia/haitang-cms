# 登录授权

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
