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

返回 [分页响应](../overview.md#分页响应)，`list` 元素含 `group_id`、`group_name`、`title`、`image_url`、`link_url`、`description`、`sort`、`status`。

## 获取轮播图详情

```
GET /api/admin/banners/item/:id
```

## 创建轮播图

```
POST /api/admin/banners
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `group_id` | `i64` | 所属轮播图组 ID |
| `title` | `string` | 标题 |
| `image_url` | `string` | 可选，图片地址 |
| `link_url` | `string` | 可选，跳转链接 |
| `description` | `string` | 可选，描述 |
| `sort` | `i64` | 可选，排序 |
| `status` | `i64` | 可选，`0` 禁用，`1` 启用 |

## 更新轮播图

```
PUT /api/admin/banners/:id
```

字段与创建相同，均为可选。

## 删除轮播图

```
DELETE /api/admin/banners/:id
```
