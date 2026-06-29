# 管理后台 UI 约定

本文档描述 `admin-web` 管理后台中与菜单、标签等相关的 UI 约定。API 字段与路由说明见 [管理接口](./admin/README.md)。文章正文 Markdown 编辑与公开渲染选型见 [Markdown 内容选型](./markdown.md)。

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

- 侧边栏 / 横向 / 分栏菜单（`AsideSubMenu`、`AsideMenuLeaf`、`HorizontalSubMenu`、`ColumnSubMenu`）
- 顶栏标签页（Tabs）
- 面包屑、菜单搜索等

### 后端与配置

- 菜单 CRUD 接口中的 `icon` 字段存储上述字符串，后端不做类型校验，由前端按规则渲染。
- 字典项 `site_logo` 等 Logo 地址使用独立解析逻辑（`resolveSiteLogoUrl`），**不适用**本节的菜单 icon 规则。

---

## 表单分栏与响应式宽度

表单中**不宜撑满整行**的控件（日期时间选择器、短输入框等），应放在 `el-form-item` 内，用 `el-row` + `el-col` 限制最大宽度，并在列内令控件 `width: 100%`，由列宽决定实际展示宽度。

### 默认分栏规则

与 Element Plus 栅格一致，断点含义如下：

| 断点 | 宽度 | 列 span | 说明 |
|------|------|---------|------|
| `xs` | &lt; 768px | `24` | 小屏占满一行 |
| `sm` | ≥ 768px | — | 常规右侧标签（`label-width: 88px`） |
| `lg` | ≥ 1200px | `8` | 超宽屏三列并排（如文章基本信息） |

标准写法（单项）：

```vue
<el-form-item label="展示时间">
  <el-row>
    <el-col :xs="24" :lg="8">
      <el-date-picker v-model="value" type="datetime" style="width: 100%" />
    </el-col>
  </el-row>
</el-form-item>
```

多项并排（如文章基本信息：分类、展示时间、状态）：

```vue
<el-row :gutter="16">
  <el-col :xs="24" :lg="8">
    <el-form-item label="分类">...</el-form-item>
  </el-col>
  <el-col :xs="24" :lg="8">
    <el-form-item label="展示时间">...</el-form-item>
  </el-col>
  <el-col :xs="24" :lg="8">
    <el-form-item label="状态">...</el-form-item>
  </el-col>
</el-row>
```

`lg="8"` 时三列合计 24 格，最宽屏（≥ 1200px）一行展示；**< 1200px** 时基本信息各字段占满一行。

### 窄屏上下布局

宽度 **< 768px**（与 Element `xs`、Markdown 编辑器紧凑模式一致）时，表单整体切换为**上下布局**：

- `label-position="top"`，标签在控件上方
- 取消固定 `label-width`
- 分栏区域各 `el-col` 使用 `:xs="24" :lg="8"`，窄屏下每字段独占一行

```vue
<el-form
  :label-position="isFormStacked ? 'top' : 'right'"
  :label-width="isFormStacked ? undefined : '88px'"
>
```

`isFormStacked` 使用 `useBreakpoints(breakpointsEnum).smaller('sm')`。

### 约定

- **需要全宽的控件**（标题、正文、Markdown 编辑器等）不套 `el-col`，保持默认占满 `el-form-item` 内容区。
- **短控件**（日期时间、数字、短下拉等）使用上表 `:xs="24" :sm="12" :md="8"`，除非该页有明确布局需求。
- 列内控件统一 `style="width: 100%"`，避免固定像素宽度；外层由 `el-col` 自适应。
- 同一表单内多处使用分栏时，优先复用相同 span，保持视觉一致。

### 参考实现

| 路径 | 说明 |
|------|------|
| `admin-web/src/views/content/posts/components/PostFormDrawer.vue` | 文章基本信息三列并排（`:lg="8"`） |
| `admin-web/src/views/home/index.vue` | 首页统计卡片与面板分栏（`:xs="24" :sm="12" :lg="6"` 等） |

