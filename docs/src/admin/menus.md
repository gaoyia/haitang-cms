# 菜单

```
GET /api/admin/menu-groups
POST /api/admin/menu-groups
PUT /api/admin/menu-groups/:id
DELETE /api/admin/menu-groups/:id

GET /api/admin/menus/overview?lang=zh-cn
GET /api/admin/menus?group_id=1&lang=zh-cn
GET /api/admin/menus/item/:id?lang=zh-cn
POST /api/admin/menus
PUT /api/admin/menus/:id
DELETE /api/admin/menus/:id
```

创建/更新 body 支持 `lang` 字段指定文案语言；结构字段（`parent_id`、`sort`、`icon` 等）存于 meta。多语言 `title` / `path` 须分语言多次 PUT，每次同时传 `title` 与 `path` 避免空路径覆盖。

`admin_sidebar` 菜单组（id=0）为内置只读；公开站菜单组如 `site_header`、`site_footer` 可编辑。
