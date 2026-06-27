# 字典

## 获取全部字典项

返回已按 `lang` 解析 value 的扁平列表。

**请求**

```
GET /api/dicts?lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，语言码 |

**响应字段（节选）**

| 字段 | 说明 |
|------|------|
| `code` | 字典 code |
| `label` | 显示名 |
| `value` | 当前语言解析后的值 |
| `description` | 描述 |
| `sort` | 排序 |

---

## 按 code 获取字典项

**请求**

```
GET /api/dicts/site_name?lang=en-us
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `code` | path | `string` | 字典 code |
| `lang` | query | `string` | 可选 |

---

## 获取字典键值映射

便于前台一次性加载全部字典为 `code → value` 对象。

**请求**

```
GET /api/dicts/map?lang=zh-cn
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，语言码 |

**响应示例**

```json
{
    "code": 0,
    "message": "ok",
    "data": {
        "site_name": "海棠 CMS",
        "site_default_locale": "zh-cn"
    }
}
```
