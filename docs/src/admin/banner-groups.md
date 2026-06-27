# 轮播图组

## 获取轮播图组列表

```
GET /api/admin/banner-groups
```

返回全部组，按 `sort` 升序。不分页。

## 获取轮播图组详情

```
GET /api/admin/banner-groups/:id
```

## 创建轮播图组

```
POST /api/admin/banner-groups
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `string` | 组名称 |
| `code` | `string` | 唯一标识，公开接口 `GET /api/banners?code=` 使用 |
| `description` | `string` | 可选，描述 |
| `sort` | `i64` | 可选，排序 |
| `status` | `i64` | 可选，`0` 禁用，`1` 启用 |

## 更新轮播图组

```
PUT /api/admin/banner-groups/:id
```

字段与创建相同，均为可选。

## 删除轮播图组

```
DELETE /api/admin/banner-groups/:id
```

组下仍有轮播图时返回 400。
