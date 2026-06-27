# 菜单

## 菜单组

```
GET /api/admin/menu-groups
GET /api/admin/menu-groups/:id
POST /api/admin/menu-groups
PUT /api/admin/menu-groups/:id
DELETE /api/admin/menu-groups/:id
```

创建/更新 body 字段：`name`、`code`、`description`、`sort`、`status`。响应含 `readonly` 字段；`admin_sidebar`（id=0）为内置只读，列表中虚拟展示，不可增删改。

## 菜单项

```
GET /api/admin/menus/overview?lang=zh-cn
GET /api/admin/menus?group_id=1&lang=zh-cn
GET /api/admin/menus/item/:id?lang=zh-cn
POST /api/admin/menus
PUT /api/admin/menus/:id
DELETE /api/admin/menus/:id
```

| 接口 | 说明 |
|------|------|
| `overview` | 全部菜单组及其菜单树 |
| `menus?group_id=` | 指定组的菜单树；`group_id=0` 返回内置后台侧边栏树 |
| `menus/item/:id` | 单个菜单项（扁平视图） |

创建 body 字段：`group_id`、`parent_id`、`title`、`path`、`icon`、`permission`、`sort`、`status`、`lang`。

更新 body 字段同上，均为可选。多语言 `title` / `path` 须分语言多次 PUT，每次同时传 `title` 与 `path`，避免空路径覆盖。

删除时若仍有子菜单，接口返回 400。

## 当前用户导航

```
GET /api/admin/nav?code=admin_sidebar
```

按当前用户权限过滤后台侧边栏菜单树。目前仅支持 `code=admin_sidebar`；公开站菜单组（如 `site_header`）请使用 `menus?group_id=` 接口。

`admin_sidebar` 菜单组为内置只读；公开站菜单组如 `site_header`、`site_footer` 可编辑。
