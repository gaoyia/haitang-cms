# 获取轮播图列表

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
