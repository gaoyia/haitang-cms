# 字典

## 获取字典 meta 列表

```
GET /api/admin/dicts?lang=zh-cn&page=1&page_size=10
```

| 参数 | 位置 | 类型 | 说明 |
|------|------|------|------|
| `lang` | query | `string` | 可选，列表「当前值」预览语言；缺省为站点默认语言 |
| `page` | query | `i64` | 可选，页码，默认 1 |
| `page_size` | query | `i64` | 可选，每页条数，默认 10，上限 100 |

返回 [分页响应](../overview.md#分页响应)，`list` 元素为 `DictMetaListView`（`code`、`label`、`description`、`translatable`、`sort`、**`preview_value`**）。非多语言项的 `preview_value` 为全球值；多语言项为 `lang` 指定语言下的值（无则 fallback 至默认语言）。

## 获取字典详情（meta + values）

```
GET /api/admin/dicts/:code
```

路径参数为字典 **code**（非数字 id）。`values` 为 `lang → value` 映射。

## 创建字典

```
POST /api/admin/dicts
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | `string` | 唯一标识 |
| `label` | `string` | 后台显示名 |
| `translatable` | `bool` | 是否多语言 |
| `description` | `string` | 可选，描述 |
| `sort` | `i64` | 可选，排序 |
| `value` | `string` | 初始 value |
| `lang` | `string` | 可选，多语言项的语言码 |

## 更新字典 meta

```
PUT /api/admin/dicts/:code
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `label` | `string` | 可选 |
| `description` | `string` | 可选 |
| `translatable` | `bool` | 可选 |
| `sort` | `i64` | 可选 |

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

删除 meta 及全部 value 行。
