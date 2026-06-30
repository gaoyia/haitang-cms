# 轮播图

## 获取轮播图列表

```
GET /api/admin/banners?group_id=1&page=1&page_size=10
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `group_id` | query | `i64` | 可选，按组筛选 |
| `page` | query | `i64` | 可选，页码 |
| `page_size` | query | `i64` | 可选，每页条数 |

返回 [分页响应](../overview.md#分页响应)，`list` 元素含 `group_id`、`group_name`、`title`、`image_url`、`link_url`、`description`、`meta_json`、`sort`、`status`。

`meta_json` 为 JSON 对象字符串，按语言码存储首页 Hero 文案，例如：

```json
{
  "zh-cn": {
    "badge": "让文字「如风流动」🎐「如星闪耀」✨",
    "title": "让数字内容 <em>如花绽放</em> 🎉",
    "description": "副文案",
    "tags": ["公开 API", "Vue 3"],
    "actions": [
      { "label": "浏览新闻", "url": "/zh-cn/categories/news", "variant": "primary" },
      { "label": "了解更多", "url": "/zh-cn/about", "variant": "secondary" }
    ]
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `group_id` | `i64` | 所属轮播图组 ID |
| `title` | `string` | 管理端标题 |
| `image_url` | `string` | 可选，图片地址 |
| `link_url` | `string` | 可选，整图点击跳转 |
| `description` | `string` | 可选，管理端描述 / Hero 回退文案 |
| `meta_json` | `string` | 可选，多语言 Hero 文案，默认 `{}` |
| `sort` | `i64` | 可选，排序 |
| `status` | `i64` | 可选，`0` 禁用，`1` 启用 |

## 获取轮播图详情

```
GET /api/admin/banners/:id
```

## 创建轮播图

```
POST /api/admin/banners
```

字段见上表。

## 更新轮播图

```
PUT /api/admin/banners/:id
```

字段与创建相同，均为可选。

## 删除轮播图

```
DELETE /api/admin/banners/:id
```
