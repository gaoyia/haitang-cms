# 字典

## 获取字典 meta 列表

```
GET /api/admin/dicts
```

## 获取字典详情（meta + values）

```
GET /api/admin/dicts/:code
```

路径参数为字典 **code**（非数字 id）。

## 创建字典

```
POST /api/admin/dicts
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | `string` | 唯一标识 |
| `label` | `string` | 后台显示名 |
| `translatable` | `bool` | 是否多语言 |
| `value` | `string` | 初始 value |
| `lang` | `string` | 可选，多语言项的语言码 |

## 更新字典 meta

```
PUT /api/admin/dicts/:code
```

## 更新字典 values

```
PUT /api/admin/dicts/:code/values
```

**请求体**

```json
{
    "values": {
        "zh-cn": "海棠 CMS",
        "en-us": "Haitang CMS"
    }
}
```

## 删除字典

```
DELETE /api/admin/dicts/:code
```
