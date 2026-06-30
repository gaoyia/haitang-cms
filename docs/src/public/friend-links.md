# 友情链接

```
GET /api/friend-links
```

返回已启用友链列表，按 `sort` 升序。

## 响应示例

```json
{
  "code": 0,
  "message": "success",
  "data": [
    {
      "title": "Rust",
      "url": "https://www.rust-lang.org/",
      "image_url": "/static/uploads/seed/1/friend-link-rust.jpg"
    }
  ]
}
```

公开站页脚亦通过服务端模板直接渲染，无需前端单独请求。
