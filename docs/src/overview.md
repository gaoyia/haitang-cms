# 海棠 CMS API 概述

海棠 CMS 是一个基于 Rocket + Toasty ORM + SQLite 的内容管理系统。

## 基础信息

| 项目 | 值 |
|------|-----|
| 基础地址 | `http://127.0.0.1:9000` |
| 数据格式 | JSON |
| 授权方式 | Bearer Token (JWT) |

## 接口分类

| 分类 | 路径前缀 | 授权 | 说明 |
|------|---------|------|------|
| 公开接口 | `/api` | 无需 | 面向前端页面的只读接口 |
| 管理接口 | `/api/admin` | 需要 Token | 后台管理 CRUD 接口 |

## 通用响应格式

所有接口均返回统一的 JSON 结构：

```json
{
    "code": 0,
    "message": "ok",
    "data": {}
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | `i32` | 状态码，`0` 表示成功，非 `0` 表示错误 |
| `message` | `string` | 状态描述 |
| `data` | `object / null` | 响应数据，失败时为 `null` |

## 多语言

公开读接口支持查询参数 `lang`（如 `zh-cn`、`en-us`）。未传时使用字典项 `site_default_locale` 作为 fallback。公开 Tera 页面使用 URL 前缀 `/<lang>/...`（如 `/en-us/posts`）。数据模型见 [多语言数据模型](./i18n-data-model.md)。