Markdown 编辑器窄屏（宽度 **< 768px**）工具栏折叠为「工具」菜单，见 [Markdown 内容选型](./markdown.md#窄屏适配)。

---

## 整体布局

管理端壳层基于 Koi Admin 模板，主题配置抽屉（顶栏齿轮）可选 **纵向 / 分栏 / 经典 / 混合 / 横向** 五种桌面布局，配置写入 `globalStore.layout` 并持久化到 `localStorage`。

| 断点 | 行为 |
|------|------|
| **≥ 768px** | 按主题配置渲染对应桌面布局壳；显示顶部分页标签栏 |
| **< 768px** | 切换为 `LayoutMobile`（顶栏汉堡 + 抽屉菜单）；**隐藏**标签栏 |

**路由内容不重挂载**：`<Main>` 通过 `Teleport` 挂到各布局壳内的 `LayoutMainMount` 节点，切换窄屏/桌面或主题布局壳时 Main 实例不断。

其他与宽度相关的逻辑：

| 断点 | 行为 |
|------|------|
| **< 1200px** | 侧栏自动折叠（`isCollapse`）；折叠宽度 **64px**（与 Element Plus 一致，见 `settings.asideMenuCollapseWidth`）；文章基本信息等 `:lg="8"` 字段改为单列 |
| **< 768px** | 业务表单 `label-position="top"` 等，见上文窄屏约定 |

| 路径 | 说明 |
|------|------|
| `admin-web/src/layouts/index.vue` | 布局入口，`Teleport` 挂载 `<Main />` |
| `admin-web/src/layouts/components/LayoutMainMount.vue` | 各布局壳内 Main 挂载点 |
| `admin-web/src/layouts/useLayoutMainMount.ts` | 挂载点注册（防壳切换竞态） |
| `admin-web/src/layouts/LayoutMobile/` | 窄屏顶栏与抽屉 |
| `admin-web/src/layouts/components/ThemeConfig/` | 布局样式与界面配置 |
| `admin-web/src/stores/modules/global.ts` | `layout`、`isCollapse`、`menuWidth` 等 |
| `admin-web/src/hooks/screen/index.ts` | `breakpointsEnum`（`sm` = 767px） |

---

## 纵向 / 经典侧栏菜单

**纵向**（`LayoutVertical`）与**经典**（`LayoutClassic`）共用 `AsideSubMenu` + `AsideMenuLeaf`，由 `el-menu` 的 `:collapse="globalStore.isCollapse"` 与组件 `:collapse` 同步控制展开/折叠 DOM。

### 组件职责

| 组件 | 职责 |
|------|------|
| `AsideSubMenu.vue` | 递归渲染子菜单（`el-sub-menu`）并分发叶子项 |
| `AsideMenuLeaf.vue` | 顶级/叶子路由项（`el-menu-item`、点击跳转） |
| `types.ts` | `AsideMenuItem` 菜单树节点类型 |

分栏（`ColumnSubMenu`）、横向（`HorizontalSubMenu`）、窄屏抽屉仍使用各自组件，**不适用**本节插槽约定。

### Element Plus 折叠插槽约定

须与 [Element Plus Menu 折叠行为](https://element-plus.org/zh-CN/component/menu.html) 一致，避免用 CSS 隐藏双份 DOM：

| 状态 | 子菜单 `#title` | 叶子项 |
|------|-----------------|--------|
| **展开**（`collapse=false`） | `menu-title-wrap`：图标 + 文案 | 默认插槽内 `menu-title-wrap`（图标 + 文案），**不使用** `#title` |
| **折叠**（`collapse=true`） | 仅 `KoiGlobalIcon` | 默认插槽：图标；`#title`：文案（供 tooltip） |

折叠时的 tooltip 由 Element Plus 提供，**不再**在 `#title` 外包 `el-tooltip`。

### 样式

- 折叠窄栏全局样式：`admin-web/src/styles/element.scss` 中 `.el-menu--vertical.el-menu--collapse`
- 布局壳内边距：折叠时 `LayoutVertical` / `LayoutClassic` 的 `:has(.el-menu--collapse)` 去掉左右 `menu-pad` 留白
- 叶子项激活态（背景、左侧指示条）：`AsideMenuLeaf.vue`

### 手测验收清单

| 场景 | 预期 |
|------|------|
| 宽度 ≥ 1200px，纵向或经典布局 | 每项仅 **一个** 图标；文案与图标同行 |
| 宽度 < 1200px 或手动折叠 | 子菜单与叶子项仅显示居中图标；悬停显示 tooltip 标题 |
| 点击叶子项（资源、字典等） | 路由跳转正常；激活项有高亮与左边框 |
| 点击子菜单 | 弹出二级菜单；图标与文案正常 |

---

## Markdown 编辑器静态依赖

管理后台文章正文使用 [md-editor-v3](https://github.com/imzbf/md-editor-v3)。该库默认从 **unpkg CDN** 按需注入代码高亮、KaTeX、Mermaid 等 UMD 脚本；Edge 等浏览器的跟踪防护会拦截这些请求。

### 方案

| 依赖 | 处理方式 |
|------|----------|
| highlight / katex / mermaid / echarts / cropper / prettier / screenfull | 同源静态文件，目录 `admin-web/public/md-editor-vendors/` |

运行时 URL 重写见 `admin-web/src/plugins/mdEditorConfig.ts`（在 `main.ts` 最早引入）。目录内 `README.md` 说明各文件用途与版本对齐关系。

### 同步与升级

克隆仓库后若缺少 vendor 文件，在 `admin-web` 下执行：

```bash
pnpm run sync:md-editor-vendors
```

升级 `md-editor-v3` 后须核对 `scripts/syncMdEditorVendors.mjs` 中的 unpkg 版本是否与新版默认 CDN 一致，重新同步并提交 `public/md-editor-vendors/`。

---

## 相关代码

| 路径 | 说明 |
|------|------|
| `admin-web/src/layouts/components/Menu/AsideSubMenu.vue` | 纵向/经典侧栏子菜单 |
| `admin-web/src/layouts/components/Menu/AsideMenuLeaf.vue` | 纵向/经典侧栏叶子项 |
| `admin-web/src/layouts/components/Menu/types.ts` | `AsideMenuItem` 类型 |
| `admin-web/src/settings.ts` | `asideMenuCollapseWidth`（64px） |
| `admin-web/src/styles/element.scss` | 折叠窄栏全局样式 |
| `admin-web/src/utils/iconDisplay.ts` | `parseIconDisplay`、`hasIconDisplay` |
| `admin-web/src/utils/localIcons.ts` | 本地 SVG 文件名白名单 |
| `admin-web/src/components/KoiGlobalIcon/Index.vue` | 统一渲染组件 |
| `admin-web/src/config/index.ts` | `SVG_PREFIX` 本地 SVG 前缀 |
| `admin-web/src/components/KoiMarkdownEditor/Index.vue` | 文章正文 Markdown 编辑器（见 [Markdown 内容选型](./markdown.md)） |
| `admin-web/public/md-editor-vendors/` | 编辑器第三方 UMD 静态依赖 |
| `admin-web/src/plugins/mdEditorConfig.ts` | 编辑器 CDN → 本地静态路径 |
| `admin-web/scripts/syncMdEditorVendors.mjs` | 从 unpkg 同步 vendor 文件 |
