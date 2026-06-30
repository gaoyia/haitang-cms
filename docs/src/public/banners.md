# 获取轮播图列表

按轮播图组 `code` 返回已启用的条目，按 `sort` 升序。公开首页（`/<lang>/`）将 `home_banner` 组的图片与 Hero 文案用于首屏；切换轮播时文案同步切换。无数据时使用默认渐变背景与静态文案。

**请求**

```
GET /api/banners?code=home_banner&lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `code` | query | `string` | 轮播图组 code，如 `home_banner` |
| `lang` | query | `string` | 可选，语言码；省略时使用站点默认语言 |

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `title` | 管理端标题 |
| `image_url` | 图片地址 |
| `link_url` | 整图点击跳转，可为空 |
| `sort` | 排序 |
| `hero` | 当前语言解析后的 Hero 文案对象 |

**`hero` 对象**

| 字段 | 说明 |
|------|------|
| `badge` | 角标 |
| `title` | 主标题（可含 `<em>`） |
| `description` | 描述 |
| `tags` | 标签字符串数组 |
| `actions` | 按钮数组，每项含 `label`、`url`、`variant`（`primary` / `secondary`） |

原始多语言数据存于管理端 `meta_json`，公开接口按 `lang` 解析并回退至默认语言。
