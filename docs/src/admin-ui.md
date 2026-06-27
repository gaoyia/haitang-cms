# 管理后台 UI 约定

本文档描述 `admin-web` 管理后台中与菜单、标签等相关的 UI 约定。API 字段与路由说明见 [管理接口](./admin-api.md)。

---

## 菜单 icon 字段展示规则

菜单项、路由 meta、标签页等处的 `icon` 字段为**字符串**，前端通过全局组件 `KoiGlobalIcon` 统一渲染。解析逻辑位于 `admin-web/src/utils/iconDisplay.ts` 中的 `parseIconDisplay`。

### 判定顺序

按以下顺序依次判断（**前面的规则优先**）：

| 优先级 | 条件 | 展示方式 | 说明 |
|--------|------|----------|------|
| 1 | 字符串为空或仅空白 | 不展示 | 组件不渲染任何内容 |
| 2 | 含有 `http://` 或 `https://` | 图片 | 使用 `<img>`，`src` 为完整 URL |
| 3 | 以英文字母（`A–Z` / `a–z`）开头 | 图标名 | 见下文「图标名」 |
| 4 | 以上均不满足 | Emoji / 文本 | 原样输出字符（如 emoji、符号） |

> URL 判定优先于字母规则，避免 `https://...` 被误判为图标名。

### 图标名（字母开头）

以 `koi-` 开头的名称加载 `admin-web/src/assets/icons/` 下的本地 SVG（如 `koi-home`、`koi-document`）。

不以 `koi-` 开头、且符合字母开头规则的名称，按 **Element Plus 图标组件名** 解析（如 `Setting`、`QuestionFilled`）。

本地 SVG 前缀由配置项 `SVG_PREFIX`（当前为 `koi-`）决定，须与 `assets/icons` 下文件名一致。

**无效图标名不展示**：`koi-*` 名称须在 `assets/icons/` 中存在对应 `.svg` 文件；Element Plus 名称须为已注册的组件名。否则视为无图标，标签页等处不渲染、不占位（由 `hasIconDisplay` 判定）。

### 示例

| `icon` 字段值 | 类型 | 效果 |
|---------------|------|------|
| `koi-home` | 图标名 | 本地 SVG「首页」 |
| `koi-tree` | 无效图标名 | 本地无此 SVG，不显示、不占位 |
| `Setting` | 图标名 | Element Plus 设置图标 |
| `https://example.com/icon.png` | 图片 | 加载远程图片 |
| `📄` | Emoji | 显示文档 emoji |
| `🏠` | Emoji | 显示房屋 emoji |
| *(空)* | 空 | 不显示图标 |

### 使用范围

以下位置均通过 `KoiGlobalIcon` 消费同一套规则，无需单独处理：

- 侧边栏 / 横向 / 分栏菜单（`AsideSubMenu`、`HorizontalSubMenu`、`ColumnSubMenu`）
- 顶栏标签页（Tabs）
- 面包屑、菜单搜索等

### 后端与配置

- 菜单 CRUD 接口中的 `icon` 字段存储上述字符串，后端不做类型校验，由前端按规则渲染。
- 字典项 `site_logo` 等 Logo 地址使用独立解析逻辑（`resolveSiteLogoUrl`），**不适用**本节的菜单 icon 规则。

---

## 相关代码

| 路径 | 说明 |
|------|------|
| `admin-web/src/utils/iconDisplay.ts` | `parseIconDisplay`、`hasIconDisplay` |
| `admin-web/src/utils/localIcons.ts` | 本地 SVG 文件名白名单 |
| `admin-web/src/components/KoiGlobalIcon/Index.vue` | 统一渲染组件 |
| `admin-web/src/config/index.ts` | `SVG_PREFIX` 本地 SVG 前缀 |
