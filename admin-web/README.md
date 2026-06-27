# 海棠 CMS 管理后台

基于 Vue 3 + Vite + Element Plus 的后台管理框架，对接海棠 CMS Rust 后端（`/api/admin/*`）。

## 技术栈

- Vue 3.5 + Vite 8 + TypeScript
- Element Plus 2.x（全量引入）
- Pinia + vue-router 5 + axios
- Sass + UnoCSS + vue-i18n
- 主题：运行时 CSS 变量（支持主色切换、明暗模式、多种布局）

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发服务器（端口 5174，代理 /dev-api → 后端 9000）
pnpm dev

# 类型检查
pnpm type-check

# 生产构建
pnpm build
```

需同时启动后端：

```bash
# 项目根目录
cargo run
```

默认管理员：`admin` / `admin123`

## 目录说明

| 目录 | 说明 |
|------|------|
| `src/layouts/` | 主布局（6 种布局模式 + Tags-View） |
| `src/views/login/` | 登录页（左图右表单） |
| `src/views/home/` | 首页示例 |
| `src/views/placeholder/` | 业务页占位与示例 |
| `src/utils/theme*.ts` | 主题切换逻辑 |
| `src/styles/` | 全局样式与布局 CSS 变量 |
| `src/api/system/auth.ts` | 登录 / 用户信息 API |

## 致谢

本项目的布局架构、主题切换机制与登录界面设计，参考了 [KOI-UI](https://gitee.com/KoiKite/koi-ui) 开源项目。

KOI-UI 采用 [MIT License](https://opensource.org/licenses/MIT) 许可，Copyright (c) 2025 于心。

感谢 KOI-UI 作者提供的优秀后台框架参考实现。
